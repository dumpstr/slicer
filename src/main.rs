fn main() {
    let test_string = String::from("hello gamer");

    let _word = first_word(&test_string[..]);

    let test_string_literal = "hello gamer";

    let _word = first_word(test_string_literal);

    println!("{}", _word);

    let a = [11, 22, 33, 44, 55, 66];
    let slice = &a[3..5];
    for element in slice.iter().rev() {
        println!("{}", element);
    }

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

