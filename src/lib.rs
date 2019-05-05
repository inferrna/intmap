type keytype = usize;

struct Entry<T> {
    pub key: keytype,
    pub value: T
}

impl<T> Entry<T> {
    pub fn new (key: keytype, value: T) -> Entry<T> {
        Entry { key: key,
              value: value }
    }
}

pub struct IntMap<T> {
    entries: Vec<Vec<Entry<T>>>,
    stride: keytype
}

impl<T> IntMap<T> {
    pub fn new (stride: keytype) -> IntMap<T> {
        IntMap { entries: (0..stride).map(|_| Vec::with_capacity(stride)).collect(),
                  stride: stride}
    }
    fn find_pos (&self, key: keytype) -> Option<usize> {
        return self.entries[key % self.stride].iter().position(|e| e.key == key);
    }
    pub fn put(&mut self, key: keytype, value: T) {
        let pos = self.find_pos(key);
        if let Some(i) = pos {
            self.entries[key % self.stride][i].value = value;
        } else {
            self.entries[key % self.stride].push(Entry::new(key, value));
        }
    }
    pub fn get(&self, key: keytype) -> Option<&T> {
        let pos = self.find_pos(key);
        match pos {
            Some(i) => Some(&self.entries[key % self.stride][i].value),
            None => None
        }
    }
}
  
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
