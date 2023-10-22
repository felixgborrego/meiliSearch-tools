use clap::{Parser, Subcommand};

mod import_task;
mod index_stats;
mod meilisearch_queue_stats;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Import a folder of JSON files into an HTTP API
    Import {
        // The folder containing the JSON files
        #[arg(long)]
        folder: String,

        // The name of the index to import into
        #[arg(long)]
        index_name: String,
        // The URL of the HTTP API
        #[arg(long)]
        host_api: String,
        // The API key to use
        #[arg(long)]
        api_key: String,
    },

    /// Print stats from a Meilisearch API
    IndexStats {
        // The URL of the HTTP API
        #[arg(long)]
        host_api: String,
        // The API key to use
        #[arg(long)]
        api_key: String,
    },

    TaskStats {
        // The URL of the HTTP API
        #[arg(long)]
        host_api: String,
        // The API key to use
        #[arg(long)]
        api_key: String,
    },
}

fn main() {
    println!("Meilisearch tools!");
    let matches = Cli::parse();

    match matches.command {
        Some(Commands::Import {
            folder,
            host_api,
            index_name,
            api_key,
        }) => {
            println!(
                "Importing files from {} to {} in index {}",
                folder, host_api, index_name
            );
            let result = import_task::import_files(&folder, &host_api, &index_name, &api_key);

            match result {
                Ok(_) => println!("Import successful"),

                Err(e) => println!("Import failed: {}", e),
            }
        }
        Some(Commands::IndexStats { host_api, api_key }) => {
            println!("Printing stats from {}", host_api);
            index_stats::print_stats(&host_api, &api_key)
        }
        Some(Commands::TaskStats { host_api, api_key }) => {
            println!("Printing stats from {}", host_api);
            meilisearch_queue_stats::print_queue_stats(&host_api, &api_key)
        }
        None => println!("No command specified"),
    }
}
