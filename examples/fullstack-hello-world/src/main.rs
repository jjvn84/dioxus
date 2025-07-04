//! Run with:
//!
//! ```sh
//! dx serve --platform web
//! ```

#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

fn app() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| "...".to_string());
    let server_future = use_server_future(get_server_data)?;

    rsx! {
        document::Link { href: asset!("/assets/hello.css"), rel: "stylesheet" }
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
        button {
            onclick: move |_| async move {
                let data = get_server_data().await?;
                println!("Client received: {}", data);
                text.set(data.clone());
                post_server_data(data).await?;
                Ok(())
            },
            "Run a server function!"
        }
        "Server said: {text}"
    }
}

#[server]
async fn post_server_data(data: String) -> ServerFnResult {
    println!("Server received: {}", data);

    Ok(())
}

#[server]
async fn get_server_data() -> ServerFnResult<String> {
    Ok(reqwest::get("https://httpbin.org/ip").await?.text().await?)
}

fn main() {
    dioxus::launch(app);
}
