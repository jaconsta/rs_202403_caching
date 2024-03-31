use std::error::Error;

use crate::{contacts::Contact, state::AppState};

static QUERY: &str = "
SELECT id, name, email, created_at, updated_at
from contacts
";

pub async fn list_contacts(state: AppState) -> Result<Vec<Contact>, Box<dyn Error>> {
    let contacts = sqlx::query_as(QUERY).fetch_all(&state.database).await?;
    Ok(contacts)
}

