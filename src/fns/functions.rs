use super::ops;
use std::fs;
use std::env;
use std::io::prelude::*;
use std::process::Command;

pub fn add() {
    // create tmp file with the dummy data
    let r = fs::File::create("/tmp/mothtask.tmp");
    if r.is_err() {
        println!("moth: failed to create temporary file")
    }
    let mut file = r.ok().unwrap();
    let r = file.write("Insert values after the colons - prio must be a number, description can be multi-lined\n\nPRIO:\nSTATUS:open\nTITLE:\nDESC:".as_bytes());
    if r.is_err() {
        println!("moth: failed to write to temporary file")
    }

    // spawn editor instance to edit file
    let editor_r = env::var("EDITOR");
    let editor = if editor_r.is_err() {
        println!("moth: failed to get $EDITOR from environment vars. proceeding with nano as default.");
        "nano".to_string()
    } else {
        editor_r.ok().unwrap()
    };
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("{} /tmp/mothtask.tmp", editor))
        .status()
        .expect("moth: failed to spawn default editor");

    // parse file to get values, initialize Item object, add to vec, rewrite to file
    let r = fs::File::open("/tmp/mothtask.tmp");
    if r.is_err() {
        println!("moth: failed to open temporary file to read")
    }
    let mut file = r.ok().unwrap();
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        println!("moth: failed to read temporary file")
    }
    let mut strsplt: Vec<&str> = contents.split("\n").collect();
    strsplt.remove(0); strsplt.remove(0);
    let item_r = ops::deserialize_item(&strsplt.join("\n"));
    if item_r.is_err() {
        println!("moth: malformed item entry.");
        return
    }
    let item = item_r.ok().unwrap();
    let items_r = ops::read_items_from_file(&load_path());
    if items_r.is_err() {
        println!("moth: failed to read items from {}", load_path())
    }
    let mut items = items_r.ok().unwrap();
    items.push(item);
    match ops::write_items_to_file(items, &load_path()) {
        Err(_) => {
            println!("moth: failed to write items to {}", load_path())
        }
        Ok(_) => {
            println!("moth: item added")
        }
    }
}

pub fn add_with_args(args: Vec<String>) {
    let items_result = ops::read_items_from_file(&load_path());
    if items_result.is_err() {
        println!("moth: failed to read items from {}", load_path())
    }
    let mut items = items_result.ok().unwrap();
    let mut new_title = String::new();
    for i in 2..args.len() {
        new_title = format!("{} {}", new_title, args[i]);
    }
    items.push(ops::Item {
        title: new_title,
        description: "".to_string(),
        priority: 1,
        status: "open".to_string()
    });
    let ret = ops::write_items_to_file(items, &load_path());
    if ret.is_err() {
        println!("moth: error writing to file {}", load_path())
    }
}

pub fn list() {
    let items_r = ops::read_items_from_file(&load_path());
    if items_r.is_err() {
        println!("moth: failed to read items from {}", load_path());
        return
    }
    let mut items = items_r.ok().unwrap();
    items.sort_by(|a, b| a.cmp(b));
    println!("moth: no args: showing all open items");
    let mut count: u8 = 0;
    for i in items {
        count += 1;
        if i.status == "open".to_string() {
            println!("{}: {}", count, i.title);
        }
    }
}

pub fn list_with_args(args: Vec<String>) {
    let mut newargs: Vec<String> = vec![];
    for i in 2..args.len() {
        newargs.push(args[i].clone());
    }
    let status_query = newargs.join(" ");
    let items_r = ops::read_items_from_file(&load_path());
    if items_r.is_err() {
        println!("moth: failed to read items from {}", load_path());
        return
    }
    let mut items = items_r.ok().unwrap();
    items.sort_by(|a, b| a.cmp(b));
    println!("moth: showing all items with status: {}", status_query);
    let mut count: u8 = 0;
    for i in items {
        count += 1;
        if status_query == "all".to_string() {
            println!("{}: {}", count, i.title);
        }
        else if i.status == status_query {
            println!("{}: {}", count, i.title);
        }
    }

}

pub fn del(arg: Vec<u8>) {
    let items_r = ops::read_items_from_file(&load_path());
    if items_r.is_err() {
        println!("moth: failed to read items from {}", load_path());
        return
    }
    let mut items = items_r.ok().unwrap();
    for &i in &arg {
        if i >= items.len() as u8 {
            println!("moth: id above range");
            return
        }
    }
    items.sort_by(|a, b| a.cmp(b));

    let mut sub = 0;
    for i in arg {
        items.remove((i - sub) as usize);
        sub += 1;
    }

    let ret = ops::write_items_to_file(items, &load_path());
    if ret.is_err() {
        println!("moth: error writing to file {}", load_path())
    }
}

pub fn load(pre_arg: &String) {
    let arg: String;
    if !pre_arg.starts_with("/") {
        arg = format!("{}/{}", env::var("PWD").ok().unwrap(), pre_arg);
    } else {
        arg = String::from(pre_arg);
    }
    match fs::File::create(&format!("{}/.moth/loaded.txt", env::var("HOME").ok().unwrap())) {
        Ok(mut file) => {
            let _ = file.write(arg.as_bytes());
        }
        Err(msg) => {
            println!("moth: failed to open or create load file: {msg}");
        }
    }
}

