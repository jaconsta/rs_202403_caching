mod contact;

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{contacts::Contact, state::AppState};

#[derive(Serialize, Deserialize)]
pub struct ContactsResponse {
    pub contacts: Vec<Contact>,
}

#[derive(Serialize, Deserialize)]
pub struct ContactResponse {
    pub contact: Contact,
}

pub async fn contacts_list(
    State(state): State<AppState>,
    // ) -> Result<Json<Vec<Contact>>, StatusCode> {
) -> Result<Json<ContactsResponse>, StatusCode> {
    match contact::list::list_contacts(state).await {
        Ok(c) => Ok(Json(ContactsResponse { contacts: c })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn contacts_create(
    State(state): State<AppState>,
    Json(body): Json<contact::create::CreateContactBody>,
) -> Result<(StatusCode, Json<ContactResponse>), StatusCode> {
    match contact::create::create(state, body).await {
        Ok(c) => Ok((StatusCode::CREATED, Json(ContactResponse { contact: c }))),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
