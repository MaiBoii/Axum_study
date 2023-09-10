use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};


/* ---------------------------------- 티켓 타입 --------------------------------- */
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title:String,
}

/* --------------------------------- 모델 컨트롤러 -------------------------------- */
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,

}

/* --------------------------------- 생성자 --------------------------------- */
//본인을 만들어내는 생성자
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            //Arc의 기본값으로 생성
            tickets_store: Arc::default(),
        })
    }
}

/* --------------------------------- CRUD 구현 -------------------------------- */
// ModelController의 CRUD 기능 구현하기
impl ModelController{
    // CREATE: 새로운 티켓 만들어내기
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket>{
        // 
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));
        
        Ok(ticket)
    }

    //LIST: 만들어둔 티켓 전부 보여주기
    pub async fn list_ticket(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    //DELETE: 있는 티켓 id로 지정해서 삭제하기
    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket>{
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}

#[derive(Deserialize)]
pub struct TicketForCreate{
    pub title: String,
}