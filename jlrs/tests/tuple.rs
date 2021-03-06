use jlrs::prelude::*;
use jlrs::util::JULIA;

#[test]
fn create_cast_tuple0() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();
        jlrs.frame(1, |_global, frame| {
            let t0 = Tuple0();
            let v = Value::new(frame, t0)?;
            assert!(v.is::<Tuple0>());
            assert!(v.cast::<Tuple0>().is_ok());
            Ok(())
        })
        .unwrap();
    })
}

#[test]
fn create_cast_tuple1() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();
        jlrs.frame(1, |_global, frame| {
            let t1 = Tuple1(1u64);
            let v = Value::new(frame, t1)?;
            assert!(v.is::<Tuple1<u64>>());
            assert!(v.cast::<Tuple1<u64>>().is_ok());
            Ok(())
        })
        .unwrap();
    })
}

#[test]
fn create_cast_tuple2() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();
        jlrs.frame(1, |_global, frame| {
            let t2 = Tuple2(1u64, -3i32);
            let v = Value::new(frame, t2)?;
            assert!(v.is::<Tuple2<u64, i32>>());
            assert!(v.cast::<Tuple2<u64, i32>>().is_ok());
            Ok(())
        })
        .unwrap();
    })
}
