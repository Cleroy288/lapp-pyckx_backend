//! CollectionApp - App instance for managing collections

use crate::domain::{AppInstance, AppModule};

/// Collection app metadata
const COLLECTION_APP: AppInstance = AppInstance::new(
    "collection",
    "Collection",
    "Create and manage collections of items, bookmarks, and resources",
);

/// CollectionApp - Manages user collections
#[derive(Debug, Clone)]
pub struct CollectionApp {
    info: AppInstance,
}

impl CollectionApp {
    pub fn new() -> Self {
        Self {
            info: COLLECTION_APP,
        }
    }
}

impl Default for CollectionApp {
    fn default() -> Self {
        Self::new()
    }
}

impl AppModule for CollectionApp {
    fn info(&self) -> &AppInstance {
        &self.info
    }
}
