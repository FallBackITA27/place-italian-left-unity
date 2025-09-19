#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum Color {
    Black,
    DarkGray,
    Gray,
    LightGray,
    White,
    DeepRed,
    Red,
    Orange,
    Gold,
    Yellow,
    LightYellow,
    DarkGreen,
    Green,
    LightGreen,
    DarkTeal,
    Teal,
    LightTeal,
    DarkBlue,
    Blue,
    Cyan,
    Indigo,
    LightIndigo,
    DarkPurple,
    Purple,
    LightPurple,
    DarkPink,
    Pink,
    LightPink,
    DarkBrown,
    Brown,
    Beige,
    Transparent,
    MediumGray,
    DarkRed,
    LightRed,
    DarkOrange,
    DarkGoldenrod,
    Goldenrod,
    LightGoldenrod,
    DarkOlive,
    Olive,
    LightOlive,
    DarkCyan,
    LightCyan,
    LightBlue,
    DarkIndigo,
    DarkSlateBlue,
    SlateBlue,
    LightSlateBlue,
    DarkPeach,
    Peach,
    LightPeach,
    LightBrown,
    DarkTan,
    Tan,
    LightTan,
    DarkBeige,
    LightBeige,
    DarkStone,
    Stone,
    LightStone,
    DarkSlate,
    Slate,
    LightSlate,
}

impl Color {
    pub fn is_premium(&self) -> bool {
        match self {
            Self::Black
            | Self::DarkGray
            | Self::Gray
            | Self::LightGray
            | Self::White
            | Self::DeepRed
            | Self::Red
            | Self::Orange
            | Self::Gold
            | Self::Yellow
            | Self::LightYellow
            | Self::DarkGreen
            | Self::Green
            | Self::LightGreen
            | Self::DarkTeal
            | Self::Teal
            | Self::LightTeal
            | Self::DarkBlue
            | Self::Blue
            | Self::Cyan
            | Self::Indigo
            | Self::LightIndigo
            | Self::DarkPurple
            | Self::Purple
            | Self::LightPurple
            | Self::DarkPink
            | Self::Pink
            | Self::LightPink
            | Self::DarkBrown
            | Self::Brown
            | Self::Beige
            | Self::Transparent => false,
            Self::MediumGray
            | Self::DarkRed
            | Self::LightRed
            | Self::DarkOrange
            | Self::DarkGoldenrod
            | Self::Goldenrod
            | Self::LightGoldenrod
            | Self::DarkOlive
            | Self::Olive
            | Self::LightOlive
            | Self::DarkCyan
            | Self::LightCyan
            | Self::LightBlue
            | Self::DarkIndigo
            | Self::DarkSlateBlue
            | Self::SlateBlue
            | Self::LightSlateBlue
            | Self::DarkPeach
            | Self::Peach
            | Self::LightPeach
            | Self::LightBrown
            | Self::DarkTan
            | Self::Tan
            | Self::LightTan
            | Self::DarkBeige
            | Self::LightBeige
            | Self::DarkStone
            | Self::Stone
            | Self::LightStone
            | Self::DarkSlate
            | Self::Slate
            | Self::LightSlate => true,
        }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Self::Black => String::from("Black"),
            Self::DarkGray => String::from("Dark Gray"),
            Self::Gray => String::from("Gray"),
            Self::LightGray => String::from("Light Gray"),
            Self::White => String::from("White"),
            Self::DeepRed => String::from("Deep Red"),
            Self::Red => String::from("Red"),
            Self::Orange => String::from("Orange"),
            Self::Gold => String::from("Gold"),
            Self::Yellow => String::from("Yellow"),
            Self::LightYellow => String::from("Light Yellow"),
            Self::DarkGreen => String::from("Dark Green"),
            Self::Green => String::from("Green"),
            Self::LightGreen => String::from("Light Green"),
            Self::DarkTeal => String::from("Dark Teal"),
            Self::Teal => String::from("Teal"),
            Self::LightTeal => String::from("Light Teal"),
            Self::DarkBlue => String::from("Dark Blue"),
            Self::Blue => String::from("Blue"),
            Self::Cyan => String::from("Cyan"),
            Self::Indigo => String::from("Indigo"),
            Self::LightIndigo => String::from("Light Indigo"),
            Self::DarkPurple => String::from("Dark Purple"),
            Self::Purple => String::from("Purple"),
            Self::LightPurple => String::from("Light Purple"),
            Self::DarkPink => String::from("Dark Pink"),
            Self::Pink => String::from("Pink"),
            Self::LightPink => String::from("Light Pink"),
            Self::DarkBrown => String::from("Dark Brown"),
            Self::Brown => String::from("Brown"),
            Self::Beige => String::from("Beige"),
            Self::Transparent => String::from("Transparent"),
            Self::MediumGray => String::from("Medium Gray"),
            Self::DarkRed => String::from("Dark Red"),
            Self::LightRed => String::from("Light Red"),
            Self::DarkOrange => String::from("Dark Orange"),
            Self::DarkGoldenrod => String::from("Dark Goldenrod"),
            Self::Goldenrod => String::from("Goldenrod"),
            Self::LightGoldenrod => String::from("Light Goldenrod"),
            Self::DarkOlive => String::from("Dark Olive"),
            Self::Olive => String::from("Olive"),
            Self::LightOlive => String::from("Light Olive"),
            Self::DarkCyan => String::from("Dark Cyan"),
            Self::LightCyan => String::from("Light Cyan"),
            Self::LightBlue => String::from("Light Blue"),
            Self::DarkIndigo => String::from("Dark Indigo"),
            Self::DarkSlateBlue => String::from("Dark Slate Blue"),
            Self::SlateBlue => String::from("Slate Blue"),
            Self::LightSlateBlue => String::from("Light Slate Blue"),
            Self::DarkPeach => String::from("Dark Peach"),
            Self::Peach => String::from("Peach"),
            Self::LightPeach => String::from("Light Peach"),
            Self::LightBrown => String::from("Light Brown"),
            Self::DarkTan => String::from("Dark Tan"),
            Self::Tan => String::from("Tan"),
            Self::LightTan => String::from("Light Tan"),
            Self::DarkBeige => String::from("Dark Beige"),
            Self::LightBeige => String::from("Light Beige"),
            Self::DarkStone => String::from("Dark Stone"),
            Self::Stone => String::from("Stone"),
            Self::LightStone => String::from("Light Stone"),
            Self::DarkSlate => String::from("Dark Slate"),
            Self::Slate => String::from("Slate"),
            Self::LightSlate => String::from("Light Slate"),
        }
    }
}

