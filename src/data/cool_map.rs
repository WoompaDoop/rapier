use std::iter::{self, Extend, FromIterator, FusedIterator};
use std::{
    collections::BTreeMap,
    ops::{Index, IndexMut},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct CoolKey {
    key_base: u32,
    count: u32,
}

impl Default for CoolKey {
    fn default() -> Self {
        Self::from_raw_parts(crate::INVALID_U32, crate::INVALID_U32)
    }
}

impl CoolKey {
    pub fn from_raw_parts(key_base: u32, count: u32) -> CoolKey {
        CoolKey { key_base, count }
    }

    pub fn into_raw_parts(self) -> (u32, u32) {
        (self.key_base, self.count)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct CoolMap<T> {
    tree: BTreeMap<CoolKey, T>,
}

impl<T> Default for CoolMap<T> {
    fn default() -> Self {
        Self {
            tree: Default::default(),
        }
    }
}

impl<T> CoolMap<T> {
    pub fn new() -> Self {
        CoolMap {
            tree: BTreeMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.tree.clear();
    }

    pub fn insert(&mut self, key_base: u32, value: T) -> CoolKey {
        let count = self.tree.keys().filter(|x| x.key_base == key_base).count() as u32;
        let key = CoolKey { key_base, count };
        self.tree.insert(key, value);
        key
    }

    pub fn remove(&mut self, k: &CoolKey) -> Option<T> {
        self.tree.remove(k)
    }

    pub fn contains_key(&self, k: &CoolKey) -> bool {
        self.tree.contains_key(k)
    }

    pub fn get(&self, k: &CoolKey) -> Option<&T> {
        self.tree.get(k)
    }

    pub fn get_mut(&mut self, k: &CoolKey) -> Option<&mut T> {
        self.tree.get_mut(k)
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&CoolKey, &T)> {
        self.tree.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&CoolKey, &mut T)> {
        self.tree.iter_mut()
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.tree.values()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.tree.values_mut()
    }

    pub fn ensure_exists(&mut self, key: &CoolKey, default: T) -> &mut T {
        if !self.contains_key(key) {
            self.tree.insert(*key, default);
        }

        self.tree.get_mut(key).unwrap()
    }
}

impl<T> Index<CoolKey> for CoolMap<T> {
    type Output = T;

    fn index(&self, key: CoolKey) -> &T {
        &self.tree[&key]
    }
}

impl<T> IndexMut<CoolKey> for CoolMap<T> {
    fn index_mut(&mut self, key: CoolKey) -> &mut T {
        self.tree.get_mut(&key).unwrap()
    }
}
