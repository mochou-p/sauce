// mochou-p/sauce/src/main.rs

#![feature(str_as_str)]
#![feature(stmt_expr_attributes)]

mod lua;
mod ffi;
mod help;
mod configure;
mod rebuild;
mod clean;


#[cfg(debug_assertions)]
const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/root/");
#[cfg(not(debug_assertions))]
const ROOT: &str = "/";

const SYSTEM_CONFIG: &str = "etc/sauce.lua";

fn main() {
    command(std::env::args().skip(1));
}

fn command(mut args: impl Iterator<Item = String>) {
    let subcommands = "help | configure | rebuild | clean";

    let Some(word) = args.next() else {
        return eprintln!("missing command: \x1b[36m{subcommands}\x1b[0m");
    };

    match word.as_str() {
        "help"      => help     ::command(    ),
        "configure" => configure::command(args),
        "rebuild"   => rebuild  ::command(args),
        "clean"     => clean    ::command(    ),
        other       => eprintln!("unknown command:  \x1b[31m`{other}`\x1b[0m\n  known commands: \x1b[32m{subcommands}\x1b[0m")
    }
}

