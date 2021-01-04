use enum_dispatch::enum_dispatch;

#[repr(u8)]
pub enum Alignment {
    LE = 1,
    NE = 2,
    CE = 3,
    LN = 4,
    NN = 5,
    CN = 6,
    LG = 7,
    NG = 8,
    CG = 9,
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[enum_dispatch]
pub trait TagTrait {}

#[enum_dispatch(TagTrait)]
pub enum Any {
    Drawing(Drawing),
    Reset(Reset),
    Align(Align),
    Position(Position),
    Origin(Origin),
}

pub struct Drawing(pub u64);
impl TagTrait for Drawing {}

pub struct Reset(pub Option<String>);
impl TagTrait for Reset {}

pub struct Align(pub Alignment);
impl TagTrait for Align {}

pub struct Position(pub Point);
impl TagTrait for Position {}

pub struct Origin(pub Point);
impl TagTrait for Origin {}
