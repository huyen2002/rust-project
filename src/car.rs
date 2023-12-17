pub struct Car {
    name: String,
    cost: u32,
}
pub fn init_car(_name: String, _cost: u32) -> Car {
    Car {
        name: _name,
        cost: _cost,
    }
}

impl Car {
    pub fn print_info(&self) {
        println!("Info of car: {}", self.get_info())
    }
}

pub trait Summary {
    fn get_info(&self) -> String;
}

impl Summary for Car {
    fn get_info(&self) -> String {
        format!("name: {}, cost:{}", self.name, self.cost)
    }
}
