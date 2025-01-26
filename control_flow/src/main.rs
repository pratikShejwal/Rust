fn main() {
    let y = true;
   let x = if y {10} else {20}; //if else can be used as expression
   println!("{x}");

   //Loops
   let mut num = 0;
    loop{
        if num == 5{
            num = num+1;
            continue;
        }
        println!("{num}");
        if num==10{
            break;
        }
        num = num+1;  
    }

    //Example of expressions using loop
   let result = loop{ 
        println!("{num}");
        if num==10{
            break 10+1;
        }
        num = num+1;  
    };
    println!("{result}"); 

    //Loops with lables
    let mut num1 = 1;
    let mut new_num = 10;
   'outer_loop:loop{
        if(new_num == 5)
        {
            break'outer_loop;
        }
        new_num = new_num -1;
    }

    // while loop

    let mut nm = 3;
    while nm!=0{
        println!("{nm}");
        nm = nm-1;
    }
    // for loop
    let arr = [1,2,3,4,5];
    for element in arr{
        println!("{element}");
    }

    for x in 1..=10{
        println!("{x}");
    }
}


