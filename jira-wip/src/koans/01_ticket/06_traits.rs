use crate::path_to_enlightenment::visibility::ticket::Status;

impl PartialEq for Status {
    fn eq(&self, other: &Status) -> bool {
        match (self, other) {
            (Status::ToDo, Status::ToDo) => true,
            (Status::Blocked, Status::Blocked) => true,
            (Status::Done, Status::Done) => true,
            (Status::InProgress, Status::InProgress) => true,
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        assert_eq!(Status::ToDo == Status::ToDo, true);
        assert_eq!(Status::Done == Status::ToDo, false);
        assert_eq!(Status::InProgress == Status::ToDo, false);
        assert_eq!(Status::InProgress == Status::InProgress, true);
    }
}
