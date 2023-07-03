use super::recap::Ticket;
use std::collections::HashMap;

struct TicketStore {
    /// The collection of stored tickets.
    data: HashMap<u32, Ticket>,
}

impl TicketStore {
    pub fn new() -> TicketStore {
        TicketStore {
            data: HashMap::new(),
        }
    }

    pub fn save(&mut self, ticket: Ticket, id: u32) {
        self.data.insert(id, ticket);
    }

    pub fn get(&self, id: &u32) -> &Ticket {
        if !self.data.contains_key(id) {
            panic!("No such id")
        } else {
            self.data.get(id).unwrap()
        }
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
        let ticket_id = Faker.fake();

        store.save(ticket.clone(), ticket_id);

        assert_eq!(store.get(&ticket_id), &ticket);
    }

    #[test]
    #[should_panic]
    fn a_missing_ticket() {
        let ticket_store = TicketStore::new();
        let ticket_id = Faker.fake();

        ticket_store.get(&ticket_id);
    }

    #[test]
    fn inserting_a_ticket_with_an_existing_id_overwrites_previous_ticket() {
        let first_ticket = generate_ticket(Status::ToDo);
        let second_ticket = generate_ticket(Status::ToDo);
        let ticket_id = Faker.fake();
        let mut store = TicketStore::new();

        store.save(first_ticket.clone(), ticket_id);
        assert_eq!(store.get(&ticket_id), &first_ticket);

        store.save(second_ticket.clone(), ticket_id);
        assert_eq!(store.get(&ticket_id), &second_ticket);
    }

    fn generate_ticket(status: Status) -> Ticket {
        let description = (0..3000).fake();
        let title = (1..50).fake();

        create_ticket(title, description, status)
    }
}
