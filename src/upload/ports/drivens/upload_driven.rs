use std::error::Error;
use async_trait::async_trait;

use crate::share::models::Torrent;
use crate::share::models::Media;

#[async_trait]
pub trait UploadDriven:Send + Sync {
    fn upload(&self,torrent:Torrent) -> Result<Media,Box<dyn Error>>;
}
