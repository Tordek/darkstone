pub struct NoteEditor {
    path: std::path::PathBuf,
    state: crate::util::Query<InternalState, String>,
}

struct InternalState {
    content: iced::widget::text_editor::Content,
    preview: Vec<iced::widget::markdown::Item>,
}

#[derive(Debug, Clone)]
pub enum Message {
    None(url::Url),
    Edit(iced::widget::text_editor::Action),
    Loaded(Result<String, std::io::ErrorKind>),
}

impl NoteEditor {
    pub fn from_path(path: std::path::PathBuf) -> (Self, iced::Task<Message>) {
        (
            Self {
                path: path.clone(),
                state: crate::util::Query::Pending,
            },
            iced::Task::perform(crate::util::read_file(path), Message::Loaded),
        )
    }

    pub fn view(self: &Self) -> iced::Element<'_, Message> {
        match &self.state {
            crate::util::Query::Pending => iced::widget::Text::new("Loading...").into(),
            crate::util::Query::Loaded(InternalState { content, preview }) => {
                let main_body = {
                    let editor = iced::widget::TextEditor::new(&content).on_action(Message::Edit);

                    let preview_component = iced::widget::markdown::view(
                        preview,
                        iced::widget::markdown::Settings::default(),
                        iced::widget::markdown::Style::from_palette(
                            iced::Theme::TokyoNightStorm.palette(),
                        ),
                    )
                    .map(Message::None);
                    iced::widget::Row::new()
                        .push(editor)
                        .push(preview_component)
                };

                iced::widget::column![
                    iced::widget::container(main_body).height(iced::Length::Fill),
                    iced::widget::text(self.path.to_string_lossy())
                ]
                .height(iced::Length::Fill)
                .into()
            }
            crate::util::Query::Error(e) => iced::widget::Text::new(e.clone()).into(),
        }
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Edit(action) => {
                if let crate::util::Query::Loaded(InternalState { content, preview }) =
                    &mut self.state
                {
                    content.perform(action);
                    *preview = iced::widget::markdown::parse(&content.text()).collect();
                }
                iced::Task::none()
            }
            Message::Loaded(Ok(contents)) => {
                let content = iced::widget::text_editor::Content::with_text(&contents.clone());
                let preview = iced::widget::markdown::parse(&content.text()).collect();
                self.state = crate::util::Query::Loaded(InternalState { content, preview });
                iced::Task::none()
            }
            Message::Loaded(Err(e)) => {
                self.state = crate::util::Query::Error(format!("Failed to load file: {:?}", e));
                iced::Task::none()
            }
            Message::None(url) => {
                print!("{}", url);
                iced::Task::none()
            }
        }
    }
    pub fn subscription(self: &Self) -> iced::Subscription<Message> {
        match &self.state {
            crate::util::Query::Loaded(InternalState { .. }) => {
                iced::keyboard::on_key_press(|key, modifiers| {
                    if key == iced::keyboard::Key::Character("s".into()) && modifiers.control() {
                        Some(Message::Edit(iced::widget::text_editor::Action::Edit(
                            iced::widget::text_editor::Edit::Paste("Hello".to_string().into()),
                        )))
                    } else {
                        None
                    }
                })
            }
            _ => iced::Subscription::none(),
        }
    }
}
