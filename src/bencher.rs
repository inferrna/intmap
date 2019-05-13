extern crate rand;
#[macro_use]
extern crate criterion;
use criterion::Criterion;
use criterion::black_box;
use self::rand::Rng;
use intmap::{IntMap,Keytype};
use std::sync::Arc;

fn many_put(cnt: usize, stride: usize) {
    let mut hm = IntMap::<String>::new(stride);
    let mut rng = rand::thread_rng();
    for _ in 0..cnt {
        let k: Keytype = rng.gen();
        let s = format!("Street - {}", k);
        hm.put(k, s);
    }
}

fn many_new(stride: usize) {
    let mut hm = black_box(IntMap::<String>::new(stride));
}

fn many_get(hm: Arc<IntMap<String>>, cnt: usize ) {
    let mut rng = rand::thread_rng();
    for _ in 0..cnt {
        let k: Keytype = rng.gen_range(0, cnt);
        hm.get(k);
    }
}

fn bench_put(c: &mut Criterion) {
    for (cnt, stride) in vec![(100000usize, 2000usize), (100000, 1000), (100000, 200), (10000, 2000), (10000, 1000), (10000, 200)] {
        c.bench_function(format!("put {} values to {} chained map", cnt, stride).as_mut_str(), move |b| b.iter(|| many_put(black_box(cnt), black_box(stride))));
    }
}

fn bench_new(c: &mut Criterion) {
    for stride in vec![2000usize, 1000,  200] {
        c.bench_function(format!("create {} chained map", stride).as_mut_str(), move |b| b.iter(|| many_new(black_box(stride))));
    }
}

fn bench_get(c: &mut Criterion) {
    for (cnt, stride) in vec![(100000usize, 2000usize), (100000, 1000), (100000, 200), (10000, 2000), (10000, 1000), (10000, 200)] {
        let mut hm = IntMap::<String>::new(stride);
        let mut rng = rand::thread_rng();
        for _ in 0..cnt {
            let k: Keytype = rng.gen();
            let s = format!("Street - {}", k);
            hm.put(k, s);
        }
        let hma = Arc::new(hm);
        c.bench_function(format!("get from {} chained map filled with {} values", stride, cnt).as_mut_str(), move |b| b.iter(|| many_get(hma.clone(),black_box(cnt))));
    }
}

criterion_group!(benches, bench_put, bench_get, bench_new);
criterion_main!(benches);