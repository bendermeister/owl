use crate::tesc::*;
use crate::{
    config::Config,
    store::Store,
    task::Task,
    time::{self, Duration},
};

#[derive(Debug, clap::Args)]
pub struct Args {}

fn format_task(prefix_pad: usize, task: &Task) {
    let prefix_pad = prefix_pad - task.prefix.len();
    print!("    {}{} ", task.prefix, " ".repeat(prefix_pad));

    if task.scheduled.is_some() {
        print!("{}S{} ", green(), reset());
    } else {
        print!("{}D{} ", red(), reset());
    }

    print!("{}TASK{}: {}", blue(), reset(), task.title);
    println!();
}

pub fn run(_: &Config, store: &Store, _: &Args) {
    let prefix_pad = store
        .tasks
        .iter()
        .map(|task| task.prefix.len())
        .max()
        .unwrap_or_default();

    let mut start = time::Stamp::today();
    let end = start.add_duration(Duration::days(7));

    let get_stamp = |task: &Task| match (task.scheduled, task.deadline) {
        (Some(stamp), _) => Some(stamp),
        (_, Some(stamp)) => Some(stamp),
        _ => None,
    };

    let mut tasks: Vec<_> = store
        .tasks
        .iter()
        .map(|task| (get_stamp(task), task))
        .filter(|(stamp, _)| stamp.is_some())
        .map(|(stamp, task)| (stamp.unwrap(), task))
        .filter(|(stamp, _)| *stamp < end)
        .collect();

    tasks.sort_by(|(a, _), (b, _)| a.cmp(b));

    let mut tasks = tasks.into_iter().peekable();

    println!("{}Overdue{}", red(), reset());

    while let Some((stamp, _)) = tasks.peek() {
        if *stamp >= start {
            break;
        }
        let (_, task) = tasks.next().unwrap();
        format_task(prefix_pad, task);
    }

    while start < end {
        println!("{}{}{}", bold(), start.to_pretty_string(), reset());
        start = start.add_duration(Duration::days(1));

        while let Some((stamp, _)) = tasks.peek() {
            if *stamp >= start {
                break;
            }
            let (_, task) = tasks.next().unwrap();
            format_task(prefix_pad, task);
        }
    }
}
