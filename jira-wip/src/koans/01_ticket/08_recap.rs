#[derive(PartialEq, Debug, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Blocked,
    Done,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
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
    #[test]
    fn the_next_step_of_your_journey() {
        let i_am_ready_to_continue = true;

        assert!(i_am_ready_to_continue);
    }
}
