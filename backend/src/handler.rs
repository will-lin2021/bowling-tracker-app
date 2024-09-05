use crate::{
    model::{DateModel, GameModel},
    schema::{CreateGameSchema, FilterOptions, UpdateGameSchema},
    AppState,
};
use common::{GameInfo, Game, DateRecord, GameResponse, RecordResponse, ErrorResponse};

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Built API with Rust, PostgreSQL, and Actix Web";

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": MESSAGE
    }))
}

// GET
#[get("/games")]
async fn get_record_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Get query options
    let limit = opts.limit.unwrap_or(12);
    let offset = (opts.page.unwrap_or(1) -1) * limit;

    // Query dates within query filter options
    let dates_query = sqlx::query_as!(
        DateModel,
        "SELECT * FROM dates
            ORDER by date DESC
            LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32,
    )
    .fetch_all(&data.db)
    .await;

    if dates_query.is_err() {
        let error_response = ErrorResponse {
            status: "error".to_string(),
            message: "Error occured while querying dates".to_string()

        };

        return HttpResponse::InternalServerError()
        .json(json!(error_response));
    }
    let date_models = dates_query.unwrap();

    // Query games on each date
    let date_ids: Vec<uuid::Uuid> = date_models.iter().map(|model| model.date_id).collect();
    let mut games_per_date: Vec<Vec<GameInfo>> = Vec::new();
    for date_id in date_ids {
        let games_query = sqlx::query_as!(
            GameModel,
            "SELECT * FROM games
                WHERE date_id=$1
                ORDER BY game_no ASC",
            date_id,
        )
        .fetch_all(&data.db)
        .await;

        if games_query.is_err() {
            let error_response = ErrorResponse {
                status: "error".to_string(),
                message: "Error occured while querying games on given date".to_string(),
            };

            return HttpResponse::InternalServerError()
            .json(json!(error_response));
        }
        let game_infos: Vec<GameInfo> = games_query.unwrap().iter().map(|game_model| {
            GameInfo {
                game_id: game_model.game_id,
                game_no: game_model.game_no,
                score_str: game_model.score_str.clone(),
            }
        }).collect();

        games_per_date.push(game_infos);
    }

    // Create DateRecord list
    let dates_records: Vec<DateRecord> = date_models.iter().zip(games_per_date).map(|(date_model, game_infos)| {
        DateRecord {
            date_id: date_model.date_id,
            date: date_model.date,
            games: game_infos,
        }
    }).collect();

    // Create RecordResponse
    let date_list_response = RecordResponse {
        status: "success".to_string(),
        results: date_models.len() as i32,
        dates: dates_records,
    };

    // Return response
    HttpResponse::Ok()
    .json(json!(date_list_response))
}

#[get("/games/{game_id}")]
async fn get_game_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Get game_id
    let game_id = path.into_inner();

    // Query for game with given game_id
    let game_query = sqlx::query_as!(
        GameModel,
        "SELECT * FROM games
            WHERE game_id=$1",
        game_id,
    )
    .fetch_one(&data.db)
    .await;

    if game_query.is_err() {
        let error_response = ErrorResponse {
            status: "error".to_string(),
            message: "Error occured while querying for game with given game_id".to_string(),
        };

        return HttpResponse::InternalServerError()
        .json(json!(error_response));
    }
    let game_model = game_query.unwrap();

    // Query for date from game
    let date_query = sqlx::query_as!(
        DateModel,
        "SELECT * FROM dates
            WHERE date_id=$1",
        game_model.date_id,
    )
    .fetch_one(&data.db)
    .await;

    if date_query.is_err() {
        let error_response = ErrorResponse {
            status: "error".to_string(),
            message: "Error occured while querying for date with given date_id".to_string(),
        };

        return HttpResponse::InternalServerError()
        .json(json!(error_response));
    }
    let date_model = date_query.unwrap();

    // Create GameResponse
    let game_response = GameResponse {
        status: "success".to_string(),
        data: Game {
            date_id: date_model.date_id,
            date: date_model.date,
            game_info: GameInfo {
                game_id: game_model.game_id,
                game_no: game_model.game_no,
                score_str: game_model.score_str
            }
        }
    };

    // Return response
    HttpResponse::Ok()
    .json(json!(game_response))
}

