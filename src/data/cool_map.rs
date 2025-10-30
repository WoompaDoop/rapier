use std::iter::{self, Extend, FromIterator, FusedIterator};
use std::{
    collections::BTreeMap,
    ops::{Index, IndexMut},
};

use parry::utils::hashmap::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct CoolKey {
    index: u32,
}

impl Default for CoolKey {
    fn default() -> Self {
        Self::from_raw_parts(crate::INVALID_U32)
    }
}

impl CoolKey {
    pub fn from_raw_parts(index: u32) -> CoolKey {
        CoolKey { index }
    }

    pub fn into_raw_parts(self) -> u32 {
        self.index
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct CoolMap<T> {
    tree: BTreeMap<CoolKey, T>,
    index_to_key: Vec<CoolKey>,
    key_to_index: HashMap<CoolKey, u32>,
}

impl<T> Default for CoolMap<T> {
    fn default() -> Self {
        Self { tree: Default::default(), index_to_key: Default::default(), key_to_index: Default::default() }
    }
}

impl<T> CoolMap<T> {
    pub fn new() -> Self {
        CoolMap {
            tree: BTreeMap::new(),
            index_to_key: vec![],
            key_to_index: HashMap::default(),
        }
    }

    pub fn clear(&mut self) {
        self.tree.clear();
    }

    pub fn insert(&mut self, key_base: u32, value: T) -> CoolKey {
        let key = CoolKey { index: key_base };
        self.tree.insert(key, value);
        self.remake_index_maps();
        key
    }

    pub fn remove(&mut self, k: &CoolKey) -> Option<T> {
        let element = self.tree.remove(k);
        self.remake_index_maps();
        element
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

    pub fn index_to_key(&self, index: u32) -> CoolKey {
        self.index_to_key[index as usize]
    }

    pub fn key_to_index(&self, key: &CoolKey) -> u32 {
        self.key_to_index[key]
    }

    fn remake_index_maps(&mut self) {
        self.index_to_key.clear();
        self.key_to_index.clear();

        self.index_to_key
            .resize(self.tree.len(), CoolKey { index: 0 });
        for (idx, key) in self.tree.keys().enumerate() {
            self.index_to_key[idx] = *key;
            self.key_to_index.insert(*key, idx as u32);
        }
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
