use std::collections::HashMap;

use crate::context::Context;
use crate::time::{Duration, Stamp};
use crate::todo::Todo;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(long)]
    timespan: Option<String>,

    #[clap(long)]
    interval: Option<String>,
}

fn parse_timespan(part: &str) -> Stamp {
    if let Ok(stamp) = part.parse() {
        return stamp;
    }

    if let Ok(duration) = part.parse() {
        let stamp = Stamp::today().add_duration(duration);
        return stamp;
    }

    panic!("failed to parse timespan");
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Entry {
    stamp: Stamp,
    todos: Vec<Todo>,
}

pub fn run(context: &Context, args: Args) -> Result<(), anyhow::Error> {
    let timespan = args.timespan.unwrap_or_else(|| "today;+7D".into());
    let interval = args.interval.unwrap_or_else(|| "1D".into());

    let mut timespan = timespan.trim().split(";");

    let start = match timespan.next() {
        Some(p) => p,
        None => panic!("--timespan does not include a timespan"),
    };

    let end = match timespan.next() {
        Some(p) => p,
        None => {
            panic!("--timespan does not include an end: should be seperated by ';' from the start")
        }
    };

    let start = parse_timespan(start);
    let end = parse_timespan(end);

    let interval = match interval.parse::<Duration>() {
        Ok(d) => d,
        Err(err) => panic!("could not parse duration: {:?}", err),
    };

    let mut entries = Vec::new();
    let mut iter = start;

    while iter <= end {
        entries.push(Entry {
            stamp: iter,
            todos: Vec::new(),
        });
        iter = iter.add_duration(interval);
    }

    let mut todos = context
        .store
        .todos
        .clone()
        .into_iter()
        .filter(|todo| {
            let stamp = match (todo.scheduled, todo.deadline) {
                (Some(stamp), _) => stamp,
                (_, Some(stamp)) => stamp,
                _ => return false,
            };
            start <= stamp && stamp <= end
        })
        .collect::<Vec<_>>();

    let file_map = context
        .store
        .files
        .iter()
        .map(|file| (file.id, file.path.as_path()))
        .collect::<HashMap<_, _>>();

    for i in 0..entries.len() {
        let top_limit = entries
            .get(i + 1)
            .map(|e| e.stamp)
            .unwrap_or_else(|| end.add_duration(Duration::days(1)));

        let mut insert = Vec::with_capacity(todos.len());
        let mut retain = Vec::with_capacity(todos.len());

        insert.clear();
        retain.clear();

        for todo in todos.into_iter() {
            let stamp = match (todo.scheduled, todo.deadline) {
                (Some(stamp), _) => stamp,
                (_, Some(stamp)) => stamp,
                _ => unreachable!(),
            };
            if stamp < top_limit {
                insert.push(todo);
            } else {
                retain.push(todo);
            }
        }

        todos = retain;

        let insert = insert.into_iter().map(|t| Todo {
            title: t.title,
            file: file_map.get(&t.file).unwrap().to_path_buf(),
            deadline: t.deadline,
            scheduled: t.scheduled,
            line_number: t.line_number,
        });

        entries[i].todos.extend(insert.into_iter());
    }

    match context.output_format {
        crate::context::OutputFormat::Colorful => format_plain(&entries),
        crate::context::OutputFormat::Plain => format_plain(&entries),
        crate::context::OutputFormat::Json => format_json(&entries),
    }
}

fn format_plain(entries: &[Entry]) -> Result<(), anyhow::Error> {
    for entry in entries.iter() {
        println!("{}", entry.stamp.to_pretty_string());
        for todo in entry.todos.iter() {
            println!(
                "\t{}:{} TODO: {}",
                todo.file.as_os_str().to_string_lossy(),
                todo.line_number,
                todo.title
            );
        }
    }

    Ok(())
}

fn format_json(entries: &[Entry]) -> Result<(), anyhow::Error> {
    let body = serde_json::to_string(entries)?;
    print!("{}", body);
    Ok(())
}
