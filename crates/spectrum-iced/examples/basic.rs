//! Visual Iced adapter demo.

use iced::{
    Element, Fill,
    widget::{column, container, text},
};
use spectrum_core::Color;
use spectrum_iced::IcedColorAdapter;

fn main() -> iced::Result {
    iced::application(|| Demo, update, view).centered().run()
}

#[derive(Default)]
struct Demo;

fn update(_: &mut Demo, _: ()) {}

fn view(_: &Demo) -> Element<'_, ()> {
    let title = Color::new(125, 92, 255);
    let body = Color::new(232, 238, 247);

    container(column![
        text("Void Spectrum Iced").size(34).color(title.color()),
        text("Spectrum colors are converted through adapter traits.")
            .size(18)
            .color(body.color())
    ])
    .padding(32)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}
