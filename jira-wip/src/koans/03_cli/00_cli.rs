//! When you are ready, uncomment the appropriate lines from src/main.rs and
//! run `cargo run --bin jira-wip` in your terminal!
use super::id_generation::TicketId;
use super::store_recap::{
    Status, TicketDescription, TicketDraft, TicketPatch, TicketStore, TicketTitle,
};
use std::error::Error;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(structopt::StructOpt, Clone)]
pub enum Command {
    /// Create a ticket on your board.
    Create {
        #[structopt(long)]
        title: TicketTitle,
        #[structopt(long)]
        description: TicketDescription,
    },
    /// Edit the details of an existing ticket.
    Edit {
        /// Id of the ticket you want to edit.
        #[structopt(long)]
        id: TicketId,
        /// New status of the ticket.
        #[structopt(long)]
        status: Option<Status>,
        /// New description of the ticket.
        #[structopt(long)]
        description: Option<TicketDescription>,
        /// New title for your ticket.
        #[structopt(long)]
        title: Option<TicketTitle>,
    },
    /// Delete a ticket from the store passing the ticket id.
    Delete {
        #[structopt(long)]
        ticket_id: TicketId,
    },
    /// List all existing tickets.
    List,
}

/// `structopt` relies on `FromStr` to know how to parse our custom structs and enums
/// from the string passed in as input by a user.
///
/// Parsing is fallible: we need to declare what error type we are going to return if
/// things go wrong and implement the `from_str` function.
impl FromStr for Status {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "todo" => Ok(Status::ToDo),
            "blocked" => Ok(Status::Blocked),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(ParsingError("Invalid status!".to_owned())),
        }
    }
}

impl FromStr for TicketTitle {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match TicketTitle::new(s.to_string()) {
            Err(e) => Err(ParsingError(e.to_string())),
            Ok(title) => Ok(title),
        }
    }
}

impl FromStr for TicketDescription {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match TicketDescription::new(s.to_string()) {
            Err(e) => Err(ParsingError(e.to_string())),
            Ok(description) => Ok(description),
        }
    }
}

/// Our error struct for parsing failures.
#[derive(Debug)]
pub struct ParsingError(String);

impl Error for ParsingError {}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

pub fn handle_command(
    ticket_store: &mut TicketStore,
    command: Command,
) -> Result<(), Box<dyn Error>> {
    match command {
        Command::Create { description, title } => {
            todo!()
        }
        Command::Edit {
            id,
            title,
            description,
            status,
        } => {
            todo!()
        }
        Command::Delete { ticket_id } => match ticket_store.delete(&ticket_id) {
            Some(deleted_ticket) => println!(
                "The following ticket has been deleted:\n{:?}",
                deleted_ticket
            ),
            None => println!(
                "There was no ticket associated to the ticket id {:?}",
                ticket_id
            ),
        },
        Command::List => {
            todo!()
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_status_fails_to_be_parsed() {
        let invalid_status = "Not a good status";
        assert!(Status::from_str(invalid_status).is_err());
    }
}
