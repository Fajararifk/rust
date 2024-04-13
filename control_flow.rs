use std::io;
//IF EXPRESSIONS

fn main() {
    lyrics();
    let number = 6;

    if number < 10 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 6 } else { 10 };
    println!("The value of number is {}", number);

    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 3 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut counter = 0;
    'counting_up: loop {
        println!("counter = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("End counter = {}", counter);
    program_while(3);
    let a = [10, 20, 30, 40, 50];
    print_array_with_index(&a);
    print_looping();
    convert_degree();
    println!("Choose number for fibonacci : ");
    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("Failed to read line");
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };
    // Change this to generate a different Fibonacci number
    let result = fibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, result);
}

fn program_while(x:i32){
    let mut number = x;
    while number != 0 {
        println!("number : {}", number);
        number -= 1;
    }
    println!("End number = {}", number);
}

fn print_array_with_index(arr: &[i32]) {
    let mut index = 0;

    while index < arr.len() {
        println!("Index {}: the value is: {}", index, arr[index]);
        index += 1;
    }
}
fn print_looping(){
    for number in (1..4).rev() {
        println!("number {}", number);
    }
    println!("LiftOff");
}


fn convert_degree(){
    loop {
        println!("Choose an option:");
        println!("1. Convert Fahrenheit to Celsius");
        println!("2. Convert Celsius to Fahrenheit");
        println!("3. Quit");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter temperature in Fahrenheit:");

                let mut fahrenheit = String::new();

                io::stdin().read_line(&mut fahrenheit)
                    .expect("Failed to read line");

                let fahrenheit: f32 = match fahrenheit.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid number");
                        continue;
                    }
                };

                let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
                println!("Temperature in Celsius: {:.2}", celsius);
            }
            2 => {
                println!("Enter temperature in Celsius:");

                let mut celsius = String::new();

                io::stdin().read_line(&mut celsius)
                    .expect("Failed to read line");

                let celsius: f32 = match celsius.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid number");
                        continue;
                    }
                };

                let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
                println!("Temperature in Fahrenheit: {:.2}", fahrenheit);
            }
            3 => {
                println!("Exiting program...");
                break;
            }
            _ => {
                println!("Invalid option, please try again");
            }
        }
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut fib = (0, 1);

    for i in 2..=n {
        fib = (fib.1, fib.0 + fib.1);
        println!("Fibonacci number at position {}: {} , fib 0 : {} ", i, fib.1, fib.0);
    }

    fib.1
}

fn lyrics() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas, my true love gave to me:", days[i]);
        for j in (0..=i).rev() {
            if i == 0 && j == 0 {
                println!("{}.", gifts[j]);
            } else if j == 0 {
                println!("And {}", gifts[j]);
            } else {
                println!("{}", gifts[j]);
            }
        }
        println!();
    }
}