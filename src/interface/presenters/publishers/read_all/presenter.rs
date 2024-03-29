use async_trait::async_trait;
use axum::Json;
use sqlx::Sqlite;

use crate::interface::{presenters::Presenter, repositories::publisher::read_all::Publisher};

use super::ReadAll;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, ReadAll, Sqlite, (), Vec<Publisher>, Json<Vec<Publisher>>> for ReadAll {}
