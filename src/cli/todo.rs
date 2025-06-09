use crate::context::Context;
use crate::table;
use crate::todo;

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
    let todos = todo::get_all(&context.todo_path)?;

    let header = table::Row::new()
        .add_col("Title".into())
        .add_col("Scheduled".into())
        .add_col("Deadline".into());

    let mut table = table::Table::new(header);

    for todo in todos {
        let row = table::Row::new()
            .add_col(todo.title)
            .add_col(todo.scheduled.map_or("".into(), |v| v.to_string()))
            .add_col(todo.deadline.map_or("".into(), |v| v.to_string()));
        table.push(row);
    }

    print!("{}", table);

    Ok(())
}
