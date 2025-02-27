use fred::types::{Expiration, SetOptions};
use serde::Deserialize;

use crate::{contacts::Contact, state::AppState};

#[derive(Deserialize)]
pub struct UpdateContactBody {
    pub name: String,
    pub email: String,
}

static QUERY: &str = "
UPDATE contacts 
SET name = $1, email = $2, updated_at = now()
WHERE id = $3
RETURNING id, name, email, created_at, updated_at
";

pub async fn update_by_id(
    state: AppState,
    id: i64,
    body: UpdateContactBody,
) -> Result<Option<Contact>, Box<dyn std::error::Error>> {
    let res: Option<Contact> = sqlx::query_as(QUERY)
        .bind(body.name)
        .bind(body.email)
        .bind(id)
        .fetch_optional(&state.database)
        .await?;

    if let Some(contact) = &res {
        let contact = contact.clone();
        let state = state.clone();
        tokio::spawn(async move {
            // Write through caching.
            let _ = state.cache.set(
                id,
                &contact,
                Some(Expiration::EX(60)),
                Some(SetOptions::XX), // Only if key exists
                false,
            );
        });
    }

    Ok(res)
}
