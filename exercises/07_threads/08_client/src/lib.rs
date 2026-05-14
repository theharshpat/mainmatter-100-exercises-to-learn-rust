use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: flesh out the client implementation.
pub struct TicketStoreClient {
    sender: Sender<Command>,  // me: to send a new msg request to server. response back channel provided in command args.
}

impl TicketStoreClient {
    // Feel free to panic on all errors, for simplicity.
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        let (res_sender, res_receiver) = std::sync::mpsc::channel();

        let _ = self.sender.send(Command::Insert { 
            draft, 
            response_channel: res_sender 
        });

        res_receiver.recv().unwrap()
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        let (res_sender, res_receiver) = std::sync::mpsc::channel();

        let _ = self.sender.send(Command::Get { id, response_channel: res_sender });

        res_receiver.recv().unwrap()
    }
}

// me: this is the only useful entry point that client should care about now.
// me: it returns client its own struct with its methods to interact with server
// me: TicketStoreClient impl methods -> already running server created by fn server (as only that gets arg of channel reveiver upon creation of launch)
// me: server needs to respond via res channel as otherwise server is not directly reachable
// me: server's response_channel should respond to -> TicketStoreClient impl methods, which should send msg by sender channel object. along with that it must provide arg to get the response back.
// me: so TicketStoreClient impl methods respond to -> tests method calls on TicketStoreClient

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

// No longer public! This becomes an internal detail of the library now.
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
