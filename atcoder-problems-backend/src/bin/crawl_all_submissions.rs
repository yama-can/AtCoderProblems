use actix_web::rt::time;
use anyhow::Result;
use atcoder_client::AtCoderClient;
use atcoder_problems_backend::crawler::WholeContestCrawler;
use atcoder_problems_backend::utils::init_log_config;
use log::{error, info};
use sql_client::initialize_pool;
use sql_client::models::Contest;
use sql_client::simple_client::SimpleClient;
use std::{env, time::Duration};

#[actix_web::main]
async fn main() {
    init_log_config().unwrap();
    info!("Started");
    let url = env::var("SQL_URL").expect("SQL_URL is not set.");

    let revel_session = env::var("ATCODER_SESSION").expect("ATCODER_SESSION is not set.");

    loop {
        info!("Start new loop");

        match load_contest(&url).await {
            Ok(contests) => {
                for contest in contests {
                    finish_one_contest(&url, &contest.id, &revel_session).await;
                }
            }
            Err(e) => {
                error!("Failed to load the contests: {:?}", e);
                sleep_1sec().await;
            }
        }
    }
}

async fn finish_one_contest(url: &str, contest_id: &str, revel_session: &str) {
    loop {
        info!("Starting {}", contest_id);
        match crawl_one_contest(url, contest_id, revel_session).await {
            Ok(_) => {
                info!("Finished {}", contest_id);
                return;
            }
            Err(e) => {
                error!("Error while crawling {}: {:?}", contest_id, e);
                sleep_1sec().await;
            }
        }
    }
}

async fn crawl_one_contest(
    url: &str,
    contest_id: &str,
    revel_session: &str,
) -> Result<()> {
    let db = initialize_pool(url).await?;
    let client = AtCoderClient::new(revel_session).await?;
    let crawler = WholeContestCrawler::new(db, client.clone(), contest_id);
    crawler.crawl().await?;
    Ok(())
}

async fn load_contest(url: &str) -> Result<Vec<Contest>> {
    let db = initialize_pool(url).await?;
    let contests = db.load_contests().await?;
    Ok(contests)
}

async fn sleep_1sec() {
    time::sleep(Duration::from_secs(1)).await;
}
