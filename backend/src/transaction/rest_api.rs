use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder, http::header::ContentType};
use serde::Deserialize;
use chrono::prelude::*;
use super::super::is_authorized;
use super::super::webserver::AppState;

#[get("/api/v1/transactions/all")]
async fn get_all(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::get_all(&data.pool).await {
		Ok(res) => return HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[derive(Deserialize)]
struct TransactionPost {
	account_id: u32,
	recipient_id: u32,
	status: u8,
	timestamp: DateTime<Utc>,
	amount: i32,
	comment: Option<String>,
	tag_ids: Option<Vec<u32>>,
}

#[post("/api/v1/transactions")]
async fn post(data: web::Data<AppState>, req: HttpRequest, body: web::Json<TransactionPost>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}", e))
	};

	let transaction = super::Transaction {
		id: None,
		currency_id: None,
		user_id: user_id,
		account_id: body.account_id,
		recipient_id: body.recipient_id,
		status: match body.status {
			0 => super::TransactionStatus::Withheld,
			1 => super::TransactionStatus::Completed,
			_ => return HttpResponse::BadRequest().content_type(ContentType::json()).body("{{\"error\":\"Invalid status\"}}"),
		},
		timestamp: body.timestamp,
		amount: body.amount,
		comment: body.comment.clone(),
		tag_ids: body.tag_ids.clone(),
	};

	match super::add(&data.pool, &transaction).await {
		Ok(_) => return HttpResponse::Ok().content_type(ContentType::json()).body(""),
		Err(e) => return HttpResponse::BadRequest().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[put("/api/v1/transactions/{transaction_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<TransactionPost>, transaction_id: web::Path<u32>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}",e))
	};

	let transaction = super::Transaction {
		id: Some(transaction_id.into_inner()),
		currency_id: None,
		user_id: user_id,
		account_id: body.account_id,
		recipient_id: body.recipient_id,
		status: match body.status {
			0 => super::TransactionStatus::Withheld,
			1 => super::TransactionStatus::Completed,
			_ => return HttpResponse::BadRequest().content_type(ContentType::json()).body("{{\"error\":\"Invalid status\"}}"),
		},
		timestamp: body.timestamp,
		amount: body.amount,
		comment: body.comment.clone(),
		tag_ids: body.tag_ids.clone(),
	};

	match super::update(&data.pool, &transaction).await {
		Ok(_) => return HttpResponse::Ok().content_type(ContentType::json()).body(""),
		Err(e) => return HttpResponse::BadRequest().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}",e)),
	}
}

#[delete("/api/v1/transactions/{transaction_id}")]
async fn delete(data: web::Data<AppState>, req: HttpRequest, transaction_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}",e))
	};

	return match super::delete_by_id(&data.pool, transaction_id.into_inner()).await {
		Ok(_) => HttpResponse::Ok().content_type(ContentType::json()).body(""),
		Err(e) => HttpResponse::BadRequest().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}",e)),
	};
}