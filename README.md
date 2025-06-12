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

## Roadmap
see Issues
