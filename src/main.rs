use inline_rust::inline_rust;

fn main() {
    let foo: u8 = 5;
    inline_rust!(r#"println!("hi {}", foo)"#.to_string(), foo: u8);
}
