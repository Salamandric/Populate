// #[cfg(test)]
// mod test_functions;
mod doll;
mod sql;
use iced::{
     Alignment::Center, Element, Font, Length::{Fill, FillPortion}, Rectangle, Renderer, Subscription, Task, Theme, font, futures::task, overlay::menu::State, system, widget::{Container, button, column, container, pick_list, row, scrollable, table, text},
};

use crate::sql::SqlHandler;

fn main() -> iced::Result {
    iced::application(Populate::default, Populate::update, Populate::view)
    .theme(Populate::theme)
    .run()
}
#[derive(Default)]
struct Populate {
    page: Page,
    theme: Option<Theme>,
    doll_list: Vec<doll::Doll>,
}

#[derive(Default)]
enum Page {
    #[default]
    Creator,
}
#[derive(Debug, Clone)]
enum Message {
    GoToCreator,
    DollsFetched(Vec<doll::Doll>),
    FetchDolls,
    ThemeChanged(Theme)
}

impl Populate {

    fn theme(&self) -> Option<Theme> {
        self.theme.clone()
    }

    fn update(state: &mut Self, message: Message) -> Task<Message> {
        match message {
            Message::GoToCreator => {
                state.page = Page::Creator;
                Task::none()
            }
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
        }
    }

    fn view(&self) -> Element<'_, Message> {

        let doll_table = {
            fn bold(header: &str) -> impl Into<Element<'_, Message, Theme, Renderer>> {
                text(header).font(Font {
                    weight: font::Weight::Bold,
                    ..Font::DEFAULT
        
                })
            }
            let columns: [table::Column<'_, '_, &doll::Doll, Message, iced::Theme, _>; 7] = [
                
                table::column(bold("Id"), |doll: &doll::Doll| text(&doll.id)),
                table::column(bold("Surname"), |doll: &doll::Doll| text(&doll.lname)),
                table::column(bold("Given Name(s)"), |doll: &doll::Doll| text(&doll.fname)),
                table::column(bold("Gender"), |doll: &doll::Doll| text(&doll.gender)),
                table::column(bold("Hunger"), |doll: &doll::Doll| {
                    text(&doll.needs[0]).style( 
                        match &doll.needs[0] {
                            ..128 => text::default,
                            ..=254 => text::warning,
                            _ => text::danger,
                    })
                    
                }),
                
                table::column(bold("Mood"), |doll: &doll::Doll| {
                    text(&doll.needs[1]).style( 
                        match &doll.needs[1] {
                            ..128 => text::default,
                            ..=254 => text::warning,
                            _ => text::danger,
                    })
                    
                }),

                table::column(bold("Energy"), |doll: &doll::Doll| {
                    text(&doll.needs[2]).style( 
                        match &doll.needs[2] {
                            ..128 => text::default,
                            ..=254 => text::warning,
                            _ => text::danger,
                    })
                    
                }),
            ];
            table(columns, &self.doll_list)
            .padding_x(15)
            .padding_y(5)
            .separator(2)
        };
        
        let header: Container< Message, Theme, Renderer> = {
            container( row![
                pick_list(Theme::ALL, self.theme.clone(), Message::ThemeChanged)
            ]
            .align_y(Center)
            
            )
            .align_x(Center)
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
                    scrollable(doll_table),
                    column![

                        button("Refresh Dolls").style(|theme: &Theme, status| {

                            match status {
                                _ => button::primary(theme, status)
                            }
                        })
                        .on_press(Message::FetchDolls),

                        button("Make Doll").style(|theme: &Theme, status| {

                            match status {
                                _ => button::primary(theme, status)
                            }
                        })
                    ]
                    ]
                ].spacing(10)
            )
            .padding(5)
            .width(Fill),
        }
        .width(Fill)
        .into()

    }
}