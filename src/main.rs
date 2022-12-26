
mod qr_code;

use iced::Settings;
use qr_code::Counter;
use iced::Application;
use iced::window;

fn main() -> iced::Result {    
    Counter::run(Settings {
        window: window::Settings {
            size: (500, 800),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}