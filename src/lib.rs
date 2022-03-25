#![feature(test)]

extern crate test;

mod generated;

// running 2 tests
// test bench_builder ... bench:         115 ns/iter (+/- 3)
// test bench_manual  ... bench:          24 ns/iter (+/- 1)

#[bench]
fn bench_builder(b: &mut test::Bencher) {
    use generated::*;
    use molecule::prelude::*;

    let s = MyStruct::default();
    b.iter(|| {
        MyStruct2Builder::default()
            .s1(s.clone())
            .s2(s.clone())
            .s3(s.clone())
            .build()
    })
}

#[bench]
fn bench_manual(b: &mut test::Bencher) {
    use molecule::bytes::{BufMut, BytesMut};

    let a = [0u8; 96];
    b.iter(|| {
        let mut ret = BytesMut::with_capacity(288);
        ret.put(&a[..]);
        ret.put(&a[..]);
        ret.put(&a[..]);
        ret.freeze()
    })
}
