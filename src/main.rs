//! Exit with a given exit code.
//! 
//! This program takes a decimal-formatted integer as an argument and exits,
//! using said integer as the exit code. Because dependents will depend on the
//! exit code of the program, if an invalid value is passed (or the incorrect
//! number of values), the program will loop forever instead of returning. This
//! program seems at first blush not to be useful, but I have often found
//! that the functionality it provides is necessary for a great deal of shell
//! scripts.

use std::env;
use std::process::exit;

fn main() {
    let args = get_args(true);
    
    let mut code = match args.len() {
        1 => match args[0].parse() {
                Ok(x)  => x,
                Err(_) => loop {},
        }
        _ => loop {}
    };

    if code >= 255 { code = 255; };

    exit(code);
}

fn get_args(drop_ppath: bool) -> Vec<String> {
    let mut args = env::args();
    if drop_ppath { args.next(); }
    args.collect()
}

