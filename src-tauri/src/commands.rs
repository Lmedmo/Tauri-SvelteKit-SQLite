use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Task {
    pub id: i64,
    pub value: String,
    pub completed: i8,
    pub date_completed: String,
    pub project_id: i64,  
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_tasks() -> Result<Vec<Task>, String>{
    let url = "sqlite://sqlite.db";

    let pool = sqlx::sqlite::SqlitePool::connect(url).await.expect("unable to connect");

    let sql = "SELECT * FROM tasks";

    let query = sqlx::query_as::<_, Task>(sql);
    
    let response = query.fetch_all(&pool).await.expect("unable to list tasks");

    pool.close().await;

    Ok(response)
}

