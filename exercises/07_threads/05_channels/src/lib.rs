use std::sync::mpsc::{Receiver, Sender};
use crate::data::TicketDraft;
use crate::store::TicketStore;

pub mod data;
pub mod store;

pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

pub fn server(receiver: Receiver<Command>) {
    let mut db = TicketStore::new();

    loop {
        let message = receiver.recv();
        match message {
            Ok(command) => {
                print!("Received command");
                match command {
                    Command::Insert(ticket) => {
                        db.add_ticket(ticket);
                    },
                }
            },
            _ => panic!("Error"),
        }
    }
}
