# inline-rust

Have you ever wanted to execute arbitrary Rust code at runtime? Well now you can!

```rust
use inline_rust::inline_rust;

fn main() {
    let foo: u8 = 5;
    inline_rust!(r#"println!("hi {}", foo)"#.to_string(), foo: u8);
}
```

``` shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/inline-rust`
hi 5
```


# License
MIT.
