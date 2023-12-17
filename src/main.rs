mod person;
use person::init_person;
mod car;
use car::init_car;
mod activity;
use activity::{Activity, TypeShoes};
mod errors_handle;
pub mod point;

fn main() {
    let car = init_car("Lexus".to_string(), 1000);
    let person_1 = init_person(
        "Huyen".to_string(),
        21,
        car,
        "male".to_string(),
        (90, 60, 90),
        TypeShoes::Sneaker,
        Activity::Walk {
            v: 10,
            street: "Nguyen Trai".to_string(),
        },
    );

    person_1.check_gender();
    person_1.print_info();
}
