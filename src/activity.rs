#![allow(dead_code)]

use std::fmt;

pub enum TypeShoes {
    Sneaker,
    Sandal,
    Boot,
}

pub enum Activity {
    Say(String),
    Cry,
    Sleep,
    Smile,
    Walk { v: u32, street: String },
}
impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cry => write!(f, "cry"),
            Self::Sleep => write!(f, "sleep"),
            Self::Smile => write!(f, "smile"),
            Self::Say(s) => write!(f, "say: {}", s),
            Self::Walk { v, street } => {
                let walk_str = format!("walk with {} km/h on {} street", v, street);
                write!(f, "{}", walk_str)
            }
        }
    }
}
impl fmt::Display for TypeShoes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Sneaker => "sneaker",
                Self::Sandal => "sandal",
                Self::Boot => "boot",
            }
        )
    }
}
