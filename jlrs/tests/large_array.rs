use jlrs::prelude::*;
use jlrs::util::JULIA;

#[test]
fn create_large_array() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.frame(1, |_, frame| {
            let array = Value::new_array::<f32, _, _>(frame, &[1, 1, 1, 1, 1, 1, 1, 1, 1][..]);
            assert!(array.is_ok());
            Ok(())
        })
        .unwrap();
    });
}

#[test]
fn move_large_array() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.frame(1, |_, frame| {
            let array = Value::move_array(frame, vec![1u64], &[1, 1, 1, 1, 1, 1, 1, 1, 1][..]);
            assert!(array.is_ok());
            Ok(())
        })
        .unwrap();
    });
}

#[test]
fn borrow_large_array() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.frame(1, |_, frame| {
            let mut data = vec![1u32];
            let array = Value::borrow_array(frame, &mut data, &[1, 1, 1, 1, 1, 1, 1, 1, 1][..]);
            assert!(array.is_ok());
            Ok(())
        })
        .unwrap();
    });
}

#[test]
fn create_large_array_requires_slot() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.frame(0, |_, frame| {
            let array = Value::new_array::<f32, _, _>(frame, &[1, 1, 1, 1, 1, 1, 1, 1, 1][..]);
            assert!(array.is_err());
            Ok(())
        })
        .unwrap();
    });
}

#[test]
fn move_large_array_requires_slot() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.frame(0, |_, frame| {
            let array = Value::move_array(frame, vec![1u64], &[1, 1, 1, 1, 1, 1, 1, 1, 1][..]);
            assert!(array.is_err());
            Ok(())
        })
        .unwrap();
    });
}

#[test]
fn borrow_large_array_requires_slot() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.frame(0, |_, frame| {
            let mut data = vec![1u32];
            let array = Value::borrow_array(frame, &mut data, &[1, 1, 1, 1, 1, 1, 1, 1, 1][..]);
            assert!(array.is_err());
            Ok(())
        })
        .unwrap();
    });
}
