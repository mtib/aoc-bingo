use rocket::routes;
use rocket_cors::CorsOptions;

mod health;
mod leaderboard;

trait MountRoutes {
    fn mount_routes(self: Self) -> Self;
}

impl MountRoutes for rocket::Rocket<rocket::Build> {
    fn mount_routes(self: Self) -> Self {
        self.mount("/", routes![health::health])
            .mount("/leaderboard", routes![leaderboard::index])
    }
}

pub fn build() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount_routes()
        .attach(CorsOptions::default().to_cors().unwrap())
}
