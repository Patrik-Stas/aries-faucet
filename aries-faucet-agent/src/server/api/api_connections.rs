use actix_web::{HttpRequest, HttpResponse, web};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_qs::actix::QsQuery;

use crate::application::Application;
use crate::application::error::BusinessError;
use crate::application::storage::connections::Connection;
use crate::application::storage::filters::build_label_filter;
use crate::server::response::{Resp, RespResult};

#[derive(Deserialize, Serialize, Debug)]
pub struct ConnectionResource {
    pub(crate) id: String,
    pub(crate) label: String,
}

impl From<Connection> for ConnectionResource {
    fn from(connection: Connection) -> Self {
        ConnectionResource {
            id: connection.id.to_string(),
            label: connection.label,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestCreateConnection {
    pub(crate) label: String,
}

pub async fn create(app_data: web::Data<Application>,
                    req: web::Json<RequestCreateConnection>) -> RespResult {
    let mut resource: RequestCreateConnection = req.into_inner();
    let id = app_data.service_connections.create(resource.label).await?;
    Resp::ok(json!({ "id": id })).to_json_result()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryConnections {
    keyword: Option<String>,
}

#[derive(Deserialize)]
pub struct QueryFilterConnections {
    label: Option<String>
}

pub async fn get_many(app_data: web::Data<Application>,
                      query: QsQuery<QueryFilterConnections>) -> RespResult {
    let filter = build_label_filter(query.label.clone());
    let list = app_data.service_connections.get_many(filter).await?;
    let connections: Vec<ConnectionResource> = list.into_iter().map(|b| ConnectionResource::from(b)).collect();
    Resp::ok(connections).to_json_result()
}

pub async fn get_by_id(app_data: web::Data<Application>,
                       req: HttpRequest) -> RespResult {
    let id = req.match_info().get("id")
        .ok_or(anyhow!("api_connections.rs :: get_by_id >> was invoked by id param is missing"))?;
    let connection = app_data.service_connections
        .get_by_id(id).await?;
    Resp::ok(ConnectionResource::from(connection))
        .to_json_result()
}

pub async fn delete_by_id(app_data: web::Data<Application>,
                          req: HttpRequest) -> RespResult {
    let id = req.match_info().get("id")
        .ok_or(anyhow!("api_connections.rs :: delete_by_id >> was invoked by id param is missing"))?;
    app_data.service_connections.delete_by_id(id).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn update_by_id(app_data: web::Data<Application>,
                          req: HttpRequest,
                          payload: web::Json<RequestCreateConnection>) -> RespResult {
    let id = req.match_info().get("id")
        .ok_or(anyhow!("api_connections.rs :: update_by_id >> was invoked by id param is missing"))?;
    app_data.service_connections.update_by_id(id, &payload.label).await?;
    Ok(HttpResponse::Ok().finish())
}
