use std::fmt::Display;

pub struct Table {
    header: Row,
    body: Vec<Row>,
}

pub struct Row {
    cols: Vec<String>,
}

impl Table {
    pub fn new(header: Row) -> Self {
        Self {
            header,
            body: Vec::new(),
        }
    }

    pub fn push(&mut self, row: Row) {
        self.body.push(row)
    }

    fn col_lengths(&self) -> Vec<usize> {
        let mut lengths = self.header.col_lengths();

        for col in self.body.iter().map(|col| col.col_lengths()) {
            lengths = col
                .into_iter()
                .zip(lengths)
                .map(|(a, b)| a.max(b))
                .collect();
        }

        lengths
    }
}

impl Row {
    pub fn new() -> Self {
        Self { cols: Vec::new() }
    }

    pub fn add_col(mut self, element: String) -> Self {
        self.cols.push(element);
        self
    }

    pub fn col_lengths(&self) -> Vec<usize> {
        self.cols.iter().map(|s| s.len()).collect()
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lengths: Vec<_> = self.col_lengths().into_iter().map(|v| v + 2).collect();

        for (i, title) in self.header.cols.iter().enumerate() {
            write!(f, "{}{}", title, " ".repeat(lengths[i] - title.len()))?;
        }
        write!(f, "\n")?;

        for row in self.body.iter() {
            for (i, element) in row.cols.iter().enumerate() {
                write!(f, "{}{}", element, " ".repeat(lengths[i] - element.len()))?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
