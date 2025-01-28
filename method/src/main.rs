struct Rect{
    height : u32,
    width : u32,
}
impl Rect{
    fn area(&self)->u32{
    self.height*self.width
    }
    fn hold(&self,other:&Rect)->bool
    {
        self.height >= other.height &&  self.height >= other.height
    }
}
fn main() {
    let rs = Rect{
        height : 10,
        width : 20,
    };
    let rs_2 = Rect{
        height : 10,
        width : 20,
    };
    println!("{}",rs.area());
    println!("{}",rs.hold(&rs_2));
}


