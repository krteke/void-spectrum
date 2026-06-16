//! Visual Iced adapter demo.

use iced::{
    Element, Fill,
    widget::{column, container, text},
};
use spectrum_core::{Color, Length, LengthUnit, Radius, ShadowLayer};
use spectrum_iced::{IcedColorAdapter, IcedLengthAdapter, IcedRadiusAdapter, IcedShadowAdapter};

fn main() -> iced::Result {
    iced::application(|| Demo, update, view).centered().run()
}

#[derive(Default)]
struct Demo;

fn update(_: &mut Demo, _: ()) {}

fn view(_: &Demo) -> Element<'_, ()> {
    let title = Color::new(125, 92, 255);
    let body = Color::new(232, 238, 247);
    let width = Length::new(520.0, LengthUnit::Px)
        .expect("finite")
        .length()
        .expect("px maps to Iced fixed length");
    let radius = Radius::new(Length::new(14.0, LengthUnit::Px).expect("finite"))
        .expect("non-negative")
        .radius()
        .expect("px maps to Iced radius");
    let shadow = ShadowLayer::new(
        Color::new_rgba(0, 0, 0, 96),
        Length::new(0.0, LengthUnit::Px).expect("finite"),
        Length::new(10.0, LengthUnit::Px).expect("finite"),
        Length::new(24.0, LengthUnit::Px).expect("finite"),
        Length::new(0.0, LengthUnit::Px).expect("finite"),
    )
    .expect("valid shadow")
    .shadow()
    .expect("px shadow maps to Iced shadow");

    container(column![
        text("Void Spectrum Iced").size(34).color(title.color()),
        text("Spectrum colors are converted through adapter traits.")
            .size(18)
            .color(body.color())
    ])
    .width(width)
    .style(move |_| {
        iced::widget::container::Style::default()
            .border(iced::Border {
                radius,
                ..iced::Border::default()
            })
            .shadow(shadow)
    })
    .padding(32)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}
