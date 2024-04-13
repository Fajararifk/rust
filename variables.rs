fn main(){
    // Variable
    let mut x = 5;
    println!("The value of x is: {}",x);
    x = 6;
    println!("The value of x is: {}",x);

    // Constant
    const TWO_HOURS_IN_SECONDS: u32 = 60 * 60 * 2;
    println!("The value of Dua Jam dalam Detik is: {}",TWO_HOURS_IN_SECONDS);

    // Shadowing
    let y = 5;
    let y = y + 1;
    println!("--Before spinner. Value y is {y}");
    {
        let y = y * 2;
        println!("The value of y in the spinner scope is : {y}");
    }

    println!("--After spinner. Value y is: {y}");

    let spaces = "     ";
    let spaces = spaces.len();
    println!("Spaces are represented by: {spaces}");

    //using mut 
    // let mut spasi = "     ";
    // let spasi = spasi.len();
    // println!("Spaces are represented by: {spaces}")
    // Hasilnya harus menghapus mut karena mengubah string menjadi number
}