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
pub trait TagTrait {
    fn tag(&self) -> &'static str;
}

macro_rules! define_tag {
    ($type:ty, $tag:ident) => {
        impl TagTrait for $type {
            fn tag(&self) -> &'static str { stringify!($tag) }
        }
    };
}

#[enum_dispatch(TagTrait)]
pub enum Any {
    Drawing(Drawing),
    Reset(Reset),
    Align(Align),
    Position(Position),
    Origin(Origin),
}

pub struct Drawing(pub u64);
define_tag!(Drawing, p);

pub struct Reset(pub Option<String>);
define_tag!(Reset, r);

pub struct Align(pub Alignment);
define_tag!(Align, an);

pub struct Position(pub Point);
define_tag!(Position, pos);

pub struct Origin(pub Point);
define_tag!(Origin, org);
