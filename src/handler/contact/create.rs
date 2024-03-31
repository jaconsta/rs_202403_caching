use serde::Deserialize;

use crate::{contacts::Contact, state::AppState};

#[derive(Deserialize)]
pub struct CreateContactBody {
    pub name: String,
    pub email: String,
}

static QUERY: &str = "
INSERT INTO contacts
(name, email)
VALUES ($1, $2)
RETURNING id, name, email, created_at, updated_at
";

pub async fn create(
    state: AppState,
    body: CreateContactBody,
) -> Result<Contact, Box<dyn std::error::Error>> {
    let contact = sqlx::query_as(QUERY)
        .bind(&body.name)
        .bind(&body.email)
        .fetch_one(&state.database)
        .await?;
    Ok(contact)
}
