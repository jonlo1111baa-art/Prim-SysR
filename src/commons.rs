//! ### Commons
//! 
//! in this module common structs and enums are held
//! for use in other modules and crates
//! Note: Remember to move to external crate

pub struct Twocoord{
    pub x : i64,
    pub y : i64
}
pub struct UMOinput{
    pub a : i64,
    pub b : i64,
    pub op : String
}
pub struct Opstrings{
    pub sum : String,
    pub minus : String,
    pub times : String,
    pub quatient : String
}
