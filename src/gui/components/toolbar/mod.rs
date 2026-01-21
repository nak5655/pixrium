use crate::core::logics::tools::Tools;
use crate::core::logics::Console;
use crate::FreyaServices;
use freya::prelude::*;

use crate::gui::components::toggle_button::*;

pub fn toolbar(mut console: State<Console<FreyaServices>>) -> impl IntoElement {
    let active_tool = use_memo(move || console.read().active_tool);

    rect()
        .width(Size::auto())
        .height(Size::fill())
        .child(
            ToggleButton::new()
                .child(svg(freya_icons::lucide::hand()))
                .on_select(move |_| console.write().active_tool = Tools::Pan)
                .selected(*active_tool.read() == Tools::Pan),
        )
        .child(
            ToggleButton::new()
                .child(svg(freya_icons::lucide::brush()))
                .on_select(move |_| console.write().active_tool = Tools::Brush)
                .selected(*active_tool.read() == Tools::Brush),
        )
}
