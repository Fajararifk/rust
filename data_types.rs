use std::io;
//let guess: u32 = "42".parse().expect("Not a number!");

// let guess = "42".parse().expect("Not a number!");
// |         ^^^^^        ----- type must be known at this point
// |
// = note: cannot satisfy `<_ as FromStr>::Err == _`
// help: consider giving `guess` an explicit type
// |
// 2;\\|     let guess: /* Type */ = "42".parse().expect("Not a number!");
// | 
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);
    // FLOATING-POINT TYPES
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{} {}", x, y);
    let a = 2.0; // f64
    let b: f32 = 3.0; // f32
    println!("{}",a+b);

    //numeric operation
    let sum = 5 + 10;
    println!("sum : {}", sum);
    let difference = 95.5 - 4.3;
    println!("difference : {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("product : {}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("quotient : {}", quotient);
    let truncated = -5 / 3; // Results in -1
    println!("truncated : {}", truncated);
    // remainder
    let remainder = 43 % 5;
    println!("remainder : {}", remainder);

    // BOOLEAN
    let t = true;
    let f: bool = false;
    println!("t : {}", t);
    println!("f : {}", f);


    // CHARACTER
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat : {}", heart_eyed_cat);
    println!("Z : {}", z);
    println!("c : {}", c);

    // Macam-macam Compound types (TIPE DATA YANG DI KELOMPOKKAN MENJADI 1 UNIT)
    // TUPLE TYPE

    //let tup: (i32,f64,u8) = (500,6.4,1);
    //println!("{}", tup); //^^^ `(i32, f64, u8)` cannot be formatted with the default formatter

    let tuple = (500, 6.4, 1);

    let (x, y, z) = tuple;

    println!("The value of y is: {y}");

    let xyz: (i32, f64, u8) = (500, 6.4, 1);
    //println!("The value of tuple is {}",xyz); //^^^ `(i32, f64, u8)` cannot be formatted with the default formatter
    let five_hundred = xyz.0;
    println!("five hundred is {}",five_hundred);
    let six_point_four = xyz.1;
    println!("six point four is {}",six_point_four);
    let one = xyz.2;
    println!("one is {}",one);

    //ARRAY TYPE
    let array = [1,2,3,4,5,6,7,8,9,10];
    let first_array = array[0];
    let last_array = array[array.len() - 1];
    println!("first array is {}",first_array);
    println!("last array is {}",last_array);
    println!("All arrays are {:?}" , array ); //harus pake :?
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("All months are {:?}" , months);
    let array_fix: [i32; 5] = [1, 2, 3, 4, 5];
    println!("fix indexing : {:?}", array_fix);
    let array_value_index = [3; 5];
    println!("fix value and index : {:?}", array_value_index);


    let mut index = String::new();
    println!("Please input your index arrays : ");
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = array[index];

    println!("The value of the element at index {index} is: {element}");
}

