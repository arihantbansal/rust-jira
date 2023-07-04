use super::store_recap::TicketStore;
use std::fs::read_to_string;
use std::path::Path;

/// Fetch authentication parameters from a configuration file, if available.
pub fn load(path: &Path) -> TicketStore {
    println!("Reading data from {:?}", path);
    // Read the data in memory, storing the value in a string
    match read_to_string(path) {
        Ok(data) => {
            // Deserialize configuration from YAML format
            serde_yaml::from_str(&data).expect("Failed to parse serialised data.")
        }
        Err(e) => match e.kind() {
            // The file is missing - this is the first time you are using IronJira!
            std::io::ErrorKind::NotFound => {
                // Return default configuration
                TicketStore::new()
            }
            // Something went wrong - crash the CLI with an error message.
            _ => panic!("Failed to read data."),
        },
    }
}

/// Save tickets on disk in the right file.
pub fn save(ticket_store: &TicketStore, path: &Path) {
    // Serialize data to YAML format
    let content = serde_yaml::to_string(ticket_store).expect("Failed to serialize tickets");
    println!("Saving tickets to {:?}", path);
    // Save to disk
    std::fs::write(path, content).expect("Failed to write tickets to disk.")
}

#[cfg(test)]
mod tests {
    use super::super::store_recap::{
        Status, TicketDescription, TicketDraft, TicketStore, TicketTitle,
    };
    use super::*;
    use fake::Fake;
    use tempfile::NamedTempFile;

    #[test]
    fn load_what_you_save() {
        let mut store = TicketStore::new();
        let draft = generate_ticket_draft();
        store.save(draft);

        // We use the `tempfile` crate to generate a temporary path on the fly
        // which will be cleaned up at the end of the test.
        // See https://docs.rs/tempfile/3.1.0/tempfile/ for more details.
        let temp_path = NamedTempFile::new().unwrap().into_temp_path();

        save(&store, temp_path.as_ref());
        let loaded_store = load(temp_path.as_ref());

        assert_eq!(store, loaded_store);
    }

    fn generate_ticket_draft() -> TicketDraft {
        let description = TicketDescription::new((0..3000).fake()).unwrap();
        let title = TicketTitle::new((1..50).fake()).unwrap();

        TicketDraft { title, description }
    }
}
