struct Ticket {
    title: String,
    description: String,
    status: Status,
}

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
    fn a_blocked_ticket() {
        let ticket = Ticket {
            title: "A ticket title".into(),
            description: "A heart-breaking description".into(),
            status: Status::Blocked,
        };

        match ticket.status {
            Status::Blocked => println!("Great, as expected!"),
            Status::ToDo | Status::InProgress | Status::Done => {
                panic!("The ticket is not blocked!")
            }
        }
    }
}
