enum Status {
    ToDo,
    InProgress,
    Blocked,
    Done,
}

struct Ticket {
    title: String,
    description: String,
    status: Status,
}

fn create_ticket(title: String, description: String, status: Status) -> Ticket {
    if title.is_empty() || description.is_empty() || title.len() > 50 || description.len() > 3000 {
        panic!()
    }

    Ticket {
        title,
        description,
        status,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::Fake;

    #[test]
    #[should_panic]
    fn title_cannot_be_empty() {
        let description = (0..3000).fake();

        create_ticket("".into(), description, Status::ToDo);
    }

    #[test]
    #[should_panic]
    fn title_cannot_be_longer_than_fifty_chars() {
        let description = (0..3000).fake();
        let title = (51..10_000).fake();

        create_ticket(title, description, Status::ToDo);
    }

    #[test]
    #[should_panic]
    fn description_cannot_be_longer_than_3000_chars() {
        let description = (3001..10_000).fake();
        let title = (1..50).fake();

        create_ticket(title, description, Status::ToDo);
    }

    #[test]
    fn valid_tickets_can_be_created() {
        let description = (0..3000).fake();
        let title = (1..50).fake();
        let status = Status::Done;

        create_ticket(title, description, status);
    }
}
