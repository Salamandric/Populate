mod pages {

}
use iced::{

    Alignment::Center, Length::Fill, Task, Theme, widget::{button, column, container, row, scrollable, slider, text},
};
use crate::{Message, Page, doll::{self, Doll}, pages::doll_table, sql};

#[derive(Debug, Clone)]
pub enum CreatorMessage {
    DollMakerSliderChanged(i32),
    CreateDollRandom
}

type Mc = CreatorMessage;

#[derive(Debug, Clone)]
pub struct Creator {
    doll_slider: i32,
    doll_list: Vec<Doll>,
}

impl Creator {
    pub fn new() -> Self {
        Self {
            doll_slider: 1,
            doll_list: vec![],
        }
    }
}

impl Page for Creator {
    
    fn update(&mut self, message: crate::Message) -> Task<Message> {
        if let Message::CreatorMessage(msg) = message {
            match msg {
                Mc::DollMakerSliderChanged(val) => {
                    self.doll_slider = val;
                    Task::none()
                }
                Mc::CreateDollRandom => {
                    Task::future(doll::create_dolls_random(self.doll_slider))
                    .then(|list|Task::future(sql::add_doll_many(list)))
                    .then(|_| Task::done(Message::FetchDolls))
                }
            }
        }
        else {Task::none()}
    }

    fn view(&self) -> iced::Element<'_, crate::Message> {
        container(
                row![
                scrollable(doll_table(&self.doll_list)).spacing(5),
                column![

                    text!("Showing {} dolls", self.doll_list.len()),

                    button("Refresh Dolls").style(|theme: &Theme, status| {

                        match status {
                            _ => button::primary(theme, status)
                        }
                    })
                    .width(Fill)
                    .on_press(Message::FetchDolls),

                    button("Make Doll").style(|theme: &Theme, status| {

                        match status {
                            _ => button::primary(theme, status)
                        }
                    })
                    .width(Fill)
                    .on_press(Message::CreatorMessage(CreatorMessage::CreateDollRandom)),

                    text!("Dolls to make: {:2}",self.doll_slider),

                    slider(0..=100, self.doll_slider, |v| Message::CreatorMessage(CreatorMessage::DollMakerSliderChanged(v)))
                    .step(1)
                    .shift_step(5)
                    .height(32)
                    .width(Fill),

                ].padding(20)
                .spacing(5)
                .align_x(Center)
                ]
            )
            .width(Fill)
            .into()
            

    }
}

