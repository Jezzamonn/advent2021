#[derive(Debug)]
#[derive(PartialEq)]
pub enum Bracket {
    Round,
    Square,
    Curly,
    Angle,
}

impl Bracket {
    pub fn from_opening_char(c: char) -> Option<Self> {
        match c {
            '(' => Some(Self::Round),
            '[' => Some(Self::Square),
            '{' => Some(Self::Curly),
            '<' => Some(Self::Angle),
            _ => None,
        }
    }

    pub fn from_closing_char(c: char) -> Option<Self> {
        match c {
            ')' => Some(Self::Round),
            ']' => Some(Self::Square),
            '}' => Some(Self::Curly),
            '>' => Some(Self::Angle),
            _ => None,
        }
    }

    pub fn opening_char(&self) -> char {
        match self {
            Self::Round => '(',
            Self::Square => '[',
            Self::Curly => '{',
            Self::Angle => '<',
        }
    }

    pub fn closing_char(&self) -> char {
        match self {
            Self::Round => ')',
            Self::Square => ']',
            Self::Curly => '}',
            Self::Angle => '>',
        }
    }

    /// Arbitrary score provided by the problem.
    pub fn score_if_invalid(&self) -> i32 {
        match self {
            Self::Round => 3,
            Self::Square => 57,
            Self::Curly => 1197,
            Self::Angle => 25137,
        }
    }

    pub fn score_if_unmatched(&self) -> i32 {
        match self {
            Self::Round => 1,
            Self::Square => 2,
            Self::Curly => 3,
            Self::Angle => 4,
        }
    }
}