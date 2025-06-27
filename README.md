[![cargo-tests.yml](https://github.com/bendermeister/owl/actions/workflows/rust.yml/badge.svg)](https://github.com/bendermeister/owl/actions/workflows/rust.yml)
# owl
My favourite things from Org Mode without a dependency on Emacs

only cargo probably works too but you have to have the necessary rust toolchain and sqlite3 drivers intalled

## Features
### Task System
```sh
owl agenda
```
Lists all todos present in markdown files in the given directory. A todo is written as: 

```markdown
# Uni

## Course 1
### TASK: Exercise 1
> SCHEDULED: today
### TASK: Exercise 2
> SCHEDULED: 2025-07-01

## Course 2
### TASK: Project 1
> DEADLINE: 2025-07-02 12:00
> SCHEDULED: 2025-07-01 13:00
### TASK: Project 2
> SCHEDULED: 2025-07-02 13:00
### TASK: Test 3
> SCHEDULED: 2025-07-02 16:30
```

### Search
```sh
owl 'path/to/owl/directory' search 'this is the search phrase'
```

outputs all files currently present in the given directory sorted by tf-idf on the search phrase.

## Roadmap
see Issues
