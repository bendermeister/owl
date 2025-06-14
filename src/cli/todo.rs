use crate::context::Context;
use crate::table;
use crate::todo::Todo;

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

pub fn run(context: Context, args: Args) -> Result<(), anyhow::Error> {
    match args.command {
        Command::List(args) => run_list(context, args),
    }
}

fn run_list(context: Context, _: ListArgs) -> Result<(), anyhow::Error> {
    let todos: Result<Vec<Todo>, anyhow::Error> = context
        .db
        .prepare(
            "
            SELECT 
                todos.title, 
                todos.deadline, 
                todos.scheduled,
                todos.line,
                files.path
            FROM todos INNER JOIN files ON todos.file = files.id;",
        )?
        .query(rusqlite::params![])?
        .and_then(|row| {
            Ok(Todo {
                title: row.get(0)?,
                deadline: row.get(1)?,
                scheduled: row.get(2)?,
                line_number: row.get(3)?,
                file: row.get::<_, String>(4)?.into(),
            })
        })
        .collect();

    let todos = todos?;

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
