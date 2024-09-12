#[derive(Debug, Clone)]
pub struct Media {
    pub torrent_id: String,   //torrent id
    pub torrent_hash: String, // hash torrent
    pub id: String,           // unrestricted id
    pub name: String,         // name
    pub size: i64,            // size
    pub quality: String,      // quality
}

impl Media {
    pub fn new(torrent_id: &str, torrent_hash: &str, id: &str, name: &str, size: i64, quality: &str) -> Self {
        Media {
            torrent_id: String::from(torrent_id),
            torrent_hash: String::from(torrent_hash),
            id: String::from(id),
            name: String::from(name),
            size,
            quality: String::from(quality),
        }
    }
}