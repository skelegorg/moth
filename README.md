moth is a minimal to-do manager.

## usage 
moth \[COMMAND\] \<optional args>

### commands:

| command | what it does |
|:-----|---|
|add|enters item creation dialogue|
|add \<task name>|creates item with empty description, priority 1 and status 'open'|
|view \<id>|prints all information pertaining to given item|
|del \<id>|deletes item with given id|
|list \<status>|lists ids + titles of items with given status (default open) sorted by priority - arg "all" prints all items|
|edit \<id>|enters item edit dialogue|
|close \<id>|changes item status to "closed"|
|clear|deletes all items with status "closed"|
|load \<path>| load a given project file. default file is ~/.moth/default.moth|


## installation

compile and install moth using cargo.

1. `cargo build --release`
2. the binary will be located in target/release/moth. from there install it to where you'd like. 

note: sometimes, build will fail and say that "std::process::ExitCode" is unsafe and can't compile. i've fixed this by just running "rustup update".

moth is inspired by [bug](http://vicerveza.homeunix.net/~viric/soft/bug/) by Lluís Batlle i Rossell.