// POST
#[post("/games/")]
async fn create_game_handler(
    body: web::Json<CreateGameSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Query for whether the date exists
    let existing_date_query = sqlx::query_as!(
        DateModel,
        "SELECT * FROM dates
            WHERE date=$1",
        body.date,
    )
    .fetch_one(&data.db)
    .await;

    let date_model = match existing_date_query {
        Ok(model) => model,
        Err(_) => {
            let new_date_query = sqlx::query_as!(
                DateModel,
                "INSERT INTO dates
                    (date) VALUES ($1)
                    RETURNING *",
                body.date,
            )
            .fetch_one(&data.db)
            .await;

            if new_date_query.is_err() {
                let error_response = ErrorResponse {
                    status: "error".to_string(),
                    message: "Error occured while attempting to add new date to database".to_string(),
                };

                return HttpResponse::InternalServerError()
                .json(json!(error_response));
            }
            new_date_query.unwrap()
        }
    };

    // Query number of games on date
    let num_games_query = sqlx::query!(
        "SELECT COUNT(*) FROM games
            WHERE date_id=$1",
        date_model.date_id,
    )
    .fetch_one(&data.db)
    .await;

    if num_games_query.is_err() {
        let error_response = ErrorResponse {
            status: "error".to_string(),
            message: "Error occured while querying number of games played".to_string(),
        };

        return HttpResponse::InternalServerError()
        .json(json!(error_response));
    }
    let num_games_opt = num_games_query.unwrap();

    if num_games_opt.count.is_none() {
        let error_response = ErrorResponse {
            status: "error".to_string(),
            message: "Error occured while fetching number of games played".to_string(),
        };

        return HttpResponse::InternalServerError()
        .json(json!(error_response));
    }
    let game_no = num_games_opt.count.unwrap() + 1;

    // Query insert into database
    let new_game_query = sqlx::query_as!(
        GameModel,
        "INSERT INTO games
            (date_id, game_no, score_str) VALUES ($1, $2, $3)
            RETURNING *",
        date_model.date_id,
        game_no as i32,
        body.score_str,
    )
    .fetch_one(&data.db)
    .await;

    if let Err(_) = new_game_query {
        let error_response = ErrorResponse {
            status: "error".to_string(),
            message: "Error occured while adding new game".to_string(),
        };

        return HttpResponse::InternalServerError()
        .json(json!(error_response));
    }
    let game_model = new_game_query.unwrap();

    // Create GameResponse
    let game_response = GameResponse {
        status: "success".to_string(),
        data: Game {
            date_id: date_model.date_id,
            date: date_model.date,
            game_info: GameInfo {
                game_id: game_model.game_id,
                game_no: game_model.game_no,
                score_str: game_model.score_str,
            }
        }
    };

    // Return response
    HttpResponse::Ok()
    .json(json!(game_response))
}

