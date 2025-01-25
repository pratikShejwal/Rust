fn main() {
    let i:i8 = 32; // range will be 2^n-1 to (2^n-1) - 1
   let j:u8 = 2;// range 0 to 2^n-1

//let overflow:i8 = 456; // if we execute in production release i.e cargo run --release it performs two's complement wrapping
    // if we execute in debug mode it will throw error

let random_num:i32 = 400;
let overflow_integer = random_num.wrapping_add(2); // code will run in debug mode
let overflow_integer =match random_num.checked_add(2){
    Some(num) => num,
    None =>
    {
        println!("Can't add");
        return;
    }
};
    println!("{i}");

    //Floating Numbers f32 and f64
    let float_num = 3.0; // by default type is f64,and more precision
    let float_num2:f32 = 2.0;
    println!("{float_num}");
    println!("{float_num2}");

    let result:f32 = 5_f32/3_f32;
    println!("{result}");

    //Characters in Rust
let ch:char = 'p';
println!("{ch}");

//Compount Types

//Tuples
let tup:(f64,i32,u8) = (2.0,56,3); // grouping different types of values, tuple is immutable;

//Accessing Tuple

let (val1,val2,val3) = tup;
println!("Value 1 is {val1}"); //2.0
println!("Value 2 is {val2}"); //56
println!("Value 3 is {val3}"); //3

println!("Value 1 is {}",tup.0); //2.0
println!("Value 2 is {}",tup.1); //56
println!("Value 3 is {}",tup.2); //3

//Array

let arr = [1,2,3,4,5];
let new_arr = [1;5]; //[1,1,1,1,1]
println!("{}",arr[0]);

}
