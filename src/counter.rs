use iced::widget::{button, row, text};
use iced::{Alignment, Element, Sandbox};

pub struct Counter {
    state: i64
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            state: 0
        }
    }
    
}

impl Sandbox for Counter {
    type Message =  Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Decrement => {
                self.state -= 1;
            }
            Message::Increment => {
                self.state += 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        row![
            button("+")
                .on_press(Message::Increment),
            text(self.state),
            button("-")
                .on_press(Message::Decrement),
        ] 
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

#[test]
fn test() {
    let mut counter = Counter::new();

    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);
    
    assert_eq!(counter.state, 1)
}