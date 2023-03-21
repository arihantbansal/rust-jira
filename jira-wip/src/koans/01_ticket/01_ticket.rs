pub struct Ticket {
    title: String,
    description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_first_ticket() {
        let ticket_one = Ticket {
            title: "A ticket title".into(),
            description: "A heart-breaking description".into(),
        };

        assert_eq!(ticket_one.title, "A ticket title");
        assert_eq!(ticket_one.description, "A heart-breaking description");
    }
}
