use crate::format::Format;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Todo {
    pub title: String,
    pub line_number: usize,
    pub path: PathBuf,
}

impl Todo {
    fn new(title: &str, line_number: usize, path: &Path) -> Self {
        Self {
            title: title.into(),
            line_number,
            path: path.into(),
        }
    }
}

pub fn parse<P: AsRef<Path>>(body: &str, path: P) -> Vec<Todo> {
    let path: &Path = path.as_ref();
    match Format::new(path) {
        Format::Unknown => Vec::new(),
        Format::Markdown => parse_md(body, path),
        Format::Typst => parse_typst(body, path),
        Format::Latex => parse_latex(body, path),

        Format::Shell | Format::Python | Format::Nix => parse_shell_like(body, path),

        // parse c like programming languages
        Format::C
        | Format::JavaScript
        | Format::Go
        | Format::Java
        | Format::CPP
        | Format::Rust
        | Format::Typescript
        | Format::CSharp => parse_clike(body, path),
    }
}

fn parse_latex(body: &str, path: &Path) -> Vec<Todo> {
    body.lines()
        .enumerate()
        .map(|(n, l)| (n + 1, l.trim()))
        .filter(|(_, l)| l.starts_with("% TODO:"))
        .map(|(n, l)| (n, &l[7..]))
        .map(|(n, l)| (n, l.trim()))
        .map(|(n, l)| Todo::new(l, n, path))
        .collect()
}

fn parse_shell_like(body: &str, path: &Path) -> Vec<Todo> {
    body.lines()
        .enumerate()
        .map(|(n, l)| (n + 1, l.trim()))
        .filter(|(_, l)| l.starts_with("# TODO:"))
        .map(|(n, l)| (n, &l[7..]))
        .map(|(n, l)| (n, l.trim()))
        .map(|(n, l)| Todo::new(l, n, path))
        .collect()
}

fn parse_typst(body: &str, path: &Path) -> Vec<Todo> {
    let mut todos = Vec::new();
    for (line_number, line) in body.lines().enumerate() {
        let line_number = line_number + 1;
        if let Some(title) = line.trim().strip_prefix("- TODO:") {
            todos.push(Todo::new(title.trim(), line_number, path));
        }
        if let Some(title) = line.strip_prefix("= TODO:") {
            todos.push(Todo::new(title.trim(), line_number, path));
        }
        if let Some(title) = line.strip_prefix("== TODO:") {
            todos.push(Todo::new(title.trim(), line_number, path));
        }
        if let Some(title) = line.strip_prefix("=== TODO:") {
            todos.push(Todo::new(title.trim(), line_number, path));
        }
        if let Some(title) = line.strip_prefix("==== TODO:") {
            todos.push(Todo::new(title.trim(), line_number, path));
        }
        if let Some(title) = line.strip_prefix("===== TODO:") {
            todos.push(Todo::new(title.trim(), line_number, path));
        }
        if let Some(title) = line.strip_prefix("====== TODO:") {
            todos.push(Todo::new(title.trim(), line_number, path));
        }
    }
    todos
}

fn parse_md(body: &str, path: &Path) -> Vec<Todo> {
    let mut todos = Vec::new();
    for (line_number, line) in body.lines().enumerate() {
        let line_number = line_number + 1;
        if let Some(title) = line.trim().strip_prefix("- TODO:") {
            todos.push(Todo::new(title.trim(), line_number, path));
        }
        if let Some(title) = line.strip_prefix("# TODO:") {
            todos.push(Todo::new(title.trim(), line_number, path));
        }
        if let Some(title) = line.strip_prefix("## TODO:") {
            todos.push(Todo::new(title.trim(), line_number, path));
        }
        if let Some(title) = line.strip_prefix("### TODO:") {
            todos.push(Todo::new(title.trim(), line_number, path));
        }
        if let Some(title) = line.strip_prefix("#### TODO:") {
            todos.push(Todo::new(title.trim(), line_number, path));
        }
        if let Some(title) = line.strip_prefix("##### TODO:") {
            todos.push(Todo::new(title.trim(), line_number, path));
        }
        if let Some(title) = line.strip_prefix("###### TODO:") {
            todos.push(Todo::new(title.trim(), line_number, path));
        }
    }
    todos
}

fn parse_clike(body: &str, path: &Path) -> Vec<Todo> {
    body.lines()
        .enumerate()
        .map(|(n, l)| (n + 1, l.trim()))
        .filter(|(_, l)| l.starts_with("// TODO:"))
        .map(|(n, l)| (n, &l[8..]))
        .map(|(n, l)| (n, l.trim()))
        .map(|(n, l)| Todo::new(l, n, path))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_shell_like() {
        let body = r#"
import numpy as np

def fib(n):
    # TODO: this is slow
    if n < 2:
        return 1
    else:
        return fib(n - 1) + fib(n - 2)

# TODO: learn an actual language

print("Hello World")
"#;
        let path: &Path = "/home/main.py".as_ref();

        let expected = vec![
            Todo::new("this is slow", 5, path),
            Todo::new("learn an actual language", 11, path),
        ];
        let got = parse(body, path);
        assert_eq!(expected, got);
    }

    #[test]
    fn test_parse_clike() {
        let body = r#"
#include <stdio.h>

// TODO: code hashmap

int main() {
    printf("Hello World\n");
    return 0;
    // TODO: error handling
}

"#;

        let path: &Path = "/home/home/main.c".as_ref();

        let expected = vec![
            Todo::new("code hashmap", 4, path),
            Todo::new("error handling", 9, path),
        ];

        let got = parse(body, path);

        assert_eq!(expected, got);
    }

    #[test]
    fn test_parse_typst() {
        let path: &Path = "/home/main.typ".as_ref();
        let body = r#"
= This is a normal heading
= TODO: level 1
== TODO: level 2
=== TODO: level 3
==== TODO: level 4
===== TODO: level 5
====== TODO: level 6
This is a normal line
- TODO: list level 1
  - TODO: list level 2
"#;

        let expected = vec![
            Todo::new("level 1", 3, path),
            Todo::new("level 2", 4, path),
            Todo::new("level 3", 5, path),
            Todo::new("level 4", 6, path),
            Todo::new("level 5", 7, path),
            Todo::new("level 6", 8, path),
            Todo::new("list level 1", 10, path),
            Todo::new("list level 2", 11, path),
        ];

        let got = parse(body, path);

        assert_eq!(expected, got);
    }

    #[test]
    fn test_parse_md() {
        let path: &Path = "/home/main.md".as_ref();
        let body = r#"
# This is a normal heading
# TODO: level 1
## TODO: level 2
### TODO: level 3
#### TODO: level 4
##### TODO: level 5
###### TODO: level 6
This is a normal line
- TODO: list level 1
  - TODO: list level 2
"#;

        let expected = vec![
            Todo::new("level 1", 3, path),
            Todo::new("level 2", 4, path),
            Todo::new("level 3", 5, path),
            Todo::new("level 4", 6, path),
            Todo::new("level 5", 7, path),
            Todo::new("level 6", 8, path),
            Todo::new("list level 1", 10, path),
            Todo::new("list level 2", 11, path),
        ];

        let got = parse(body, path);

        assert_eq!(expected, got);
    }

    #[test]
    fn test_parse_latex() {
        let body = r#"
\documentclass{article}
\begin{document}
First document. This is a simple example, with no 
extra parameters or packages included.
% TODO: do something
\end{document}
"#;

        let path: &Path = "/home/main.tex".as_ref();
        let expected = vec![Todo::new("do something", 6, path)];
        let got = parse(body, path);
        assert_eq!(expected, got);
    }
}
