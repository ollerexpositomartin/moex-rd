use crate::share::models::Media;
use crate::share::models::Torrent;
use crate::upload::ports::drivens::UploadDriven;
use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct RdRepository {}


impl UploadDriven for RdRepository {
    fn upload(&self, torrent: Torrent) -> Result<Media, Box<dyn Error>> {
        return Ok(Media::new("", "", "", "", 21, ""));
    }
}

impl RdRepository {
    pub fn new() -> Box<dyn UploadDriven> {
        Box::new(Self{})
    }
}
