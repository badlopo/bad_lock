mod core;
mod macros;
mod r#impl;

fn main() {
    println!("{:2x?}", get_md5!("Hello, world!"));
}