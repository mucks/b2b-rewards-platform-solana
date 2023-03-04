mod api;
mod db;
mod middleware;
mod schema;

fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
}
