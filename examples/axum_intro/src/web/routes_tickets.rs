use crate::{Error, Result};
use crate::model::{ModelContoller, Ticket, TicketForCreate};
use axum::routing::{get, post, delete};
use axum::extract::{State, Path};
use axum::{Json, Router};


pub fn routes(mc: ModelContoller) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket))
        .with_state(mc)
}

// region:          --- REST Handlers


async fn create_ticket(
    mc: ModelContoller, 
    Json(ticket_fc): Json<TicketForCreate>
    ) -> Result<Json<Ticket>> {
    println!("->> {:12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(
    State(mc): State<ModelContoller>,
    ) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:12} - list_tickets", "HANDLER");

    let tickets = mc.list_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelContoller>,
    Path(id): Path<u64>,
    ) -> Result<Json<Ticket>> {
    println!("->> {:15} - delete_ticket", "HANDLER");

    let ticket = mc.delete_ticket(id).await?;

    Ok(Json(ticket))
}
// endregion:          --- REST Handlers
