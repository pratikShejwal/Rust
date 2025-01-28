#[derive(Debug)]
struct Rect{
    height : u32,
    width : u32,
}

struct User{
    name: String,
    email:String,
    password:u8,
    active:bool,

}
fn main() {  
    let rs = Rect{
        height : 10,
        width : 20,
    };
    let area = rect_area(&rs);
    println!("are of {:?} is {}",rs,area);
   
let mut user_1 = User{
    name:String::from("Pratik"),
    active:true,
    password:123,
    email:String::from("pratikshejwal88@gmail.com"),
};

//to move user_1 value to new user_2
let user_2 =User{
name:String::from("Harsh"),
..user_1 // automatically get all values of user_1 into user_2
};

print!("{}",user_1.name);
user_1.name = String::from("Raven");
let new_user = create_user(String::from("Ritik"));
let new_user1 = create_user(String::from("Pratik"));
println!("{}", new_user.name);
println!("{}", new_user1.name);
}
fn create_user(name:String)->User{
    User{
    name, // if parameter name is same as struct key we can write only once
    active:true,
    password:123,
    email:String::from("p@gmail.com"),
    }
    

    
}

fn rect_area(rs:&Rect)->u32{
    rs.height * rs.width
}