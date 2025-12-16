use backend::{DatabaseManager, build as build_api};

#[tokio::main]
async fn main() {
    let db_manager = DatabaseManager::new("./data/db.sqlite").expect("Failed to create database manager");
    db_manager.init();

    let rocket = build_api(db_manager.get_pool().clone()).ignite().await.unwrap();
    let rocket_shutdown = rocket.shutdown();

    let task = tokio::task::spawn(async {
        println!("Starting backend server...");
        rocket.launch().await.unwrap();
        println!("Backend server has stopped.");
    });

    tokio::pin!(task);

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Shutdown signal received.");
            rocket_shutdown.notify();
        }
        _ = &mut task => {
            println!("Backend server task has completed.");
        }
    };
}
