use gamble_core::games::roullete::{Bet, bet};

use axum::{Json, Router, routing::post};

pub fn router() -> Router {
    Router::new()
    .route("/bet", post(path_bet))   
}

async fn path_bet(Json((bet_value, amount)): Json<(Bet, u64)>) -> Json<u64> {
    let result = bet(amount, bet_value);

    Json(result)
}