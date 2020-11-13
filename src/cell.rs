#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Food,
    Tail,
    Uptoleft,
    Uptoright,
    Downtoleft,
    Downtoright,
    Horizontal,
    Vertical,
    Headsup,
    Headsdown,
    Headsleft,
    Headsright,
}

impl Cell {
    pub fn match_to_symbol(&self) -> &'static str {
        match self {
            Self::Empty => " ",
            Self::Food => "X",
            Self::Tail => "×",
            Self::Uptoleft => "╗",
            Self::Uptoright => "╔",
            Self::Downtoleft => "╝",
            Self::Downtoright => "╚",
            Self::Horizontal => "═",
            Self::Vertical => "║",
            Self::Headsup => "^",
            Self::Headsdown => "V",
            Self::Headsleft => "<",
            Self::Headsright => ">",
        }
    }
}

pub const WALL: &'static str = "░";
pub const CEILING: &'static str = "░";
pub const TOP_LEFT_CORNER: &'static str = "▒";
pub const TOP_RIGHT_CORNER: &'static str = "▒";
pub const BOTTOM_LEFT_CORNER: &'static str = "▒";
pub const BOTTOM_RIGHT_CORNER: &'static str = "▒";
