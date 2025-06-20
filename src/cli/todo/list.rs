use crate::context::Context;
use crate::todo::Todo;
use serde_json;
use std::collections::HashMap;

fn list_todos_plain(todos: Vec<Todo>) -> Result<(), anyhow::Error> {
    for todo in todos.iter() {
        print!("{} TODO: {}", todo.file.as_os_str().to_string_lossy(), todo.title);
        if let Some(stamp) = todo.scheduled {
            print!("SCHEDULED: {}", stamp.to_pretty_string());
        }
        if let Some(stamp) = todo.deadline {
            print!("DEADLINE: {}", stamp.to_pretty_string());
        }
        println!();
    }
    Ok(())
}

fn list_todos_json(todos: Vec<Todo>) -> Result<(), anyhow::Error> {
    let output = serde_json::to_vec_pretty(&todos)?;
    let output = String::from_utf8(output)?;
    println!("{}", output);
    Ok(())
}

#[derive(Debug, clap::Args)]
pub struct Args {}

pub fn run(context: &Context, _: Args) -> Result<(), anyhow::Error> {
    let file_map = context
        .store
        .files
        .iter()
        .map(|f| (f.id, f.path.as_path()))
        .collect::<HashMap<_, _>>();

    let todos = context.store.todos.iter().map(|t| Todo {
        title: t.title.to_owned(),
        file: file_map.get(&t.file).unwrap().to_path_buf(),
        deadline: t.deadline,
        scheduled: t.scheduled,
        line_number: t.line_number,
    }).collect::<Vec<_>>();

    match context.output_format {
        crate::context::OutputFormat::Colorful => list_todos_plain(todos),
        crate::context::OutputFormat::Plain => list_todos_plain(todos),
        crate::context::OutputFormat::Json => list_todos_json(todos),
    }
}
