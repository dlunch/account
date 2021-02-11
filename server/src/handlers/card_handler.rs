use async_trait::async_trait;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use tonic::{Response, Status};

use crate::config::Config;
use crate::db::models;
use crate::db::schema;

use super::base;

mod pb {
    tonic::include_proto!("card");
}

use pb::{CardItem, CardListResponse};

pub struct CardHandler {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl CardHandler {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>, config: Config) -> pb::card_server::CardServer<Self> {
        let token_secret = config.token_secret;

        pb::card_server::CardServer::with_interceptor(Self { pool }, move |req| base::check_auth(req, &token_secret))
    }
}

#[async_trait]
impl pb::card_server::Card for CardHandler {
    async fn list(&self, request: tonic::Request<()>) -> Result<Response<CardListResponse>, Status> {
        use schema::cards::dsl;

        let user_id = base::get_user_id(request);

        let cards = dsl::cards
            .filter(dsl::user_id.eq(user_id))
            .load::<models::Card>(&self.pool.get().unwrap())
            .unwrap();

        let items = cards
            .into_iter()
            .map(|x| CardItem {
                id: x.id.to_string(),
                r#type: x.type_,
                display_name: x.display_name,
            })
            .collect::<Vec<_>>();

        let response = CardListResponse { items };

        Ok(Response::new(response))
    }
}
