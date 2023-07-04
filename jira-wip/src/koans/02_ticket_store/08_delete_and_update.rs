use super::id_generation::TicketId;
use super::recap::Status;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::error::Error;

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
        let timestamp = Utc::now();
        let ticket = Ticket {
            id,
            title: draft.title,
            description: draft.description,
            status: Status::ToDo,
            created_at: timestamp.clone(),
            updated_at: timestamp,
        };
        self.data.insert(id, ticket);
        id
    }

    pub fn get(&self, id: &TicketId) -> Option<&Ticket> {
        self.data.get(id)
    }

    pub fn list(&self) -> Vec<&Ticket> {
        self.data.values().collect()
    }

    pub fn update(&mut self, id: &TicketId, patch: TicketPatch) -> Option<&Ticket> {
        if let Some(ticket) = self.data.get_mut(id) {
            if let Some(title) = patch.title {
                ticket.title = title;
            }
            if let Some(description) = patch.description {
                ticket.description = description;
            }
            if let Some(status) = patch.status {
                ticket.status = status;
            }

            ticket.updated_at = Utc::now();

            Some(ticket)
        } else {
            None
        }
    }

    pub fn delete(&mut self, id: &TicketId) -> Option<DeletedTicket> {
        self.data.remove(id).map(|ticket| DeletedTicket {
            ticket: ticket,
            deleted_at: Utc::now(),
        })
    }

    fn generate_id(&mut self) -> TicketId {
        self.current_id += 1;
        self.current_id
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TicketTitle(String);

impl TicketTitle {
    pub fn new(title: String) -> Result<Self, ValidationError> {
        if title.is_empty() {
            return Err(ValidationError("Title cannot be empty!".to_string()));
        }
        if title.len() > 50 {
            return Err(ValidationError(
                "A title cannot be longer than 50 characters!".to_string(),
            ));
        }
        Ok(Self(title))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TicketDescription(String);

impl TicketDescription {
    pub fn new(description: String) -> Result<Self, ValidationError> {
        if description.len() > 3000 {
            Err(ValidationError(
                "A description cannot be longer than 3000 characters!".to_string(),
            ))
        } else {
            Ok(Self(description))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TicketPatch {
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
    pub status: Option<Status>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeletedTicket {
    ticket: Ticket,
    deleted_at: DateTime<Utc>,
}

impl DeletedTicket {
    pub fn ticket(&self) -> &Ticket {
        &self.ticket
    }
    pub fn deleted_at(&self) -> &DateTime<Utc> {
        &self.deleted_at
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ValidationError(String);

impl Error for ValidationError {}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    id: TicketId,
    title: TicketTitle,
    description: TicketDescription,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Ticket {
    pub fn title(&self) -> &TicketTitle {
        &self.title
    }
    pub fn description(&self) -> &TicketDescription {
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
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};
    use std::time::Duration;

    #[test]
    fn updating_nothing_leaves_the_updatable_fields_unchanged() {
        let mut store = TicketStore::new();
        let draft = generate_ticket_draft();
        let ticket_id = store.save(draft.clone());

        let patch = TicketPatch {
            title: None,
            description: None,
            status: None,
        };
        let updated_ticket = store.update(&ticket_id, patch).unwrap();

        assert_eq!(draft.title, updated_ticket.title);
        assert_eq!(draft.description, updated_ticket.description);
        assert_eq!(Status::ToDo, updated_ticket.status);
    }

    #[test]
    fn trying_to_update_a_missing_ticket_returns_none() {
        let mut store = TicketStore::new();
        let ticket_id = Faker.fake();
        let patch = generate_ticket_patch(Status::Done);

        assert_eq!(store.update(&ticket_id, patch), None);
    }

    #[test]
    fn update_works() {
        let mut store = TicketStore::new();
        let draft = generate_ticket_draft();
        let patch = generate_ticket_patch(Status::Done);
        let ticket_id = store.save(draft.clone());

        // Let's wait a bit, otherwise `created_at` and `updated_at`
        // might turn out identical (ᴗ˳ᴗ)
        std::thread::sleep(Duration::from_millis(100));
        let updated_ticket = store.update(&ticket_id, patch.clone()).unwrap();

        assert_eq!(patch.title.unwrap(), updated_ticket.title);
        assert_eq!(patch.description.unwrap(), updated_ticket.description);
        assert_eq!(patch.status.unwrap(), updated_ticket.status);
        assert_ne!(updated_ticket.created_at(), updated_ticket.updated_at());
    }

    #[test]
    fn delete_works() {
        let mut store = TicketStore::new();
        let draft = generate_ticket_draft();
        let ticket_id = store.save(draft.clone());
        let ticket = store.get(&ticket_id).unwrap().to_owned();

        let deleted_ticket = store.delete(&ticket_id).unwrap();

        assert_eq!(deleted_ticket.ticket(), &ticket);
        assert_eq!(store.get(&ticket_id), None);
    }

    #[test]
    fn deleting_a_missing_ticket_returns_none() {
        let mut store = TicketStore::new();
        let ticket_id = Faker.fake();

        assert_eq!(store.delete(&ticket_id), None);
    }

    #[test]
    fn list_returns_all_tickets() {
        let n_tickets = 100;
        let mut store = TicketStore::new();

        for _ in 0..n_tickets {
            let draft = generate_ticket_draft();
            store.save(draft);
        }

        assert_eq!(n_tickets, store.list().len());
    }

    #[test]
    fn on_a_single_ticket_list_and_get_agree() {
        let mut store = TicketStore::new();

        let draft = generate_ticket_draft();
        let id = store.save(draft);

        assert_eq!(vec![store.get(&id).unwrap()], store.list());
    }

    #[test]
    fn list_returns_an_empty_vec_on_an_empty_store() {
        let store = TicketStore::new();

        assert!(store.list().is_empty());
    }

    #[test]
    fn title_cannot_be_empty() {
        assert!(TicketTitle::new("".into()).is_err())
    }

    #[test]
    fn title_cannot_be_longer_than_fifty_chars() {
        // Let's generate a title longer than 51 chars.
        let title = (51..10_000).fake();

        assert!(TicketTitle::new(title).is_err())
    }

    #[test]
    fn description_cannot_be_longer_than_3000_chars() {
        let description = (3001..10_000).fake();

        assert!(TicketDescription::new(description).is_err())
    }

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
        assert_eq!(retrieved_ticket.created_at(), retrieved_ticket.updated_at());
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

    #[test]
    fn ids_are_not_reused() {
        let n_tickets = 100;
        let mut store = TicketStore::new();

        for expected_id in 1..n_tickets {
            let draft = generate_ticket_draft();
            let ticket_id = store.save(draft);
            assert_eq!(expected_id, ticket_id);
            assert!(store.delete(&ticket_id).is_some());
        }
    }

    fn generate_ticket_draft() -> TicketDraft {
        let description = TicketDescription::new((0..3000).fake()).unwrap();
        let title = TicketTitle::new((1..50).fake()).unwrap();

        TicketDraft { title, description }
    }

    fn generate_ticket_patch(status: Status) -> TicketPatch {
        let patch = generate_ticket_draft();

        TicketPatch {
            title: Some(patch.title),
            description: Some(patch.description),
            status: Some(status),
        }
    }
}
