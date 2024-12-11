#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use rocket::tokio::sync::Mutex;
use std::sync::Arc;
use std::path::{Path};

mod crawler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, start_crawl, results])
        .mount("/static", rocket::fs::FileServer::from("static"))
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[post("/start_crawl", data = "<urls>")]
async fn start_crawl(urls: String) -> String {
    let urls: Vec<String> = urls.split(',').map(|s| s.to_string()).collect();
    let results = Arc::new(Mutex::new(vec![]));

    // Start the crawl asynchronously
    tokio::spawn(async move {
        crawler::crawl_urls(&urls, results.clone()).await;
    });

    "Crawl Started!".to_string()
}

#[get("/results")]
async fn results() -> String {
    // Return the crawl results as a JSON string
    let results = vec![
        // Sample data, replace with real results
        "{ \"url\": \"http://example.com\", \"title\": \"Example Domain\" }",
        "{ \"url\": \"http://another-site.com\", \"title\": \"Another Site\" }"
    ];

    serde_json::to_string(&results).unwrap()
}
