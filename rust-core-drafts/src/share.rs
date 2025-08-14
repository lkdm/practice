use std::{collections::HashMap, ops::Deref, sync::Arc};

use crate::{draft::Draft, error::Result};
use nutype::nutype;
use thiserror::Error;
use uuid::{Timestamp, Uuid};

pub struct ShareMetadata {
    shared_at: Option<Timestamp>,
    // method: ShareMethod,
}

pub trait ShareMethod: Send + Sync {
    fn id(&self) -> &str;
    fn label(&self) -> &str;
    fn name(&self) -> &str;
    fn share(&self, draft: &Draft) -> Result<()>;
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct SharePluginId(Uuid);

impl From<Uuid> for SharePluginId {
    fn from(id: Uuid) -> Self {
        SharePluginId(id)
    }
}

impl From<SharePluginId> for Uuid {
    fn from(spid: SharePluginId) -> Self {
        spid.0
    }
}

#[derive(Debug, Error)]
pub enum PluginRegistrationError {
    #[error("invalid uuid: {0}")]
    InvalidUuid(#[from] uuid::Error),
}

impl TryFrom<&str> for SharePluginId {
    type Error = PluginRegistrationError;
    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
        let uuid = Uuid::parse_str(s)?;
        Ok(SharePluginId(uuid))
    }
}

struct SharePluginInfo {
    pub id: SharePluginId,
    pub label: String,
}

pub struct ShareRegistry {
    methods: HashMap<SharePluginId, Arc<dyn ShareMethod>>,
}

impl ShareRegistry {
    pub fn register<M: ShareMethod + 'static>(&mut self, method: M) -> Result<()> {
        let id: SharePluginId = method.id().try_into()?;
        self.methods.insert(id, Arc::new(method));
        Ok(())
    }

    pub fn get(&self, name: &SharePluginId) -> Option<Arc<dyn ShareMethod>> {
        self.methods.get(name).cloned()
    }

    pub fn list(&self) -> Vec<(&SharePluginId, &Arc<dyn ShareMethod>)> {
        self.methods.iter().collect()
    }
}
