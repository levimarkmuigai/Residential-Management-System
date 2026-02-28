use std::error::Error;

use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    db::utils::get_client,
    entities::{notice::DisplayNotice, request::DbRequest},
};
