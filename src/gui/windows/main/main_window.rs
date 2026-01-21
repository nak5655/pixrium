use crate::core::logics::Console;
use crate::core::math::LatLon;
use crate::gui::components::canvas::canvas_view;
use crate::gui::components::docks::layer::layer_dock;
use crate::gui::components::toolbar;
use crate::gui::windows::main::main_menu;
use crate::FreyaServices;
use freya::prelude::*;
use std::f32::consts::PI;

pub fn main_window(mut console: State<Console<FreyaServices>>) -> impl IntoElement {
    let look_at = use_memo(move || LatLon::from(console.read().session.viewport_state.look_at));
    let fov = use_memo(move || console.read().session.viewport_state.fov.0);

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
        .child(
            rect()
                .main_align(Alignment::End)
                .padding((2., 4.))
                .spacing(12.)
                .direction(Direction::Horizontal)
                .height(Size::auto())
                .width(Size::fill())
                .child(format!(
                    "{:.2}°N, {:.2}°E",
                    look_at.read().lat * -180.0 / PI,
                    look_at.read().lon * 180.0 / PI
                ))
                .child(format!("FOV {:.2}°", *fov.read() * 180.0 / PI)),
        )
}
