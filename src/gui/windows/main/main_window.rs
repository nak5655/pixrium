use crate::core::input::InputSignal;
use crate::core::math::LatLon;
use crate::gui::components::canvas::canvas_view;
use crate::gui::components::toolbar;
use crate::gui::windows::main::{main_menu, MainState};
use crate::{FreyaServices, OutputChannel};
use freya::prelude::*;
use freya_radio::prelude::use_radio;
use glam::Vec3;
use std::f32::consts::PI;
use std::sync::mpsc::Sender;

pub fn main_window(input: State<Sender<InputSignal<FreyaServices>>>) -> impl IntoElement {
    let canvas_radio = use_radio::<MainState, OutputChannel>(OutputChannel::Canvas);

    rect()
        .theme_background()
        .expanded()
        .content(Content::Flex)
        .direction(Direction::Vertical)
        .child(main_menu(input))
        .child(
            rect()
                .expanded()
                .height(Size::flex(1.))
                .direction(Direction::Horizontal)
                .child(toolbar(input))
                .child(
                    rect().expanded().child(
                        ResizableContainer::new()
                            .direction(Direction::Horizontal)
                            .panel(ResizablePanel::new(70.0).child(canvas_view(input)))
                            .panel(ResizablePanel::new(30.0).child(
                                rect().direction(Direction::Vertical).width(Size::fill()), //.child(layer_dock(console)),
                            )),
                    ),
                ),
        )
        .maybe_child(canvas_radio.read().canvas.as_ref().map(|canvas| {
            rect()
                .main_align(Alignment::End)
                .padding((2., 4.))
                .spacing(12.)
                .direction(Direction::Horizontal)
                .height(Size::auto())
                .width(Size::fill())
                .child(format!(
                    "{:.2}°N, {:.2}°E",
                    <Vec3 as Into<LatLon>>::into(canvas.look_at).lat * -180.0 / PI,
                    <Vec3 as Into<LatLon>>::into(canvas.look_at).lon * 180.0 / PI
                ))
                .child(format!("FOV {:.2}°", canvas.fov * 180.0 / PI))
        }))
}
