use crate::IntMap;
use std::thread;
use std::sync::{Arc, Mutex};
use rand::Rng;


#[test]
fn put_and_get() {
    let mut hm = IntMap::<String>::new(8);
    hm.put(99, "Alley".to_string());
    hm.put(73, "Street".to_string());
    assert_eq!(hm.get(99).unwrap(), &"Alley".to_string());
    assert_eq!(hm.get(73).unwrap(), &"Street".to_string());
}

#[test]
fn remove() {
    let mut hm = IntMap::<String>::new(8);
    hm.put(99, "Alley".to_string());
    hm.put(73, "Street".to_string());
    assert_eq!(hm.remove(99).unwrap(), "Alley".to_string());
    assert_eq!(hm.remove(73).unwrap(), "Street".to_string());
    assert_eq!(hm.get(99), None);
    assert_eq!(hm.remove(73), None);
}

#[test]
fn rehash() {
    let mut hm = IntMap::<String>::new(2);
    let mut vals = vec![];
    let mut rng = rand::thread_rng();
    for i in 0..1555 {
        let k = rng.gen_range(0, 99999);
        let v = format!("Street - {}", k);
        vals.push((k, v.clone()));
        hm.put(k, v);
    }
    for (i, (e, v)) in vals.iter().enumerate() {
        assert_eq!(Some(v), hm.get(*e), "at iteration {}", i);
    }
    //panic!("I want to believe.")
}

#[test]
fn not_exists() {
    let mut hm = IntMap::<String>::new(8);
    hm.put(99, "Alley".to_string());
    hm.put(73, "Street".to_string());
    assert_eq!(hm.get(88), None);
    assert_eq!(hm.remove(72), None);
}

#[test]
fn contains_value() {
    let mut hm = IntMap::<String>::new(8);
    hm.put(99, "Alley".to_string());
    hm.put(73, "Street".to_string());
    assert_eq!(hm.contains_value("Alley".to_string()), true);
    assert_eq!(hm.contains_value("Streets".to_string()), false);
}

#[test]
fn contains_key() {
    let mut hm = IntMap::<String>::new(8);
    hm.put(99, "Alley".to_string());
    hm.put(73, "Street".to_string());
    assert_eq!(hm.contains_key(73), true);
    assert_eq!(hm.contains_key(88), false);
}

#[test]
fn thread_get_string() {
    let mut hm = IntMap::<String>::new(8);
    let counter = Arc::new(Mutex::new(0));
    hm.put(99, "Alley".to_string());
    hm.put(73, "Street".to_string());
    let hma = Arc::new(hm);
    let mut handles = vec![];
    for ev in &[(99, "Alley"), (73, "Street")] {
        let hm = hma.clone();
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let val = hm.get(ev.0).unwrap();
            let mut num = counter.lock().unwrap();
            *num += (val == ev.1 ) as usize;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(*counter.lock().unwrap(), 2);
}

#[test]
fn thread_get_i64() {
    let mut hm = IntMap::<i64>::new(8);
    let counter = Arc::new(Mutex::new(0));
    hm.put(99, 999);
    hm.put(73, -888);
    let hma = Arc::new(hm);
    let mut handles = vec![];
    for ev in &[(99, 999i64), (73, -888)] {
        let hm = hma.clone();
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let val = hm.get(ev.0).unwrap();
            let mut num = counter.lock().unwrap();
            *num += (*val == ev.1 ) as usize;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(*counter.lock().unwrap(), 2);
}

#[test]
fn thread_put() {
    let hm = Arc::new(Mutex::new(IntMap::<String>::new(8)));
    let mut handles = vec![];
    for ev in &[(99, "Alley"), (73, "Street")] {
        let hm = Arc::clone(&hm);
        let handle = thread::spawn(move || {
            hm.lock().unwrap().put(ev.0, ev.1.to_string());
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let mut hma = hm.lock().unwrap();
    assert_eq!(hma.get(99).unwrap(), &"Alley".to_string());
    assert_eq!(hma.remove(73).unwrap(), "Street".to_string());
}