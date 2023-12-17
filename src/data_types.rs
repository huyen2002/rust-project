fn add_value(mut des: i32, value:i32)->i32{
    des = des + value;
    return des;
}

struct Item {
    name: String,
    cost: u32
}
fn main() {
    let x : i32 = 2;
    println!("Value of variable x is: {}", x);

    let x = 3;
    println!("Shadowing variable x: {}", x);

    let y = add_value(x, 5); // copy value of x to func

    println!("{}", y);

    println!("{}", x); // can access x because x is not moved to add_value function
   
   let temp :i32 = 10;
   match temp {
    10 => println!("this is full point"),
    _ => println!("this is another point")
   }
}
