fn main() {
   let str = String::from("Hello World");
let s = &str[0..5];
let t = &str[..];
println!("{s}");
// str.clear(); // cannot mutate String when reference is valid
println!("{t}");
}
