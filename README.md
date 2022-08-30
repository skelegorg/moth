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
|list \<status>|lists ids + titles of items with given status (default open) sorted by priority|
|edit \<id>|enters item edit dialogue|
|close \<id>|changes item status to "closed"|
|load \<path>| load a given project file. default file is ~/.moth/default.moth|


## installation

compile moth using gcc or any c compiler
1. gcc moth.c -o moth
2. sudo mv moth /usr/bin

moth is heavily inspired by [bug](http://vicerveza.homeunix.net/~viric/soft/bug/) by Lluís Batlle i Rossell.

this program seeks to adhere to the suckless philosophy.
