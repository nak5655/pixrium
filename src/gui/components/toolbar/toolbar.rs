use crate::core::input::InputSignal;
use crate::core::logics::tools::Tools;
use crate::gui::components::toggle_button::*;
use crate::gui::windows::main::MainState;
use crate::{FreyaServices, OutputChannel};
use freya::prelude::*;
use freya_radio::hooks::use_radio;
use std::sync::mpsc::Sender;

pub fn toolbar(mut input: State<Sender<InputSignal<FreyaServices>>>) -> impl IntoElement {
    let toolbar_radio = use_radio::<MainState, OutputChannel>(OutputChannel::Toolbar);

    rect()
        .width(Size::auto())
        .height(Size::fill())
        .child(
            ToggleButton::new()
                .child(svg(freya_icons::lucide::hand()))
                .on_select(move |_| _ = input.write().send(InputSignal::ChooseTool(Tools::Pan)))
                .selected(toolbar_radio.read().toolbar.active_tool == Tools::Pan),
        )
        .child(
            ToggleButton::new()
                .child(svg(freya_icons::lucide::brush()))
                .on_select(move |_| _ = input.write().send(InputSignal::ChooseTool(Tools::Brush)))
                .selected(toolbar_radio.read().toolbar.active_tool == Tools::Brush),
        )
}
