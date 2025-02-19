use reqwest;

pub async fn fetch_http(url: &str) {
    match reqwest::get(url).await {
        Ok(resp) => {
            println!("Status: {}", resp.status());
            if let Ok(body) = resp.text().await {
                println!("Response:\n{}", &body[..100.min(body.len())]); // Print first 100 chars
            }
        }
        Err(err) => println!("Request failed: {}", err),
    }
}
