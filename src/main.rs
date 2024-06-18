mod core;
mod macros;
mod r#impl;
mod io;
mod cli;

fn main() {
    cli::BadLockCli::delegate_main();
}