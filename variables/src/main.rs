fn main() {
   let a:i32 = 28;
   println!("{a}");
   {
    let x = 32;
    let x = x +10;
    println!("{x}");
    println!(x);
   }
}
