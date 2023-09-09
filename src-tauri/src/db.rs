use sqlx::{ migrate::MigrateDatabase, Sqlite, SqlitePool };

const DB_URL: &str = "sqlite://sqlite.db";

// Check for DB, create if non existent
pub async fn init() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    create_schema().await;
    
    // Uncomment the fn below and run to make some records for testing
    insert_dev_records().await;
}

// Create Schema
async fn create_schema() {
    let pool = SqlitePool::connect(DB_URL).await.expect("unable to connect");
    let sql = "
        PRAGMA foreign_keys = ON ;
        CREATE TABLE IF NOT EXISTS projects
        (
            id              INTEGER    PRIMARY KEY    NOT NULL,
            name            TEXT                      NOT NULL
        );

        CREATE TABLE IF NOT EXISTS tasks
        (
            id              INTEGER    PRIMARY KEY    NOT NULL,
            value           TEXT                      NOT NULL,
            completed       INTEGER                   NOT NULL,
            date_completed  TEXT,
            project_id      INTEGER                   NOT NULL,
            FOREIGN KEY (project_id)   REFERENCES projects (id) ON UPDATE SET NULL ON DELETE SET NULL
        );
    ";
    
    let query = sqlx::query(&sql);
    let result = query.execute(&pool).await.unwrap();
    println!("Create Schema result: {:?}", result);   
    pool.close().await;
}

// Create some test records in each table
async fn insert_dev_records() {
    let pool = SqlitePool::connect(DB_URL).await.expect("unable to connect");
    let sql = "
        INSERT INTO projects (name)
        VALUES ('Awesome Current Product'), ('Top Secret Product'), ('Super Top Secret Product');

        INSERT INTO tasks (value, completed, date_completed, project_id)
        VALUES ('Design the UI',                    0,      NULL,                   3),
               ('Design DB Schema',                 0,      NULL,                   3),
               ('Build prototype app',              0,      NULL,                   3),
               ('Design a cool logo',               1,      DATE('2023-04-22'),     3),
               ('Refactor component lib',           0,      NULL,                   2),
               ('Add input sanitization to ipc',    0,      NULL,                   2),
               ('Security audit testing for v1.5',  0,      NULL,                   1),
               ('Add Dark Mode',                    1,      DATE('2023-04-20'),     1),
               ('Fix UI glitch',                    1,      DATE('2023-04-20'),     1);
    ";
    
    let query = sqlx::query(&sql);
    let result = query.execute(&pool).await.unwrap();
    println!("Create Records result: {:?}", result);   
    pool.close().await;
}