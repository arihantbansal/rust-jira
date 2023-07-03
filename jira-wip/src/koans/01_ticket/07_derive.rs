#[derive(PartialEq, Debug)]
pub enum Status {
    ToDo,
    InProgress,
    Blocked,
    Done,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assertions() {
        assert_eq!(Status::ToDo, Status::ToDo);
        assert_ne!(Status::Done, Status::ToDo);
        assert_ne!(Status::InProgress, Status::ToDo);
        assert_eq!(Status::InProgress, Status::InProgress);
    }
}
