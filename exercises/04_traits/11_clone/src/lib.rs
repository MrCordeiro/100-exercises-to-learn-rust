// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.

pub struct Summary {
    pub title: String,
    pub status: String,
}

#[derive(Clone)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    // Clone an instance of Ticket before using it
    (ticket.clone(), ticket.summary())
}
