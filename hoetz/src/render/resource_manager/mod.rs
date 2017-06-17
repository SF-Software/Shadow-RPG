pub mod font;
pub mod texture;

use lru_time_cache::LruCache;
use std::cmp::Ord;
use std::borrow::Borrow;


pub struct ResourceManager<'l, K, R, L>
    where K: Ord + Clone,
          L: 'l + ResourceLoader<'l, R>
{
    loader: &'l mut L,
    cache: LruCache<K, R>,
}

impl<'l, K, R, L> ResourceManager<'l, K, R, L>
    where K: Ord + Clone,
          L: ResourceLoader<'l, R>
{
    pub fn new(loader: &'l mut L, n: usize) -> Self {
        ResourceManager {
            cache: LruCache::with_capacity(n),
            loader: loader,
        }
    }
    pub fn get<D>(&mut self, details: &D) -> &R
        where L: ResourceLoader<'l, R, Args = D>,
              D: Ord + Clone,
              K: Borrow<D> + for<'a> From<&'a D>
    {
        if !self.cache.contains_key(details) {
            let res = self.loader.load(details).unwrap();
            self.cache.insert(details.into(), res);
        }
        self.cache.get(details).unwrap()
    }
    pub fn get_mut<D>(&'l mut self, details: &D) -> &mut R
        where L: ResourceLoader<'l, R, Args = D>,
              D: Ord + Clone,
              K: Borrow<D> + for<'a> From<&'a D>
    {
        if !self.cache.contains_key(details) {
            let res = self.loader.load(details).unwrap();
            self.cache.insert(details.into(), res);
        }
        self.cache.get_mut(details).unwrap()
    }
}




pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;
    fn load(&'l mut self, data: &Self::Args) -> Result<R, String>;
}

