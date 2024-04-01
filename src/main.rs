
// #[warn(unused_assignments)]
use std::io;
// use std::mem::zeroed;
// std::fmt::Display;
fn main() {
//     let mut x= 4;  
//         println!("x is {}", x);
// //shadowing
//         {
//             let x=x + 1;
//             println!("x is {}", x);
//         }
 
//     x = x+3;
//     println!("x is {}", x+2);


//     const SEC_IN_MINS: u32 = 60;
//     println!("SEC_IN_MINS is {}", SEC_IN_MINS);

                        //PRIMITIVE DATA TYPES
// let _x = 2; 
//  let _true_false = false;
//  let letter: char = '9';

                        //TURPLES
// let mut tur: (i32,bool,char) = (10,false,'s');
// tur.0 = 10;
// println!("{}", tur.0);


                            //ARRAYS
// let mut arr:[i32;2] = [1,2];
// arr[0] = 10;
// println!("{}",arr[0]);

// let x:u32 = 4;
// let y = x;
// println!("{},{}",x,y);


/////////////////////////////CONSOLE INPUT /////////////
// let mut input = String::new();
// io::stdin().read_line(&mut input).expect("failed to read btn lines");
// println!("{}",input);



/////////////////////////////// ARITHMETIC AND TYPE CASTING ////////
// let x: f64 = 255.0;
// let y: f64 =  10.0;
// let z = x/y;
// println!("{}",z);

// let x = 25_000_i64;
// let y = 10_i32;
// let z = x / y as i64;
// println!("{}",z);    

// let mut input = String::new();
// io::stdin().read_line(&mut input).expect("msg");
// let int_input: i64 = input.trim().parse().unwrap();
// println!("{}",int_input);



//////////////////// CONDITIONS AND CONTROL FLOWS //////////////////
// let x = 2_f32 <= 2.2_f32;
// let y = !(false || x);
// println!("{}, {}",x,y);

let mut input  = String::new();
io::stdin().read_line(&mut input).expect("msg");
let int_input: i64 = input.trim().parse().unwrap();
if int_input == 1{
    println!("first number");
}
else if int_input == 2{
    println!("second number");
}
else if int_input == 3 {
    println!("third number");
} else{
    println!("Any other number");
    if int_input <= 9{
        let z:i32 = (9 as i64 - int_input).try_into().unwrap();
        println!("{} is less than 9 by {}", int_input, z);
    } 
    else {
        let z:i32 = (int_input - 9 as i64 ).try_into().unwrap();
        println!("{} is greater than 9 by {}", int_input, z);
    }

}

}
