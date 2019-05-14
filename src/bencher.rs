extern crate rand;
#[macro_use]
extern crate criterion;
use criterion::Criterion;
use criterion::black_box;
use self::rand::Rng;
use intmap::{IntMap,Keytype};
use std::sync::Arc;

fn many_put_string(cnt: usize, stride: usize) {
    let mut hm = IntMap::<String>::new(stride);
    let mut rng = rand::thread_rng();
    for _ in 0..cnt {
        let k: Keytype = rng.gen();
        let s = format!("Street - {}", k);
        hm.put(k, s);
    }
}

fn many_put_i32(cnt: usize, stride: usize) {
    let mut hm = IntMap::<i32>::new(stride);
    let mut rng = rand::thread_rng();
    for i in 0..cnt {
        let k: Keytype = rng.gen();
        hm.put(k, i as i32);
    }
}

fn many_new<T: std::fmt::Debug>(stride: usize) {
    let hm = black_box(IntMap::<T>::new(stride));
}

fn many_get<T>(hm: Arc<IntMap<T>>, cnt: usize ) where T: std::fmt::Debug + Clone {
    let mut rng = rand::thread_rng();
    for _ in 0..cnt {
        let k: Keytype = rng.gen_range(0, cnt);
        hm.get(k);
    }
}

fn bench_put_i32(c: &mut Criterion) {
    for (cnt, stride) in vec![(100000usize, 2000usize), (100000, 1000), (100000, 200), (10000, 2000), (10000, 1000), (10000, 200)] {
        c.bench_function(format!("i32 put {} values to {} chained map", cnt, stride).as_mut_str(), move |b| b.iter(|| many_put_i32(black_box(cnt), black_box(stride))));
    }
}
fn bench_put_string(c: &mut Criterion) {
    for (cnt, stride) in vec![(100000usize, 2000usize), (100000, 1000), (100000, 200), (10000, 2000), (10000, 1000), (10000, 200)] {
        c.bench_function(format!("str put {} values to {} chained map", cnt, stride).as_mut_str(), move |b| b.iter(|| many_put_string(black_box(cnt), black_box(stride))));
    }
}

fn bench_new_i32(c: &mut Criterion) {
    for stride in vec![2000usize, 1000,  200] {
        c.bench_function(format!("i32 create {} chained map", stride).as_mut_str(), move |b| b.iter(|| many_new::<String>(black_box(stride))));
    }
}
fn bench_new_string(c: &mut Criterion) {
    for stride in vec![2000usize, 1000,  200] {
        c.bench_function(format!("str create {} chained map", stride).as_mut_str(), move |b| b.iter(|| many_new::<i32>(black_box(stride))));
    }
}

fn bench_get_i32(c: &mut Criterion) {
    for (cnt, stride) in vec![(100000usize, 2000usize), (100000, 1000), (100000, 200), (10000, 2000), (10000, 1000), (10000, 200)] {
        let mut hm = IntMap::<i32>::new(stride);
        let mut rng = rand::thread_rng();
        for i in 0..cnt {
            let k: Keytype = rng.gen();
            hm.put(k, i as i32);
        }
        let hma = Arc::new(hm);
        c.bench_function(format!("i32 get from {} chained map filled with {} values", stride, cnt).as_mut_str(), move |b| b.iter(|| many_get::<i32>(hma.clone(),black_box(cnt))));
    }
}
fn bench_get_string(c: &mut Criterion) {
    for (cnt, stride) in vec![(100000usize, 2000usize), (100000, 1000), (100000, 200), (10000, 2000), (10000, 1000), (10000, 200)] {
        let mut hm = IntMap::<String>::new(stride);
        let mut rng = rand::thread_rng();
        for _ in 0..cnt {
            let k: Keytype = rng.gen();
            let s = format!("Street - {}", k);
            hm.put(k, s);
        }
        let hma = Arc::new(hm);
        c.bench_function(format!("str get from {} chained map filled with {} values", stride, cnt).as_mut_str(), move |b| b.iter(|| many_get::<String>(hma.clone(),black_box(cnt))));
    }
}


criterion_group!(benches, bench_new_i32, bench_new_string, bench_put_i32, bench_put_string, bench_get_i32, bench_get_string);
criterion_main!(benches);