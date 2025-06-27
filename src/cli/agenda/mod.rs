use crate::tesc::*;
use crate::{
    config::Config,
    store::Store,
    task::Task,
    time::{self, Duration},
};

#[derive(Debug, clap::Args)]
pub struct Args {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Wrapper<'a> {
    Deadline(&'a Task),
    Scheduled(&'a Task),
}

impl<'a> Wrapper<'a> {
    fn stamp(&self) -> time::Stamp {
        match self {
            Wrapper::Deadline(task) => task.deadline.unwrap(),
            Wrapper::Scheduled(task) => task.scheduled.unwrap(),
        }
    }
    fn format(&self, prefix_pad: usize) {
        let (is_scheduled, task) = match self {
            Wrapper::Deadline(task) => (false, task),
            Wrapper::Scheduled(task) => (true, task),
        };

        // print prefix
        print!(
            "  {}{}{}{} ",
            magenta(),
            task.prefix,
            reset(),
            " ".repeat(prefix_pad - task.prefix.len())
        );

        if is_scheduled {
            print!("{}{}S{} {} ", green(), bold(), reset(), task.scheduled.unwrap().to_clocktime());
        } else {
            print!("{}{}D{} {} ", red(), bold(), reset(), task.deadline.unwrap().to_clocktime());
        }

        print!("{}", task.title);
        println!();
    }
}

struct Agenda<'a> {
    overdue: Vec<Wrapper<'a>>,
    entries: Vec<Entry<'a>>,
}

struct Entry<'a> {
    stamp: time::Stamp,
    tasks: Vec<Wrapper<'a>>,
}

pub fn run(_: &Config, store: &Store, _: &Args) {
    let prefix_pad = store
        .tasks
        .iter()
        .map(|task| task.prefix.len())
        .max()
        .unwrap_or_default();

    let mut tasks = Vec::with_capacity(store.tasks.len());

    for task in store.tasks.iter() {
        if let Some(stamp) = task.scheduled {
            tasks.push((stamp, Wrapper::Scheduled(task)));
        }
        if let Some(stamp) = task.deadline {
            tasks.push((stamp, Wrapper::Deadline(task)));
        }
    }

    tasks.sort_by(|(a, _), (b, _)| a.cmp(b));

    let mut agenda = Agenda {
        overdue: Vec::with_capacity(tasks.len()),
        entries: Vec::with_capacity(tasks.len()),
    };

    let mut index = 0;

    let today = time::Stamp::today();
    while index < tasks.len() && tasks[index].0 < today {
        agenda.overdue.push(tasks[index].1);
        index += 1;
    }

    let mut start = time::Stamp::today();
    let end = start.add_duration(Duration::days(7));

    while start <= end {
        let mut entry = Entry {
            stamp: start,
            tasks: Vec::with_capacity(tasks.len() - index),
        };

        start = start.add_duration(Duration::days(1));

        while index < tasks.len() && tasks[index].0 < start {
            entry.tasks.push(tasks[index].1);
            index += 1;
        }

        agenda.entries.push(entry);
    }

    println!("{}{}Overdue{}", red(), bold(), reset());
    for task in agenda.overdue.iter() {
        task.format(prefix_pad);
    }
    for entry in agenda.entries.iter() {
        println!("{}{}{}", bold(), entry.stamp.to_pretty_string(), reset());
        for task in entry.tasks.iter() {
            task.format(prefix_pad);
        }
    }
}
