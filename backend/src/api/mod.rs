use rocket::{Config, routes};
use rocket_cors::CorsOptions;

use crate::db::DbPool;

mod game;
mod health;
mod leaderboard;

trait ConfigureRocket {
    fn mount_routes(self: Self) -> Self;
    fn config(self: Self) -> Self;
}

impl ConfigureRocket for rocket::Rocket<rocket::Build> {
    fn mount_routes(self: Self) -> Self {
        self.mount("/", routes![health::health])
            .mount(
                "/leaderboard",
                routes![leaderboard::index, leaderboard::bingo_all],
            )
            .mount(
                "/game",
                routes![
                    game::create,
                    game::get_members,
                    game::get_all_puzzles,
                    game::create_membership,
                    game::delete_membership,
                    game::get_completion,
                ],
            )
    }

    fn config(self: Self) -> Self {
        let base = self.figment().clone();
        self.configure(
            base.merge((Config::LOG_LEVEL, "off"))
                .merge((Config::CLI_COLORS, "false")),
        )
    }
}

pub fn build(pool: DbPool) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .manage(pool)
        .mount_routes()
        .config()
        .attach(CorsOptions::default().to_cors().unwrap())
}
