use reqwest;
use scraper::{Html, Selector};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn crawl_urls(urls: &[String], results: Arc<Mutex<Vec<(String, String)>>>) {
    let mut handles = vec![];

    for url in urls {
        let url = url.clone();
        let results = results.clone();

        let handle = tokio::spawn(async move {
            if let Ok(title) = fetch_page_title(&url).await {
                let mut results = results.lock().await;
                results.push((url.clone(), title));
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn fetch_page_title(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);

    // Use an HTML selector to find the title
    let selector = Selector::parse("title").unwrap();
    if let Some(element) = document.select(&selector).next() {
        Ok(element.text().collect::<Vec<_>>().join(""))
    } else {
        Ok("No Title Found".to_string())
    }
}
