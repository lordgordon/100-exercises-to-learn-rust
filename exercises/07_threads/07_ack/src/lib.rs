use std::sync::mpsc::{Receiver, Sender};
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_sender: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_sender: Sender<Ticket>,
    }
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert{draft, response_sender}) => {
                let id = store.add_ticket(draft);
                let result = response_sender.send(id);
                result.unwrap();
            }
            Ok(Command::Get{id, response_sender}) => {
                let ticket = store.get(id).unwrap();
                let result = response_sender.send(ticket.clone());
                result.unwrap();
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break
            },
        }
    }
}
