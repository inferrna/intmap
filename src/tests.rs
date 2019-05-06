use crate::{IntMap, Entry};
use std::thread;
use std::sync::{Arc, Mutex};
use core::borrow::Borrow;

#[test]
fn create_and_put() {
    let mut hm = IntMap::<String>::new(8);
    hm.put(99, "Alley".to_string());
    hm.put(73, "Street".to_string());
    assert_eq!(hm.get(99).unwrap(), &"Alley".to_string());
    assert_eq!(hm.remove(73).unwrap(), "Street".to_string());
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
fn contains() {
    let mut hm = IntMap::<String>::new(8);
    hm.put(99, "Alley".to_string());
    hm.put(73, "Street".to_string());
    assert_eq!(hm.contains_value("Alley".to_string()), true);
    assert_eq!(hm.contains_value("Streets".to_string()), false);
    assert_eq!(hm.contains_key(73), true);
    assert_eq!(hm.contains_key(88), false);
}

#[test]
fn thread_get() {
    let mut hm = Arc::new(Mutex::new(IntMap::<String>::new(8)));
    let mut counter = Arc::new(Mutex::new(0));
    hm.lock().unwrap().put(99, "Alley".to_string());
    hm.lock().unwrap().put(73, "Street".to_string());
    let mut handles = vec![];
    for ev in &[(99, "Alley"), (73, "Street")] {
        let hm = Arc::clone(&hm);
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let val = hm.lock().unwrap().get(ev.0).unwrap().to_owned();
            let mut num = counter.lock().unwrap();
            *num += if val == ev.1.to_string() {1} else {0};
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
    let mut hm = Arc::new(Mutex::new(IntMap::<String>::new(8)));
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