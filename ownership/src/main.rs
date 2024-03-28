fn main() {
    // let a = 45;
    // let b = a;
    // let s1 = String::from("hello world");
    // let s2 = s1.clone();
    // println!("{}",s1);
    let s1 = give_ownership();
    
    let s2 = String::from("hello world");
    let s3 = takes_give_ownership(s2);
    let s4 = calculate_length(&s3);

    println!("s1 = {}, s3, = {}, s4, = {}", s1, s3, s4);

    let mut r = String::from("hello world 2");
    let s5 = &r;
    let s6 = &r;

    println!("s5 = {}, s6, = {}", s5, s6);

    let s7 = &mut r;
    println!("s7 = {},", s7);
    let s8 = "hello world 3";
    let word = first_word(s8);
    println!("word = {}", word);

}

fn give_ownership() -> String {
    let some_string = String::from("hello world");
    some_string
}
fn takes_give_ownership(a_string:String) -> String {
    a_string
}

fn calculate_length(s:&String) -> usize {
    s.len()
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
