[![cargo-tests.yml](https://github.com/bendermeister/owl/actions/workflows/rust.yml/badge.svg)](https://github.com/bendermeister/owl/actions/workflows/rust.yml)
# owl
My favourite things from Org Mode without a dependency on Emacs

## Features
### Task System / Agenda
```sh
owl agenda
```
Will produce an overview over your current tasks with their individual
scheduled times, and deadlines. It looks for tasks in every markdown file in
its search path which can be configured.

A file like this

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

would produce the following output:

```
Overdue
Sat 28 Jun 2025
Sun 29 Jun 2025
Mon 30 Jun 2025
  Uni/Course 1      S       Exercise 1
Tue 01 Jul 2025
  Uni/Course 1      S       Exercise 2
  Uni/Course 2      S 13:00 Project 1 (D 2025-07-02 12:00)
Wed 02 Jul 2025
  Uni/Course 2      S 13:00 Project 2
  Uni/Course 2      S 16:30 Test 3
Thu 03 Jul 2025
Fri 04 Jul 2025
```
