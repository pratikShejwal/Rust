const ab = 32; // constant can be declare in global scope
fn main() {
   let mut a:i32 = 28; // mutable variable
   a = 2;
   println!("{a}")
   println!("{a}");
   {
    let x = 32;
    let x = x +10; // shadowing
    println!("{x}");
   }
}
