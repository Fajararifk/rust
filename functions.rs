fn main() {
    println!("Hello, world!");

    another_function();
    another_function_param(5);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1 //JANGAN PAKE SEMIKOLON
    };
    println!("The value of y is: {}", y);
    
    let lima = five();
    println!("The value of lima is: {}", lima);

    let tambah_satu = plus_one(20);
    println!("The value of tambah_satu is: {}", tambah_satu);
}

fn another_function() {
    println!("Another function.");
}

fn another_function_param(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x+1
}