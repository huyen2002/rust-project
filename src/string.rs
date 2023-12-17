
fn convert_to_uppercase( s: String)->String{
  s.to_uppercase().to_string()
}

fn main(){
    let mut s = String::from(" Hello");
    s.push_str(", world!  11111");
   let y = convert_to_uppercase(s.clone()); 
   println!("{}", s);
   println!("{}", y);

   let z = &mut s;
   z.push_str("now");
   println!("{}", s);

   let t = &s;
   println!("{}", t);


   let temp: String = "Good morning".to_string();
   println!("{}", temp);

  }