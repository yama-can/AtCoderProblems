use anyhow::Result;
use atcoder_client::AtCoderClient;
use atcoder_problems_backend::crawler::WholeContestCrawler;
use atcoder_problems_backend::utils::init_log_config;
use log::info;
use sql_client::initialize_pool;
use std::env;

#[actix_web::main]
async fn main() -> Result<()> {
    init_log_config()?;
    info!("Started");
    let url = env::var("SQL_URL").expect("SQL_URL should be set as environmental variable.");
    let revel_session = env::var("ATCODER_SESSION").expect("ATCODER_SESSION is not set.");
    let contest_id = env::args()
        .nth(1)
        .expect("contest_id is not set.\nUsage: cargo run --bin crawl_whole_contest <contest_id>");
    let db = initialize_pool(&url).await?;
    let client = AtCoderClient::new(&revel_session).await?;
    let crawler = WholeContestCrawler::new(db, client, contest_id);
    crawler.crawl().await?;
    Ok(())
}
