use actix_web::{HttpRequest, HttpResponse, web};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_qs::actix::QsQuery;

use crate::application::Application;
use crate::application::error::BusinessError;
use crate::application::storage::connections::ConnectionResource;
use crate::application::storage::filters::build_label_filter;
use crate::server::response::{Resp, RespResult};

pub async fn get_health() -> RespResult {
    Resp::ok(json!({ "status": "success" })).to_json_result()
}
