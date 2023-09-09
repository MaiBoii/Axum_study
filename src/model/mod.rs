use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};


//region: ---Ticket
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title:String,
}

pub struct ModelController {
    ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>,
    

}

#[derive(Deserialize)]
pub struct TicketForCreate{
    pub title: String,
}