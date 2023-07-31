use crate::config::Config;
use crate::storage::StorageEngine;

pub struct InnerDB<'eng>{
    storage_engine: &'eng dyn StorageEngine,
    config:Config,
}