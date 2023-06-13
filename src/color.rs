#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White,
    Black,
}

pub const WHITE: Color = Color::White;
pub const BLACK: Color = Color::Black;

impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::White => "White",
                Self::Black => "Black",
            }
        )
    }
}

//A color can be inverted using the "!" operator.
impl core::ops::Not for Color {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}