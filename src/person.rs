use crate::activity::{Activity, TypeShoes};

use crate::car::{Car, Summary};

pub struct Person {
    name: String,
    age: u8,
    car: Car,
    gender: String,
    measurements: (u8, u8, u8), //tuples
    shoes: TypeShoes,
    activity: Activity,
}

pub fn init_person(
    _name: String,
    _age: u8,
    _car: Car,
    _gender: String,
    (_chest, _waist, _hip): (u8, u8, u8),
    _shoes: TypeShoes,
    _activity: Activity,
) -> Person {
    Person {
        name: _name,
        age: _age,
        car: _car,
        gender: _gender,
        measurements: (_chest, _waist, _hip),
        shoes: _shoes,
        activity: _activity,
    }
}
impl Person {
    pub fn check_gender(&self) {
        if self.gender == "male" {
            println!("this is man")
        } else {
            println!("this is woman")
        }
    }

    pub fn print_info(&self) {
        println!("person: {}", self.get_info());
        self.car.print_info();
    }
}

impl Summary for Person {
    fn get_info(&self) -> String {
        format!(
            "Name:{}, Age:{}, Gender:{}, Measurements:{}-{}-{}, Shoes:{}, Activity:{}",
            self.name,
            self.age,
            self.gender,
            self.measurements.0,
            self.measurements.1,
            self.measurements.2,
            self.shoes.to_string(),
            self.activity.to_string()
        )
    }
}
