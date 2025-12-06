use backend::{DatabaseManager, build as build_api};

#[tokio::main]
async fn main() {
    DatabaseManager::new().init();

    let rocket = build_api().ignite().await.unwrap();
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
