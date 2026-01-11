use crate::core::logics::Console;
use crate::core::math::LatLon;
use crate::gui::components::canvas::canvas_view;
use crate::gui::components::docks::layer::layer_dock;
use crate::gui::windows::main::main_menu;
use crate::FreyaServices;
use freya::prelude::*;

pub fn main_window(mut console: State<Console<FreyaServices>>) -> impl IntoElement {
    let look_at = use_memo(move || LatLon::from(console.read().session.viewport_state.look_at));
    let fov = use_memo(move || console.read().session.viewport_state.fov.0);

    rect()
        .theme_background()
        .expanded()
        .direction(Direction::Vertical)
        .child(main_menu(console))
        .child(
            rect()
                .width(Size::fill())
                .height(Size::percent(90.0))
                .child(
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
        )
        .child(
            rect()
                .direction(Direction::Horizontal)
                .height(Size::fill())
                .width(Size::fill())
                .child(format!(
                    "LookAt {:.2}, {:.2}",
                    look_at.read().lat,
                    look_at.read().lon
                ))
                .child(format!(" | FOV {:.2}", fov.read())),
        )
}
