#[cfg(test)]
mod test_functions;
mod doll;
mod sql;

use std::fmt::Alignment;

use iced::{
     Alignment::Center, Element, Font, Length::Fill, Renderer, Size, Subscription, Task, Theme, alignment::Horizontal::Right, font, theme::palette::Background, widget::{Container, button, column, container, pick_list, row, scrollable, slider, table, text, text_input}, window,
};

use crate::doll::{Doll, create_dolls_random};

fn main() -> iced::Result {
    iced::application(Populate::default, Populate::update, Populate::view)
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
#[derive(Default)]
struct Populate {
    page: Page,
    theme: Option<Theme>,
    doll_list: Vec<Doll>,
    doll_slider: i32,
    subs: u8,
}

#[derive(Default)]
enum Page {
    #[default]
    Creator,
}

#[derive(Debug, Clone)]
enum Message {
    GoToCreator,
    DollsFetched(Vec<Doll>),
    FetchDolls,
    CreateDoll,
    CreateRandom,
    DollCreated(Doll),
    ThemeChanged(Theme),
    DollMakerSliderChanged(i32),
}

impl Populate {

    fn theme(&self) -> Option<Theme> {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        
        for bits in 1..self.subs.bit_width() {
            
        }
         return Subscription::none()
    }

    fn update(state: &mut Self, message: Message) -> Task<Message> {
        match message {
            Message::GoToCreator => {
                state.page = Page::Creator;
                Task::none()
            },
            Message::CreateDoll => Task::perform(
                doll::create_doll(None),
                Message::DollCreated
            ),
            Message::CreateRandom => {
                Task::future(doll::create_dolls_random(state.doll_slider))
                .then(|list|Task::future(sql::add_doll_many(list))).then(|_| Task::done(Message::FetchDolls))
            },
            Message::DollCreated(doll) => {
                Task::future(
                sql::add_doll(doll),
                ).then(|_| Task::done(Message::FetchDolls))
            },
            Message::FetchDolls => Task::perform(
                sql::list_dolls(),
                Message::DollsFetched
            ),
            Message::DollsFetched(dolls) => {
                state.doll_list = dolls;
                Task::none()
            },
            Message::ThemeChanged(newtheme) => {
                state.theme = Some(newtheme);
                Task::none()
            }
            Message::DollMakerSliderChanged(val) => {
                state.doll_slider=val;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {

        let doll_table = {
            fn bold(header: &str) -> impl Into<Element<'_, Message, Theme, Renderer>> {
                text(header).font(Font {
                    weight: font::Weight::Bold,
                    ..Font::DEFAULT
                }).center()
            }

            let idwidth = 500;
            let fieldwidth = 120;
            let datawidth = 60;
            let totalwidth = {idwidth+(fieldwidth*3)+(datawidth*3)+5};
            let columns: [table::Column<'_, '_, &Doll, Message, iced::Theme, _>; 7] = [
                
                table::column(bold("Id"),           |doll: &Doll| text(&doll.id).font(Font::MONOSPACE))
                .width(idwidth),

                table::column(bold("Surname"),      |doll: &Doll| text(&doll.lname))
                .width(fieldwidth),

                table::column(bold("Given Name(s)"),|doll: &Doll| text(&doll.fname))
                .width(fieldwidth),

                table::column(bold("Gender"),       |doll: &Doll| text(&doll.sex))
                .width(fieldwidth),

                table::column(bold("Hunger"),       |doll: &Doll| {
                    text!("{:03}",&doll.data[0]).style( 
                        match &doll.data[0] {
                            ..128 => text::default,
                            ..=254 => text::warning,
                            _ => text::danger,
                    })
                }).width(datawidth),
                
                table::column(bold("Mood"), |doll: &Doll| {
                    text!("{:03}",&doll.data[1]).style( 
                        match &doll.data[1] {
                            ..128 => text::default,
                            ..=254 => text::warning,
                            _ => text::danger,
                    })
                }).width(datawidth),

                table::column(bold("Energy"), |doll: &Doll| {
                    text!("{:03}",&doll.data[2]).style( 
                        match &doll.data[2] {
                            ..128 => text::default,
                            ..=254 => text::warning,
                            _ => text::danger,
                    }).center()
                    
                }).width(datawidth),
            ];
            table(columns, &self.doll_list)
            .separator(2)
            .width(totalwidth)
            
        };

        
        
        let header: Container< Message, Theme, Renderer> = {
            container( row![
                pick_list(Theme::ALL, self.theme.clone(), Message::ThemeChanged)
            ]
            .align_y(Center)
            
            )
            .align_x(Right)
            .align_y(Center)
            .padding(10)
            .width(Fill)
            .style(|theme: &Theme| {
                
                container::primary(theme)
                
            })
        };
        
        match self.page {
            //Doll Creation Page
            Page::Creator => 
            
            container(
                column![
                    header,
                    row![
                    scrollable(doll_table).spacing(5),
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
                        .on_press(Message::CreateRandom),

                        text!("Dolls to make: {:02}",self.doll_slider),

                        slider(0..=100,self.doll_slider,Message::DollMakerSliderChanged)
                        .step(5)
                        .shift_step(1)
                        .height(32)
                        .width(Fill),

                    ].padding(20)
                    .spacing(5)
                    .align_x(Center)
                    ]
                ].spacing(20)
            )
            .width(Fill),
        }
        .width(Fill)
        .into()

    }
}