//Penguasaan (ownership) adalah fitur paling unik dari Rust dan memiliki implikasi mendalam untuk sisa bahasa tersebut.
//Ini memungkinkan Rust untuk membuat jaminan keselamatan memori tanpa memerlukan kolektor sampah, sehingga penting untuk memahami bagaimana penguasaan bekerja. Dalam bab ini, kita akan membahas tentang penguasaan serta beberapa fitur terkait: peminjaman, irisan, dan bagaimana Rust menyusun data di dalam memori.

//Penguasaan (ownership) adalah serangkaian aturan yang mengatur bagaimana sebuah program Rust mengelola memori. Semua program harus mengelola cara penggunaan memori komputer saat berjalan. Beberapa bahasa memiliki pengumpulan sampah (garbage collection) yang secara teratur mencari memori yang tidak lagi digunakan saat program berjalan; 
//dalam bahasa lain, programmer harus secara eksplisit mengalokasikan dan membebaskan memori. Rust menggunakan pendekatan ketiga: memori dikelola melalui sistem penguasaan dengan serangkaian aturan yang diperiksa oleh kompiler. Jika ada aturan yang dilanggar, program tidak akan dikompilasi. Tidak ada fitur penguasaan yang akan memperlambat program Anda saat berjalan.


//Karena penguasaan (ownership) adalah konsep baru bagi banyak programmer, dibutuhkan waktu untuk terbiasa dengannya. 
//Kabar baiknya adalah semakin berpengalaman Anda dengan Rust dan aturan sistem penguasaan, semakin mudah Anda akan menemukan cara mengembangkan kode secara alami yang aman dan efisien. Teruslah berlatih!

//Ketika Anda memahami penguasaan, Anda akan memiliki dasar yang kokoh untuk memahami fitur-fitur yang membuat Rust unik. 
//Dalam bab ini, Anda akan mempelajari penguasaan dengan bekerja melalui beberapa contoh yang berfokus pada struktur data yang sangat umum: string.

//Tumpukan (stack) dan heap adalah konsep yang tidak perlu sering dipikirkan dalam banyak bahasa pemrograman. Namun, dalam bahasa pemrograman sistem seperti Rust,
//apakah sebuah nilai berada di tumpukan atau heap memengaruhi perilaku bahasa dan mengapa Anda harus membuat keputusan tertentu. Bagian dari penguasaan akan dijelaskan dalam hubungannya dengan tumpukan dan heap nanti dalam bab ini, jadi berikut adalah penjelasan singkat sebagai persiapan.


//Baik tumpukan (stack) maupun heap adalah bagian dari memori yang tersedia bagi kode Anda untuk digunakan saat runtime, tetapi mereka memiliki struktur yang berbeda. Tumpukan menyimpan nilai dalam urutan yang diterimanya dan menghapus nilai dalam urutan yang berlawanan. Ini disebut sebagai last in, first out (terakhir masuk, pertama keluar). Bayangkan tumpukan piring: ketika Anda menambahkan lebih banyak piring, Anda menaruhnya di atas tumpukan, dan ketika Anda membutuhkan sebuah piring, Anda mengambil satu dari atas. 
//Menambahkan atau menghapus piring dari tengah atau bawah tidak akan berfungsi dengan baik! Menambahkan data disebut sebagai pushing ke tumpukan, dan menghapus data disebut sebagai popping dari tumpukan. Semua data yang disimpan di tumpukan harus memiliki ukuran yang diketahui dan tetap. Data dengan ukuran yang tidak diketahui pada saat kompilasi atau ukuran yang mungkin berubah harus disimpan di heap.

fn main(){
    let s1 = String::from("hello");
    let s2 = s1.clone(); //kalau tidak di clone maka s1 tidak bisa digunakan

    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");
    let x = 5;
    takes_ownership(s);
    makes_copy(x);

    let hello = String::from("hello");

    let (halo, len) = calculate_length(hello);

    println!("The length of '{}' is {}.", halo, len);

    let mut s = String::from("hello");

    //let r1 = &mut s; //------ first mutable borrow occur
    //let r2 = &mut s; // ^^^^^^ second mutable borrow occurs here
    
    //ERROR
    // {
    //     let r1 = &mut s;
    // }
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2); 
    // tidak boleh meminjam lebih dari 1 mutable dari data yang sama
    // untuk meminjam gunakan 2 blok terpisah secara bergantian
    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    {

        let r2 = &mut s;
        println!("{}", r2);
    }

    //ERROR
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

    let r3 = &s; // no problem
    let r4 = &s; // no problem
    println!("{} and {}", r3, r4);
    // variables r1 and r2 will not be used after this point

    let r5 = &mut s; // no problem
    println!("{}", r5);

    // tidak bisa
    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// fn dangle() -> &String { // expected named lifetime parameter
//     let test = String::from("hello");

//     &test
// }

fn no_dangle() -> String {
    let test = String::from("hello");

    test
}