use crate::state::AppState;

static QUERY: &str = "
DELETE FROM contacts
WHERE id = $1
";

pub async fn delete_by_id(state: AppState, id: i64) -> Result<u64, Box<dyn std::error::Error>> {
    let res = sqlx::query(QUERY).bind(id).execute(&state.database).await?;

    tokio::spawn(async move {
        let _ = state.cache.del(id).await;
    });

    Ok(res.rows_affected())
}
