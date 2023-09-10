use anyhow::Error;
/* ------------------------------ RestAPI 구현하기 ------------------------------ */
use crate::Result;

use axum::{Json, extract::{State, Path, FromRef}, Router, routing::{post, delete}};
use crate::model::{ModelController, Ticket, TicketForCreate};

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController
}


pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_ticket))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDELER");

    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_ticket(
    State(mc): State<ModelController>,
) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - create_ticket", "HANDELER");
    
    let tickets = mc.list_ticket().await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc):State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!(">>> {:<15} - delete_ticket", "HANDLER");

    let ticket = mc.delete_ticket(id).await?;

    Ok(Json(ticket))
}


/* -------------------------------------------------------------------------- */