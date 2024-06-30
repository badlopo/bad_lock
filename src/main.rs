mod core;
mod macros;
mod io;
mod cli;
mod runner;

fn main() {
    cli::BadLockCli::delegate_main();
}