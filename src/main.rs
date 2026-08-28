#[cfg(test)]
mod test_functions;

mod doll;
mod sqlhandler;

use iced::{
     Element, Font, Length::Fill, Renderer, Subscription, Task, Theme, font, overlay::menu::State, widget::{button, column, container, row, scrollable, table, text},
};

use crate::{sqlhandler::SqlHandler};



fn main() -> iced::Result {
    
    

    iced::application(Populate::new, Populate::update, Populate::view)
    
    .run()
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
}
struct Populate {
    doll_handler: SqlHandler,
    page: Page,
    doll_list: Vec<doll::Doll>
}

impl Populate {

    fn new() -> Self {
    let handler = SqlHandler::new();
    let list = handler.list_dolls();

    let app = Populate {
        doll_handler: handler,
        page: Page::Creator,
        doll_list: list
        
    };
    app
}

    fn update(state: &mut Self, message: Message) -> Task<Message> {
        match message {
            Message::GoToCreator => {
                state.page = Page::Creator;
                Task::none()
            }
            Message::FetchDolls => {
                state.doll_handler.list_dolls();
                Task::none()
            },
            Message::DollsFetched(dolls) => {
                state.doll_list = dolls;
                Task::none()
            },
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
            .padding_x(10)
            .padding_y(5)
            .separator(1)
        };

        match self.page {

            //Doll Creation Page
            Page::Creator => container(row![
                scrollable(doll_table).spacing(10),
                column![
                    button("Refresh List").on_press(Message::FetchDolls),
                    button("New Doll")
                ]
            ])
            .padding(10)
            .align_left(Fill)
            ,
        }.into()

    }

    async fn make_dolls(num: u32) {
        
    }
}