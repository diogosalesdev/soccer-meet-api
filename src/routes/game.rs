use actix_web::{web, Responder, HttpResponse, get};
use serde_json::json;
use sqlx::{query_as};

use crate::AppState;

#[get("/games")]
pub async fn get_games(data: web::Data<AppState>) -> impl Responder {
    let query_result = query_as!(
        GameModel,
        "SELECT * FROM games"
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Something bad heppened while fetching the games!";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}))
    }

    let fields = query_result.unwrap();

    let json_response = json!({
        "status": "success",
        "results": games.len(),
        "games": games
    });
    HttpResponse::Ok().json(json_response)
}