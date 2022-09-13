// moth is by skelegorg - skelegorg@protonmail.com

use std::env;
use std::fs;
use std::process::ExitCode;

mod fns;

fn main() -> ExitCode {
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
                        fns::functions::add();
                    } else {
                        fns::functions::add_with_args(args);
                    }
                },
                "list" => {
                    if args.len() == 2 {
                        fns::functions::list();
                    } else {
                        fns::functions::list_with_args(args);
                    }
                },
                "del" => {
                    if args.len() == 2 {
                        println!("moth: id argument required for \'del\'.")
                    } else {
                        let arg = args[2].parse::<u8>();
                        if arg.is_ok() {
                            fns::functions::del(arg.unwrap() - 1);
                        } else {
                            println!("moth: id argument must be an integer.")
                        }
                    }
                },
                "view" => println!("view"),
                "edit" => println!("edit"),
                "close" => println!("close"),
                "load" => {
                    if args.len() == 2 {
                        fns::functions::load(&format!("{}/.moth/default.moth", env::var("HOME").ok().unwrap()));
                        println!("moth: loaded default config at ~/.moth/default.moth")
                    } else {
                        fns::functions::load(&args[2]);
                    }
                },
                _ => println!("moth: option {} not found.", arg)
            }
            ExitCode::SUCCESS
        },
        None => {
            println!("moth is a minimal todo manager\n{}", helpstring);
            ExitCode::FAILURE
        }
    }
}
