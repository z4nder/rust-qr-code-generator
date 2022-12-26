
mod qr_code;

use iced::Settings;
use qr_code::QRGenerator;
use iced::Sandbox;
use iced::window;

fn main() -> iced::Result {    
    QRGenerator::run(Settings {
        window: window::Settings {
            size: (500, 800),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}