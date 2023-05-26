pub use schema::GlintsConfig;
use shaku::{module, Component, Module, ModuleBuildContext};

mod schema;

impl<M: Module> Component<M> for GlintsConfig {
    type Interface = GlintsConfig;
    type Parameters = ();

    fn build(
        _context: &mut ModuleBuildContext<M>,
        _params: Self::Parameters,
    ) -> Box<Self::Interface> {
        Box::new(GlintsConfig::read())
    }
}

module! {
    pub ConfigModule {
        components = [ GlintsConfig ],
        providers = [],
    }
}

pub fn build_module() -> ConfigModule {
    ConfigModule::builder().build()
}
