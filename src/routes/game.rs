use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::query_as;

use crate::{models::game::CreateGameSchema, AppState};

#[get("/games")]
pub async fn get_games(data: web::Data<AppState>) -> impl Responder {
    let query_result = query_as!(GameModel, "SELECT * FROM games")
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let message = "Something bad heppened while fetching the games!";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}));
    }

    let fields = query_result.unwrap();

    let json_response = json!({
        "status": "success",
        "results": games.len(),
        "games": games
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/games/game")]
async fn create_game(
    body: web::Json<CreateGameSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = query_as!(
        GameModel,
        "INSERT into games (field_name, address, date) values ($1, $2, $3) returning *",
        body.field_name.to_string(),
        body.address.to_string(),
        body.day.to_string(),
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(game) => {
            let game_response = serde_json::json!({"status": "success", "data": serde_json::json!(
                "game": game
            )});
            return HttpResponse::Ok().json(game_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates uniuqe constrait!")
            {
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": "fail", "message": "Duplicate Key"}));
            }
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", e)}));
        }
    }
}

#[get("games/game/{id}")]
async fn get_game_id(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let game_id = path.into_inner();
    let query_result = query_as!(GameModel, "SELECT * FROM games WHERE id = $1", game_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(game) => {
            let game_response = serde_json::json!({"status": "success", "data": serde_json::json!(
                "game": game
            )});
            return HttpResponse::Ok().json(game_response);
        }
        Err(_) => {
            let message = format!("Note with ID: {} not found!", game_id);
            return HttpResponse::NotFound().json(json!({"status": "fail", "message": message}));
        }
    }
}