pub fn edit(arg: u8) {
    let items_r = ops::read_items_from_file(&load_path());
    if items_r.is_err() {
        println!("moth: failed to read items from {}", load_path());
        return
    }
    let mut items = items_r.ok().unwrap();
    if arg >= items.len() as u8 {
        println!("moth: id above range")
    }
    items.sort_by(|a, b| a.cmp(b));
    let item: &ops::Item = &items[arg as usize];
    let r = fs::File::create("/tmp/mothtask.tmp");
    if r.is_err() {
        println!("moth: failed to create temporary file")
    }
    let mut file = r.ok().unwrap();
    let r = file.write(format!("Insert values after the colons - prio must be a number, description can be multi-lined\n\nPRIO:{}\nSTATUS:{}\nTITLE:{}\nDESC:{}", item.priority, item.status, item.title, item.description).as_bytes());
    items.remove(arg.into());
    if r.is_err() {
        println!("moth: failed to write to temporary file")
    }

    // spawn editor instance to edit file
    let editor_r = env::var("EDITOR");
    let editor = if editor_r.is_err() {
        println!("moth: failed to get $EDITOR from environment vars. proceeding with nano as default.");
        "nano".to_string()
    } else {
        editor_r.ok().unwrap()
    };
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("{} /tmp/mothtask.tmp", editor))
        .status()
        .expect("moth: failed to spawn default editor");

    // parse file to get values, initialize Item object, add to vec, rewrite to file
    let r = fs::File::open("/tmp/mothtask.tmp");
    if r.is_err() {
        println!("moth: failed to open temporary file to read")
    }
    let mut file = r.ok().unwrap();
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        println!("moth: failed to read temporary file")
    }
    let mut strsplt: Vec<&str> = contents.split("\n").collect();
    strsplt.remove(0); strsplt.remove(0);
    let item_r = ops::deserialize_item(&strsplt.join("\n"));
    if item_r.is_err() {
        println!("moth: malformed item entry.");
        return
    }
    let nitem = item_r.ok().unwrap();
    items.insert(arg.into(), nitem);
    match ops::write_items_to_file(items, &load_path()) {
        Err(_) => {
            println!("moth: failed to write items to {}", load_path())
        }
        Ok(_) => {
            println!("moth: item edited")
        }
    }
}

pub fn view(arg: u8) {
    let items_r = ops::read_items_from_file(&load_path());
    if items_r.is_err() {
        println!("moth: failed to read items from {}", load_path());
        return
    }
    let mut items = items_r.ok().unwrap();
    if arg >= items.len() as u8 {
        println!("moth: id above range")
    }
    items.sort_by(|a, b| a.cmp(b));
    let item: &ops::Item = &items[arg as usize];
    println!("priority: {}", item.priority);
    println!("status: {}", item.status);
    println!("\n{}\n\n{}", item.title, item.description);
}

pub fn close(arg: Vec<u8>) {
    let items_r = ops::read_items_from_file(&load_path());
    if items_r.is_err() {
        println!("moth: failed to read items from {}", load_path());
        return
    }
    let mut items = items_r.ok().unwrap();
    for &i in &arg {
        if i >= items.len() as u8 {
            println!("moth: id above range")
        }
    }
    items.sort_by(|a, b| a.cmp(b));
    for &i in &arg {
        items[i as usize].status = "closed".to_string();
    }
    match ops::write_items_to_file(items, &load_path()) {
        Err(_) => {
            println!("moth: failed to write items to {}", load_path())
        }
        Ok(_) => {
            println!("moth: item closed")
        }
    }
}

pub fn clear() {
    let items_r = ops::read_items_from_file(&load_path());
    if items_r.is_err() {
        println!("moth: failed to read items from {}", load_path());
        return
    }
    let mut items = items_r.ok().unwrap();
    items.sort_by(|a, b| a.cmp(b));
    let mut l: Vec<usize> = vec![];
    for i in 0..items.len() {
        if items[i].status == "closed".to_string() {
            l.push(i);
        }
    }
    let num = l.len();
    println!("moth: clearing {} closed item(s)", num);
    let mut sub = 0;
    for i in l {
        items.remove(i - sub);
        sub += 1;
    }
    match ops::write_items_to_file(items, &load_path()) {
        Err(_) => {
            println!("moth: failed to write items to {}", load_path())
        }
        Ok(_) => {
            println!("moth: item(s) cleared")
        }
    }
}

fn load_path() -> String {
    match fs::File::open(&format!("{}/.moth/loaded.txt", env::var("HOME").ok().unwrap())) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).ok().unwrap();
            contents
        }
        Err(..) => {
            println!("moth: failed to open load file. ensure it exists at ~/.moth/loaded.txt");
            return format!("{}/.moth/default.moth", env::var("HOME").ok().unwrap())
        }
    }
}
