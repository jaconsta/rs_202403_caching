use std::error::Error;

use fred::types::Expiration;

use crate::{contacts::Contact, state::AppState};

static QUERY: &str = "
SELECT id, name, email, created_at, updated_at
FROM contacts 
WHERE id = $1
";

pub async fn find_by_id(state: AppState, id: i64) -> Result<Option<Contact>, Box<dyn Error>> {
    let cached: Option<Contact> = state.cache.get(id).await.unwrap_or(None);
    if let Some(contact) = cached {
        return Ok(Some(contact));
    }

    let res: Option<Contact> = sqlx::query_as(QUERY)
        .bind(id)
        .fetch_optional(&state.database)
        .await?;

    if let Some(res) = &res {
        let contact = res.clone();
        let state = state.clone();
        tokio::spawn(async move {
            // reminder: set LRU (Least recently used) deletion policy
            let _ = state
                .cache
                .set(id, &contact, Some(Expiration::EX(60)), None, false)
                .await;
        });
    }

    Ok(res)
}