impl TryFrom<[u8; 4]> for Color {
    type Error = ();
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        match value {
            [_, _, _, alpha] if alpha != u8::MAX => Err(()),
            [0x00, 0x00, 0x00, _] => Ok(Self::Black),
            [0x3C, 0x3C, 0x3C, _] => Ok(Self::DarkGray),
            [0x78, 0x78, 0x78, _] => Ok(Self::Gray),
            [0xD2, 0xD2, 0xD2, _] => Ok(Self::LightGray),
            [0xFF, 0xFF, 0xFF, _] => Ok(Self::White),
            [0x60, 0x00, 0x18, _] => Ok(Self::DeepRed),
            [0xED, 0x1C, 0x24, _] => Ok(Self::Red),
            [0xFF, 0x7F, 0x27, _] => Ok(Self::Orange),
            [0xF6, 0xAA, 0x09, _] => Ok(Self::Gold),
            [0xF9, 0xDD, 0x3B, _] => Ok(Self::Yellow),
            [0xFF, 0xFA, 0xBC, _] => Ok(Self::LightYellow),
            [0x0E, 0xB9, 0x68, _] => Ok(Self::DarkGreen),
            [0x13, 0xE6, 0x7B, _] => Ok(Self::Green),
            [0x87, 0xFF, 0x5E, _] => Ok(Self::LightGreen),
            [0x0C, 0x81, 0x6E, _] => Ok(Self::DarkTeal),
            [0x10, 0xAE, 0x82, _] => Ok(Self::Teal),
            [0x13, 0xE1, 0xBE, _] => Ok(Self::LightTeal),
            [0x60, 0xF7, 0xF2, _] => Ok(Self::DarkBlue),
            [0x28, 0x50, 0x9E, _] => Ok(Self::Blue),
            [0x40, 0x93, 0xE4, _] => Ok(Self::Cyan),
            [0x6B, 0x50, 0xF6, _] => Ok(Self::Indigo),
            [0x99, 0xB1, 0xFB, _] => Ok(Self::LightIndigo),
            [0x78, 0x0C, 0x99, _] => Ok(Self::DarkPurple),
            [0xAA, 0x38, 0xB9, _] => Ok(Self::Purple),
            [0xE0, 0x9F, 0xF9, _] => Ok(Self::LightPurple),
            [0xCB, 0x00, 0x7A, _] => Ok(Self::DarkPink),
            [0xEC, 0x1F, 0x80, _] => Ok(Self::Pink),
            [0xF3, 0x8D, 0xA9, _] => Ok(Self::LightPink),
            [0x68, 0x46, 0x34, _] => Ok(Self::DarkBrown),
            [0x95, 0x68, 0x2A, _] => Ok(Self::Brown),
            [0xF8, 0xB2, 0x77, _] => Ok(Self::Beige),
            [0xDE, 0xFA, 0xCE, _] => Ok(Self::Transparent),
            [0xAA, 0xAA, 0xAA, _] => Ok(Self::MediumGray),
            [0xA5, 0x0E, 0x1E, _] => Ok(Self::DarkRed),
            [0xFA, 0x80, 0x72, _] => Ok(Self::LightRed),
            [0xE4, 0x5C, 0x1A, _] => Ok(Self::DarkOrange),
            [0x9C, 0x84, 0x31, _] => Ok(Self::DarkGoldenrod),
            [0xC5, 0xAD, 0x31, _] => Ok(Self::Goldenrod),
            [0xE8, 0xD4, 0x5F, _] => Ok(Self::LightGoldenrod),
            [0x4A, 0x6B, 0x3A, _] => Ok(Self::DarkOlive),
            [0x5A, 0x94, 0x4A, _] => Ok(Self::Olive),
            [0x84, 0xC5, 0x73, _] => Ok(Self::LightOlive),
            [0x0F, 0x79, 0x9F, _] => Ok(Self::DarkCyan),
            [0xBB, 0xFA, 0xF2, _] => Ok(Self::LightCyan),
            [0x7D, 0xC7, 0xFF, _] => Ok(Self::LightBlue),
            [0x4D, 0x31, 0xB8, _] => Ok(Self::DarkIndigo),
            [0x4A, 0x42, 0x84, _] => Ok(Self::DarkSlateBlue),
            [0x7A, 0x71, 0xC4, _] => Ok(Self::SlateBlue),
            [0xB5, 0xAE, 0xF1, _] => Ok(Self::LightSlateBlue),
            [0x9B, 0x52, 0x49, _] => Ok(Self::DarkPeach),
            [0xD1, 0x80, 0x78, _] => Ok(Self::Peach),
            [0xFA, 0xB6, 0xA4, _] => Ok(Self::LightPeach),
            [0xDB, 0xA4, 0x63, _] => Ok(Self::LightBrown),
            [0x7B, 0x63, 0x52, _] => Ok(Self::DarkTan),
            [0x9C, 0x84, 0x6B, _] => Ok(Self::Tan),
            [0xD6, 0xB5, 0x94, _] => Ok(Self::LightTan),
            [0xD1, 0x80, 0x51, _] => Ok(Self::DarkBeige),
            [0xFF, 0xC5, 0xA5, _] => Ok(Self::LightBeige),
            [0x6D, 0x64, 0x3F, _] => Ok(Self::DarkStone),
            [0x94, 0x8C, 0x6B, _] => Ok(Self::Stone),
            [0xCD, 0xC5, 0x9E, _] => Ok(Self::LightStone),
            [0x33, 0x39, 0x41, _] => Ok(Self::DarkSlate),
            [0x6D, 0x75, 0x8D, _] => Ok(Self::Slate),
            [0xB3, 0xB9, 0xD1, _] => Ok(Self::LightSlate),
            _ => Err(()),
        }
    }
}
