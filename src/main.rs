// moth is by skelegorg - skelegorg@protonmail.com

use std::env;
use std::fs;
use std::process::ExitCode;

mod fns;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let helpstring = String::from("
usage: moth [OPTION] <optional args>

OPTIONS:

add                       : opens dialogue to create a new item
add   <optional arg>      : adds a new item with priority 1, status \"open\", title [argument], and an empty description
list | ls                 : lists all items with the status \"open\"
list  <optional arg>      : lists all items with the status [arg] - arg \"all\" prints all items
del  | rm <arg>           : deletes item with id [arg]
edit  <arg>               : opens edit dialogue for item with id [arg]
view  <arg>               : prints all data regarding item with id [arg]
close <arg>               : chanegs the status of item with id [arg] to \"closed\"
clear                     : deletes all items with status \"closed\"
load  <opt file path>     : loads a different .moth file. running with no argument loads the default.
    ");

    // check for existance of default file - create if gone
    if !std::path::Path::new(&format!("{}/.moth/default.moth", env::var("HOME").ok().unwrap())).exists() {
        println!("moth: it looks like this is your first time using moth. creating default file at ~/.moth/default.moth");
        let r = fs::create_dir(&format!("{}/.moth", env::var("HOME").ok().unwrap()));
        if r.is_err() {
            println!("moth: failed to create default directory.")
        }
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
                "ls" => {
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
                        let mut nargs: Vec<u8> = vec![];
                        for i in 2..args.len() {
                            let arg = args[i].parse::<u8>();
                            if arg.is_ok() {
                                nargs.push(arg.ok().unwrap() - 1);
                            } else {
                                println!("moth: id argument must be an integer.")
                            }
                        }
                        if nargs.len() > 0 {
                            fns::functions::del(nargs);
                        } else {
                            println!("moth: no valid inputs")
                        }
                    }
                },
                "rm" => {
                    if args.len() == 2 {
                        println!("moth: id argument required for \'del\'.")
                    } else {
                        let mut nargs: Vec<u8> = vec![];
                        for i in 2..args.len() {
                            let arg = args[i].parse::<u8>();
                            if arg.is_ok() {
                                nargs.push(arg.ok().unwrap() - 1);
                            } else {
                                println!("moth: id argument must be an integer.")
                            }
                        }
                        if nargs.len() > 0 {
                            fns::functions::del(nargs);
                        } else {
                            println!("moth: no valid inputs")
                        }
                    }
                },
                "edit" => {
                    if args.len() == 2 {
                        println!("moth: id argument required for \'edit\'.")
                    } else {
                        let arg = args[2].parse::<u8>();
                        if arg.is_ok() {
                            fns::functions::edit(arg.unwrap() - 1);
                        } else {
                            println!("moth: id argument must be an integer.")
                        }
                    }
                },
                "view" => {
                    if args.len() == 2 {
                        println!("moth: id argument required for \'view\'.")
                    } else {
                        let arg = args[2].parse::<u8>();
                        if arg.is_ok() {
                            fns::functions::view(arg.unwrap() - 1);
                        } else {
                            println!("moth: id argument must be an integer.")
                        }
                    }
                },
                "close" => {
                    if args.len() == 2 {
                        println!("moth: id argument required for \'close\'.")
                    } else {
                        let mut nargs: Vec<u8> = vec![];
                        for i in 2..args.len() {
                            let arg = args[i].parse::<u8>();
                            if arg.is_ok() {
                                nargs.push(arg.ok().unwrap() - 1);
                            } else {
                                println!("moth: id argument must be an integer.")
                            }
                        }
                        if nargs.len() > 0 {
                            fns::functions::close(nargs);
                        } else {
                            println!("moth: no valid inputs")
                        }
                    }
                },
                "clear" => {
                    fns::functions::clear()
                },
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
