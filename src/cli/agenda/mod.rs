use crate::tesc::*;
use crate::time::ClockTime;
use crate::time::Date;
use crate::{config::Config, store::Store, task::Task, time::Duration};

#[derive(Debug, clap::Args)]
pub struct Args {
    /// filter for a specific prefix
    #[clap(long)]
    prefix: Option<String>,

    /// should subtasks be listed or not
    #[clap(long)]
    subtask: bool,

    /// until which date the agenda should  be generated
    /// possible formats:
    /// - "<x>d" x number of days after start
    /// - "<x>w" x number of weeks after start
    /// - "<x>m" x number of months after start
    /// - "<x>y" x number of years after start
    #[clap(long, verbatim_doc_comment)]
    until: Option<String>,
}

struct Agenda<'a> {
    overdue: Vec<&'a Task>,
    entries: Vec<Entry<'a>>,
}

struct Entry<'a> {
    stamp: Date,
    tasks: Vec<&'a Task>,
}

fn clock_range_format(start: Option<ClockTime>, end: Option<ClockTime>) -> String {
    match (start, end) {
        (Some(start), Some(end)) => format!("{} - {}", start, end),
        (Some(start), None) => format!("{}        ", start),
        (None, Some(end)) => format!("        {}", end),
        (None, None) => " ".repeat(13),
    }
}

fn task_print(task: &Task, prefix_pad: usize, subtask: bool) {
    print!(
        "  {}{}{}{} ",
        magenta(),
        task.prefix,
        reset(),
        " ".repeat(prefix_pad - task.prefix.len())
    );

    if let Some(span) = task.scheduled {
        print!("{}{}S{} ", green(), bold(), reset());
        print!("{} ", clock_range_format(span.start, span.end));
    } else if let Some(span) = task.deadline {
        print!("{}{}D{} ", red(), bold(), reset());
        print!("{} ", clock_range_format(span.start, span.end));
    }

    print!("{}", task.title);

    if !task.subtasks.is_empty() {
        let is_done = task.subtasks.iter().filter(|t| t.is_done()).count();
        print!(" [{}/{}]", is_done, task.subtasks.len());
    }

    if task.scheduled.is_some() {
        if let Some(span) = task.deadline {
            print!(
                " ({}{}D{} {})",
                red(),
                bold(),
                reset(),
                span.date.to_pretty_string().trim()
            );
        }
    }

    println!();

    if subtask {
        for subtask in task.subtasks.iter() {
            println!("{}           {}", " ".repeat(prefix_pad), subtask);
        }
    }
}

fn parse_until(until: &str, start: Date) -> Date {
    if let Ok(until) = until.parse() {
        return until;
    }
    let duration = until.parse().expect("could not evaluate --until flag");
    start
        .add_duration(duration)
        .expect("could not evaluate --until flag")
}

pub fn run(_: &Config, store: &Store, args: &Args) {
    let get_stamp = |task: &Task| match (task.scheduled, task.deadline) {
        (Some(stamp), _) => Some(stamp),
        (_, Some(stamp)) => Some(stamp),
        _ => None,
    };

    let prefix_filter = args.prefix.as_deref().unwrap_or("");
    let prefix_filter = |task: &Task| task.prefix.starts_with(prefix_filter);

    let mut start = Date::today();

    let until = args.until.as_deref().unwrap_or("7d");
    let end = parse_until(until, start);

    let mut tasks = store
        .tasks
        .iter()
        .map(|task| (get_stamp(task), task))
        .filter(|(stamp, _)| stamp.is_some())
        .map(|(stamp, task)| (stamp.unwrap(), task))
        .filter(|(stamp, _)| stamp.date < end)
        .filter(|(_, task)| prefix_filter(task))
        .collect::<Vec<_>>();

    tasks.sort_by(|(a, _), (b, _)| a.cmp(b));

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

    let today = Date::today();
    while !tasks.is_empty() && tasks[0].0.date < today {
        agenda.overdue.push(tasks[0].1);
        tasks = &tasks[1..];
    }

    while !tasks.is_empty() && tasks[0].0.date < start {
        tasks = &tasks[1..];
    }

    while start < end {
        let mut entry = Entry {
            stamp: start,
            tasks: Vec::with_capacity(tasks.len()),
        };

        start = start.add_duration(Duration::Day(1)).unwrap();

        while !tasks.is_empty() && tasks[0].0.date < start {
            entry.tasks.push(tasks[0].1);
            tasks = &tasks[1..];
        }
        agenda.entries.push(entry);
    }

    println!("{}{}Overdue{}", red(), bold(), reset());
    for task in agenda.overdue.iter() {
        task_print(task, prefix_pad, args.subtask);
    }
    for entry in agenda.entries.iter() {
        println!("{}{}{}", bold(), entry.stamp.to_pretty_string(), reset());
        for task in entry.tasks.iter() {
            task_print(task, prefix_pad, args.subtask);
        }
    }
}
