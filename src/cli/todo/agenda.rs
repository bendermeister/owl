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
struct Agenda {
    overdue: Vec<Todo>,
    entries: Vec<Entry>,
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

    let file_map = context
        .store
        .files
        .iter()
        .map(|file| (file.id, file.path.as_path()))
        .collect::<HashMap<_, _>>();

    let mut agenda = Agenda {
        overdue: Vec::new(),
        entries,
    };

    for todo in context.store.todos.iter() {
        let stamp = match (todo.scheduled, todo.deadline) {
            (Some(stamp), _) => stamp,
            (_, Some(stamp)) => stamp,
            _ => continue,
        };

        if stamp < start {
            agenda.overdue.push(Todo {
                title: todo.title.clone(),
                file: file_map.get(&todo.file).unwrap().to_path_buf(),
                deadline: todo.deadline,
                scheduled: todo.scheduled,
                line_number: todo.line_number,
            });
            continue;
        }

        if stamp > end {
            continue;
        }

        for i in 0..agenda.entries.len() {
            let limit = agenda
                .entries
                .get(i + 1)
                .map(|e| e.stamp)
                .unwrap_or_else(|| end.add_duration(Duration::days(1)));
            if stamp < limit {
                agenda.entries[i].todos.push(Todo {
                    title: todo.title.clone(),
                    file: file_map.get(&todo.file).unwrap().to_path_buf(),
                    deadline: todo.deadline,
                    scheduled: todo.scheduled,
                    line_number: todo.line_number,
                });
                break;
            }
        }
    }

    match context.output_format {
        crate::context::OutputFormat::Colorful => format_plain(&agenda),
        crate::context::OutputFormat::Plain => format_plain(&agenda),
        crate::context::OutputFormat::Json => format_json(&agenda),
    }
}

fn format_plain(agenda: &Agenda) -> Result<(), anyhow::Error> {
    println!("Overdue");
    for todo in agenda.overdue.iter() {
        println!(
            "\t{}:{} TODO: {}",
            todo.file.as_os_str().to_string_lossy(),
            todo.line_number,
            todo.title
        );
    }

    for entry in agenda.entries.iter() {
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

fn format_json(agenda: &Agenda) -> Result<(), anyhow::Error> {
    let body = serde_json::to_string(agenda)?;
    print!("{}", body);
    Ok(())
}
