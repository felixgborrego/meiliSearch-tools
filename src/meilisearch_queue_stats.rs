use reqwest;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct MeilisearchQueueStats {
    results: Vec<Task>,
    total: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    status: String,
}

fn fetch_queue_stats(api_host: &str, api_key: &str) -> reqwest::Result<(usize, usize, usize)> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(60))
        .connect_timeout(Duration::from_secs(60))
        .build()?;

    let api_url = format!("{}/tasks?statuses=processing,failed", api_host);

    let mut count_processing = 0;
    let mut count_failed = 0;
    let mut total = 0;
    loop {
        let response = client
            .get(&api_url)
            .header("Authorization", format!("Bearer {}", api_key))
            .send()?;
        if response.status().as_u16() == 408 {
            println!("Request timeout, retrying...");
        } else {
            println!("Response status: {}", response.status());
            //println!("Response string body {}", response.strbytes()?.)
            let queue_stats = response.json::<MeilisearchQueueStats>()?;

            count_processing = queue_stats
                .results
                .iter()
                .filter(|task| task.status == "processing")
                .count();

            count_failed = queue_stats
                .results
                .iter()
                .filter(|task| task.status == "failed")
                .count();
            total = queue_stats.total;
            break;
        }
    }
    Ok((total, count_failed, count_processing))
}

pub fn print_queue_stats(api_host: &str, api_key: &str) {
    loop {
        match fetch_queue_stats(api_host, api_key) {
            Ok((total, failed, processing)) => {
                println!(
                    "Total number in the queue ##{}## with 'processing' {}  and 'failed'  {}",
                    total, processing, failed
                );
            }
            Err(e) => {
                eprintln!("Error fetching queue stats: {:?}", e);
            }
        }
        // wait
        std::thread::sleep(Duration::from_secs(1));
    }
}
