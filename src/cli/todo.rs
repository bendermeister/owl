use crate::context::Context;
use crate::table;
use crate::todo::Todo;
use serde_json;
use std::collections::HashMap;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// todo commands
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// List all current todos
    List(ListArgs),
}

#[derive(Debug, clap::Args)]
struct ListArgs {}

pub fn run(context: &Context, args: Args) -> Result<(), anyhow::Error> {
    match args.command {
        Command::List(args) => run_list(context, args),
    }
}

fn list_todos_plain(todos: Vec<Todo>) -> Result<(), anyhow::Error> {
    let table = table::Row::new()
        .add_col("Title".into())
        .add_col("Scheduled".into())
        .add_col("Deadline".into())
        .add_col("File".into());

    let mut table = table::Table::new(table);

    for todo in todos {
        let row = table::Row::new()
            .add_col(todo.title)
            .add_col(
                todo.scheduled
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "".into()),
            )
            .add_col(
                todo.deadline
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "".into()),
            )
            .add_col(format!(
                "{}:{}",
                todo.file.to_string_lossy(),
                todo.line_number
            ));

        table.push(row);
    }

    println!("{}", table);
    Ok(())
}

fn list_todos_json(todos: Vec<Todo>) -> Result<(), anyhow::Error> {
    let output = serde_json::to_vec_pretty(&todos)?;
    let output = String::from_utf8(output)?;
    println!("{}", output);
    Ok(())
}

fn run_list(context: &Context, _: ListArgs) -> Result<(), anyhow::Error> {
    let file_map = context
        .store
        .files
        .iter()
        .map(|f| (f.id.clone(), f.path.as_path()))
        .collect::<HashMap<_, _>>();

    let todos = context.store.todos.iter().map(|t| Todo {
        title: t.title.to_owned(),
        file: file_map.get(&t.file).unwrap().to_path_buf(),
        deadline: t.deadline.clone(),
        scheduled: t.scheduled.clone(),
        line_number: t.line_number,
    }).collect::<Vec<_>>();

    match context.output_format {
        crate::context::OutputFormat::Colorful => list_todos_plain(todos),
        crate::context::OutputFormat::Plain => list_todos_plain(todos),
        crate::context::OutputFormat::Json => list_todos_json(todos),
    }
}
