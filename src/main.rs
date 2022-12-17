use iced::widget::{button, container, row, column, text, text_input};
use iced::{executor, Application, Command, Length, Settings};
use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcReceiver};
use iced::Color;
// use iced::widget::column;
use self::theme::Theme;
use self::widget::Element;
use std::collections::HashMap;
use std::process::Command as ProcessCommand;
use std::env;
use std::io;
use std::{thread, time};


// #[macro_use]
// extern crate stdweb;

// use stdweb::web::Window;
    
// use serde::Deserialize;

// #[derive(Deserialize, Debug)]
// struct Ip {  

// #[derive(Deserialize, Debug, Copy)]
// struct Person {
//     name: String,
//     age: i32
// }


fn main() {      

    let args: Vec<String> = env::args().collect();
    println!("value of args {:?}", args);

    let mut s = args[1].clone();

    for x in s.chars(){
        println!("value of x {:?}", x);
    }

    // s.pop();      // remove last
    // if s.len() > 0 {
    //     s.remove(0);  // remove first
    // }
    let (server0, server_name0) = IpcOneShotServer::<Vec<(String, String)>>::new().unwrap();

    println!("server_name0: {:?}", server_name0);
    println!("&cloned_string: {:?}", args[1].clone());

    // let result = std::panic::catch_unwind(|| IpcSender::<Vec<(String, String)>>::connect(args[1].clone()).unwrap());
    // assert!(result.is_err()); 

    // fn do_step_1() -> Result<(), MyError> {
    //     println!("DoStep1");
    // }

    let tx2 = IpcSender::<Vec<(String, String)>>::connect(args[1].clone()).unwrap();

    // match IpcSender::<Vec<(String, String)>>::connect(args[1].clone()) {
    //     t::IpcSender::<Vec<(String, String)>> => {
    //         println!("the function unwrapped successfully {:?}", t);
    //     }
    //     Err(E) => {
    //         println!("there was some error", E);
    //     }
    // }

    let tx1 = IpcSender::<Vec<(String, String)>>::connect(server_name0.clone()).unwrap();   
    // let tx2 = IpcSender::<Vec<(String, String)>>::connect(args[1].clone()).unwrap();
    


    // let tx = IpcSender::<Vec<(String, String)>>::connect(args[1].clone()).unwrap();

    // let wait_time = time::Duration::from_millis(1000);
    // thread::sleep(wait_time);

    // process_channel_handler(args[1].clone());
    // input_handler.join().unwrap(); 

    // App::run(Settings::default()).unwrap();
    // let apphandler = thread::spawn(||{
    //     appthread();
    // });
    // let webhandler = thread::spawn(||{
    //     webthread();
    // });

    // apphandler.join().unwrap();
    // webhandler.join().unwrap();
}

// fn clean_channel_name(stringval: &str) -> String{
//     let mut finalval = "".to_string();
//     for x in stringval.chars(){
//         let intval = x.to_digit(10);
//         match x.to_digit(10){
//             Some(digit) => {
//                 finalval = finalval + &digit.to_string();
//                 // println!("intermediate value of finalval: {:?}", finalval);
//             }
//             None => {},
//             // None => println!("skipping not a digit"),
//         }
//     }
//     // org.rust-lang.ipc-channel.9068538348177729993 
//     finalval = "org.rust-lang.ipc-channel.-".to_string() + &finalval;
//     finalval
// }



fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn process_channel_handler(process_name: String){



    let datavec = vec![("John".to_string(), "23".to_string())];

    println!("inside process_channel_handler");

    // print_type_of(&process_name_clone);
    // print_type_of(&args[1]);

    println!("value of process_name: {:?}", process_name.clone());
    let tx = IpcSender::<Vec<(String, String)>>::connect(process_name.clone());
    println!("value for tx: {:?}", tx);

    // match IpcSender::connect(argsdata.to_string())  {
    //     Ok(v) => {
    //         let tx: IpcSender<Vec<(String, String)>> = v;
    //     }
    //     Err(e) => println!("there was some error {:?}", e.to_string()),
    // }
    
    // let tx0:IpcSender<Vec<(String, String)>> = IpcSender::connect(argsdata).unwrap();

    // let rx0 = IpcReceiver::connect(args[1]).unwrap();

    // tx0.send(datavec).unwrap();


    // let sentdata = rx0.recv().unwrap();
    
    // println!("value of sentdata: {:?}", sentdata);

}

// fn webthread(){
//     println!("inside webthread");
// }

// fn appthread(){
//     println!("inside testthread");
//     App::run(Settings::default()).unwrap();
// }

#[tokio::main]
async fn getrequest(url: String) -> Result<String, Box<dyn std::error::Error>> {

    println!("Fetching {:?}...", url);
    let res = reqwest::get(url).await?;

    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    ProcessCommand::new("sh")
        .arg("-C")
        .arg("/Users/peterweyand/Code/rustprojects/project1.1/project1_1/src/test.sh")
        .spawn()
        .expect("sh command failed to start");

    // let hello = output.stdout;

    // println!("{}", body);

    // Command::new("sh")
    //     .arg("-C")
    //     .arg("/Users/peterweyand/Code/rustprojects/project1.1/project1_1/src/test.sh")
    //     .spawn()
    //     .expect("sh command failed to start");

    Ok(body)
}

