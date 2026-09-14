

use iced::{Task, widget::{self, container, text}};

use crate::{Message, Page};

#[derive(Debug, Clone)]
pub enum BreederMessage {
    NewDoll
}
#[derive(Debug, Clone)]
pub struct Breeder{
    
}

impl Breeder {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Page for Breeder {

    fn update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        container(text("hi")).into()
    }
}



