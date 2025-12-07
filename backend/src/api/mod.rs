use rocket::{Config, routes};
use rocket_cors::CorsOptions;

mod health;
mod leaderboard;

trait ConfigureRocket {
    fn mount_routes(self: Self) -> Self;
    fn config(self: Self) -> Self;
}

impl ConfigureRocket for rocket::Rocket<rocket::Build> {
    fn mount_routes(self: Self) -> Self {
        self.mount("/", routes![health::health])
            .mount("/leaderboard", routes![leaderboard::index])
    }

    fn config(self: Self) -> Self {
        let base = self.figment().clone();
        self.configure(
            base.merge((Config::LOG_LEVEL, "off"))
                .merge((Config::CLI_COLORS, "false")),
        )
    }
}

pub fn build() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount_routes()
        .config()
        .attach(CorsOptions::default().to_cors().unwrap())
}
