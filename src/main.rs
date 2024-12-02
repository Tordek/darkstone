mod notes;

struct Darkstone {
    notes: notes::Notes,
}

#[derive(Debug, Clone)]
enum Message {
    Notes(notes::Message),
}

impl Darkstone {
    fn new() -> (Self, iced::Task<Message>) {
        let (notes, notes_task) = notes::Notes::new(format!(
            "{}/src/0de5",
            std::env::var("HOME")
                .expect("How is the env not set")
                .to_string()
        ));
        (Self { notes }, iced::Task::map(notes_task, Message::Notes))
    }
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Notes(message) => self.notes.update(message).map(Message::Notes),
        }
    }
    fn view(&self) -> iced::Element<'_, Message> {
        notes::Notes::view(&self.notes).map(Message::Notes)
    }
}

pub fn main() -> iced::Result {
    iced::application("Darkstone", Darkstone::update, Darkstone::view)
        .theme(|_| iced::Theme::TokyoNightStorm)
        .run_with(|| Darkstone::new())
}
