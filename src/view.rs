use iced::{Element, Length, Command, Error, Application, Renderer, Theme, ContentFit};
use iced::widget::{ column, text, row, Row, Text, Container, Image, container};
use iced::widget::image::Handle;
use crate::view::Message::ImageLoaded;
use crate::view::View::LoadedImage;


#[derive(Debug)]
pub enum View {
    Loading,
    LoadedImage { img: Handle },
}

#[derive(Debug)]
pub enum Message {
    ImageLoaded(Result<Handle, reqwest::Error>)
}


impl Application for View {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (View::Loading,
         Command::perform(get_image(), ImageLoaded))
    }

    fn title(&self) -> String {
        String::from("Rust News")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        println!("Update!!");
        println!("Added message :: {:?}", message);
        match message {
            ImageLoaded(Ok(img)) => {
                *self = { LoadedImage { img } };
                Command::none()
            }
            _ => {
                *self = View::Loading;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        println!("Added View message :: {:?}", self);

        let content = match self {
            View::Loading => {
                column![text("Loading Image ....").size(40)]
                    .width(Length::Shrink)
            }
            LoadedImage { img } => {
                column![
                     Image::new(img.to_owned()).content_fit(ContentFit::Fill)
                 ]
            }
        };

        container(content).height(Length::Fill).width(Length::Fill).into()
    }
}

pub async fn get_image() -> Result<Handle, reqwest::Error> {
    println!("Getting image");
    let by = reqwest::get("https://en.wikipedia.org/wiki/Portable_Network_Graphics#/media/File:PNG_transparency_demonstration_1.png").await?.bytes().await?;
    Ok(Handle::from_memory(by.as_ref().to_owned()))
}
