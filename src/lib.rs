type Keytype = usize;

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

pub struct IntMap<T> {
    entries: Vec<Vec<Entry<T>>>,
    stride: Keytype,
}

impl<T> IntMap<T> {
    pub fn new (stride: Keytype) -> IntMap<T> {
        IntMap { entries: (0..stride).map(|_| Vec::with_capacity(stride)).collect(),
                  stride: stride}
    }
    fn find_pos (&self, key: Keytype) -> Option<usize> {
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
            return Some(self.entries[key % self.stride].swap_remove(i).value);
        } else {
            return None;
        }
    }
}
impl<T: PartialEq> IntMap<T> {
    pub fn contains_value(&self, value: T) -> bool {
        let res = self.entries.iter().flatten().find(|e| e.value == value);
        match res {
            Some(_) => true,
            None => false
        }
    }
}

#[cfg(test)]
mod tests;
