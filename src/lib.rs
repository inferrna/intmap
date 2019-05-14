use std::ops::Deref;

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

pub struct IntMap<T> {
    /* Double hashing implemented with nested vectors */
    entries: Vec<Option<Entry<T>>>,
    offsets: Vec<usize>,
    stride: Keytype,
    step: Keytype
}

impl<T> IntMap<T> {
    pub fn new (stride: Keytype) -> IntMap<T> {
        let mut offsets: Vec<usize> = (0..stride).map(|_| 0).collect();
        //offsets[0] = stride as usize;
        IntMap { entries: (0..stride*stride).map(|_| None).collect(),
                 offsets: offsets,
                  stride: stride,
                    step: stride}
    }
    fn get_offset(&self, key: Keytype) -> usize {
        let hash = key % self.stride;
        (self.stride * hash) as usize + self.offsets[hash]
    }
    fn inc_offset(&mut self, key: Keytype) {
        let hash = key % self.stride;
        self.offsets[hash] += 1;
    }
    fn find_pos (&self, key: Keytype) -> Option<usize> {
        /* Remainder as secondary hash value */
        let (off_current, mut off_next) = (self.get_offset(key), self.get_offset(key+1));
        while off_next < off_current {
            off_next += self.stride;
        }
        assert!(off_current<off_next, format!("off_current {} >= off_next {}", off_current, off_next));
        return self.entries.iter()
            .enumerate()
            .cycle()
            .skip(off_current)
            .take(off_next - off_current)
            .filter(|(i, e)| e.as_ref().is_some())
            .map(|(i, e)| i)
            .next();
    }
    fn rehash(&mut self) {
        let mut new_map = IntMap::<T>::new(self.stride + self.step);
        for o in &mut self.entries {
            if let Some(e) = o {
                let es = o.take().unwrap();
                new_map.put(es.key, es.value);
            }
        }
        *self = new_map;
    }
    fn find_free (&mut self, key: Keytype, req: Keytype) -> usize {
        let offset = self.get_offset(key);
        let fpos = self.entries.iter().skip(offset).position(|e| e.is_none());
        if let Some(i) = fpos {
            return i + offset;
        } else if req == self.stride {
            self.rehash();
            return self.find_free(key, 0);
        } else {
            let next_key = key+1;
            let want_free = self.get_offset(next_key);
            let next_free = self.find_free(next_key, req+1);
            self.entries.swap(want_free, next_free);
            self.inc_offset(next_key);
            return want_free;
        }
    }
    pub fn put(&mut self, key: Keytype, value: T) {
        let bkt_idx= key % self.stride;
        if let Some(i) = self.find_pos(key) {
            self.entries[i].as_mut().unwrap().value = value;
        } else {
            let idx = self.find_free(bkt_idx, 0);
            self.entries[idx] = Some(Entry::new(key, value));
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
            Some(i) => Some(&self.entries[i].as_ref().unwrap().value),
            None => None
        }
    }

    pub fn remove(&mut self, key: Keytype) -> Option<T> {
        if let Some(i) = self.find_pos(key) {
            let mut res = &mut self.entries[i];
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
        let res = self.entries.iter().filter_map(|e| e.as_ref()).find(|e| e.value == value);
        match res {
            Some(_) => true,
            None => false
        }
    }
}

#[cfg(test)]
mod tests;
