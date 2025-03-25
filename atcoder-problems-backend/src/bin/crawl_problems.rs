use atcoder_client::AtCoderClient;
use atcoder_problems_backend::crawler::ProblemCrawler;
use atcoder_problems_backend::utils::init_log_config;
use sql_client::initialize_pool;
use std::env;

#[actix_web::main]
async fn main() {
    init_log_config().unwrap();
    log::info!("Started");
    let url = env::var("SQL_URL").expect("SQL_URL is not set.");
    let revel_session = env::var("ATCODER_SESSION").expect("ATCODER_SESSION is not set.");

    let db = initialize_pool(&url).await.unwrap();
    let client = AtCoderClient::new(&revel_session)
        .await
        .expect("AtCoder authentication failure");
    let crawler = ProblemCrawler::new(db, client);
    crawler.crawl().await.expect("Failed to crawl");

    log::info!("Finished");
}
