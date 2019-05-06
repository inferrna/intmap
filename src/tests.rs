use crate::IntMap;

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
