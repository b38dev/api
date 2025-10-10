use anyhow::Result;
use futures::future::Shared;
use futures::{Future, FutureExt};
use std::pin::Pin;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, Semaphore};

#[derive(Clone)]
pub struct TaskQueue<K, V> {
    inner: Arc<
        Mutex<
            HashMap<
                K,
                Shared<
                    Pin<Box<dyn Future<Output = Result<V, Arc<anyhow::Error>>> + Send + 'static>>,
                >,
            >,
        >,
    >,
    semaphore: Arc<Semaphore>,
}

impl<K, V> TaskQueue<K, V>
where
    K: Eq + std::hash::Hash + Clone + Send + 'static,
    V: Clone + Send + 'static,
{
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    pub async fn get_or_spawn<F, Fut>(&self, key: K, task: F) -> Result<V, Arc<anyhow::Error>>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Result<V>> + Send + 'static,
    {
        if let Some(existing) = self.inner.lock().await.get(&key) {
            return existing.clone().await;
        }

        let permit = self.semaphore.clone().acquire_owned().await.unwrap();

        let fut = async move {
            let result = task().await.map_err(|e| Arc::new(e));
            drop(permit);
            result
        }
        .boxed()
        .shared();

        self.inner.lock().await.insert(key.clone(), fut.clone());

        let result = fut.await;
        self.inner.lock().await.remove(&key);
        result
    }
}
