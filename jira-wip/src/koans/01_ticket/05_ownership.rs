use super::visibility::ticket::Status;

pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
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

#[cfg(test)]
mod tests {
    use super::super::visibility::ticket::Status;
    use super::{create_ticket, Ticket};

    fn verify_without_tampering() {
        let ticket: Ticket = create_ticket("A title".into(), "A description".into(), Status::ToDo);

        assert_eq!(ticket.description(), "A description");
        assert_eq!(ticket.title(), "A title");
    }
}
