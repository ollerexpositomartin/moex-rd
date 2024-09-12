use std::error::Error;

use actix_web::{web, HttpResponse};
use async_trait::async_trait;

use crate::share::models::Torrent;

#[async_trait]
pub trait UploadDriver: Send + Sync {
    async fn upload(&self, torrent: web::Json<Torrent>) -> Result<HttpResponse,Box<dyn Error>>;
}
