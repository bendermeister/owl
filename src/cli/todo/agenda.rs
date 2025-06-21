use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use crate::context::Context;
use crate::time::{Duration, Stamp};
use crate::todo::Todo;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(long)]
    timespan: Option<String>,

    #[clap(long)]
    interval: Option<String>,

    #[clap(long)]
    path_len: Option<String>,
}

enum PathLength {
    Full,
    None,
    Len(usize),
}

impl PathLength {
    fn cut_path(&self, path: PathBuf) -> Option<PathBuf> {
        let len = match self {
            PathLength::Full => return Some(path),
            PathLength::None => return None,
            PathLength::Len(len) => *len,
        };

        let mut components = path.components().collect::<Vec<_>>();

        while components.len() > len {
            components.remove(0);
        }

        let mut path = PathBuf::new();

        for c in components.into_iter() {
            path.push(c);
        }

        Some(path)
    }
}

impl FromStr for PathLength {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "full" => return Ok(Self::Full),
            "none" => return Ok(Self::None),
            _ => (),
        };
        Ok(Self::Len(s.parse()?))
    }
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
    let path_length = match args
        .path_len
        .unwrap_or_else(|| "2".into())
        .parse::<PathLength>()
    {
        Ok(p) => p,
        Err(err) => panic!("--path_length could not be parsed: {:?}", err),
    };

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
        crate::context::OutputFormat::Colorful => format_plain(&agenda, path_length),
        crate::context::OutputFormat::Plain => format_plain(&agenda, path_length),
        crate::context::OutputFormat::Json => format_json(&agenda),
    }
}

fn format_plain(agenda: &Agenda, path_length: PathLength) -> Result<(), anyhow::Error> {
    let gray = "\x1b[90m";
    let reset = "\x1b[0m";
    let green = "\x1b[32m";
    let red = "\x1b[31m";

    let render_file = |path: PathBuf, line_number: usize| {
        let mut path = path_length
            .cut_path(path)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        if !path.is_empty() {
            path.push(':');
            path.push_str(&line_number.to_string());
        }
        path
    };

    println!("{}Overdue{}", red, reset);
    for todo in agenda.overdue.iter() {
        println!(
            "\t{}{}{} TODO: {}",
            gray,
            render_file(todo.file.clone(), todo.line_number),
            reset,
            todo.title
        );
    }

    for entry in agenda.entries.iter() {
        let mut paths = Vec::new();
        for todo in entry.todos.iter() {
            paths.push(render_file(todo.file.clone(), todo.line_number));
        }

        let pad = paths.iter().map(|p| p.len()).max().unwrap_or(0) + 2;

        let paths = paths
            .into_iter()
            .map(|path| format!("{}{}{}{}", gray, path, " ".repeat(pad - path.len()), reset));

        println!("{}", entry.stamp.to_pretty_string());
        for (todo, path) in entry.todos.iter().zip(paths) {
            let t = if todo.scheduled.is_some() {
                format!("{}S{}", green, reset)
            } else {
                format!("{}D{}", red, reset)
            };

            println!("\t{} {}TODO: {}", t, path, todo.title);
        }
    }

    Ok(())
}

fn format_json(agenda: &Agenda) -> Result<(), anyhow::Error> {
    let body = serde_json::to_string(agenda)?;
    print!("{}", body);
    Ok(())
}
