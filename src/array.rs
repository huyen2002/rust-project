fn change_array(arr: &mut [i32]) -> &[i32]{
  // change arr by add 1 to each element
  for i in 0..arr.len(){
    arr[i] += 1;
  }
  return arr;
}

fn print_array(arr: &[i32]) {
  for item in arr.iter() {
    println!("{}", item)
  }
}
fn add_to_vec(vec: Vec<i32>) -> Vec<i32>{
  let mut vec = vec;
  vec.push(10); // vec can change size
  return vec;
}
fn main(){

  // note: array can't fix size. If you want to fix size => use Vec<T>
  // Vector store data in heap
  // Array store data in stack
  // Array is faster than Vector
  // Array can't change size
  // Vector can change size
  // Array can't be passed to function
  // Vector can be passed to function
  // Array can be passed to function by using reference
  // Vector can be passed to function by using reference
  
  let mut array_1 = [1,2,3,4];

  change_array(&mut array_1); // can pass array to function by using reference


  let array_2 = &array_1;

 print_array(&array_1);

 print_array(array_2);


 let vec_1 = vec![1,2];
 let vec_2 = add_to_vec(vec_1); // can pass vec to function
println!("{:?}", vec_2);
}