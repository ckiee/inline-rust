use inline_rust::inline_rust;

fn main() {
    let foo: u8 = 5;
    let bar: String = "hello".to_string();
    inline_rust!(r#"
println!("gonna say {} {} times", bar, foo);
for _ in 0..foo {
    println!("{}", bar);
}
"#, foo: u8, bar: String);
}
