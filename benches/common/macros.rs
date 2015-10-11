#![macro_use]

macro_rules! bench_binop(
    ($name: ident, $t1: ty, $t2: ty, $binop: ident) => {
        #[bench]
        fn $name(bh: &mut Bencher) {
            const LEN: usize = 1 << 13;

            let mut rng = IsaacRng::new_unseeded();

            let elems1: Vec<$t1> = (0usize .. LEN).map(|_| rng.gen::<$t1>()).collect();
            let elems2: Vec<$t2> = (0usize .. LEN).map(|_| rng.gen::<$t2>()).collect();
            let mut i = 0;

            bh.iter(|| {
                i = (i + 1) & (LEN - 1);

                unsafe {
                    test::black_box(elems1.get_unchecked(i).$binop(*elems2.get_unchecked(i)))
                }
            })
        }
    }
);
