use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};

pub mod user;
pub mod requests;
pub mod responses;
pub mod auth;
pub mod domain;
