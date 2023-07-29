use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::hash_map::{Entry, Iter, IterMut};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct Dictionary<
    K: BorshSerialize + BorshDeserialize + Clone + Eq + Hash,
    V: BorshSerialize + BorshDeserialize + Clone,
> {
    pub data: HashMap<K, V>,
}

impl<K, V> Dictionary<K, V>
where
    K: BorshSerialize + BorshDeserialize + Clone + Eq + Hash,
    V: BorshSerialize + BorshDeserialize + Clone,
{
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.data.insert(key, value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.get_mut(key)
    }

    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        self.data.entry(key)
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.data.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.data.values()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.data.values_mut()
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    pub fn iter(&self) -> Iter<K, V> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        self.data.iter_mut()
    }

    pub fn as_vec(&self) -> Vec<(K, V)> {
        self.data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

impl<K, V> std::ops::Index<K> for Dictionary<K, V>
where
    K: BorshSerialize + BorshDeserialize + Clone + Eq + Hash,
    V: BorshSerialize + BorshDeserialize + Clone,
{
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        &self.data[&index]
    }
}

impl<K, V> std::ops::IndexMut<K> for Dictionary<K, V>
where
    K: BorshSerialize + BorshDeserialize + Clone + Eq + Hash,
    V: BorshSerialize + BorshDeserialize + Clone,
{
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.data.get_mut(&index).unwrap()
    }
}

impl<K, V> std::ops::Index<&K> for Dictionary<K, V>
where
    K: BorshSerialize + BorshDeserialize + Clone + Eq + Hash,
    V: BorshSerialize + BorshDeserialize + Clone,
{
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        &self.data[index]
    }
}

impl<K, V> std::ops::IndexMut<&K> for Dictionary<K, V>
where
    K: BorshSerialize + BorshDeserialize + Clone + Eq + Hash,
    V: BorshSerialize + BorshDeserialize + Clone,
{
    fn index_mut(&mut self, index: &K) -> &mut Self::Output {
        self.data.get_mut(index).unwrap()
    }
}

impl<K, V> BorshSerialize for Dictionary<K, V>
where
    K: BorshSerialize + BorshDeserialize + Clone + Eq + Hash,
    V: BorshSerialize + BorshDeserialize + Clone,
{
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let vec = self.as_vec();
        vec.serialize(writer)
    }
}

impl<K, V> BorshDeserialize for Dictionary<K, V>
where
    K: BorshSerialize + BorshDeserialize + Clone + Eq + Hash,
    V: BorshSerialize + BorshDeserialize + Clone,
{
    fn deserialize(reader: &mut &[u8]) -> std::io::Result<Self> {
        let vec: Vec<(K, V)> = BorshDeserialize::deserialize(reader)?;
        let map = vec.into_iter().collect::<HashMap<K, V>>();
        Ok(Dictionary { data: map })
    }
}
