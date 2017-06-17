pub mod font;
pub mod texture;

use lru_time_cache::LruCache;
use std::cmp::Ord;
use std::borrow::Borrow;
use std::rc::Rc;

pub struct ResourceManager<'l, K, R, L>
    where K: Ord + Clone,
          L: 'l + ResourceLoader<'l, R>
{
    loader: &'l L,
    cache: LruCache<K, Rc<R>>,
}

impl<'l, K, R, L> ResourceManager<'l, K, R, L>
    where K: Ord + Clone,
          L: ResourceLoader<'l, R>
{
    pub fn new(loader: &'l L, n: usize) -> Self {
        ResourceManager {
            cache: LruCache::with_capacity(n),
            loader: loader,
        }
    }
    pub fn get<D>(&mut self, details: &D) -> Rc<R>
        where L: ResourceLoader<'l, R, Args = D>,
              D: Ord + Clone,
              K: Borrow<D> + for<'a> From<&'a D>
    {
        if !self.cache.contains_key(details) {
            let res = Rc::new(self.loader.load(details).unwrap());
            self.cache.insert(details.into(), res);
        }
        self.cache.get(details).unwrap().clone()
    }
}




pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;
    fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}

