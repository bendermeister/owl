use fast_glob::glob_match;

use crate::tesc::*;
use crate::{
    config::Config,
    store::Store,
    task::Task,
    time::{self, Duration},
};

#[derive(Debug, clap::Args)]
pub struct Args {
    /// filter for a specific prefix with a glob
    #[clap(long)]
    glob: Option<String>,

    /// filter for a specific prefix
    #[clap(long)]
    prefix: Option<String>,
}

struct Agenda<'a> {
    overdue: Vec<&'a Task>,
    entries: Vec<Entry<'a>>,
}

struct Entry<'a> {
    stamp: time::Stamp,
    tasks: Vec<&'a Task>,
}

fn task_print(task: &Task, prefix_pad: usize) {
    let prefix_pad = prefix_pad - task.prefix.len();
    print!(
        "  {}{}{}{} ",
        magenta(),
        task.prefix,
        reset(),
        " ".repeat(prefix_pad)
    );

    if let Some(stamp) = task.scheduled {
        let stamp = stamp.to_clocktime();
        print!("{}{}S{} ", green(), bold(), reset());
        print!("{}{}{} ", green(), stamp, reset());
    } else if let Some(stamp) = task.deadline {
        let stamp = stamp.to_clocktime();
        print!("{}{}D{} ", red(), bold(), reset());
        print!("{}{}{} ", red(), stamp, reset());
    }

    print!("{}", task.title);

    if let Some(stamp) = task.deadline {
        print!(" ({}{}D{} {})", red(), bold(), reset(), stamp);
    }
    println!()
}

pub fn run(_: &Config, store: &Store, args: &Args) {
    let get_stamp = |task: &Task| match (task.scheduled, task.deadline) {
        (Some(stamp), _) => Some(stamp),
        (_, Some(stamp)) => Some(stamp),
        _ => None,
    };

    let glob_filter = args.glob.as_deref().unwrap_or("**");
    let glob_filter = |task: &Task| glob_match(glob_filter.as_bytes(), task.prefix.as_bytes());

    let prefix_filter = args.prefix.as_deref().unwrap_or("");
    let prefix_filter = |task: &Task| task.prefix.starts_with(prefix_filter);

    let mut start = time::Stamp::today();
    let end = start.add_duration(Duration::days(7));

    let tasks = store
        .tasks
        .iter()
        .map(|task| (get_stamp(task), task))
        .filter(|(stamp, _)| stamp.is_some())
        .map(|(stamp, task)| (stamp.unwrap(), task))
        .filter(|(stamp, _)| *stamp < end)
        .filter(|(_, task)| glob_filter(task))
        .filter(|(_, task)| prefix_filter(task))
        .collect::<Vec<_>>();

    let prefix_pad = tasks
        .iter()
        .map(|(_, task)| task.prefix.len())
        .max()
        .unwrap_or_default();

    let mut agenda = Agenda {
        overdue: Vec::with_capacity(tasks.len()),
        entries: Vec::with_capacity(8),
    };

    let mut tasks = &tasks[..];

    let today = time::Stamp::today();
    while !tasks.is_empty() && tasks[0].0 < today {
        agenda.overdue.push(tasks[0].1);
        tasks = &tasks[1..];
    }

    while !tasks.is_empty() && tasks[0].0 < start {
        tasks = &tasks[1..];
    }

    while start < end {
        let mut entry = Entry {
            stamp: start,
            tasks: Vec::with_capacity(tasks.len()),
        };

        start = start.add_duration(Duration::days(1));

        while !tasks.is_empty() && tasks[0].0 < start {
            entry.tasks.push(tasks[0].1);
            tasks = &tasks[1..];
        }
        agenda.entries.push(entry);
    }

    println!("{}{}Overdue{}", red(), bold(), reset());
    for task in agenda.overdue.iter() {
        task_print(task, prefix_pad);
    }
    for entry in agenda.entries.iter() {
        println!("{}{}{}", bold(), entry.stamp.to_pretty_string(), reset());
        for task in entry.tasks.iter() {
            task_print(task, prefix_pad);
        }
    }
}
