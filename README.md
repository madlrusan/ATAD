
# Web Crawler with Simple UI

-- Rusan Ioana Madalina
## Overview

This project is a **Rust-based web crawler** with a **simple web interface** for interacting with the crawler. The user can input a list of URLs, start the crawl process, and view the crawl results through a basic UI built using **Rocket** (Rust web framework) and **HTML**.

The crawler fetches web pages, extracts the page titles, and displays them in the UI. The application demonstrates the use of asynchronous programming in Rust, web scraping using the **scraper** crate, and serving static files via **Rocket**.

## Features

- **URL Input**: The user can input a list of URLs (comma-separated).
- **Crawl Process**: The user starts the crawl process, which asynchronously fetches and parses the pages.
- **Results Display**: After the crawl is complete, the results (URL and title) are displayed on the web interface.

## Prerequisites

Before running the project, ensure you have the following installed:

- **Rust**: Follow the installation instructions at [Rust official website](https://www.rust-lang.org/learn/get-started).
- **Cargo**: This comes with Rust and is used for building and running the project.

## Project Structure

The project consists of two primary parts:

- **Backend (Rust)**: Handles the web crawling and serves the UI.
- **Frontend (HTML, CSS, JavaScript)**: Provides a simple UI to interact with the backend.

### Project Files

- **`Cargo.toml`**: Contains the project dependencies.
- **`src/main.rs`**: Backend code, including web routes and the crawl logic.
- **`src/crawler.rs`**: Contains the crawling logic for fetching page titles.
- **`static/index.html`**: The HTML UI for the web interface.
- **`static/style.css`** (optional): For styling the UI.
- **`static/script.js`** (optional): JavaScript for handling UI interactions (e.g., sending POST requests to start the crawl).

## Setup and Installation

1. **Clone the repository**:

   Clone the repository to your local machine or create a new project and copy the relevant code into your `src` and `static` directories.

2. **Install Dependencies**:

   Open a terminal and navigate to the project directory. Run the following command to install the necessary dependencies:

   ```sh
   cargo build
   ```

3. **Run the Application**:

   Once the dependencies are installed, you can run the application with:

   ```sh
   cargo run
   ```

   The Rocket server will start, and you should see output like:

   ```
   Rocket has launched from http://localhost:8000
   ```

   The application will now be available in your browser at [http://localhost:8000](http://localhost:8000).

## Backend: Rust with Rocket

The backend uses the **Rocket** framework to handle HTTP requests and serve static files. It also manages the web crawling process, including URL parsing and title extraction.

### Key Routes

1. **`GET /`**: Serves the `index.html` page from the `static` folder.
2. **`POST /start_crawl`**: Accepts a comma-separated list of URLs, starts the crawl process, and returns a confirmation message.
3. **`GET /results`**: Returns the crawl results in JSON format, including the URLs and their respective titles.

### Crawling Logic

The crawling logic is implemented in **`src/crawler.rs`**. It uses the **`reqwest`** crate to fetch web pages and the **`scraper`** crate to extract the title from the HTML content.

```rust
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
```

This function starts an asynchronous task for each URL, fetching the page and extracting the title.

## Frontend: HTML, CSS, JavaScript

The UI is built using a basic **HTML** page that allows the user to input URLs and start the crawling process. It also displays the crawl status and the results.

### `index.html`

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Simple Web Crawler</title>
    <style>
        body { font-family: Arial, sans-serif; padding: 20px; }
        input, button { margin-bottom: 10px; padding: 8px; }
        #results { margin-top: 20px; }
    </style>
</head>
<body>
    <h1>Simple Web Crawler</h1>
    <label for="urls">Enter URLs (comma-separated):</label><br>
    <input type="text" id="urls" placeholder="http://example.com, http://another-site.com"><br>
    <button onclick="startCrawl()">Start Crawl</button>

    <div id="status"></div>
    <div id="results"></div>

    <script>
        async function startCrawl() {
            const urls = document.getElementById('urls').value;

            if (!urls) {
                alert("Please enter some URLs!");
                return;
            }

            // Show loading status
            document.getElementById('status').innerText = "Crawl in progress...";

            // Send the URLs to the backend
            const response = await fetch('/start_crawl', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ urls: urls })
            });

            const data = await response.text();
            document.getElementById('status').innerText = data;

            // Fetch results after crawl is done
            setTimeout(fetchResults, 1000); // Wait for the crawl to complete
        }

        async function fetchResults() {
            const response = await fetch('/results');
            const data = await response.json();

            // Display results
            const resultsDiv = document.getElementById('results');
            if (data.results && data.results.length > 0) {
                resultsDiv.innerHTML = "<ul>" + data.results.map(result => `<li>${result.url}: ${result.title}</li>`).join('') + "</ul>";
            } else {
                resultsDiv.innerHTML = "No results yet.";
            }
        }
    </script>
</body>
</html>
```

The `startCrawl()` function sends the list of URLs to the backend, and `fetchResults()` retrieves the crawl results.

## Improvements

### Real-Time Updates

You can further improve the UI to show real-time progress using **WebSockets** or **Server-Sent Events** for continuous updates during the crawl process.

### Error Handling

Currently, there’s minimal error handling. Enhancing error management (e.g., network errors, invalid URLs) would make the application more robust.

### Performance

For a more scalable application, consider implementing **rate limiting** for crawling and **parallelism** optimization for handling a large number of URLs concurrently.

---

## Conclusion

This project provides a solid foundation for building a simple web crawler with a basic UI. It demonstrates key Rust concepts such as asynchronous programming, web scraping, and web development using the Rocket framework. The project can be extended with additional features like live progress updates, error handling, and visualizations.