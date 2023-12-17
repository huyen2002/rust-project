use std::{
    any::type_name,
    fmt::{self},
};


#[derive(Debug)]

pub struct Point<T> {
    x: T, // accept i32 and float
    y: T,
}

impl<T: fmt::Display> Point<T> {
    fn get_info(&self) -> String {
        format!("x: {}, y: {}", self.x, self.y)
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn check_point<T: Copy>(point: &Point<T>) -> Result<String, String>
where
    T: fmt::Display,
{
    if type_of(point.x) == "i32" && type_of(point.y) == "i32" {
        return Ok(point.get_info());
    }

    Err(CreateTypeError::InvalidType.to_string())
}
impl fmt::Display for CreateTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyType => write!(f, "variable is empty"),
            Self::InvalidType => write!(f, "variable is invalid type"),
        }
    }
}

impl<T> Point<T> {
    fn init(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub fn add_point<T>(point_1: Point<T>, point_2: Point<T>) -> Point<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    Point::init(point_1.x + point_2.x, point_1.y + point_2.y)
}
#[derive(Debug)]
enum CreateTypeError {
    InvalidType,
    EmptyType,
}

fn main() {
    let point_1 = Point::init(1, 2);
    let point_2 = Point::init("1", "2");
    let point_3 = Point::init(3, 4);

    let check_1 = check_point(&point_1);
    let check_2 = check_point(&point_2);

    println!("Check 1: {:?}", check_1);
    println!("Check 2: {:?}", check_2);
    println!("{}", CreateTypeError::EmptyType.to_string());

    let result = add_point(point_1, point_3);
    println!("Result: {}", result.get_info());
}

// #[cfg(test)]
// mod tests{
//     use crate::{Point, add_point, check_point, CreateTypeError};

//     #[test]
//     fn check_add_point(){
//         let point_1 = Point::init(1, 2);
//         let point_2=Point::init(3, 4);
//         assert_eq!(add_point(point_1, point_2), Point::init(4, 6));
//     }

//     #[test]
//     fn check_invalid_type_point(){
//         let point_1 = Point::init("1", "2");
//         assert_eq!(check_point(&point_1), Err(CreateTypeError::InvalidType.to_string()));
//     }

//     #[test]
//     fn check_valid_type_point(){
//         let point = Point::init(1, 2);
//         assert_eq!(check_point(&point), Ok(point.get_info()));
//     }
// }
impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
