use std::sync::{Arc, Mutex};

pub type Keytype = usize;

struct Entry<T> {
    pub key: Keytype,
    pub value: T
}

impl<T> Entry<T> {
    pub fn new (key: Keytype, value: T) -> Entry<T> {
        Entry { key: key,
              value: value }
    }
}

pub struct _IntMap<T> {
    /* Double hashing implemented with nested vectors */
    entries: Vec<Vec<Entry<T>>>,
    stride: Keytype,
}

impl<T> _IntMap<T> {
    pub fn new (stride: Keytype) -> _IntMap<T> {
        _IntMap { entries: (0..stride).map(|_| Vec::with_capacity(stride)).collect(),
                  stride: stride}
    }
    fn find_pos (&self, key: Keytype) -> Option<usize> {
        /* Remainder as secondary hash value */
        return self.entries[key % self.stride].iter().position(|e| e.key == key);
    }
    pub fn put(&mut self, key: Keytype, value: T) {
        let pos = self.find_pos(key);
        if let Some(i) = pos {
            self.entries[key % self.stride][i].value = value;
        } else {
            self.entries[key % self.stride].push(Entry::new(key, value));
        }
    }
    pub fn contains_key(&self, key: Keytype) -> bool {
        let pos = self.find_pos(key);
        match pos {
            Some(_) => true,
            None => false
        }
    }
    pub fn get(&self, key: Keytype) -> Option<&T> {
        let pos = self.find_pos(key);
        match pos {
            Some(i) => Some(&self.entries[key % self.stride][i].value),
            None => None
        }
    }
    pub fn remove(&mut self, key: Keytype) -> Option<T> {
        let pos = self.find_pos(key);
        if let Some(i) = pos {
            /* Order doesn't matter, so I use swap_remove as it is O(1) operation */
            return Some(self.entries[key % self.stride].swap_remove(i).value);
        } else {
            return None;
        }
    }
}

/* contains_value implemented only for PartialEq capable types */
impl<T: PartialEq> _IntMap<T> {
    pub fn contains_value(&self, value: T) -> bool {
        let res = self.entries.iter().flatten().find(|e| e.value == value);
        match res {
            Some(_) => true,
            None => false
        }
    }
}

pub struct IntMap<T> {
    real_map: Arc<Mutex<_IntMap<T>>>
}

impl<T> IntMap<T> {
    pub fn new (stride: Keytype) -> IntMap<T> {
        let map = _IntMap::<T>::new(stride);
        IntMap { real_map: Arc::new(Mutex::new(map)) }
    }
    pub fn put(&mut self, key: Keytype, value: T) {
        self.real_map.lock().unwrap().put(key, value)
    }
    pub fn contains_key(&mut self, key: Keytype) -> bool {
        self.real_map.lock().unwrap().contains_key(key)
    }
    pub fn remove(&mut self, key: Keytype) -> Option<T> {
        self.real_map.lock().unwrap().remove(key)
    }
}

impl<T: Clone> IntMap<T> {
    pub fn get(&self, key: Keytype) -> Option<T> {
        let res = self.real_map.lock().unwrap().get(key).cloned();
        match res {
            Some(v) => Some(v.clone()),
            None => None
        }
    }
}

impl<T: PartialEq> IntMap<T> {
    pub fn contains_value(&self, value: T) -> bool {
        self.real_map.lock().unwrap().contains_value(value)
    }
}

impl<T> Clone for IntMap<T> {
    fn clone(&self) -> Self {
        return IntMap { real_map: self.real_map.clone() }
    }
}

#[cfg(test)]
mod tests;
