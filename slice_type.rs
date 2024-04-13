fn main(){
    let hello = String::from("hello");
    first_word(&hello);
    let s = String::from("hello world");
    string_slice(&s);
    let result = word(&hello);
    println!("{}", result);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    println!("{:?}", slice);
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
        println!("{} {}", item, i);
    }

    println!("{}", s.len());
    s.len()
}

fn string_slice(s: &str) {

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello} {world}");

    let len = s.len();
    println!("{len}");
    let slice = &s[0..len];
    let slice1 = &s[..];
    println!("{slice} - {slice1}");
}

fn word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}