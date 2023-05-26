mod models;
mod schema;

use crate::schema::hubbers::dsl::hubbers;
use async_trait::async_trait;
use diesel::QueryDsl;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use glints_infra::{ConnectionPool, InfraModule};
use shaku::{module, Component, HasComponent, Interface, Module, ModuleBuildContext};
use std::sync::Arc;

#[derive(Debug)]
pub struct Hubber {
    pub id: String,
    pub code: String,
    pub name: String,
}

pub use glints_infra::PgConnectionPool;

pub struct HubberService<Pool: ConnectionPool<AsyncPgConnection>> {
    db_connection_pool: Arc<Pool>,
}

impl<M: Module + HasComponent<PgConnectionPool>> Component<M> for HubberService<PgConnectionPool> {
    type Interface = HubberService<PgConnectionPool>;
    type Parameters = ();

    fn build(
        context: &mut ModuleBuildContext<M>,
        _params: Self::Parameters,
    ) -> Box<Self::Interface> {
        Box::new(HubberService {
            db_connection_pool: M::build_component(context),
        })
    }
}

#[async_trait]
pub trait HubberAPI: Interface {
    async fn list_hubber(&self) -> Vec<Hubber>;
}

#[async_trait]
impl<Pool: ConnectionPool<AsyncPgConnection>> HubberAPI for HubberService<Pool> {
    async fn list_hubber(&self) -> Vec<Hubber> {
        let mut connection = self.db_connection_pool.get().get().await.expect("todo");
        let result = hubbers
            .limit(10)
            .load::<models::Hubber>(&mut connection)
            .await
            .expect("todo");

        result
            .into_iter()
            .map(|i| Hubber {
                id: i.id.to_string(),
                code: i.code,
                name: i.name,
            })
            .collect()
    }
}

module! {
    pub ManagedTalentModule {
        components = [HubberService<PgConnectionPool>],
        providers = [],

        use InfraModule {
            components = [PgConnectionPool],
            providers = [],
        }
    }
}

pub fn build_module() -> ManagedTalentModule {
    ManagedTalentModule::builder(glints_infra::build_module().into()).build()
}
