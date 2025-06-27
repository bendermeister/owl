use crate::{config::Config, store::Store};

#[derive(Debug, Clone)]
pub struct Context {
    pub store: Store,
    pub config: Config,
}
