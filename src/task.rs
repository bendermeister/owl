use crate::{format::Format, time::Span};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum State {
    Task,
    Done,
}

impl State {
    pub fn is_open(&self) -> bool {
        match self {
            State::Task => true,
            State::Done => false,
        }
    }

    pub fn is_closed(&self) -> bool {
        !self.is_open()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Task {
    pub state: State,
    pub path: PathBuf,
    pub prefix: String,
    pub title: String,
    pub sources: Option<PathBuf>,
    pub deadline: Option<Span>,
    pub scheduled: Option<Span>,
    pub line_number: usize,
    pub subtasks: Vec<SubTask>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SubTask {
    Done(String),
    NotDone(String),
}

impl SubTask {
    pub fn is_done(&self) -> bool {
        match self {
            SubTask::Done(_) => true,
            SubTask::NotDone(_) => false,
        }
    }

    pub fn is_not_done(&self) -> bool {
        !self.is_done()
    }
}

impl Display for SubTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubTask::Done(title) => write!(f, "- [X] {}", title),
            SubTask::NotDone(title) => write!(f, "- [ ] {}", title),
        }
    }
}

#[derive(Debug, Clone)]
struct PrefixBuffer<'a> {
    buffer: Vec<(usize, &'a str)>,
}

impl<'a> PrefixBuffer<'a> {
    fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(6),
        }
    }

    fn pop_to(&mut self, level: usize) {
        match self.buffer.last() {
            Some((last_level, _)) if *last_level >= level => {
                self.buffer.pop();
                self.pop_to(level);
            }
            _ => (),
        }
    }

    fn push(&mut self, level: usize, prefix: &'a str) {
        self.buffer.push((level, prefix));
    }

    fn read(&self) -> String {
        let mut s = String::new();
        for (_, part) in self.buffer.iter() {
            s.push_str(part);
            s.push('/');
        }
        s.pop();
        s
    }
}

impl Task {
    fn new(state: State, title: String, prefix: String, path: PathBuf, line_number: usize) -> Self {
        Self {
            prefix,
            state,
            title,
            path,
            sources: None,
            deadline: None,
            scheduled: None,
            subtasks: vec![],
            line_number,
        }
    }

    fn handle_heading<'a>(
        line: &'a str,
        path: &Path,
        line_number: usize,
        heading_level: usize,
        prefix: &mut PrefixBuffer<'a>,
        tasks: &mut Vec<Task>,
    ) {
        let line = line.trim();
        prefix.pop_to(heading_level);

        if let Some(title) = line.strip_prefix("TASK:") {
            let title = title.trim().into();
            tasks.push(Self::new(
                State::Task,
                title,
                prefix.read(),
                path.into(),
                line_number,
            ));
        } else if let Some(title) = line.strip_prefix("DONE:") {
            let title = title.trim().into();
            tasks.push(Self::new(
                State::Done,
                title,
                prefix.read(),
                path.into(),
                line_number,
            ));
        } else {
            prefix.push(heading_level, line);
        }
    }

    /// Parses a markdown file into a list of tasks with associated prefixes
    ///
    /// # Example
    /// ```
    /// use owl::task::Task;
    /// let body = "
    /// ## Uni
    /// ### Course 1
    /// #### TASK: Exercise 1
    /// ";
    ///
    /// let expected = Task {
    ///     subtasks: vec![],
    ///     prefix: "Uni/Course 1".into(),
    ///     title: "Exercise 1".into(),
    ///     path: "/home/user/journal/uni.md".into(),
    ///     line_number: 4,
    ///     deadline: None,
    ///     scheduled: None,
    ///     sources: None,
    /// };
    ///
    /// let got = Task::parse(body, "/home/user/journal/uni.md");
    /// assert_eq!(vec![expected], got);
    /// ```
    ///
    /// # Errors
    /// erroniously formatted tasks will be ignored while parsing
    pub fn parse<P: AsRef<Path>>(body: &str, path: P) -> Vec<Task> {
        let path: &Path = path.as_ref();
        if Format::new(path) != Format::Markdown {
            return Vec::new();
        }

        let mut prefix = PrefixBuffer::new();
        let mut tasks = Vec::new();
        let body = body.lines().enumerate().map(|(n, l)| (n + 1, l));

        for (line_number, line) in body {
            if let Some(line) = line.strip_prefix("# ") {
                Self::handle_heading(line, path, line_number, 1, &mut prefix, &mut tasks);
            }
            if let Some(line) = line.strip_prefix("## ") {
                Self::handle_heading(line, path, line_number, 2, &mut prefix, &mut tasks);
            }
            if let Some(line) = line.strip_prefix("### ") {
                Self::handle_heading(line, path, line_number, 3, &mut prefix, &mut tasks);
            }
            if let Some(line) = line.strip_prefix("#### ") {
                Self::handle_heading(line, path, line_number, 4, &mut prefix, &mut tasks);
            }
            if let Some(line) = line.strip_prefix("##### ") {
                Self::handle_heading(line, path, line_number, 5, &mut prefix, &mut tasks);
            }
            if let Some(line) = line.strip_prefix("###### ") {
                Self::handle_heading(line, path, line_number, 6, &mut prefix, &mut tasks);
            }

            if let Some(subtask) = line.strip_prefix("- [X]") {
                if let Some(task) = tasks.last_mut() {
                    task.subtasks.push(SubTask::Done(subtask.trim().into()))
                }
            }

            if let Some(subtask) = line.strip_prefix("- [ ]") {
                if let Some(task) = tasks.last_mut() {
                    task.subtasks.push(SubTask::NotDone(subtask.trim().into()))
                }
            }

            if let Some(stamp) = line.strip_prefix("> DEADLINE:") {
                let stamp = match stamp.trim().parse::<Span>() {
                    Ok(stamp) => stamp,
                    Err(err) => {
                        log::warn!("ignoring parsing error in deadline: {:?}", err);
                        continue;
                    }
                };
                if let Some(task) = tasks.last_mut() {
                    task.deadline = Some(stamp);
                }
            }

            if let Some(span) = line.strip_prefix("> SCHEDULED:") {
                let span = match span.trim().parse::<Span>() {
                    Ok(s) => s,
                    Err(err) => {
                        log::warn!("ignoring parsing error in scheduled: {:?}", err);
                        continue;
                    }
                };
                if let Some(task) = tasks.last_mut() {
                    task.scheduled = Some(span);
                }
            }
        }

        log::info!("parsed tasks from file: {:?}", path);

        tasks
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task_parse() {
        let path = "/home/to/some/folder.md";
        let body = "
# Uni
## Course 1
### TASK: Exercise 1
";
        let expected = vec![Task {
            subtasks: vec![],
            prefix: "Uni/Course 1".into(),
            title: "Exercise 1".into(),
            path: path.into(),
            sources: None,
            deadline: None,
            scheduled: None,
            line_number: 4,
        }];

        assert_eq!(expected, Task::parse(body, path));
    }
}
