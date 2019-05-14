use std::cmp::min;
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
    entries: Vec<Option<Entry<T>>>,
    offsets: Vec<usize>,
    stride: Keytype
}

impl<T> _IntMap<T> {
    pub fn new (stride: Keytype) -> _IntMap<T> {
        let mut offsets: Vec<usize> = (0..stride).map(|_| 0).collect();
        //offsets[0] = stride as usize;
        _IntMap { entries: (0..stride*stride).map(|_| None).collect(),
                 offsets: offsets,
                  stride: stride}
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
        let mut inc_cnt = 0;
        let (off_current, mut off_next) = (self.get_offset(key), self.get_offset(key+1));
        while off_next <= off_current {
            off_next += self.entries.len();
            inc_cnt += 1;
        }
        return self.entries.iter()
            .enumerate()
            .cycle()
            .skip(off_current)
            .take(off_next - off_current)
            .filter(|(i, e)| e.as_ref().is_some())
            .skip_while(|(i, e)| e.as_ref().unwrap().key != key)
            .map(|(i, e)| i)
            .next();
    }
    fn rehash(&mut self) {
        let filled = self.entries.iter().filter(|e| e.is_some()).count();
        let stride_inc = min(1, (self.stride * filled) / self.entries.len());
        let mut new_map = _IntMap::<T>::new(self.stride+stride_inc);
        for o in &mut self.entries {
            if let Some(e) = o {
                let es = o.take().unwrap();
                new_map.put(es.key, es.value);
            }
        }
        *self = new_map;
    }
    fn find_free (&mut self, key: Keytype, req: Keytype) -> Option<usize> {
        //let bt = Backtrace::new();
        let (off_current, mut off_next) = (self.get_offset(key), self.get_offset(key+1));
        while off_next <= off_current {
            off_next += self.entries.len();
        }
        let fpos = self.entries.iter().cycle().skip(off_current).take(off_next - off_current).position(|e| e.is_none());
        if let Some(i) = fpos {
            let res = (i + off_current) % self.entries.len();
            return Some(res);
        } else if req == self.stride {
            self.rehash();
            return None;
        } else {
            let next_key = key+1;
            let want_free = self.get_offset(next_key) % self.entries.len();
            self.inc_offset(next_key);
            if let Some(next_free) = self.find_free(next_key, req+1) {
                dbg!(want_free); dbg!(next_free);
                self.entries.swap(want_free, next_free);
                return Some(want_free);
            } else {
                return None;
            }
        }
    }
    pub fn put(&mut self, key: Keytype, value: T) {
        if let Some(i) = self.find_pos(key) {
            self.entries[i].as_mut().unwrap().value = value;
        } else {
            loop {
                if let Some(idx) = self.find_free(key, 0) {
                    self.entries[idx] = Some(Entry::new(key, value));
                    break;
                }
            }
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
impl<T: PartialEq> _IntMap<T> {
    pub fn contains_value(&self, value: T) -> bool {
        let res = self.entries.iter().filter_map(|e| e.as_ref()).find(|e| e.value == value);
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

impl<T> Clone for IntMap<T> {
    fn clone(&self) -> Self {
        return IntMap { real_map: self.real_map.clone() }
    }
}



impl<T: PartialEq> IntMap<T> {
    pub fn contains_value(&self, value: T) -> bool {
        self.real_map.lock().unwrap().contains_value(value)
    }
}

#[cfg(test)]
mod tests;
