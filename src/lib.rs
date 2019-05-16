use std::cmp::min;

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
    entries: Vec<Option<Entry<T>>>,
    offsets: Vec<usize>,
    stride: Keytype
}

impl<T: std::fmt::Debug> IntMap<T> {
    pub fn new (stride: Keytype) -> IntMap<T> {
        let offsets: Vec<usize> = (0..stride).map(|_| 0).collect();
        IntMap { entries: (0..stride*stride).map(|_| None).collect(),
                 offsets: offsets,
                  stride: stride}
    }
    //Get absolute offset for hash of the given key
    fn get_offset(&self, key: Keytype) -> usize {
        let hash = key % self.stride;
        (self.stride * hash) as usize + self.offsets[hash]
    }
    fn inc_offset(&mut self, key: Keytype) {
        let hash = key % self.stride;
        self.offsets[hash] += 1;
    }
    fn paired_offsets(&self, key: Keytype) -> (usize, usize) {
        let (off_current, mut off_next) = (self.get_offset(key), self.get_offset(key+1));
        while off_next <= off_current {
            off_next += self.entries.len();     //Compensate loop
        }
        return (off_current, off_next);
    }
    //Position relative to the known offset
    fn find_pos (&self, key: Keytype) -> Option<usize> {
        let (off_current, off_next) = self.paired_offsets(key);
        return self.entries.iter()
            .enumerate()
            .cycle()
            .skip(off_current)               //Known offset
            .take(off_next - off_current)    //Next offset as a border
            .filter(|(_i, e)| e.as_ref().is_some())
            .skip_while(|(_i, e)| e.as_ref().unwrap().key != key)
            .map(|(i, _e)| i)
            .next();
    }
    fn rehash(&mut self) {
        let filled = self.entries.iter().filter(|e| e.is_some()).count();
        let stride_inc = min(1, (self.stride * filled) / self.entries.len()); //Increment total size by ~filled
        let mut new_map = IntMap::<T>::new(self.stride+stride_inc);
        for o in &mut self.entries {
            if let Some(_e) = o {
                let es = o.take().unwrap();
                new_map.put(es.key, es.value);
            }
        }
        *self = new_map;
    }
    fn find_free (&mut self, key: Keytype, req: Keytype) -> Option<usize> {
        //let bt = Backtrace::new();
        let (off_current, off_next) = self.paired_offsets(key);
        let fpos = self.entries.iter().cycle().skip(off_current).take(off_next - off_current).position(|e| e.is_none());
        if let Some(i) = fpos {
            let res = (i + off_current) % self.entries.len();
            return Some(res);
        } else if req == self.stride {                          //Just checked all possible buckets
            self.rehash();
            return None;                                        //Needed to break recursion below after rehash
        } else {
            let next_key = key+1;
            let want_free = self.get_offset(next_key) % self.entries.len();
            self.inc_offset(next_key);
            if let Some(next_free) = self.find_free(next_key, req+1) {
                dbg!(want_free); dbg!(next_free);
                self.entries.swap(want_free, next_free);  //Move owner to head of its buckets. If head is already owned - repeat.
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
            let mut cnt = 0;
            loop { //Loop until free position found
                assert!(cnt < 3, "Cant't find free position for key {} after {} tries", key, cnt);
                if let Some(idx) = self.find_free(key, 0) {
                    self.entries[idx] = Some(Entry::new(key, value));
                    break;
                }
                cnt += 1;
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
            let res = &mut self.entries[i];
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
