use warp::Filter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::sleep;

// this is one of my first projects in rust and im not too experienced I was learning on the fly!
// yeah this is a quick webhook forwarder, might fix stuff later
#[derive(Deserialize, Serialize, Debug, Clone)]
struct WebhookMessage {
    content: String,
}

#[derive(Clone)]
struct AppState {
    target_hook: String, // called it hook instead of webhook, whatever
    secret_key: String,
    rate_limit: usize,
    rate_window_secs: u64,
    clients: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
}

impl AppState {
    fn new(target: &str, key: &str, limit: usize, window: u64) -> Self {
        AppState {
            target_hook: target.to_string(),
            secret_key: key.to_string(),
            rate_limit: limit,
            rate_window_secs: window,
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn is_allowed(&self, ip: &str) -> bool {
        let mut clients = self.clients.lock().unwrap();
        let now = Instant::now();
        let timestamps = clients.entry(ip.to_string()).or_insert(Vec::new());

        // filter out old stuff
        *timestamps = timestamps.iter()
            .cloned()
            .filter(|t| now.duration_since(*t).as_secs() < self.rate_window_secs)
            .collect();

        if timestamps.len() >= self.rate_limit {
            println!("{} is rate limited", ip); 
            false
        } else {
            timestamps.push(now);
            true
        }
    }
}

#[tokio::main]
async fn main() {
    let state = AppState::new(
        "https://discord.com/api/webhooks/XXXXX/YYYYY",
        "supersecretkey123",
        5,
        60,
    );

    let forwarder = warp::post()
        .and(warp::path("forward"))
        .and(warp::header::optional::<String>("x-admin"))
        .and(warp::addr::remote())
        .and(warp::body::json())
        .map(move |key: Option<String>, addr: Option<std::net::SocketAddr>, msg: WebhookMessage| {
            let client_ip = addr.map(|a| a.ip().to_string()).unwrap_or("???".into());

            if key.as_deref() != Some(&state.secret_key) {
                println!("Unauthorized attempt from {}", client_ip);
                return warp::reply::with_status("Nope", warp::http::StatusCode::UNAUTHORIZED);
            }

            if !state.is_allowed(&client_ip) {
                return warp::reply::with_status("Too fast!", warp::http::StatusCode::TOO_MANY_REQUESTS);
            }

            let hook = state.target_hook.clone();
            let msg_copy = msg.clone();

            tokio::spawn(async move {
                match reqwest::Client::new().post(&hook).json(&msg_copy).send().await {
                    Ok(resp) => println!("Sent to webhook, status: {}", resp.status()),
                    Err(e) => eprintln!("Webhook send error: {:?}", e),
                }
            });

            warp::reply::json(&msg)
        });

    println!("ok running on http://127.0.0.1:3030 ... hopefully works");
    warp::serve(forwarder).run(([127, 0, 0, 1], 3030)).await;
}
