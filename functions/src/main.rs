fn main() {
    let x = add();
    println!("{x}");
    my_function(2,3); // calling a function is expression
    odd_even(4);
}

fn my_function(x:i32, y:i32){ // creating function is statement
    let a = x+y;
    println!("additon {a}");
}

fn add()->i32{ // function returns last expression 
    20
}

fn odd_even(x:i32)-> bool{ // while defining functions we must have to specify parameter type
    if x % 2 ==0 {
        println!("Number is Even");
        return true;
    }
    println!("Number is Odd");
    false
}



