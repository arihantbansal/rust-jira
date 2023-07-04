use super::id_generation::TicketId;
use super::recap::Status;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// On the topic of type-driven development, checkout:
/// - https://fsharpforfunandprofit.com/series/designing-with-types.html
/// - https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/
/// - https://www.youtube.com/watch?v=PLFl95c-IiU
///
#[derive(Debug, Clone, PartialEq)]
pub struct TicketDraft {
    title: String,
    description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    id: TicketId,
    title: String,
    description: String,
    status: Status,
    created_at: DateTime<Utc>,
}

struct TicketStore {
    data: HashMap<TicketId, Ticket>,
    current_id: TicketId,
}

impl TicketStore {
    pub fn new() -> TicketStore {
        TicketStore {
            data: HashMap::new(),
            current_id: 0,
        }
    }

    pub fn save(&mut self, draft: TicketDraft) -> TicketId {
        let id = self.generate_id();

        let ticket = Ticket {
            title: draft.title.to_owned(),
            description: draft.description().to_owned(),
            status: Status::ToDo,
            id,
            created_at: Utc::now(),
        };
        self.data.insert(id, ticket);
        id
    }

    pub fn get(&self, id: &TicketId) -> Option<&Ticket> {
        self.data.get(id)
    }

    fn generate_id(&mut self) -> TicketId {
        self.current_id += 1;
        self.current_id
    }
}

impl TicketDraft {
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn description(&self) -> &String {
        &self.description
    }
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
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    pub fn id(&self) -> &TicketId {
        &self.id
    }
}

pub fn create_ticket_draft(title: String, description: String) -> TicketDraft {
    if title.is_empty() {
        panic!("Title cannot be empty!");
    }
    if title.len() > 50 {
        panic!("A title cannot be longer than 50 characters!");
    }
    if description.len() > 3000 {
        panic!("A description cannot be longer than 3000 characters!");
    }

    TicketDraft { title, description }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn a_ticket_with_a_home() {
        let draft = generate_ticket_draft();
        let mut store = TicketStore::new();

        let ticket_id = store.save(draft.clone());
        let retrieved_ticket = store.get(&ticket_id).unwrap();

        assert_eq!(&ticket_id, retrieved_ticket.id());
        assert_eq!(&draft.title, retrieved_ticket.title());
        assert_eq!(&draft.description, retrieved_ticket.description());
        assert_eq!(&Status::ToDo, retrieved_ticket.status());
    }

    #[test]
    fn a_missing_ticket() {
        let ticket_store = TicketStore::new();
        let ticket_id = Faker.fake();

        assert_eq!(ticket_store.get(&ticket_id), None);
    }

    #[test]
    fn id_generation_is_monotonic() {
        let n_tickets = 100;
        let mut store = TicketStore::new();

        for expected_id in 1..n_tickets {
            let draft = generate_ticket_draft();
            let ticket_id = store.save(draft);
            assert_eq!(expected_id, ticket_id);
        }
    }

    fn generate_ticket_draft() -> TicketDraft {
        let description = (0..3000).fake();
        let title = (1..50).fake();

        create_ticket_draft(title, description)
    }
}
