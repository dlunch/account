use std::str::{self, FromStr};

use async_trait::async_trait;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use tonic::{metadata::MetadataValue, Request, Response, Status};
use uuid::Uuid;

use crate::config::Config;
use crate::models;
use crate::schema;
use crate::token;

mod pb {
    tonic::include_proto!("card");
}

use pb::{CardItem, CardListResponse};

pub struct Card {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Card {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>, config: Config) -> pb::card_server::CardServer<Card> {
        let token_secret = config.token_secret;

        pb::card_server::CardServer::with_interceptor(Self { pool }, move |req| Self::check_auth(req, &token_secret))
    }

    fn check_auth(mut req: Request<()>, token_secret: &str) -> Result<Request<()>, Status> {
        if let Some(authorization) = req.metadata().get("authorization") {
            let authorization = str::from_utf8(authorization.as_bytes()).unwrap();

            let split = authorization.split(' ').collect::<Vec<_>>();
            let (bearer, token) = (split[0], split[1]);

            if bearer.to_lowercase() != "bearer" {
                Err(Status::unauthenticated("No valid auth token"))
            } else {
                let user_id = token::decode(token, token_secret);

                req.metadata_mut().remove("user_id");
                req.metadata_mut().append("user_id", MetadataValue::from_str(&user_id).unwrap());

                Ok(req)
            }
        } else {
            Err(Status::unauthenticated("No valid auth token"))
        }
    }
}

#[async_trait]
impl pb::card_server::Card for Card {
    async fn list(&self, request: tonic::Request<()>) -> Result<Response<CardListResponse>, Status> {
        use schema::cards::dsl;

        let user_id = Uuid::from_str(request.metadata().get("user_id").unwrap().to_str().unwrap()).unwrap();

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
