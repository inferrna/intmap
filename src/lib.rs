use std::ops::Deref;

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
    /* Double hashing implemented with nested vectors */
    entries: Vec<Vec<Option<Entry<T>>>>,
    stride: Keytype,
}

impl<T> IntMap<T> {
    pub fn new (stride: Keytype) -> IntMap<T> {

        IntMap { entries: (0..stride).map(|_| (0..stride).map(|_| None).collect()).collect(),
                  stride: stride}
    }
    fn find_pos (&self, key: Keytype) -> Option<usize> {
        /* Remainder as secondary hash value */
        return self.entries[key % self.stride].iter()
            .enumerate()
            .filter(|(i, e)| e.as_ref().is_some())
            .map(|(i, e)| i)
            .next();
    }
    fn find_free (&mut self, bkt_idx: usize) -> usize {
        let fpos = self.entries[bkt_idx].iter().position(|e| e.is_none());
        if let Some(i) = fpos {
            return i;
        } else {
            let res = self.entries[bkt_idx].len();
            self.entries[bkt_idx].resize_with(res+self.stride, || None);
            return res;
        }
    }
    pub fn put(&mut self, key: Keytype, value: T) {
        let bkt_idx= key % self.stride;
        if let Some(i) = self.find_pos(key) {
            self.entries[key % self.stride][i].as_mut().unwrap().value = value;
        } else {
            let idx = self.find_free(bkt_idx);
            self.entries[bkt_idx][idx] = Some(Entry::new(key, value));
        }
    }
    pub fn contains_key(&mut self, key: Keytype) -> bool {
        let res = self.get(key);
        match res {
            Some(_) => true,
            None => false
        }
    }
    pub fn get(&self, key: Keytype) -> Option<&T> {
        let pos = self.find_pos(key);
        match pos {
            Some(i) => Some(&self.entries[key % self.stride][i].as_ref().unwrap().value),
            None => None
        }
    }

    pub fn remove(&mut self, key: Keytype) -> Option<T> {
        let bkt_idx= key % self.stride;
        if let Some(i) = self.find_pos(key) {
            let mut res = &mut self.entries[key % self.stride][i];
            let rs = res.take().unwrap().value;
            return Some(rs);
        } else {
            return None;
        }
    }
}

/* contains_value implemented only for PartialEq capable types */
impl<T: PartialEq> IntMap<T> {
    pub fn contains_value(&self, value: T) -> bool {
        let res = self.entries.iter().flatten().filter_map(|e| e.as_ref()).find(|e| e.value == value);
        match res {
            Some(_) => true,
            None => false
        }
    }
}

#[cfg(test)]
mod tests;
