// moth is by skelegorg - skelegorg@protonmail.com

use std::env;
use std::fs;

mod lib;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let helpstring = String::from("
usage: moth [OPTION] <optional args>
fill in later lol
    ");

    // check for existance of default file - create if gone
    if !std::path::Path::new(&format!("{}/.moth/default.moth", env::var("HOME").ok().unwrap())).exists() {
        println!("moth: it looks like this is your first time using moth. creating default file at ~/.moth/default.moth");
        let re = fs::File::create(&format!("{}/.moth/default.moth", env::var("HOME").ok().unwrap()));
        if re.is_err() {
            println!("moth: failed to create default file. moth operations will fail until a .moth file is loaded.")
        }
    }

    match args.get(1) {
        Some(arg) => {
            match arg.as_str() {
                "add" => {
                    if args.len() == 2 {
                        lib::add();
                    } else {
                        lib::add_with_args(args);
                    }
                },
                "view" => println!("view"),
                "del" => println!("del"),
                "list" => println!("list"),
                "edit" => println!("edit"),
                "close" => println!("close"),
                "load" => println!("load"),
                _ => println!("moth: option {} not found.", arg)
            }
            Ok(())
        },
        None => {
            println!("moth is a minimal todo manager\n{}", helpstring);
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, ""))
        }
    }
}
