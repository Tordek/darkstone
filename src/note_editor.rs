use std::clone;

use iced::theme::Custom;

pub struct NoteEditor {
    pub path: std::path::PathBuf,
    display_name: String,
    state: crate::util::Query<InternalState, String>,
}

struct InternalState {
    view_mode: ViewMode,
    content: iced::widget::text_editor::Content,
    preview: Vec<iced::widget::markdown::Item>,
}

enum ViewMode {
    Edit,
    Preview,
}

#[derive(Debug, Clone)]
pub enum Message {
    None(url::Url),
    Edit(iced::widget::text_editor::Action),
    Loaded(Result<String, std::io::ErrorKind>),
    SwitchMode,

    CustomEdit(CustomEdit),
}

#[derive(Debug, Clone)]
enum CustomEdit {
    Bold,
    Italic,
    Title1,
    Title2,
    Title3,
    Title4,
    Title5,
}

impl NoteEditor {
    pub fn from_path(
        path: std::path::PathBuf,
        display_name: String,
    ) -> (Self, iced::Task<Message>) {
        (
            Self {
                path: path.clone(),
                display_name: display_name.clone(),
                state: crate::util::Query::Pending,
            },
            iced::Task::perform(crate::util::read_file(path), Message::Loaded),
        )
    }

    pub fn view(self: &Self) -> iced::Element<'_, Message> {
        match &self.state {
            crate::util::Query::Pending => iced::widget::Text::new("Loading...").into(),
            crate::util::Query::Loaded(InternalState {
                content,
                preview,
                view_mode,
            }) => {
                let main_body: iced::Element<'_, Message> = match view_mode {
                    ViewMode::Edit => iced::widget::TextEditor::new(&content)
                        .style(
                            |theme: &iced::Theme, status| iced::widget::text_editor::Style {
                                border: iced::Border {
                                    width: 0.0,
                                    ..Default::default()
                                },
                                ..iced::widget::text_editor::default(theme, status)
                            },
                        )
                        .padding(0)
                        .height(iced::Length::Fill)
                        .on_action(Message::Edit)
                        .into(),
                    ViewMode::Preview => iced::widget::markdown::view(
                        preview,
                        iced::widget::markdown::Settings::default(),
                        iced::widget::markdown::Style::from_palette(
                            iced::Theme::TokyoNightStorm.palette(),
                        ),
                    )
                    .map(Message::None)
                    .into(),
                };

                iced::widget::column![
                    iced::widget::text(self.display_name.clone()).size(24),
                    iced::widget::container(main_body).height(iced::Length::Fill),
                    iced::widget::text(self.path.to_string_lossy())
                ]
                .spacing(4)
                .padding(8)
                .height(iced::Length::Fill)
                .into()
            }
            crate::util::Query::Error(e) => iced::widget::Text::new(e.clone()).into(),
        }
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Edit(action) => {
                if let crate::util::Query::Loaded(InternalState {
                    content, preview, ..
                }) = &mut self.state
                {
                    content.perform(action);
                    *preview = iced::widget::markdown::parse(&content.text()).collect();
                }
                iced::Task::none()
            }
            Message::Loaded(Ok(contents)) => {
                let content = iced::widget::text_editor::Content::with_text(&contents.clone());
                let preview = iced::widget::markdown::parse(&content.text()).collect();
                self.state = crate::util::Query::Loaded(InternalState {
                    content,
                    preview,
                    view_mode: ViewMode::Edit,
                });
                iced::Task::none()
            }
            Message::Loaded(Err(e)) => {
                self.state = crate::util::Query::Error(format!("Failed to load file: {:?}", e));
                iced::Task::none()
            }
            Message::SwitchMode => {
                if let crate::util::Query::Loaded(InternalState { view_mode, .. }) = &mut self.state
                {
                    *view_mode = match view_mode {
                        ViewMode::Edit => ViewMode::Preview,
                        ViewMode::Preview => ViewMode::Edit,
                    };
                }
                iced::Task::none()
            }
            Message::None(url) => {
                print!("{}", url);
                iced::Task::none()
            }
            Message::CustomEdit(custom_edit) => {
                if let crate::util::Query::Loaded(InternalState {
                    content, preview, ..
                }) = &mut self.state
                {
                    match custom_edit {
                        CustomEdit::Bold => {
                            bold(content);
                        }
                        // CustomEdit::Italic => {
                        //     content.perform(iced::widget::text_editor::Action::Italic);
                        // }
                        // CustomEdit::Title1 => {
                        //     content.perform(iced::widget::text_editor::Action::Title1);
                        // }
                        // CustomEdit::Title2 => {
                        //     content.perform(iced::widget::text_editor::Action::Title2);
                        // }
                        // CustomEdit::Title3 => {
                        //     content.perform(iced::widget::text_editor::Action::Title3);
                        // }
                        // CustomEdit::Title4 => {
                        //     content.perform(iced::widget::text_editor::Action::Title4);
                        // }
                        // CustomEdit::Title5 => {
                        //     content.perform(iced::widget::text_editor::Action::Title5);
                        // }
                        _ => {}
                    }
                    *preview = iced::widget::markdown::parse(&content.text()).collect();
                }
                iced::Task::none()
            }
        }
    }
    pub fn subscription(self: &Self) -> iced::Subscription<Message> {
        match &self.state {
            crate::util::Query::Loaded(InternalState { content, .. }) => {
                iced::keyboard::on_key_press(|key, modifiers| {
                    if modifiers.control() {
                        if key == iced::keyboard::Key::Character("b".into()) && modifiers.control()
                        {
                            Some(Message::CustomEdit(CustomEdit::Bold))
                        } else if key == iced::keyboard::Key::Character("i".into())
                            && modifiers.control()
                        {
                            // TODO: Italic
                            None
                        } else if key == iced::keyboard::Key::Character("1".into())
                            && modifiers.control()
                        {
                            // TODO: Title 1
                            None
                        } else if key == iced::keyboard::Key::Character("2".into())
                            && modifiers.control()
                        {
                            // TODO: Title 2
                            None
                        } else if key == iced::keyboard::Key::Character("3".into())
                            && modifiers.control()
                        {
                            // TODO: Title 3
                            None
                        } else if key == iced::keyboard::Key::Character("4".into())
                            && modifiers.control()
                        {
                            // TODO: Title 4
                            None
                        } else if key == iced::keyboard::Key::Character("5".into())
                            && modifiers.control()
                        {
                            // TODO: Title 5
                            None
                        } else if key == iced::keyboard::Key::Character("p".into())
                            && modifiers.control()
                        {
                            Some(Message::SwitchMode)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
            }
            _ => iced::Subscription::none(),
        }
    }
}

fn bold(content: &mut iced::widget::text_editor::Content) {
    let selection = content.selection();
    let (row, column) = content.cursor_position();
    let text = content.text();
    let line = content.lines().nth(row).unwrap().to_string();

    let mut is_bolded = true;

    if column < 2 {
        is_bolded = false;
    } else {
        if !(line.chars().nth(column - 2).unwrap() == '*'
            && line.chars().nth(column - 1).unwrap() == '*')
        {
            is_bolded = false;
        }
    }

    if let Some(selection_content) = selection.clone() {
        let end = column + selection_content.len();
        println!("{} {}", column, end);
        if end >= text.len() - 1 {
            is_bolded = false;
        } else {
            if !(text.chars().nth(end + 1).unwrap() == '*'
                && text.chars().nth(end + 2).unwrap() == '*')
            {
                is_bolded = false;
            }
        }
    }

    let editings = if is_bolded {
        vec![
            // Delete text
            iced::widget::text_editor::Edit::Delete,
            // Delete head **
            iced::widget::text_editor::Edit::Backspace,
            iced::widget::text_editor::Edit::Backspace,
            // Re-add text
            iced::widget::text_editor::Edit::Paste(selection.unwrap().into()),
            // Delete trailing **
            iced::widget::text_editor::Edit::Delete,
            iced::widget::text_editor::Edit::Delete,
        ]
    } else {
        vec![
            // Delete text
            iced::widget::text_editor::Edit::Delete,
            iced::widget::text_editor::Edit::Paste("**".to_string().into()),
            // Delete **
            iced::widget::text_editor::Edit::Paste(selection.unwrap_or("".to_string()).into()),
            iced::widget::text_editor::Edit::Paste("**".to_string().into()),
        ]
    };
    for editing in editings {
        content.perform(iced::widget::text_editor::Action::Edit(editing));
    }
}
