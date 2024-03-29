use super::Create;
use crate::domain::entities::publisher::PublisherId;
use crate::interface::repositories::publisher::create::NewPublisher;
use crate::{
    interface::{presenters::Model, repositories::publisher::PublisherRepo},
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, NewPublisher, PublisherId> for Create {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        submitted_data: NewPublisher,
    ) -> AppResult<PublisherId> {
        PublisherRepo::create(db_conn_pool, submitted_data).await
    }
}
