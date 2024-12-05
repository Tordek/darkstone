use std::io;

pub struct Notes {
    notes: crate::util::Query<Directory, io::ErrorKind>,
    current: Option<crate::note_editor::NoteEditor>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Create,
    Delete(std::path::PathBuf),
    SetCurrent(std::path::PathBuf),
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

fn dir_tree(directory: &Directory) -> iced::Element<'_, Message> {
    let mut note_list = iced::widget::Column::new();
    for file in &directory.files {
        note_list = note_list.push(
            iced::widget::row![
                iced::widget::mouse_area(
                    iced::widget::text(file.display_name.clone()).width(iced::Length::Fill)
                )
                .on_press(Message::SetCurrent(file.path.clone())),
                iced::widget::button(iced::widget::text("x"))
                    .on_press(Message::Delete(file.path.clone())),
            ]
            .padding(iced::padding::left(15)),
        );
    }
    for child in &directory.directories {
        note_list = note_list.push(iced::widget::row![
            iced::widget::mouse_area(iced::widget::row![
                iced::widget::text(if child.expanded { "-" } else { "+" })
                    .height(20)
                    .width(20),
                iced::widget::container(
                    iced::widget::text(child.display_name.clone()).color([0.5, 0.5, 0.5])
                )
                .width(iced::Length::Fill)
                .padding(iced::padding::left(15))
            ])
            .on_press(Message::Expand(child.path.clone(), !child.expanded)),
            iced::widget::button(iced::widget::text("x"))
                .on_press(Message::Delete(child.path.clone()))
        ]);
        if child.expanded {
            note_list = note_list.push(iced::widget::stack![
                iced::widget::container(dir_tree(&child)).padding(iced::padding::left(20)),
                iced::widget::container(iced::widget::vertical_rule(2))
                    .padding(iced::padding::left(10))
            ]);
        }
    }
    note_list.into()
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
            crate::util::Query::Loaded(directory) => iced::widget::scrollable(dir_tree(&directory))
                .height(iced::Length::Fill)
                .into(),
        };

        let sidebar = iced::widget::column![
            note_list,
            iced::widget::horizontal_rule(1),
            iced::widget::container(
                iced::widget::button(iced::widget::text("Create")).on_press(Message::Create)
            )
            .width(iced::Length::Fill)
        ]
        .spacing(4)
        .width(280)
        .padding(8);

        let main_view: iced::Element<'_, Message> = if let Some(current_note) = &self.current {
            crate::note_editor::NoteEditor::view(current_note).map(Message::NoteEditorMessage)
        } else {
            iced::widget::text("No note selected").into()
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
                        display_name,
                    });
                    iced::Task::done(Message::SetCurrent(path))
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
            Message::SetCurrent(path) => {
                let (state, next_task) = crate::note_editor::NoteEditor::from_path(path);
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
}

fn expand(directory: &mut Directory, path: std::path::PathBuf, open: bool) {
    for dir in &mut directory.directories {
        if dir.path == path {
            dir.expanded = open;
        }
        dir.directories
            .iter_mut()
            .map(|d| expand(d, path.clone(), open))
            .collect()
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
