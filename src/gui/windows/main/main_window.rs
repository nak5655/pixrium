use crate::core::logics::Console;
use crate::core::math::LatLon;
use crate::gui::components::canvas::canvas_view;
use crate::gui::components::docks::layer::layer_dock;
use crate::gui::components::toolbar;
use crate::gui::windows::main::main_menu;
use crate::FreyaServices;
use freya::prelude::*;
use glam::Vec3;
use std::f32::consts::PI;

pub fn main_window(mut console: State<Console<FreyaServices>>) -> impl IntoElement {
    rect()
        .theme_background()
        .expanded()
        .content(Content::Flex)
        .direction(Direction::Vertical)
        .child(main_menu(console))
        .child(
            rect()
                .expanded()
                .height(Size::flex(1.))
                .direction(Direction::Horizontal)
                .child(toolbar(console))
                .child(
                    rect().expanded().child(
                        ResizableContainer::new()
                            .direction(Direction::Horizontal)
                            .panel(ResizablePanel::new(70.0).child(canvas_view(console)))
                            .panel(
                                ResizablePanel::new(30.0).child(
                                    rect()
                                        .direction(Direction::Vertical)
                                        .width(Size::fill())
                                        .child(layer_dock(console)),
                                ),
                            ),
                    ),
                ),
        )
        .maybe_child(console.read().session.as_ref().map(|session| {
            rect()
                .main_align(Alignment::End)
                .padding((2., 4.))
                .spacing(12.)
                .direction(Direction::Horizontal)
                .height(Size::auto())
                .width(Size::fill())
                .child(format!(
                    "{:.2}°N, {:.2}°E",
                    <Vec3 as Into<LatLon>>::into(session.look_at()).lat * -180.0 / PI,
                    <Vec3 as Into<LatLon>>::into(session.look_at()).lon * 180.0 / PI
                ))
                .child(format!("FOV {:.2}°", session.fov() * 180.0 / PI))
        }))
}
