fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("s :{}, len is :{} ",s1, len);
    let s2 = String::from("Hello word!");
    let s2_first_word = first_word(&s2);
    println!("s2_first_word :{} ", s2_first_word);
}


fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        };
    }

    &s[..]  
}

