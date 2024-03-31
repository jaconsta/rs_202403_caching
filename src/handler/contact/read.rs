use std::error::Error;

use crate::{contacts::Contact, state::AppState};

static QUERY: &str = "
SELECT id, name, email, created_at, updated_at
FROM contacts 
WHERE id = $1
";

pub async fn find_by_id(state: AppState, id: i64) -> Result<Option<Contact>, Box<dyn Error>> {
    let res: Option<Contact> = sqlx::query_as(QUERY)
        .bind(id)
        .fetch_optional(&state.database)
        .await?;

    Ok(res)
}
