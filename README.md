[![cargo-tests.yml](https://github.com/bendermeister/owl/actions/workflows/rust.yml/badge.svg)](https://github.com/bendermeister/owl/actions/workflows/rust.yml)
# owl
My favourite things from Org Mode without a dependency on Emacs

## How to Build
### Debug
```sh
nix develop
cargo build
```

### Release
```sh
nix develop
cargo build --release
```

only cargo probably works too but you have to have the necessary rust toolchain and sqlite3 drivers intalled

## Features
### Todo
```sh
owl 'path/to/owl/directory' todo list
```
Lists all todos present in markdown files in the given directory. A todo is written as: 
```markdown
# TODO: this is a todo

## TODO: this is a todo with a deadline
> DEADLINE: <2025-06-20 12:00>

## TODO: this is a todo with a scheduled time
> SCHEDULED: <2025-06-20 12:00>
```

### Search
```sh
owl 'path/to/owl/directory' search 'this is the search phrase'
```

outputs all files currently present in the given directory sorted by tf-idf on the search phrase.

## Rough Roadmap
### TODO: Stemmer
currently the stemming for the tf-idf is very rudimentary, and i would like to implement some algorithm from the snowball guys
- how to do multilingual stuff in tf-idf

### TODO: Tags
using `> TAG: some_tag` should be able to tag files 

### TODO: Different Output Types
- colorful output -> for human reading (should be default)
- json output -> for computers (--json flag)
- better listbased output -> for piping into other shell stuff (--plain flag) 
in general some nicer formatting and some nicer more discoverable interface would be quite nice

### TODO: Display Web Server
```sh
owl server 8080
```
should start a simple webserver from which you are able to view your files, search them etc. 

### TODO: Multithreaded File Indexing
should be quite easily parallelizable at least the parsing is completely disjunct -> sqlite isn't though

### TODO: Better Error reporting
unwrapping `anyhow::Error` is shit error reporting

### TODO: Importer 
- a caldav to md transpiler
- a html to md transpiler
- a pdf to md transpiler
alternative would be to accept more than just markdown files
