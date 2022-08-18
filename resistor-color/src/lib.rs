use int_enum::IntEnum;
use enum_iterator::{all, Sequence};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl ResistorColor {
    pub fn get_display_name (&self) -> &str {
        match self {
            Self::Black => "Black",
            Self::Brown => "Brown",
            Self::Red => "Red",
            Self::Orange => "Orange",
            Self::Yellow => "Yellow",
            Self::Green => "Green",
            Self::Blue => "Blue",
            Self::Violet => "Violet",
            Self::Grey => "Grey",
            Self::White => "White"
        }
    }
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    // _color.get_value()
    _color.int_value() as u32
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value as u8) {
        Ok(file) => format!("{:?}", file),
        Err(_) => format!("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
