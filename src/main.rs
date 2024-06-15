mod core;
mod macros;

fn main() {
    println!("{:2x?}", get_md5!("Hello, world!"));
}