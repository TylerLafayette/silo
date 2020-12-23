use silo_core::models;
use silo_db;

use futures;
use rocket;
use rocket::*;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// A service for running a REST API.
pub struct RestService {
    db_service: Box<dyn silo_db::service::Service>,
}

impl RestService {
    pub fn new(db_service: Box<dyn silo_db::service::Service>) -> Self {
        Self { db_service }
    }
}

pub fn build_and_serve_http(service: RestService) -> Result<(), Box<dyn std::error::Error>> {
    rocket::ignite()
        .manage(service)
        .mount("/api/v1", routes![health_check, groups_post])
        .launch();

    Ok(())
}

/// Response for /health
#[derive(Serialize)]
struct HealthCheckResponse {
    pub healthy: bool,
    pub message: String,
}

/// Checks the health of the API.
#[get("/health")]
fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        healthy: true,
        message: "Silo is up and running :)".into(),
    })
}

/// Response for POST /groups
#[derive(Serialize)]
struct GroupsPostResponse {
    pub success: bool,
    pub id: i16,
}

#[post("/groups")]
fn groups_post(s: State<RestService>) -> Json<GroupsPostResponse> {
    match futures::executor::block_on(s.db_service.insert_group(&models::Group { id: 0 })) {
        Ok(id) => Json(GroupsPostResponse {
            id: id,
            success: true,
        }),
        Err(_) => Json(GroupsPostResponse {
            id: -1,
            success: false,
        }),
    }
}
