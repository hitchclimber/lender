use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Media {
    pub id: Uuid,
    pub name: String,
    pub creator: String,
    pub year: u16,
    pub category: MediaCategory,
    pub user_id: Uuid,
    pub available: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMedia {
    pub name: String,
    pub creator: String,
    pub year: i16,
    pub category: MediaCategory,
    pub user_id: Uuid,
    pub available: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMedia {
    pub name: Option<String>,
    pub creator: Option<String>,
    pub year: Option<u16>,
    pub category: Option<MediaCategory>,
    pub available: Option<bool>,
}

#[derive(Display, EnumString, Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "media_category", rename_all = "lowercase")]
pub enum MediaCategory {
    Book,
    Dvd,
    Blueray,
    Vinyl,
    Cd,
}
