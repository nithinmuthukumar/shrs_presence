use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use state::{PresenceInfo, PresenceState};
use std::{
    process::exit,
    sync::{Arc, Mutex},
    time::Duration,
};
use uuid::Uuid;

mod state;

type PState = Arc<Mutex<PresenceState>>;
#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(PresenceState::new()));
    tokio::spawn(heartbeat(state.clone()));

    let app = Router::new()
        .route("/connect", post(connect))
        .route("/disconnect", post(disconnect))
        .route("/command/add", post(add_command))
        .route("/info", get(info))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize, Clone)]
pub struct Session {
    id: String,
}
async fn connect(State(s): State<PState>, Json(body): Json<Session>) -> StatusCode {
    dbg!("Connect");
    dbg!(body.clone());
    let mut state = s.lock().unwrap();
    match Uuid::parse_str(body.id.as_str()) {
        Ok(id) => {
            state.connect(id);
            state.update_activity();
            StatusCode::OK
        }
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

async fn disconnect(State(s): State<PState>) -> StatusCode {
    let mut state = s.lock().unwrap();
    state.disconnect();

    StatusCode::OK
}
async fn info(State(s): State<PState>) -> Json<PresenceInfo> {
    let state = s.lock().unwrap();
    Json(state.info())
}

async fn add_command(State(s): State<PState>, body: String) -> StatusCode {
    let mut state = s.lock().unwrap();
    state.commands_used += 1;
    state.last_command = body;
    state.update_activity();

    StatusCode::OK
}
async fn heartbeat(s: PState) {
    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;

        let mut state = s.lock().unwrap();
        state.drop_dead_sessions();
        if state.sessions.len() == 0 {
            exit(0)
        }

        state.update_activity();
    }
}
