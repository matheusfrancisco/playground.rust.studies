use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use axum_extra::{
    headers::{authorization::{Basic, Bearer}, Authorization}, TypedHeader
};
use jsonwebtoken::{DecodingKey, Validation};
use serde_json::json;

use crate::{
    handler::GreetingHandler,
    model::{Greeting, OurJwtPayload},
};

type AppState<G> = Arc<G>;
const SECRET_SIGNING_KEY: &[u8] = b"keep_th1s_@_secret";

pub struct AxumWebServer<G: GreetingHandler> {
    app_state: AppState<G>,
}

impl<G: GreetingHandler> AxumWebServer<G> {
    pub fn new(handler: G) -> Self {
        // Create a shared state for our application. We use an Arc so that we clone the pointer to the state and
        // not the state itself.
        let app_state: AppState<G> = Arc::new(handler);
        AxumWebServer { app_state }
    }

    pub async fn start(&self) {
        let app_state = self.app_state.clone();

        // set up our application with "hello world" route at "/
        let app = Router::new()
            .route("/hello/{visitor}", get(Self::greet_visitor))
            .route("/bye", delete(Self::say_goodbye))
            .route("/login", post(Self::login))
            .route("/reset-visits", delete(Self::reset))
            .with_state(app_state);

        // start the server on port 3000
        println!("Starting server on http://0.0.0:3000");
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    /// Extract the `visitor` path parameter and use it to greet the visitor.
    /// We also use the `State` extractor to access the shared `Handler` and call the `greet` method.
    /// We use `Json` to automatically serialize the `Greeting` struct to JSON.
    async fn greet_visitor(
        State(handler): State<AppState<G>>,
        Path(visitor): Path<String>,
    ) -> Json<Greeting> {
        Json(handler.greet(visitor))
    }

    /// Say goodbye to the visitor.
    async fn say_goodbye(State(handler): State<AppState<G>>) -> String {
        handler.bye()
    }

    async fn reset(
        TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
        State(handler): State<AppState<G>>,
    ) -> impl IntoResponse {
        let token = bearer.token();
        let decoding_key = DecodingKey::from_secret(SECRET_SIGNING_KEY);

        let Ok(jwt) =
            jsonwebtoken::decode::<OurJwtPayload>(token, &decoding_key, &Validation::default())
        else {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid token"})),
            );
        };

        let username = jwt.claims.sub;
        handler.reset();

        (
            StatusCode::OK,
            Json(json!({"ok": format_args!("Visits reset by {username}")})),
        )
    }

    async fn login(
        TypedHeader(Authorization(creds)): TypedHeader<Authorization<Basic>>,
    ) -> impl IntoResponse {
        if creds.username() != "admin" || creds.password() != "password" {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized"})),
            );
        };

        let Ok(jwt) = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &OurJwtPayload::new(creds.username().to_string()),
            &jsonwebtoken::EncodingKey::from_secret(SECRET_SIGNING_KEY),
        ) else {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to generate token"})),
            );
        };

        (StatusCode::OK, Json(json!({"jwt": jwt})))
    }
}
