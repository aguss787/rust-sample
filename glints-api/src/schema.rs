use crate::APIModule;
use async_graphql::{ComplexObject, Context, Object, SimpleObject};
use glints_managed_talent::HubberAPI;
use serde::{Deserialize, Serialize};
use shaku::HasComponent;
use std::sync::Arc;

pub struct Query;

#[Object(extends)]
impl Query {
    async fn hubbers(&self, ctx: &Context<'_>) -> Vec<Hubber> {
        let module = ctx.data::<APIModule>().unwrap();
        let hubber_service: Arc<dyn HubberAPI> = module.resolve();

        hubber_service
            .list_hubber()
            .await
            .into_iter()
            .map(|i| Hubber {
                id: i.id,
                code: i.code,
                name: i.name,
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Hubber {
    pub id: String,
    pub code: String,
    pub name: String,
}

#[ComplexObject]
impl Hubber {
    async fn computed_string(&self) -> String {
        format!("{} - {} - {}", self.id, self.code, self.name)
    }
}
