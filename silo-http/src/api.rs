use silo_core::models;
use silo_db;
use silo_runner::python::PythonExecutor;
use silo_transform::matrix::*;

use actix_cors::Cors;
use actix_rt;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio;
use tokio::runtime::Runtime;

/// A service for running a REST API.
pub struct RestService {
    db_service: Box<dyn silo_db::service::Service>,
}

impl RestService {
    pub fn new(db_service: Box<dyn silo_db::service::Service>) -> Self {
        Self { db_service }
    }
}

#[post("/groups")]
async fn groups_post(service: web::Data<Arc<RestService>>) -> impl Responder {
    match service
        .db_service
        .insert_group(&models::Group { id: 0 })
        .await
    {
        Ok(id) => HttpResponse::Ok().json(models::Group { id }),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::BadRequest().json(ApiError {
                error: "error.db.generic".into(),
                message: format!("{:?}", e),
            })
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupsResponse {
    pub groups: Vec<models::Group>,
}

#[get("/groups")]
async fn groups_get(service: web::Data<Arc<RestService>>) -> impl Responder {
    match service.db_service.get_groups().await {
        Ok(groups) => HttpResponse::Ok().json(GroupsResponse { groups }),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::BadRequest().json(ApiError {
                error: "error.db.generic".into(),
                message: format!("{:?}", e),
            })
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertSubject {
    pub age: i16,
    pub length_of_stay: i16,
}

#[post("/groups/{id}/subjects")]
async fn groups_subjects_post(
    service: web::Data<Arc<RestService>>,
    web::Path(id): web::Path<i32>,
    subject: web::Json<InsertSubject>,
) -> impl Responder {
    let s = models::Subject {
        id: 0,
        group_id: id,
        age: subject.age,
        length_of_stay: subject.length_of_stay,
    };

    match service.db_service.insert_subject(&s).await {
        Ok(id) => HttpResponse::Ok().json(models::Subject { id, ..s }),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::BadRequest().json(ApiError {
                error: "error.db.generic".into(),
                message: format!("{:?}", e),
            })
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectsResponse {
    pub subjects: Vec<models::Subject>,
}

#[get("/groups/{id}/subjects")]
async fn groups_subjects_get(
    service: web::Data<Arc<RestService>>,
    web::Path(id): web::Path<i32>,
) -> impl Responder {
    match service.db_service.find_subjects_by_group_id(id).await {
        Ok(subjects) => HttpResponse::Ok().json(SubjectsResponse { subjects }),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::BadRequest().json(models::Group { id: -1 })
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertTrait {
    pub parent_id: i32,
    pub trait_name: String,
}

#[derive(Debug, Serialize)]
pub struct TraitsResponse {
    pub traits: Vec<models::SubjectTrait>,
}

#[get("/traits")]
async fn traits_get(service: web::Data<Arc<RestService>>) -> impl Responder {
    match service.db_service.get_traits().await {
        Ok(traits) => HttpResponse::Ok().json(TraitsResponse { traits }),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::BadRequest().json(ApiError {
                error: "error.db.generic".into(),
                message: format!("{:?}", e),
            })
        }
    }
}

#[post("/traits")]
async fn traits_post(
    service: web::Data<Arc<RestService>>,
    _trait: web::Json<InsertTrait>,
) -> impl Responder {
    let tr = models::SubjectTrait {
        id: 0, // Auto-generate ID
        parent_id: _trait.parent_id,
        trait_name: _trait.trait_name.clone(),
    };

    match service.db_service.insert_subject_trait(&tr).await {
        Ok(id) => HttpResponse::Ok().json(models::SubjectTrait { id, ..tr }),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::BadRequest().json(ApiError {
                error: "error.db.generic".into(),
                message: format!("{:?}", e),
            })
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertSubjectSubjectTrait {
    pub trait_id: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiCreationSuccess {
    pub id: i32,
}

#[post("/groups/{group_id}/subjects/{subject_id}/traits")]
async fn groups_subjects_traits_post(
    service: web::Data<Arc<RestService>>,
    web::Path((_, subject_id)): web::Path<(i32, i32)>,
    subject_subject_trait: web::Json<InsertSubjectSubjectTrait>,
) -> impl Responder {
    match service
        .db_service
        .insert_subject_subject_trait(subject_id, subject_subject_trait.trait_id)
        .await
    {
        Ok(id) => HttpResponse::Ok().json(ApiCreationSuccess { id }),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::BadRequest().json(ApiError {
                error: "error.db.generic".into(),
                message: format!("{:?}", e),
            })
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectTraitsResponse {
    pub traits: Vec<models::SubjectTrait>,
}

#[get("/groups/{group_id}/subjects/{subject_id}/traits")]
async fn groups_subjects_traits_get(
    service: web::Data<Arc<RestService>>,
    web::Path((_, subject_id)): web::Path<(i32, i32)>,
) -> impl Responder {
    match service
        .db_service
        .find_subject_trats_by_subject_id(subject_id)
        .await
    {
        Ok(traits) => HttpResponse::Ok().json(SubjectTraitsResponse { traits }),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::BadRequest().json(ApiError {
                error: "error.db.generic".into(),
                message: format!("{:?}", e),
            })
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatrixGenQuery {
    pub attributes: String,
    pub traits: String,
    pub fields: bool,
}

#[get("/groups/{id}/generate/matrix")]
async fn groups_generate_matrix(
    service: web::Data<Arc<RestService>>,
    web::Path(id): web::Path<i32>,
    web::Query(query): web::Query<MatrixGenQuery>,
) -> impl Responder {
    let trait_names = query.traits.split(",");
    let attribute_names = query.attributes.split(",");

    // TODO: convert to a Stream
    let mut trait_ids = vec![];
    for name in trait_names.clone() {
        match service.db_service.find_subject_trait_by_name(name).await {
            Ok(Some(s)) => {
                trait_ids.push(s.id);
            }
            _ => (),
        }
    }

    let subjects = match service.db_service.find_subjects_by_group_id(id).await {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::BadRequest().json(ApiError {
                error: "error.db.generic".into(),
                message: format!("{:?}", e),
            });
        }
    };

    let mut matrix_rows = vec![];

    for subject in subjects {
        let mut int_fields: HashMap<String, i32> = HashMap::new();
        let mut binary_fields: HashMap<String, bool> = HashMap::new();

        if attribute_names.clone().into_iter().any(|i| i == "id") {
            int_fields.insert("id".into(), subject.id.into());
        }

        if attribute_names.clone().into_iter().any(|i| i == "age") {
            int_fields.insert("age".into(), subject.age.into());
        }

        if attribute_names
            .clone()
            .into_iter()
            .any(|i| i == "length_of_stay")
        {
            int_fields.insert("length_of_stay".into(), subject.length_of_stay.into());
        }

        let traits = service
            .db_service
            .find_subject_trats_by_subject_id(subject.id)
            .await
            .unwrap();

        let trait_iter = traits.iter();

        for name in trait_names.clone() {
            if trait_iter.clone().any(|x| x.trait_name == name) {
                binary_fields.insert(name.clone().into(), true);
            } else {
                binary_fields.insert(name.clone().into(), false);
            }
        }

        matrix_rows.push(MatrixTransformerRow::new(binary_fields, int_fields));
    }

    let mut transformer = MatrixTransformerBuilder::new()
        .output_as(MatrixOutputType::Tsv)
        .with_header(query.fields);

    for name in trait_names.clone() {
        transformer = transformer.with_binary_field(name);
    }

    for name in attribute_names.clone() {
        transformer = transformer.with_int_field(name);
    }

    let t = transformer.build();

    let matrix = t.generate(matrix_rows).unwrap();

    HttpResponse::Ok().body(matrix)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobsResponse {
    jobs: Vec<models::Job>,
}

#[get("/jobs")]
async fn jobs_get(service: web::Data<Arc<RestService>>) -> impl Responder {
    match service.db_service.get_jobs().await {
        Ok(jobs) => HttpResponse::Ok().json(JobsResponse { jobs }),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::BadRequest().json(ApiError {
                error: "error.db.generic".into(),
                message: format!("{:?}", e),
            })
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobResultsResponse {
    results: Vec<models::JobResult>,
}

#[get("/jobs/{id}/results")]
async fn jobs_results_get(
    service: web::Data<Arc<RestService>>,
    web::Path(id): web::Path<i32>,
) -> impl Responder {
    match service.db_service.get_job_results(id).await {
        Ok(results) => HttpResponse::Ok().json(JobResultsResponse { results }),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::BadRequest().json(ApiError {
                error: "error.db.generic".into(),
                message: format!("{:?}", e),
            })
        }
    }
}

#[derive(Deserialize)]
pub struct JobInput {
    code: String,
}

#[derive(Serialize)]
pub struct JobOutput<'a> {
    output: &'a str,
}

#[post("/jobs/{id}/run")]
async fn jobs_run_post(
    service: web::Data<Arc<RestService>>,
    web::Path(id): web::Path<i32>,
) -> impl Responder {
    let response = PythonExecutor::new()
        .execute_sync(r#"
import requests
r = requests.get("http://localhost:3030/api/v1/groups/24/generate/matrix?traits=cough,wet_cough&attributes=age,length_of_stay&fields=true")
ret = r.text
            "#)
        .unwrap();

    HttpResponse::Ok().json(JobOutput { output: &*response })
}

pub async fn build_and_serve_http(service: RestService) -> Result<(), Box<dyn std::error::Error>> {
    let local = tokio::task::LocalSet::new();
    let sys = actix_rt::System::run_in_tokio("server", &local);

    // TODO: use config or env for port.
    let address = format!("127.0.0.1:3030");

    println!("Server running at {}", address);

    let service_arc = Arc::new(service);
    let server_res = HttpServer::new(move || {
        App::new().data(service_arc.clone()).service(
            web::scope("/api/v1")
                .service(traits_get)
                .service(traits_post)
                .service(groups_post)
                .service(groups_get)
                .service(groups_generate_matrix)
                .service(groups_subjects_post)
                .service(groups_subjects_get)
                .service(groups_subjects_traits_post)
                .service(groups_subjects_traits_get)
                .service(jobs_get)
                .service(jobs_results_get)
                .service(jobs_run_post),
        )
    })
    .bind(address)?
    .run()
    .await?;

    sys.await?;

    Ok(server_res)
}
