use rocket::routes;

mod health;

trait MountRoutes {
    fn mount_routes(self: Self) -> Self;
}

impl MountRoutes for rocket::Rocket<rocket::Build> {
    fn mount_routes(self: Self) -> Self {
        self.mount("/", routes![health::index])
    }
}

pub fn build() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount_routes()
}
