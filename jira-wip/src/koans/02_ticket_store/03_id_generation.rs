use super::recap::Ticket;
use std::collections::HashMap;

pub type TicketId = u32;

struct TicketStore {
    data: HashMap<TicketId, Ticket>,
    current_count: u32,
}

impl TicketStore {
    pub fn new() -> TicketStore {
        TicketStore {
            data: HashMap::new(),
            current_count: 0,
        }
    }

    pub fn save(&mut self, ticket: Ticket) -> TicketId {
        let id = self.generate_id();
        self.data.insert(id, ticket);
        id
    }

    pub fn get(&self, id: &TicketId) -> Option<&Ticket> {
        self.data.get(id)
    }

    fn generate_id(&mut self) -> TicketId {
        self.current_count += 1;
        self.current_count
    }
}

#[cfg(test)]
mod tests {
    use super::super::recap::{create_ticket, Status};
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn a_ticket_with_a_home() {
        let ticket = generate_ticket(Status::ToDo);
        let mut store = TicketStore::new();

        let ticket_id = store.save(ticket.clone());

        assert_eq!(store.get(&ticket_id), Some(&ticket));
        assert_eq!(ticket_id, 1);
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
            let ticket = generate_ticket(Status::ToDo);
            let ticket_id = store.save(ticket);
            assert_eq!(expected_id, ticket_id);
        }
    }

    fn generate_ticket(status: Status) -> Ticket {
        let description = (0..3000).fake();
        let title = (1..50).fake();

        create_ticket(title, description, status)
    }
}