// PATCH
#[patch("/games/{game_id}")]
async fn edit_game_handler(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateGameSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Get game_id
    let game_id = path.into_inner();

    // Query for the game
    let game_exists_query = sqlx::query_as!(
        GameModel,
        "SELECT * FROM games
            WHERE game_id=$1",
        game_id,
    )
    .fetch_one(&data.db)
    .await;

    if game_exists_query.is_err() {
        let error_response = ErrorResponse {
            status: "fail".to_string(),
            message: format!("Game with ID: {} not found", game_id),
        };

        return HttpResponse::NotFound()
        .json(json!(error_response));
    }
    let mut game_model = game_exists_query.unwrap();

    // Query for the date
    let date_query = sqlx::query_as!(
        DateModel,
        "SELECT * FROM dates
            WHERE date_id=$1",
        game_model.date_id,
    )
    .fetch_one(&data.db)
    .await;

    if date_query.is_err() {
        let error_response = ErrorResponse {
            status: "error".to_string(),
            message: "Error occurred while querying for date".to_string(),
        };

        return HttpResponse::InternalServerError()
        .json(json!(error_response));
    }
    let mut date_model = date_query.unwrap();

    // Modifying date
    if body.date.is_some() {
        // Query for whether new date exists
        let existing_date_query = sqlx::query_as!(
            DateModel,
            "SELECT * FROM dates
                WHERE date=$1",
            body.date.unwrap(),
        )
        .fetch_one(&data.db)
        .await;

        if existing_date_query.is_err() {

        }
    }

    // Modifying score_str
    if body.score_str.is_some() {

    }

    // Create GameResponse
    let game_response = GameResponse {
        status: "success".to_string(),
        data: Game {
            date_id: date_model.date_id,
            date: date_model.date,
            game_info: GameInfo {
                game_id: game_model.game_id,
                game_no: game_model.game_no,
                score_str: game_model.score_str,
            }
        }
    };

    // Return response
    HttpResponse::Ok()
    .json(json!(game_response))
}

// DELETE
#[delete("/games/{game_id}")]
async fn delete_game_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Get game_id
    let game_id = path.into_inner();

    // Query for deletion of game
    let games_deleted_query = sqlx::query_as!(
        GameModel,
        "DELETE FROM games
            WHERE game_id=$1
            RETURNING *",
        game_id,
    )
    .fetch_optional(&data.db)
    .await;

    if games_deleted_query.is_err() {
        let error_response = ErrorResponse {
            status: "error".to_string(),
            message: "Error occured while querying for game deletion".to_string(),
        };

        return HttpResponse::InternalServerError()
        .json(json!(error_response));
    }

    let game_deleted_opt = games_deleted_query.unwrap();

    if game_deleted_opt.is_none() {
        let error_response = ErrorResponse {
            status: "fail".to_string(),
            message: format!("Game with ID: {} not found", game_id).to_string(),
        };

        return HttpResponse::NotFound()
        .json(json!(error_response));
    }

    let game_deleted = game_deleted_opt.unwrap();

    // Remove date if the last game was removed
    let games_on_date_query = sqlx::query!(
        "SELECT COUNT(*) FROM dates
            WHERE date_id=$1",
        game_deleted.date_id,
    )
    .fetch_one(&data.db)
    .await;

    if games_on_date_query.is_err() {
        let error_reponse = ErrorResponse {
            status: "error".to_string(),
            message: "Unable to get number of games played on date".to_string(),
        };

        return HttpResponse::InternalServerError()
        .json(json!(error_reponse));
    }

    let games_on_date_opt = games_on_date_query.unwrap().count;

    if games_on_date_opt.is_none() {
        let error_respose = ErrorResponse {
            status: "error".to_string(),
            message: "Error getting number of games on date".to_string(),
        };

        return HttpResponse::InternalServerError()
        .json(json!(error_respose));
    }

    let games_on_date = games_on_date_opt.unwrap();

    if games_on_date == 0 {
        let delete_date_query = sqlx::query!(
            "DELETE FROM dates
                WHERE date_id=$1",
            game_deleted.date_id,
        )
        .execute(&data.db)
        .await;

        if delete_date_query.is_err() {
            let error_response = ErrorResponse {
                status: "error".to_string(),
                message: "Error deleting date from database".to_string(),
            };

            return HttpResponse::InternalServerError()
            .json(json!(error_response));
        }
    }

    // Return NoContent
    HttpResponse::NoContent()
    .finish()
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
    .service(health_checker_handler)
    .service(get_record_handler)
    .service(get_game_handler)
    .service(create_game_handler)
    .service(edit_game_handler)
    .service(delete_game_handler);

    conf.service(scope);
}
