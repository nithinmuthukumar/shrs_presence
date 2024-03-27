use std::sync::{Arc, Mutex};

use axum::{extract::State, http::StatusCode, routing::get, Router};
use state::PresenceState;

mod state;

type PState = Arc<Mutex<PresenceState>>;
#[tokio::main]
async fn main() {
    let mut state = PresenceState::new();
    state.connect();
    state.update_activity();
    let app = Router::new()
        // .route("/connect")
        // .route("/disconnect")
        .route("/session/add", get(add_session))
        .route("/session/remove", get(remove_session))
        .route("/commands/add", get(add_command))
        .with_state(Arc::new(Mutex::new(state)));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn add_session(State(mut s): State<PState>) -> StatusCode {
    let mut state = s.lock().unwrap();
    state.sessions += 1;
    state.update_activity();

    StatusCode::OK
}

async fn remove_session(State(mut s): State<PState>) -> StatusCode {
    let mut state = s.lock().unwrap();
    state.sessions -= 1;
    state.update_activity();

    StatusCode::OK
}

async fn add_command(State(mut s): State<PState>) -> StatusCode {
    let mut state = s.lock().unwrap();
    state.commands_used += 1;
    state.update_activity();

    StatusCode::OK
}
