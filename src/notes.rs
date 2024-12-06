use std::io;

use iced::Theme;

pub struct Notes {
    notes: crate::util::Query<Directory, io::ErrorKind>,
    current: Option<crate::note_editor::NoteEditor>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Create,
    Delete(std::path::PathBuf),
    SetCurrent(std::path::PathBuf, String),
    NoteEditorMessage(crate::note_editor::Message),
    LoadFiles(Result<Directory, std::io::ErrorKind>),
    Expand(std::path::PathBuf, bool),
}

#[derive(Debug, Clone)]
struct File {
    display_name: String,
    path: std::path::PathBuf,
}

#[derive(Debug, Clone)]
pub struct Directory {
    display_name: String,
    path: std::path::PathBuf,
    expanded: bool,
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl Notes {
    pub fn new(location: std::path::PathBuf) -> (Self, iced::Task<Message>) {
        (
            Self {
                notes: crate::util::Query::Pending,
                current: None,
            },
            iced::Task::perform(load_files(location), Message::LoadFiles),
        )
    }
    pub fn view<'a>(self: &Self) -> iced::Element<'_, Message> {
        let note_list: iced::Element<'_, Message> = match &self.notes {
            crate::util::Query::Pending => iced::widget::text("Loading...").into(),
            crate::util::Query::Error(e) => iced::widget::text(format!("Error: {:?}", e)).into(),
            crate::util::Query::Loaded(directory) => {
                iced::widget::scrollable(self.dir_tree(&directory))
                    .height(iced::Length::Fill)
                    .into()
            }
        };

        let sidebar = iced::widget::container(
            iced::widget::column![
                iced::widget::container(
                    iced::widget::button(crate::util::icon(crate::util::ICON_EDIT))
                        .style(crate::util::button_secondary)
                        .on_press(Message::Create)
                )
                .center_x(iced::Length::Fill)
                .width(iced::Length::Fill),
                iced::widget::horizontal_rule(1),
                note_list,
            ]
            .spacing(8),
        )
        .style(|theme| iced::widget::container::Style {
            background: Some(theme.extended_palette().background.weak.color.into()),
            ..Default::default()
        })
        .width(280)
        .padding(8);

        let main_view: iced::Element<'_, Message> = if let Some(current_note) = &self.current {
            crate::note_editor::NoteEditor::view(current_note).map(Message::NoteEditorMessage)
        } else {
            iced::widget::container(iced::widget::text("No note selected"))
                .padding(8)
                .into()
        };

