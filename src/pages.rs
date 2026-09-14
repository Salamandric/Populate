pub mod creator;
pub mod breeder;
//Widgets used on multiple pages
use iced::{
     Element, Font, Renderer, Theme,
     font, widget::{Column, column, table, text},
};

use crate::{Message, Navigation};
use crate::doll::Doll;
pub use crate::pages::creator::{Creator,CreatorMessage};
pub use crate::pages::breeder::{Breeder,BreederMessage};

pub async fn page_select(pg: Navigation) {

}


fn doll_table(list: &Vec<Doll>) -> Column<'_, Message> {
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
        }).width(datawidth)
    ];
    column!(
    table(columns, list)
    .separator(2)
    )
}