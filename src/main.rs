mod counter;
use counter::Counter;
use iced::{window::Position, Pixels, Sandbox, Settings, Size};

fn main() {
    let  mut settings = Settings::default();
    settings.window.min_size = Some(Size { width: 400.0, height: 400.0 });
    settings.default_text_size = Pixels::from(32);
    settings.window.position = Position::Centered;
    Counter::run(settings).expect("Failed to run app")
}