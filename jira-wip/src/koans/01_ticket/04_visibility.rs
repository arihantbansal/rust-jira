pub mod ticket {
    pub enum Status {
        ToDo,
        InProgress,
        Blocked,
        Done,
    }

    pub struct Ticket {
        title: String,
        description: String,
        status: Status,
    }

    pub fn create_ticket(title: String, description: String, status: Status) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty!");
        }
        if title.len() > 50 {
            panic!("A title cannot be longer than 50 characters!");
        }
        if description.len() > 3000 {
            panic!("A description cannot be longer than 3000 characters!");
        }

        Ticket {
            title,
            description,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ticket::{create_ticket, Status, Ticket};

    fn should_not_be_possible() {
        let ticket: Ticket = create_ticket("A title".into(), "A description".into(), Status::ToDo);
        // assert_eq!(ticket.description, "A description");
    }

    fn encapsulation_cannot_be_violated() {
        // let ticket = Ticket {
        //     title: "A title".into(),
        //     description: "A description".into(),
        //     status: Status::ToDo,
        // };
    }
}
