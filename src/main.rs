#[cfg(test)]
mod test_functions;
mod doll;
mod sql;
mod pages;


use iced::{
     Element, Renderer, Size, Subscription, Task, Theme,
     Alignment::Center, Length::Fill,
     window,
     widget::{column, Container, container, pick_list, row, space, },
};

use crate::doll::Doll;
use crate::pages::{Creator, CreatorMessage, Breeder, BreederMessage};


fn main() -> iced::Result {
    iced::application(Populate::new, Populate::update, Populate::view)
    .subscription(Populate::subscription)
    .theme(Populate::theme)
    .window({
        window::Settings {
            size: Size{width:1250.0,height:700.0},
            min_size: Some(Size {width:1250.0,height:720.0}),
            ..Default::default()
        }
    })
    .run()
}

trait Page {
    fn update(&mut self, message: Message) -> Task<Message>;
    fn view(&self) -> iced::Element<'_, Message>;
}
#[derive(Debug, Clone)]
enum Pages {
    Creator,
    Breeder,
}

impl Pages {
    fn get_page(state: Self) -> Box<dyn Page>  {
        match state {
            Self::Creator => Box::new(Creator::new()),
            Self::Breeder => Box::new(Breeder::new()),
        }
    }
}

struct Populate {
    pages: Vec<Box<dyn Page>>,
    theme: Option<Theme>,
    doll_list: Vec<Doll>,
    sort_table_by: SortingOrder,

    subs: u8,
    doll_slider: i32,
}

#[derive(Default, Debug, Clone)]
enum SortingOrder {
    #[default]
    Lastname,
    LastnameR,
    Id,
    IdR,
}

#[derive(Debug, Clone)]
enum Navigation {
    GoTo(Pages),
    Back,
    None,
}

#[derive(Debug, Clone)]
enum Message {
    GoToPage(Navigation),
    CreatorMessage(CreatorMessage),
    DollSelect(Doll),
    DollsFetched(Vec<Doll>),
    FetchDolls,
    CreateDoll(Doll, Doll),
    CreateDollRandom,
    DollCreated(Doll),
    ThemeChanged(Theme),
    DollMakerSliderChanged(i32),
}

impl Populate {

    fn new() -> (Self, Task<Message>) {
        
        (
            Self {
                pages: vec![Box::new(Creator::new())],
                doll_list: vec![],
                sort_table_by: SortingOrder::Id,
                subs: 0,
                doll_slider: 1,
                theme: None

            },
            Task::none()
        )
    }

    fn theme(&self) -> Option<Theme> {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        
        for bits in 1..self.subs.bit_width() {
            
        }
         return Subscription::none()
    }

    fn update(state: &mut Self, message: Message) -> Task<Message> {
        let navigation = state.pages.last_mut().unwrap().update(message.clone());
        
        match message {
            
            // //Affect Model
            // Message::CreateDoll(d1,d2) => {
            //     Task::perform(
            //     doll::create_doll(Some((d1,d2))),
            //     Message::DollCreated)
            // }
            // Message::CreateDollRandom => {
            //     Task::future(doll::create_dolls_random(state.doll_slider))
            //     .then(|list|Task::future(sql::add_doll_many(list)))
            //     .then(|_| Task::done(Message::FetchDolls))
            // }
            // Message::DollCreated(doll) => {
            //     Task::future(
            //     sql::add_doll(doll),
            //     ).then(|_| Task::done(Message::FetchDolls))
            // }
            // Message::FetchDolls => {
            //     Task::perform(
            //     sql::list_dolls(),
            //     Message::DollsFetched)
            // }

            Message::GoToPage(p) => {
                pages::page_select(p);
                Task::none()
            }
            //Global Changes
            // Message::DollsFetched(dolls) => {
            //     state.doll_list = dolls;
            //     Task::none()
            // }
            Message::ThemeChanged(newtheme) => {
                state.theme = Some(newtheme);
                Task::none()
            }
            _ => Task::none()
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column!(
            self.header(),
            self.pages.last().unwrap().view()
        )
        .into()
    }


    // Global Widgets
    fn header(&self) -> Container<'static, Message, Theme, Renderer> {
        container( row![
            space()
            .width(Fill),
            pick_list(Theme::ALL, self.theme.clone(), Message::ThemeChanged),
            ],
        )
        .align_x(Center)
        .align_y(Center)
        .padding(10)
        .width(Fill)
        .style(|theme: &Theme| {
            container::primary(theme)
        })
    }

}