use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Container, Renderer, Theme};
use iced::window::{Id, Position};
use iced::{widget, Length};
use iced::{window, Size};
use iced::Point;

#[derive(Debug, Clone, Copy)]
pub struct Window {
    pub id: Id,
    pub position: Option<Point>,
    pub size: Size,
    pub focused: bool,
}

struct Counter {
    // This will be our state of the counter app
    // a.k.a the current count value
    count: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    // Emitted when the increment ("+") button is pressed
    IncrementCount,
    // Emitted when decrement ("-") button is pressed
    DecrementCount,
}

// Implement our Counter
impl Counter {
    fn new() -> Self {
        // initialize the counter struct
        // with count value as 0.
        Self { count: 0 }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        // handle emitted messages
        match message {
            Message::IncrementCount => self.count += 1,
            Message::DecrementCount => self.count -= 1,
        }
        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let column = widget::column![
            widget::button("-").on_press(Message::DecrementCount),
            widget::text(self.count.to_string()),
            widget::button("+").on_press(Message::IncrementCount)
        ]
        .align_x(Horizontal::Center);

        let stuff_centered: Container<'_, Message, Theme, Renderer> =
            widget::Container::new(column)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .width(Length::Fill)
                .height(Length::Fill);

        stuff_centered.into()
    }
}

fn main() -> Result<(), iced::Error> {

    // Load your icon image (e.g., PNG) as a Vec<u8>
    let icon_data = include_bytes!("icon32.png");
    
    // Create an Icon from the embedded data
    let icon = window::icon::from_file_data(icon_data, None)
        .expect("Failed to create icon"); // Handle potential errors



    // run the app from main function
    iced::application("Counter Example", Counter::update, Counter::view)
        .window(window::Settings {
            position: Position::Centered,
            resizable: false,
            size: Size::new(500.0, 400.0),
            icon: Some(icon),
            ..Default::default()
        })
        .run_with(|| (Counter::new(), iced::Task::none()))
}
