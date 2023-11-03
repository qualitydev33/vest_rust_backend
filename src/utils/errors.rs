use std::{
	sync::Arc,
	fmt::{Display, Formatter}
};
use axum::{
	Json,
	http::StatusCode,
	response::{IntoResponse, Response}
};
use serde_json::json;
use tracing::{error, debug};

#[derive(Clone, Debug)]
pub enum AppError {
	// #[cfg(feature = "kafka")]
	// SerializationError(schema_registry_converter::error::SRCError),
	// #[cfg(feature = "kafka")]
	KafkaError(rdkafka::error::KafkaError),
	// #[cfg(feature = "postgres")]
	// PostgresError(sea_orm::DbErr),
	Unhandled(Arc<anyhow::Error>)
}

pub fn match_error(error: &AppError) -> (&str, String, StatusCode) {
    match error {
        // #[cfg(feature = "kafka")]
        AppError::KafkaError(e) => (
            "Internal Server Error",
            format!("{:?}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
		// #[cfg(feature = "postgres")]
        // AppError::PostgresError(e) => handle_sea_orm_db_error(e),
        // #[cfg(feature = "kafka")]
        // AppError::SerializationError(e) => (
        //     "Internal Server Error",
        //     format!("{:?}", e),
        //     StatusCode::INTERNAL_SERVER_ERROR,
        // ),
        AppError::Unhandled(e) => (
            "Internal Server Error",
            format!("{:?}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}

impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		let (error_message, log_message, status_code) = match_error(&self);

        // Log message depending on status code
        if status_code == StatusCode::INTERNAL_SERVER_ERROR {
            // alternatively log "self" instead of the "log_message"
            error!("{:?}", log_message);
        } else if status_code == StatusCode::NOT_FOUND {
            debug!("{:?}", log_message);
        }

        // Build response body with error message
        let body = Json(json!({ "error": error_message }));

        (status_code, body).into_response()
	}
}


impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError::Unhandled(Arc::new(e))
    }
}

impl From<rdkafka::error::KafkaError> for AppError {
    fn from(e: rdkafka::error::KafkaError) -> Self {
        AppError::KafkaError(e)
    }
}


// impl From<schema_registry_converter::error::SRCError> for AppError {
//     fn from(e: schema_registry_converter::error::SRCError) -> Self {
//         AppError::SerializationError(e)
//     }
// }

// impl From<sea_orm::DbErr> for AppError {
//     fn from(e: sea_orm::DbErr) -> Self {
//         AppError::PostgresError(e)
//     }
// }

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (error_message, log_message, status_code) = match_error(self);
        // This is a workaround to log error details from graphql requests
        if status_code == StatusCode::INTERNAL_SERVER_ERROR {
            error!("{:?}", log_message);
        }
        write!(f, "{}", error_message)
    }
}

fn handle_sea_orm_db_error(e: &sea_orm::DbErr) -> (&str, String, StatusCode) {
    match e {
        sea_orm::DbErr::Conn(err) => (
            "Internal Server Error",
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        sea_orm::DbErr::Exec(err) => (
            "Internal Server Error",
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        sea_orm::DbErr::Query(err) => (
            "Internal Server Error",
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        sea_orm::DbErr::RecordNotFound(err) => ("Not found", "Not found".to_string(), StatusCode::NOT_FOUND),
        sea_orm::DbErr::Custom(err) => (
            "Internal Server Error",
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        sea_orm::DbErr::Type(err) => (
            "Internal Server Error",
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        sea_orm::DbErr::Json(err) => (
            "Internal Server Error",
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        sea_orm::DbErr::Migration(err) => (
            "Internal Server Error",
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        _ => (
            "Internal Server Error",
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}
