#[cfg(test)]
mod test_functions;
mod doll;
mod sql;


use core::fmt;

use iced::{
     Element, Font, Renderer, Size, Subscription, Task, Theme,
     Alignment::Center, Length::Fill,
     font, window,
     widget::{button, Column, column, Container, container, pick_list, row, scrollable, slider, space, table, text},
};

use crate::doll::Doll;

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
    sort_table_by: SortingOrder,

    doll_inspect: Option<Doll>,    
    doll_select_1: Option<Doll>,
    doll_select_2: Option<Doll>,

    subs: u8,
    doll_slider: i32,
}
#[derive(Default, Debug, Clone)]
enum SortingOrder {
    #[default]
    LName,
    LName_R,
    CreationTime,
    CreationTimeR,


}

#[derive(Default, Debug, Clone, PartialEq)]
enum Page {
    #[default]
    Creator,
    Breeder,
}

impl Page {
    const ALL: &'static [Self] = &[
        Self::Creator,
        Self::Breeder,
    ];

    fn name(&self) -> &'static str {
        match self {
            Self::Breeder => "Breeder",
            Self::Creator => "Creator"
        }
    }
}
impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
#[derive(Debug, Clone)]
enum Message {
    None,
    DollSelect(Doll),
    GoToPage(Page),
    DollsFetched(Vec<Doll>),
    FetchDolls,
    CreateDoll(Doll, Doll),
    CreateDollRandom,
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
            //Go to pages
            Message::GoToPage(page) => {
                state.page = page;
                Task::none()
            }

            
            //Affect Model
            Message::CreateDoll(d1,d2) => {
                Task::perform(
                doll::create_doll(Some((d1,d2))),
                Message::DollCreated)
            }
            Message::CreateDollRandom => {
                Task::future(doll::create_dolls_random(state.doll_slider))
                .then(|list|Task::future(sql::add_doll_many(list)))
                .then(|_| Task::done(Message::FetchDolls))
            }
            Message::DollCreated(doll) => {
                Task::future(
                sql::add_doll(doll),
                ).then(|_| Task::done(Message::FetchDolls))
            }
            Message::FetchDolls => {
                Task::perform(
                sql::list_dolls(),
                Message::DollsFetched)
            }


            //Global Changes
            Message::DollsFetched(dolls) => {
                state.doll_list = dolls;
                Task::none()
            }
            Message::ThemeChanged(newtheme) => {
                state.theme = Some(newtheme);
                Task::none()
            }
            Message::DollMakerSliderChanged(val) => {
                state.doll_slider=val;
                Task::none()
            }
            _ => Task::none()
        }
    }

    fn view(&self) -> Element<'_, Message> {

        

        match self.page {

            //Doll Creation Page
            Page::Creator => 
            container(
                column![
                    self.header(),
                    row![
                    scrollable(self.doll_table()).spacing(5),
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
                        .on_press(Message::CreateDollRandom),

                        text!("Dolls to make: {:2}",self.doll_slider),

                        slider(0..=100, self.doll_slider, Message::DollMakerSliderChanged)
                        .step(1)
                        .shift_step(5)
                        .height(32)
                        .width(Fill),

                    ].padding(20)
                    .spacing(5)
                    .align_x(Center)
                    ]
                ].spacing(5)
            )
            .width(Fill),



            Page::Breeder =>
            container(
                column!(
                    self.header(),
                    row![
                        scrollable(self.doll_table()).spacing(5)
                    ]
                ).spacing(5)
            )
        }
        .width(Fill)
        .into()

    }


    // Global Widgets
    fn header(&self) -> Container<'static, Message, Theme, Renderer> {
        container( row![
            pick_list(Page::ALL, Some(self.page.clone()), Message::GoToPage),
            space()
            .width(Fill),
            pick_list(Theme::ALL, self.theme.clone(), Message::ThemeChanged),
            
        ])
        .align_x(Center)
        .align_y(Center)
        .padding(10)
        .width(Fill)
        .style(|theme: &Theme| {
            container::primary(theme)
        })
    }

    fn doll_table(&self) -> Column<Message> {
        fn bold(header: &str) -> impl Into<Element<'_, Message, Theme, Renderer>> {
            text(header).font(Font {
                weight: font::Weight::Bold,
                ..Font::DEFAULT
            }).center()
        }

        //let idwidth = 500;
        let fieldwidth = 120;
        let datawidth = 60;
        //let totalwidth = {(fieldwidth*3)+(datawidth*3)};
        let columns: [table::Column<'_, '_, &Doll, Message, iced::Theme, _>; _] = [
            
            // table::column(bold("Id"),           |doll: &Doll| text(&doll.id).font(Font::MONOSPACE))
            // .width(idwidth),

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
                }).center()
            }).width(datawidth),
            
            table::column(bold("Mood"), |doll: &Doll| {
                text!("{:03}",&doll.data[1]).style( 
                    match &doll.data[1] {
                        ..128 => text::default,
                        ..=254 => text::warning,
                        _ => text::danger,
                }).center()

            }).width(datawidth),

            table::column(bold("Energy"), |doll: &Doll| {
                text!("{:03}",&doll.data[2]).style( 
                    match &doll.data[2] {
                        ..128 => text::default,
                        ..=254 => text::warning,
                        _ => text::danger,
                }).center()
            }).width(datawidth),
            {
                if self.page == Page::Breeder {
                table::column("", |d: &Doll| {
                    button("Select").on_press(Message::DollSelect(d.clone()))
                })
                .width(datawidth)
                
                }
                else {table::column("", |_| {space()})}

            }
        ];
        column!(
        table(columns, &self.doll_list)
        .separator(2)
        )
    }

}