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
    //First Create sqlx migration file(sqlx cli tools)
    // db_migration(&db).await?;
    // test_qr(&db).await;

    let ui = AppWindow::new()?;
    
    
    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         // ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    ui.run()?;

    Ok(())
}


async fn db_migration(db:&sqlx::Pool<Sqlite>)->Result<(), sqlx::migrate::MigrateError>{
    let migrations: std::path::PathBuf = std::path::Path::new(&SQL_DIR).join("migrations");
    println!("{:?}",migrations);
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(db)
        .await;
    migration_results
}

async fn test_qr(db:&sqlx::Pool<Sqlite>){
    // let mut conn = db.acquire().await.unwrap();
    // let query = format!("SELECT * FROM device");

    // let account = sqlx::query(&query)
    // .execute(db)
    // .await
    // .unwrap();
    // .fetch_all(db).await.unwrap();
    let res=sqlx::query_as::<_,Foo>("SELECT * FROM device")
    .fetch_all(db).await.unwrap();
    println!("{:?}",res);
    
    
    
}


#[derive(sqlx::FromRow, Debug)]
struct Foo {
  id: i32,
  name: String,
  active:bool
}