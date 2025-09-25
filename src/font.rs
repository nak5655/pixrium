#[cfg(windows)]
pub const UI_FONT_BYTES: &[u8] = include_bytes!("..\\resources\\fonts\\NotoSans-Medium.ttf");
#[cfg(unix)]
pub const UI_FONT_BYTES: &[u8] = include_bytes!("../resources/fonts/NotoSans-Medium.ttf");

#[cfg(windows)]
pub const MONO_FONT_BYTES: &[u8] = include_bytes!("..\\resources\\fonts\\NotoSansMono-Medium.ttf");
#[cfg(unix)]
pub const MONO_FONT_BYTES: &[u8] = include_bytes!("../resources/fonts/NotoSansMono-Medium.ttf");

#[cfg(windows)]
pub const ICON_FONT_BYTES: &[u8] = include_bytes!("..\\resources\\fonts\\tabler-icons.ttf");
#[cfg(unix)]
pub const ICON_FONT_BYTES: &[u8] = include_bytes!("../resources/fonts/tabler-icons.ttf");

pub const ICON_FONT_NAME: &'static str = "tabler-icons";
pub const FONT_NAME: &'static str = "Noto Sans";
pub const FONT_NAME_MONO: &'static str = "Noto Sans Mono";

pub fn icon_font() -> iced::Font {
    iced::Font {
        weight: iced::font::Weight::Normal,
        family: iced::font::Family::Name(ICON_FONT_NAME),
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    }
}

pub fn mono_font() -> iced::Font {
    iced::Font {
        weight: iced::font::Weight::Normal,
        family: iced::font::Family::Name(FONT_NAME_MONO),
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    }
}
