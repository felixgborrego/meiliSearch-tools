use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MeilisearchStats {
    indexes: HashMap<String, IndexStats>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IndexStats {
    #[serde(rename = "numberOfDocuments")]
    number_of_documents: u64,
    #[serde(rename = "isIndexing")]
    is_indexing: bool,
}

// Fetch stats from the Meilisearch API
fn fetch_stats(api: &str, api_key: &str) -> reqwest::Result<MeilisearchStats> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(60))
        .connect_timeout(Duration::from_secs(60))
        .build()?;

    let api_url = format!("{}/stats", api);
    loop {
        let response = client
            .get(&api_url)
            .header("Authorization", format!("Bearer {}", api_key))
            .send()?;

        if response.status().as_u16() == 408 {
            println!("Request timeout, retrying...");
        } else {
            return response.json::<MeilisearchStats>();
        }
    }
}

pub fn print_stats(api_host: &str, api_key: &str) {
    loop {
        match fetch_stats(api_host, api_key) {
            Ok(stats) => {
                for (index_name, index_stats) in stats.indexes {
                    println!(
                        "Index: {}, isIndexing: {}, numberOfDocuments: {}",
                        index_name, index_stats.is_indexing, index_stats.number_of_documents
                    );
                }
            }
            Err(e) => {
                eprintln!("Error fetching stats: {}", e);
            }
        }

        // sleep for 1 second
        std::thread::sleep(Duration::from_secs(1));
    }
}
