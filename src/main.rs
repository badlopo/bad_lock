mod core;
mod macros;
mod r#impl;
mod io;
mod cli;
mod runner;

fn main() {
    cli::BadLockCli::delegate_main();
}