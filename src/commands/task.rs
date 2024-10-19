use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub done: bool,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:.3}: {}", self.id, self.name, self.done)
    }
}
