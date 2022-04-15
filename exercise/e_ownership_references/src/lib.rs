pub fn inspect(s: &String) {
    println!("{}", if s.ends_with("s") { "plural" } else { "singular"});
}

pub fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    }
}

pub fn eat(s: String) -> bool {
    s.starts_with("b") && s.contains("a")
}

pub fn add(a: &i32, b: &i32) -> i32 {
    *a + *b
}
