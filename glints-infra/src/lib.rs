use diesel_async::pooled_connection::mobc::Pool;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, PoolableConnection};
use diesel_async::AsyncPgConnection;
use glints_config::{ConfigModule, GlintsConfig};
use shaku::{module, Component, HasComponent, Interface, Module, ModuleBuildContext};
use std::sync::Arc;

pub trait ConnectionPool<T: PoolableConnection + 'static>: Interface {
    fn get(&self) -> &Pool<T>;
}

pub struct PgConnectionPool {
    pool: Pool<AsyncPgConnection>,
}

impl<M: Module + HasComponent<GlintsConfig>> Component<M> for PgConnectionPool {
    type Interface = PgConnectionPool;
    type Parameters = ();

    fn build(
        context: &mut ModuleBuildContext<M>,
        _params: Self::Parameters,
    ) -> Box<Self::Interface> {
        let config: Arc<GlintsConfig> = M::build_component(context);
        let database_url = &config.postgres.database_url;

        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
        let pool = Pool::new(config);

        Box::new(PgConnectionPool { pool })
    }
}

impl ConnectionPool<AsyncPgConnection> for PgConnectionPool {
    fn get(&self) -> &Pool<AsyncPgConnection> {
        &self.pool
    }
}

module! {
    pub InfraModule {
        components = [ PgConnectionPool ],
        providers = [],

        use ConfigModule {
            components = [ GlintsConfig ],
            providers = [],
        }
    }
}

pub fn build_module() -> InfraModule {
    InfraModule::builder(glints_config::build_module().into()).build()
}
