use iced::widget::{button, container, row, text};
use iced::{executor, Application, Command, Length, Settings};
use iced::Color;
// use iced::widget::column;
use self::theme::Theme;
use self::widget::Element;

fn main() {
    App::run(Settings::default()).unwrap();
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPress(String)
}

#[derive(Debug, Default, Clone)]
struct State {
    content: String
}

struct App{
    state: State
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (App{state:State{content: "default state".to_string()}}, Command::none())
    }

    fn title(&self) -> String {
        "Custom Theme".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ButtonPress(m) => {
                println!("The value of the button press is {:?}", m);
                self.state.content = m;
                println!("The new value of the state is {:?}", self.state.content);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        container(
            container(
                row![
                    button(text("primary"))
                        .style(theme::Button::Primary)
                        .on_press(Message::ButtonPress("primary".to_string())),
                    button(text("secondary"))
                        .style(theme::Button::Secondary)
                        .on_press(Message::ButtonPress("secondary".to_string())),
                    container(text("hello"))
                        .style(theme::Container::WhiteBackground),
                    text("The value of the button pressed is : "),
                    text(&self.state.content)
                ]
                .spacing(10),
            )
            .padding(20)
            .style(theme::Container::Bordered),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

mod widget {
    #![allow(dead_code)]
    use crate::theme::Theme;

    pub type Renderer = iced::Renderer<Theme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
}

mod theme {
    use iced::widget::{button, container, text};
    use iced::{application, color, Color};

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Theme;

    impl application::StyleSheet for Theme {
        type Style = ();

        fn appearance(&self, _style: &Self::Style) -> application::Appearance {
            application::Appearance {
                background_color: color!(0x28, 0x28, 0x28),
                text_color: color!(0xeb, 0xdb, 0xb2),
            }
        }
    }

    impl text::StyleSheet for Theme {
        type Style = ();

        fn appearance(&self, _style: Self::Style) -> text::Appearance {
            text::Appearance {
                color: color!(0xeb, 0xdb, 0xb2).into(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Container {
        #[default]
        Default,
        Bordered,
        WhiteBackground
    }

    impl container::StyleSheet for Theme {
        type Style = Container;

        fn appearance(&self, style: &Self::Style) -> container::Appearance {
            match style {
                Container::Default => container::Appearance::default(),
                Container::Bordered => container::Appearance {
                    border_color: color!(0x45, 0x85, 0x88),
                    border_width: 1.0,
                    border_radius: 4.0,
                    ..Default::default()
                },
                Container::WhiteBackground => container::Appearance{
                    background: Some(iced::Background::Color(Color::from_rgba(0.8, 0.2, 0.3, 1.0))),
                    ..Default::default()
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Button {
        #[default]
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Theme {
        type Style = Button;

        fn active(&self, style: &Self::Style) -> button::Appearance {
            match style {
                Button::Primary => button::Appearance {
                    background: color!(0x28, 0x28, 0x28).into(),
                    border_radius: 4.0,
                    border_width: 1.0,
                    border_color: color!(0x45, 0x85, 0x88),
                    ..Default::default()
                },
                Button::Secondary => button::Appearance {
                    background: color!(0x3c, 0x38, 0x36).into(),
                    ..Default::default()
                },
            }
        }
    }
}
