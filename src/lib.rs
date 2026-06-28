use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct BlobCache {
    blobs: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl BlobCache {
    pub fn new() -> Self {
        Self {
            blobs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn create_blob(&self, data: Vec<u8>) -> String {
        let id = Uuid::new_v4().to_string();
        let mut blobs = self.blobs.write().unwrap();
        blobs.insert(id.clone(), data);
        id
    }

    pub fn get_blob(&self, id: &str) -> Option<Vec<u8>> {
        let blobs = self.blobs.read().unwrap();
        blobs.get(id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blob_cache() {
        let cache = BlobCache::new();
        let data = b"hello world".to_vec();
        
        let id = cache.create_blob(data.clone());
        let retrieved = cache.get_blob(&id);
        
        assert_eq!(retrieved, Some(data));
        
        let not_found = cache.get_blob("invalid-id");
        assert_eq!(not_found, None);
    }
}
