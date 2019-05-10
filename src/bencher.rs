extern crate rand;
#[macro_use]
extern crate criterion;
use criterion::Criterion;
use criterion::black_box;
use self::rand::Rng;
use intmap::{IntMap,Keytype};

fn many_put(cnt: usize, stride: usize) {
    let mut hm = IntMap::<String>::new(stride);
    let mut rng = rand::thread_rng();
    for _ in 0..cnt {
        let k: Keytype = rng.gen();
        let s = format!("Street - {}", k);
        hm.put(k, s);
    }
}

fn bench_put(c: &mut Criterion) {
    for (cnt, stride) in vec![(100000usize, 2000usize), (100000, 200), (100000, 20), (10000, 2000), (10000, 200), (10000, 20)] {
        c.bench_function(format!("put {} {}", cnt, stride).as_mut_str(), move |b| b.iter(|| many_put(black_box(cnt), black_box(stride))));
    }
}

criterion_group!(benches, bench_put);
criterion_main!(benches);