// fn loadhttp() -> Result<(), Box<dyn std::error::Error>> {
//     println!("inside loadhttp");
//     let message = "Hello, 世界!";
//     // js! {
//     //     alert( @{message} );
//     // }
//     Ok(())
// }


#[derive(Debug, Clone)]
enum Message {
    ButtonPress(String),
    AddressChanged(String), 
    AddressEntered
}

#[derive(Debug, Default, Clone)]
struct State {
    input_value: String,
    content: String, 
    addressclicked: i32
}

struct App{
    state: State, 
    httpbody: String
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (App{
            state:State{
                content: "default state".to_string(), 
                addressclicked: 0, 
                input_value:"input coordinates skroadryder".to_string()
            }, 
            httpbody: "".to_string()
        }, Command::none())
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
            Message::AddressChanged(m) => {
                println!("The value of m is {:?}", m);
                println!("The value of mstring is {:?}", m);
                println!("The value of the input value is {:?}", self.state.input_value);

                if self.state.addressclicked == 0 {
                    self.state.addressclicked = 1;
                    let lastchar = m.chars().last().unwrap().to_string();
                    println!("value of lastchar {:?}", lastchar);
                    self.state.input_value = lastchar
                }else{
                    self.state.input_value = m
                }
            }
            Message::AddressEntered => {
                if !self.state.input_value.is_empty() {
                    println!("the input value is not empty and its value is {:?}", self.state.input_value.to_string());
                    match getrequest(self.state.input_value.to_string()){
                        Ok(body) => {
                            self.httpbody=body;
                            // println!("Value of body {:?}", self.httpbody)
                            // match loadhttp(){
                            //     Ok(_)=>{
                            //         println!("ok loadhttp");
                            //     }
                            //     Err(e) => eprintln!("Value of error {:?}", e)
                            // }
                        },
                        Err(e) => eprintln!("Value of error {:?}", e),
                    }
                }else{
                    println!("the input value is empty!");
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let input = text_input(
            &self.state.input_value,
            &self.state.input_value,
            Message::AddressChanged,
        )
        .padding(15)
        .width(iced::Length::Units(1000))
        .size(30)
        .on_submit(Message::AddressEntered);

        container(
            column![
                container(
                    input
                )
                .padding(10)
                ,
                container(
                    row![
                        button(text("primary"))
                            .style(theme::Button::Primary)
                            .on_press(Message::ButtonPress("primary".to_string())),
                        button(text("secondary"))
                            .style(theme::Button::Secondary)
                            .on_press(Message::ButtonPress("secondary".to_string())),
                    ]      
                    .spacing(10),      
                )
                .padding(10)
                ,
                container(
                    row![
                        text("The value of the button pressed is : "),
                        container(text(&self.state.content))                           
                        .style(theme::Container::WhiteBackground)
                        .padding(5),
                    ]
                    .spacing(10),
                )
                .padding(10)
            ]
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
    pub type TextInput<'a, Message> = iced::widget::TextInput<'a, Message, Renderer>;

}

mod theme {
    use iced::widget::{button, container, text, text_input};
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
                    border_radius: 3.0,
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

    #[derive(Debug, Clone, Copy, Default)]
    pub enum TextInput {
        #[default]
        Default,
    }

    impl text_input::StyleSheet for Theme {
        type Style = TextInput;
        fn active(&self, style: &Self::Style) -> text_input::Appearance{
            text_input::Appearance {
                background:  iced::Background::Color(Color{ r: 1.0, g: 1.0, b: 1.0, a: 1.0,}),
                border_color: Color{ r: 0.0, g: 0.0, b: 0.0, a: 1.0,},
                border_radius: 3.0,
                border_width: 1.0
            }
        }
        fn focused(&self, style: &Self::Style) -> text_input::Appearance{
            text_input::Appearance {
                background:  iced::Background::Color(Color{ r: 1.0, g: 1.0, b: 1.0, a: 1.0,}),
                border_color: Color{ r: 0.0, g: 0.0, b: 0.0, a: 1.0,},
                border_radius: 3.0,
                border_width: 1.0
            }
        }   
        fn placeholder_color(&self, style: &Self::Style) -> iced::Color{
            Color{ r: 0.0, g: 0.0, b: 0.0, a: 1.0,}
        }  
        fn value_color(&self, style: &Self::Style) -> iced::Color{
            Color{ r: 0.0, g: 0.0, b: 0.0, a: 1.0,}
        }   
        fn selection_color(&self, style: &Self::Style) -> iced::Color{
            Color{ r: 1.0, g: 1.0, b: 1.0, a: 1.0,}
        }  
    }
}
