use std::fs;
use std::io::prelude::*;
use std::io::Error;
use std::io::ErrorKind;

// ID DETERMINED BY ORDER READ FROM FILE - NOT SAVED IN FILE

pub struct Item {
    pub title: String,
    pub description: String,
    // remember to cap prio to u8
    pub priority: u8,
    pub status: String
}
impl Item {
    pub fn serialize(self) -> String {
        format!("PRIO:{}\nSTATUS:{}\nTITLE:{}\nDESC:{}\n", self.priority, self.status, self.title, self.description)
    }
    pub fn cmp(&self, b: &Item) -> std::cmp::Ordering {
        self.priority.cmp(&b.priority)
    }
}

pub fn deserialize_item(ser: &str) -> Result<Item, Error> {
    let lines: Vec<&str> = ser.split("\n").collect();
    let (mut prc, mut stc, mut tic) = (false, false, false);
    let mut pr: u8 = 1;
    let mut st: String = String::from("open");
    let mut ti: String = String::new();
    let mut de: String = String::new();
    for line in lines {
        let len = line.len();
        if line.starts_with("PRIO:") {
            let pr_s = line[5..len].parse::<u8>();
            if pr_s.is_ok() {
                pr = pr_s.unwrap();
                prc = true;
            }
        } else if line.starts_with("STATUS:") {
            st = line[7..len].to_string();
            stc = true;
        } else if line.starts_with("TITLE:") {
            ti = line[6..len].to_string();
            tic = true;
        } else if line.starts_with("DESC:") {
            de.push_str(&line[5..len]);
        } else {
            de.push_str(&format!("\n{}", line));
        }
    }

    if prc && stc && tic {
        Ok(
            Item {
                title: ti,
                description: de,
                priority: pr,
                status: st
        })
    } else {
        Err(Error::from(ErrorKind::InvalidData))
    }
}

pub fn read_items_from_file(fp: &str) -> Result<Vec<Item>, Error> {
    match fs::File::open(fp) {
        Err(msg) => {
            println!("moth: failed to open project at {}: {}", fp, msg);
            return Err(Error::from(ErrorKind::NotFound))
        }
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_err() {
                return Err(Error::from(ErrorKind::NotFound))
            }
            let mut items: Vec<Item> = Vec::new();
            let strsplt: Vec<&str> = contents.split("--PGBRK--\n").collect();
            if strsplt[0] == "" {
                return Ok(items)
            }
            for s in strsplt {
                let desr = deserialize_item(s);
                if desr.is_ok() {
                    items.push(desr.unwrap());
                } else {
                    println!("moth: invalid item found while reading, skipping...")
                }
            }
            Ok(items)
        }
    }
}

pub fn write_items_to_file(items: Vec<Item>, fp: &str) -> Result<(), Error> {
    // im not super happy about using an ascii file to store these but
    // i prefer it to adding non-standard dependencies to this project.
    let mut serialized_items: Vec<String> = Vec::new();
    for i in items {
        serialized_items.push(i.serialize());
    }
    match fs::File::create(fp) {
        Err(msg) => {
            println!("moth: failed to open project at {fp}: {msg}");
            return Err(Error::from(ErrorKind::NotFound))
        },
        Ok(mut file) => {
            match file.write(serialized_items.join("--PGBRK--\n").as_bytes()) {
                Err(msg) => {
                    println!("moth: failed to write to file at {fp}: {msg}");
                    return Err(Error::from(ErrorKind::Other))
                },
                Ok(_) => {
                    return Ok(())
                }
            }
        }
    }
}
