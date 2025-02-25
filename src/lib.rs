use std::collections::HashMap;

use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};

pub async fn get_show_id(show_name: &str) -> Result<String> {
    let client = Client::new();
    let url = format!(
        "https://www.imdb.com/find/?q={}&s=tt&ttype=tv",
        show_name.replace(" ", "+")
    );

    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&response);
    let selector = Selector::parse("a[href*='/title/tt']").unwrap();
    let mut results: HashMap<i32, String> = HashMap::new();

    for (count, element) in document.select(&selector).take(10).enumerate() {
        let href = element.value().attr("href").unwrap_or("");
        let title = element.text().collect::<String>();

        let id = href
            .split('/')
            .find(|s| s.starts_with("tt"))
            .unwrap_or("")
            .to_string();

        results.insert((count + 1) as i32, id.clone());

        println!("{}: {}", count + 1, title);
    }

    println!("\nWhich result?");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let key = input.trim().parse::<i32>()?;

    if let Some(id) = results.get(&key) {
        return Ok(id.clone());
    }

    Err(anyhow::anyhow!("IMDb: No results found"))
}
