use std::fs::File;
use std::io::{self, BufReader, Read};
use std::time::{Duration, Instant};

pub fn import_files(
    folder: &str,
    host_api: &str,
    index_name: &str,
    api_key: &str,
) -> Result<(), io::Error> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(60))
        .connect_timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let entries: Vec<_> = std::fs::read_dir(folder)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .collect::<Result<_, _>>()?;

    let total_files = entries.len();
    let total_time = Instant::now();
    let mut total_duration = std::time::Duration::new(0, 0);

    let url = format!(
        "{}/indexes/{}/documents?primaryKey=id",
        host_api, index_name
    );
    for (index, entry) in entries.into_iter().enumerate() {
        let file_start_time = Instant::now();

        if entry
            .file_type()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .is_file()
        {
            let path = entry.path();
            if path.extension().unwrap_or_default() == "json" {
                let file =
                    File::open(&path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                // read as string
                let mut body = String::new();
                let mut reader = BufReader::new(file);
                reader
                    .read_to_string(&mut body)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                println!(
                    "File size: {} bytes (~{:.2} MB) send to {}",
                    body.len(),
                    body.len() as f64 / 1_048_576.0,
                    &url
                );

                match client
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", api_key))
                    .body(body)
                    .send()
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            println!(" 📩 Successfully sent file: {:?}", path);
                        } else {
                            eprintln!(
                                " 💥 Failed to send file: {:?} with status: {}",
                                path,
                                response.status()
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Error sending file {:?}: {}", path, e);
                    }
                };
            }
            // sleep for 1 second
            std::thread::sleep(Duration::from_secs(5));
        }

        let duration = file_start_time.elapsed();
        total_duration += duration;
        println!(
            "Progress: ::{}::/{} - Time spent on this file: ##{:.2?}##",
            index + 1,
            total_files,
            duration
        );
    }

    let total_time_elapsed = total_time.elapsed();
    println!(
        "\n\nTotal time for all files: {:.2?}, Average time per file: {:.2?}",
        total_time_elapsed,
        total_duration / total_files as u32
    );
    Ok(())
}
