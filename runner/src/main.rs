// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

slint::include_modules!();

const DB_URL:&'static str = env!("DB_URL");
const SQL_DIR:&'static str = env!("MAIN_PASS");


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    db_migration(&db).await?;
    let ui = AppWindow::new()?;
    

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    ui.run()?;

    Ok(())
}


async fn db_migration(db:&sqlx::Pool<Sqlite>)->Result<(), sqlx::migrate::MigrateError>{
    let migrations: std::path::PathBuf = std::path::Path::new(&SQL_DIR).join("./migrations");
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(db)
        .await;
    migration_results
}