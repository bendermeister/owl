use crate::{
    config::Config,
    store::Store,
    task::Task,
    tesc::*,
    time::{Date, Duration},
};

#[derive(Debug, clap::Args)]
pub struct Args {
    /// start of the timeline format: YYYY-MM-DD
    from: String,

    /// end of th timeline format: YYYY-MM-DD
    to: Option<String>,

    /// only show tasks with the given prefix
    #[clap(long)]
    prefix: Option<String>,
}

fn task_print(task: &Task, prefix_pad: usize) {
    print!(
        "  {}{}{}{} ",
        magenta(),
        task.prefix,
        reset(),
        " ".repeat(prefix_pad - task.prefix.len())
    );

    print!("{} {}", task.state, task.title);

    if !task.subtasks.is_empty() {
        let is_done = task.subtasks.iter().filter(|t| t.is_done()).count();
        print!(" [{}/{}]", is_done, task.subtasks.len());
    }

    println!();
}

pub fn get_from_and_to(args: &Args) -> (Date, Date) {
    let from_month = |month| {
        let today = Date::today();
        if month < today.month {
            (Date::from_ymd(today.year, month, 1).unwrap(), today)
        } else {
            (Date::from_ymd(today.year - 1, month, 1).unwrap(), today)
        }
    };

    match (args.from.as_str(), args.to.as_deref()) {
        ("last", Some("month")) => {
            let from = Date::today()
                .sub_duration(Duration::Month(1))
                .unwrap()
                .to_month_begin();
            let to = from.to_month_end();
            return (from, to);
        }

        ("this", Some("month")) => {
            let from = Date::today().to_month_begin();
            let to = Date::today().to_month_end();
            return (from, to);
        }

        ("month", None) => {
            let from = Date::today().to_month_begin();
            let to = from.to_month_end();
            return (from, to);
        }

        ("jan", None) | ("january", None) => return from_month(1),
        ("feb", None) | ("february", None) => return from_month(2),
        ("mar", None) | ("march", None) => return from_month(3),
        ("apr", None) | ("april", None) => return from_month(4),
        ("may", None) => return from_month(5),
        ("jun", None) | ("june", None) => return from_month(6),
        ("jul", None) | ("july", None) => return from_month(7),
        ("aug", None) | ("august", None) => return from_month(8),
        ("sep", None) | ("september", None) => return from_month(9),
        ("oct", None) | ("october", None) => return from_month(10),
        ("nov", None) | ("november", None) => return from_month(11),
        ("dec", None) | ("december", None) => return from_month(12),
        _ => (),
    }

    let from: Date = args
        .from
        .parse()
        .expect("<from> is not a date: expected format: 'YYYY-MM-DD'");

    let mut to = Date::today();

    if let Some(date) = &args.to {
        to = date
            .parse()
            .expect("<to> is not a date: expected format: 'YYYY-MM-DD'")
    }

    (from, to)
}

pub fn run(_: &Config, store: &Store, args: &Args) {
    let (mut from, to) = get_from_and_to(args);

    let prefix = args.prefix.as_deref().unwrap_or("");

    let get_date = |task: &Task| match (task.scheduled, task.deadline) {
        (Some(date), _) => Some(date),
        (_, Some(date)) => Some(date),
        _ => None,
    };

    let mut tasks = store
        .tasks
        .iter()
        .filter(|task| task.prefix.starts_with(prefix))
        .map(|task| (get_date(task), task))
        .filter(|(a, _)| a.is_some())
        .map(|(d, t)| (d.unwrap(), t))
        .filter(|(d, _)| from <= d.date && d.date <= to)
        .collect::<Vec<_>>();

    tasks.sort_by(|(a, _), (b, _)| a.cmp(b));

    let prefix_pad = tasks.iter().map(|(_, t)| t.prefix.len()).max().unwrap_or(0);

    let to = to.add_duration(Duration::Day(1)).unwrap();

    let mut tasks = &tasks[..];

    while from < to {
        println!("{}{}{}", bold(), from.to_pretty_string(), reset());
        from = from.add_duration(Duration::Day(1)).unwrap();

        while !tasks.is_empty() && tasks[0].0.date < from {
            task_print(tasks[0].1, prefix_pad);
            tasks = &tasks[1..];
        }
    }
}
