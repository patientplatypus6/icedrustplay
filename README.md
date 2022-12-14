# icedrustplay

## to run 

`cargo build && cargo run`

## what it looks like

![ alt text for screen readers](/screenshot.png "This is what it looks like.")

## here's the problem

I want to be able to nest `row!` and `col!` variables and it doesn't look like I can. 

This works: 

```
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
```

but this 

```
    fn view(&self) -> Element<Message> {
        container(
            container(
                column![
                    row![
                        button(text("primary"))
                            .style(theme::Button::Primary)
                            .on_press(Message::ButtonPress("primary".to_string())),
                        button(text("secondary"))
                            .style(theme::Button::Secondary)
                            .on_press(Message::ButtonPress("secondary".to_string())),
                    ]
                    .spacing(10),
                    row![
                        container(text("hello"))
                            .style(theme::Container::WhiteBackground),
                        text("The value of the button pressed is : "),
                        text(&self.state.content)
                    ]
                ]
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
```

produces the following error: 

```
peterweyand@Peters-MacBook-Pro project1_1 % cargo build && cargo run
   Compiling project1_1 v0.1.0 (/Users/peterweyand/Code/rustprojects/project1.1/project1_1)
error: column! takes no arguments
  --> src/main.rs:54:17
   |
54 | /                 column![
55 | |                     row![
56 | |                         button(text("primary"))
57 | |                             .style(theme::Button::Primary)
...  |
69 | |                     ]
70 | |                 ]
   | |_________________^
   |
   = note: this error originates in the macro `column` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused imports: `button`, `row`, `text`
 --> src/main.rs:1:20
  |
1 | use iced::widget::{button, container, row, text};
  |                    ^^^^^^             ^^^  ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `iced::Color`
 --> src/main.rs:3:5
  |
3 | use iced::Color;
  |     ^^^^^^^^^^^

error[E0277]: the trait bound `iced_native::element::Element<'_, _, _>: From<u32>` is not satisfied
  --> src/main.rs:54:17
   |
53 |               container(
   |               --------- required by a bound introduced by this call
54 | /                 column![
55 | |                     row![
56 | |                         button(text("primary"))
57 | |                             .style(theme::Button::Primary)
...  |
69 | |                     ]
70 | |                 ]
   | |_________________^ the trait `From<u32>` is not implemented for `iced_native::element::Element<'_, _, _>`
   |
   = help: the following other types implement trait `From<T>`:
             <iced_native::element::Element<'a, Message, Renderer> as From<&'a str>>
             <iced_native::element::Element<'a, Message, Renderer> as From<iced::widget::ProgressBar<Renderer>>>
             <iced_native::element::Element<'a, Message, Renderer> as From<iced::widget::Rule<Renderer>>>
             <iced_native::element::Element<'a, Message, Renderer> as From<iced::widget::Slider<'a, T, Message, Renderer>>>
             <iced_native::element::Element<'a, Message, Renderer> as From<iced::widget::Space>>
             <iced_native::element::Element<'a, Message, Renderer> as From<iced_native::overlay::menu::List<'a, T, Renderer>>>
             <iced_native::element::Element<'a, Message, Renderer> as From<iced_native::widget::button::Button<'a, Message, Renderer>>>
             <iced_native::element::Element<'a, Message, Renderer> as From<iced_native::widget::checkbox::Checkbox<'a, Message, Renderer>>>
           and 14 others
   = note: required for `u32` to implement `Into<iced_native::element::Element<'_, _, _>>`
note: required by a bound in `container`
  --> /Users/peterweyand/.cargo/git/checkouts/iced-f01cba4d5e61fd0a/02182ee/native/src/widget/helpers.rs:39:19
   |
39 |     content: impl Into<Element<'a, Message, Renderer>>,
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `container`

For more information about this error, try `rustc --explain E0277`.
warning: `project1_1` (bin "project1_1") generated 2 warnings
error: could not compile `project1_1` due to 2 previous errors; 2 warnings emitted
```

which I admit is a bit beyond me. 

The point being is that in order for a UI library to be functional it should be able to nest rows and columns.

Is there a way to do this?