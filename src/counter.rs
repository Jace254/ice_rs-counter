use iced::{
    alignment::{Horizontal, Vertical}, 
    widget::{button, text, Container, Row},
    Element, 
    Sandbox
};

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

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Decrement => {
                self.state -= 1;
            }
            Message::Increment => {
                self.state += 1;
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let inc = button( text("+").width(50).height(50).size(32).vertical_alignment(Vertical::Center).horizontal_alignment(Horizontal::Center))
            .on_press(Message::Increment)
            .style(iced::theme::Button::Destructive);

        let label = text(self.state);

        let dec = button( text("-").width(50).height(50).size(32).vertical_alignment(Vertical::Center).horizontal_alignment(Horizontal::Center))
            .on_press(Message::Decrement)
            .style(iced::theme::Button::Destructive);
        let ro = Row::new().push(inc).push(label).push(dec).spacing(50).align_items(iced::Alignment::Center);
        Container::new(ro).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into()
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