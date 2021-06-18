use iced::{Scrollable, scrollable, Application, Container, Text, Element, Column, Command, Settings, Length, HorizontalAlignment, Clipboard};

enum App {
    Loading,
    Loaded(State)
}

struct State {
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
struct LoadState {
    // fill with async loaded things
}

#[derive(Debug, Clone)]
enum LoadError {
    // Placeholder for if async load fails
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<LoadState, LoadError>),
}

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        (
            App::Loading,
            Command::perform(LoadState::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("Application Boiler Plate")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match self {
            App::Loading => {
                match message {
                    // when async completes this is called & converts LoadState to State, which is then
                    // displayed using view()
                    // `_LoadState' is not used here, but will likely be needed in your program.
                    // remove the underscore and then the data in LoadState can be moved over to 
                    Message::Loaded(Ok(_load_state)) => {
                        *self = App::Loaded(State{
                            // return a State struct to be viewed
                            scroll: scrollable::State::new(),
                        })
                    },
                    Message::Loaded(Err(_)) => {
                        // what to do in the case async load fails
                    }
                }
            }
            App::Loaded(_state) => {
                match message {
                    //no messages in this boilerplate
                    _ => (),
                }
            }

        }
        Command::none()
    }
    fn view(&mut self) -> Element<Message> {
        match self {
            App::Loading => loading_message(),
            App::Loaded(State {
                    // have state variables to be accessable 
                    scroll,
                    ..
            }) => {
                let title = Text::new("Title");

                let content = Column::new()
                    .max_width(800)
                    .spacing(20)
                    .push(title);
                Scrollable::new(scroll)
                    .padding(40)
                    .push(
                        Container::new(content)
                            .width(Length::Fill)
                            .center_x()
                    )
                    .into()
            }
        }
    }
}

impl LoadState {
    // this is the function that is called to load data
    async fn load() -> Result<LoadState, LoadError> {
        Ok(LoadState{})
    }
}

// what is displayed while waiting for the `async fn load()`
fn loading_message<'a>() -> Element<'a, Message> {
    Container::new(
        Text::new("Loading...")
            .horizontal_alignment(HorizontalAlignment::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}
