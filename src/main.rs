mod variable;
mod tuple_array;
fn main() {
    variable :: variable();
    tuple_array :: compund();
    
    println!("Hello, world!");
    //variables.......................................................

    // //Number...................................
    // let x  = -3;
    // let y: u32 = 4;
    // let z = 5.0;
    // let a: i8 = 127;

    // //printing values of variables
    // print!("x:{},y:{},z:{},a:{}",x,y,z,a);
    // println!("");
    // println!("x:{}",x);
    // println!("y:{}",y);
    // println!("z:{}",z);
    // println!("a:{}",a);

    // //mutability because of "mut" keyword
    // let mut b=10;
    // println!("b:{}",b);

    // b = 30;           //we can cange value but not type we cant do this b = true
    // println!("b:{}",b);

    // //always immutable
    // const CONST: u8 = 10;
    // println!("CONST:{}",CONST);

    // //Booleans........................................
    // let is_valid_age = true;
    // if is_valid_age {
    //     println!("you are eligible to vote");
    // }else {
    //     println!("you are not eligible to vote");    
    // }

    // //String............................................
    // let greeting="Hello Anurag Welcome To The Rust World";
    // println!("{}",greeting)

}
