mod contact;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
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

pub async fn contacts_read(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ContactResponse>, StatusCode> {
    match contact::read::find_by_id(state, id).await {
        Ok(c) => match c {
            Some(x) => Ok(Json(ContactResponse { contact: x })),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn contacts_update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<contact::update::UpdateContactBody>,
) -> Result<Json<ContactResponse>, StatusCode> {
    match contact::update::update_by_id(state, id, body).await {
        Ok(c) => match c {
            Some(c) => Ok(Json(ContactResponse { contact: c })),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn contacts_delete(State(state): State<AppState>, Path(id): Path<i64>) -> StatusCode {
    match contact::delete::delete_by_id(state, id).await {
        Ok(c) => match c {
            0 => StatusCode::NOT_FOUND,
            _ => StatusCode::OK,
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