        iced::widget::row![sidebar, main_view].into()
    }

    pub fn update(self: &mut Self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Create => {
                if let crate::util::Query::Loaded(directory) = &mut self.notes {
                    let mut display_name = "Untitled".to_string();
                    let mut path = std::path::PathBuf::new()
                        .join(&directory.path)
                        .join(&display_name);
                    let mut i = 1;
                    while path.exists() {
                        display_name = format!("Untitled {}", i);
                        path = std::path::PathBuf::new()
                            .join(&directory.path)
                            .join(&display_name);
                        i = i + 1;
                    }
                    std::fs::File::create(&path).unwrap();
                    directory.files.push(File {
                        path: path.clone(),
                        display_name: display_name.clone(),
                    });
                    iced::Task::done(Message::SetCurrent(path, display_name))
                } else {
                    iced::Task::none()
                }
            }
            Message::Delete(path) => {
                if let crate::util::Query::Loaded(directory) = &mut self.notes {
                    directory.files.retain(|f| f.path != path);
                    std::fs::remove_file(&path).unwrap();
                }
                iced::Task::none()
            }
            Message::SetCurrent(path, display_name) => {
                let (state, next_task) =
                    crate::note_editor::NoteEditor::from_path(path, display_name);
                self.current = Some(state);
                next_task.map(Message::NoteEditorMessage)
            }
            Message::NoteEditorMessage(message) => {
                if let Some(current_note) = &mut self.current {
                    crate::note_editor::NoteEditor::update(current_note, message)
                        .map(Message::NoteEditorMessage)
                } else {
                    iced::Task::none()
                }
            }
            Message::LoadFiles(Ok(directory)) => {
                self.notes = crate::util::Query::Loaded(directory);
                iced::Task::none()
            }
            Message::LoadFiles(Err(e)) => {
                self.notes = crate::util::Query::Error(e);
                iced::Task::none()
            }
            Message::Expand(path, open) => {
                if let crate::util::Query::Loaded(directory) = &mut self.notes {
                    expand(directory, path, open);
                }
                iced::Task::none()
            }
        }
    }
    pub fn subscription(self: &Self) -> iced::Subscription<Message> {
        let current_note_sub = if let Some(current) = &self.current {
            crate::note_editor::NoteEditor::subscription(current).map(Message::NoteEditorMessage)
        } else {
            iced::Subscription::none()
        };

        let notes_panel_subscription = iced::keyboard::on_key_press(|key, modifiers| {
            if key == iced::keyboard::Key::Character("n".into()) && modifiers.control() {
                Some(Message::Create)
            } else {
                None
            }
        });

        iced::Subscription::batch(vec![current_note_sub, notes_panel_subscription])
    }

    fn dir_tree(self: &Self, directory: &Directory) -> iced::Element<'_, Message> {
        let mut note_list = iced::widget::Column::new();
        for child in &directory.directories {
            note_list = note_list.push(iced::widget::row![
                iced::widget::button(iced::widget::row![
                    crate::util::icon(if child.expanded {
                        crate::util::ICON_DOWN_SMALL
                    } else {
                        crate::util::ICON_RIGHT_SMALL
                    }),
                    iced::widget::container(iced::widget::text(child.display_name.clone()).style(
                        |theme: &iced::Theme| iced::widget::text::Style {
                            color: Some(theme.extended_palette().background.strong.text.into()),
                            ..Default::default()
                        }
                    ))
                    .width(iced::Length::Fill)
                    .padding(iced::padding::left(15))
                ])
                .style(crate::util::button_no_bg)
                .on_press(Message::Expand(child.path.clone(), !child.expanded)),
                iced::widget::button(crate::util::icon(crate::util::ICON_DELETE))
                    .style(crate::util::button_no_bg)
                    .on_press(Message::Delete(child.path.clone()))
            ]);
            if child.expanded {
                note_list = note_list.push(iced::widget::stack![
                    iced::widget::container(self.dir_tree(&child)).padding(iced::padding::left(20)),
                    iced::widget::container(iced::widget::vertical_rule(2))
                        .padding(iced::padding::left(10))
                ]);
            }
        }
        for file in &directory.files {
            note_list = note_list.push(
                iced::widget::row![
                    iced::widget::button(
                        iced::widget::text(file.display_name.clone()).width(iced::Length::Fill)
                    )
                    .style(
                        if self
                            .current
                            .as_ref()
                            .map_or(false, |v| v.path == file.path.clone())
                        {
                            crate::util::button_no_bg_active
                        } else {
                            crate::util::button_no_bg
                        }
                    )
                    .on_press(Message::SetCurrent(
                        file.path.clone(),
                        file.display_name.clone()
                    )),
                    iced::widget::button(crate::util::icon(crate::util::ICON_DELETE))
                        .style(crate::util::button_no_bg)
                        .on_press(Message::Delete(file.path.clone())),
                ]
                .padding(iced::padding::left(15)),
            );
        }
        note_list.into()
    }
}

fn expand(directory: &mut Directory, path: std::path::PathBuf, open: bool) {
    for dir in &mut directory.directories {
        if dir.path == path {
            dir.expanded = open;
        }
        expand(dir, path.clone(), open);
    }
}

async fn load_files(path: std::path::PathBuf) -> Result<Directory, std::io::ErrorKind> {
    println!("Loading files from {:?}", path);
    let mut files = vec![];
    let mut directories = vec![];

    let mut entries = tokio::fs::read_dir(path.clone())
        .await
        .map_err(|e| e.kind())?;

    while let Ok(Some(entry)) = entries.next_entry().await {
        let child_path = entry.path();
        let display_name = child_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        if child_path.is_dir() {
            let contents = Box::pin(load_files(child_path.clone())).await?;

            directories.push(Directory {
                display_name,
                expanded: true,
                path: child_path,
                files: contents.files,
                directories: contents.directories,
            });
        } else {
            files.push(File {
                display_name,
                path: child_path,
            });
        }
    }

    Ok(Directory {
        display_name: path.file_name().unwrap().to_string_lossy().to_string(),
        path,
        expanded: true,
        files,
        directories,
    })
}
