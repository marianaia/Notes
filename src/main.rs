use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use Notes::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = dotenv!("DATABASE_URL");
    run(database_url).await;
}
