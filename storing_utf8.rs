fn main() {
    //CREATING A NEW STRING
    let _s = String::new();
    let data = "Fajar Arif Kurniawan";

    let s = data.to_string();
    //another method using literal
    let fak = "Fajar Arif Kurniawan".to_string();
    println!("string literal : {fak}");
    println!("data : {s}");

    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is {s2}");


    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    //1
    //let tictactoe = tic + "-" + &tac + "-" + &toe;
    //tidak bisa digunakan tic karena telah di ambil oleh tictactoe
    //sedangkan yang memiliki "&" tetap bisa digunakan karena variable tersebut sedang dipinjam oleh tictactoe


    //2
    //let tictactoe = format!("{}-{}-{}",tic,tac,toe);
    //bisa digunakan karena tidak diambil kepemilikan tic oleh tictactoe

    //3
    //let tictactoe = tic.to_string() + "-" + &tac + "-" + &toe;
    //bisa di gunakan karena tidak diambil kepemilikan tic oleh tictactoe

    //4
    let tictactoe = tic.clone() + "-" + &tac + "-" + &toe;
    //bisa digunakan karena di clone terlebih dahulu

    println!("tictactoe: {}", tictactoe);
    println!("tic : {}", tic);
    println!("tac : {}", tac);
    println!("toe : {}", toe);

    //INDEXING INTO STRING

    //let hello = String::from("hello");
    //let h = hello[0];
    //println!("h : {}", h);  //^ `String` cannot be indexed by `{integer}`
    //let hurufH = &hello[0];
    //println!("h : {hurufH}"); // ^ `String` cannot be indexed by `{integer}`
    let halo = "Здравствуйте";
    //let answer = &halo[0];
    //println!("answer : {}", answer);  //^ `String` cannot be indexed by `{integer}`
    let first_char = halo.chars().nth(0);
    //let first_char = halo.chars().next(); //bisa pake ini
    //println!("first_char : {}", first_char);
    match first_char {
        Some(c) => println!("First character: {}", c),
        None => println!("The string is empty"),
    }
    if let Some(second_char) = halo.chars().nth(1) {
        println!("Character at index 1: {}", second_char);
    } else {
        println!("No character found at index 1");
    }


    for i in halo.chars() {
        println!("{i}");
    }

    for b in halo.bytes() {
        println!("{b}");
    }
}