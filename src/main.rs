mod counter;
use counter::Counter;
use iced::{Sandbox, Settings};

fn main() {
    Counter::run(Settings::default()).expect("Failed to run app")
}