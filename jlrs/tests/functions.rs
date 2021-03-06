use jlrs::prelude::*;
use jlrs::util::JULIA;

#[test]
fn call0() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.frame(1, |global, frame| {
            let func = Module::base(global).function("vect")?;
            func.call0(frame)?.unwrap();
            Ok(())
        })
        .unwrap();
    });
}

#[test]
fn call0_output() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.frame(1, |global, frame| {
            let output = frame.output()?;
            let func = Module::base(global).function("vect")?;
            func.with_output(output).call0(frame).unwrap();
            Ok(())
        })
        .unwrap();
    });
}

#[test]
fn call0_dynamic() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.dynamic_frame(|global, frame| {
            let func = Module::base(global).function("vect")?;
            func.call0(frame)?.unwrap();
            Ok(())
        })
        .unwrap();
    });
}

#[test]
fn call0_dynamic_output() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.dynamic_frame(|global, frame| {
            let output = frame.output()?;
            let func = Module::base(global).function("vect")?;
            func.with_output(output).call0(frame).unwrap();
            Ok(())
        })
        .unwrap();
    });
}

#[test]
fn call1() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.frame(2, |global, frame| {
            let func = Module::base(global).function("cos")?;
            let angle = Value::new(frame, std::f32::consts::PI)?;
            let out = func.call1(frame, angle)?.unwrap();
            out.cast::<f32>()
        });

        assert_eq!(out.unwrap(), -1.);
    });
}

#[test]
fn call1_output() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.frame(2, |global, frame| {
            let output = frame.output()?;
            let func = Module::base(global).function("cos")?;
            let angle = Value::new(frame, std::f32::consts::PI)?;
            let out = func.with_output(output).call1(frame, angle).unwrap();
            out.cast::<f32>()
        });

        assert_eq!(out.unwrap(), -1.);
    });
}

#[test]
fn call1_dynamic() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.dynamic_frame(|global, frame| {
            let func = Module::base(global).function("cos")?;
            let angle = Value::new(frame, std::f32::consts::PI)?;
            let out = func.call1(frame, angle)?.unwrap();
            out.cast::<f32>()
        });

        assert_eq!(out.unwrap(), -1.);
    });
}

#[test]
fn call1_dynamic_output() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.dynamic_frame(|global, frame| {
            let output = frame.output()?;
            let func = Module::base(global).function("cos")?;
            let angle = Value::new(frame, std::f32::consts::PI)?;
            let out = func.with_output(output).call1(frame, angle).unwrap();
            out.cast::<f32>()
        });

        assert_eq!(out.unwrap(), -1.);
    });
}

#[test]
fn call2() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.frame(3, |global, frame| {
            let func = Module::base(global).function("+")?;
            let arg0 = Value::new(frame, 1u32)?;
            let arg1 = Value::new(frame, 2u32)?;
            let out = func.call2(frame, arg0, arg1)?.unwrap();
            out.cast::<u32>()
        });

        assert_eq!(out.unwrap(), 3);
    });
}

#[test]
fn call2_output() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.frame(3, |global, frame| {
            let output = frame.output()?;
            let func = Module::base(global).function("+")?;
            let arg0 = Value::new(frame, 1u32)?;
            let arg1 = Value::new(frame, 2u32)?;
            let out = func.with_output(output).call2(frame, arg0, arg1).unwrap();
            out.cast::<u32>()
        });

        assert_eq!(out.unwrap(), 3);
    });
}

#[test]
fn call2_dynamic() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.dynamic_frame(|global, frame| {
            let func = Module::base(global).function("+")?;
            let arg0 = Value::new(frame, 1u32)?;
            let arg1 = Value::new(frame, 2u32)?;
            let out = func.call2(frame, arg0, arg1)?.unwrap();
            out.cast::<u32>()
        });

        assert_eq!(out.unwrap(), 3);
    });
}

#[test]
fn call2_dynamic_output() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.dynamic_frame(|global, frame| {
            let output = frame.output()?;
            let func = Module::base(global).function("+")?;
            let arg0 = Value::new(frame, 1u32)?;
            let arg1 = Value::new(frame, 2u32)?;
            let out = func.with_output(output).call2(frame, arg0, arg1).unwrap();
            out.cast::<u32>()
        });

        assert_eq!(out.unwrap(), 3);
    });
}

#[test]
fn call3() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.frame(4, |global, frame| {
            let func = Module::base(global).function("+")?;
            let arg0 = Value::new(frame, 1u32)?;
            let arg1 = Value::new(frame, 2u32)?;
            let arg2 = Value::new(frame, 3u32)?;
            let out = func.call3(frame, arg0, arg1, arg2)?.unwrap();
            out.cast::<u32>()
        });

        assert_eq!(out.unwrap(), 6);
    });
}

#[test]
fn call3_output() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.frame(4, |global, frame| {
            let output = frame.output()?;
            let func = Module::base(global).function("+")?;
            let arg0 = Value::new(frame, 1u32)?;
            let arg1 = Value::new(frame, 2u32)?;
            let arg2 = Value::new(frame, 3u32)?;
            let out = func
                .with_output(output)
                .call3(frame, arg0, arg1, arg2)
                .unwrap();
            out.cast::<u32>()
        });

        assert_eq!(out.unwrap(), 6);
    });
}

#[test]
fn call3_dynamic() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.dynamic_frame(|global, frame| {
            let func = Module::base(global).function("+")?;
            let arg0 = Value::new(frame, 1u32)?;
            let arg1 = Value::new(frame, 2u32)?;
            let arg2 = Value::new(frame, 3u32)?;
            let out = func.call3(frame, arg0, arg1, arg2)?.unwrap();
            out.cast::<u32>()
        });

        assert_eq!(out.unwrap(), 6);
    });
}

#[test]
fn call3_dynamic_output() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.dynamic_frame(|global, frame| {
            let output = frame.output()?;
            let func = Module::base(global).function("+")?;
            let arg0 = Value::new(frame, 1u32)?;
            let arg1 = Value::new(frame, 2u32)?;
            let arg2 = Value::new(frame, 3u32)?;
            let out = func
                .with_output(output)
                .call3(frame, arg0, arg1, arg2)
                .unwrap();
            out.cast::<u32>()
        });

        assert_eq!(out.unwrap(), 6);
    });
}

#[test]
fn call() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.frame(5, |global, frame| {
            let func = Module::base(global).function("+")?;
            let arg0 = Value::new(frame, 1u32)?;
            let arg1 = Value::new(frame, 2u32)?;
            let arg2 = Value::new(frame, 3u32)?;
            let arg3 = Value::new(frame, 4u32)?;
            let out = func.call(frame, &mut [arg0, arg1, arg2, arg3])?.unwrap();
            out.cast::<u32>()
        });

        assert_eq!(out.unwrap(), 10);
    });
}

#[test]
fn call_output() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.frame(5, |global, frame| {
            let output = frame.output()?;
            let func = Module::base(global).function("+")?;
            let arg0 = Value::new(frame, 1u32)?;
            let arg1 = Value::new(frame, 2u32)?;
            let arg2 = Value::new(frame, 3u32)?;
            let arg3 = Value::new(frame, 4u32)?;
            func.with_output(output)
                .call(frame, &mut [arg0, arg1, arg2, arg3])
                .unwrap()
                .cast::<u32>()
        });

        assert_eq!(out.unwrap(), 10);
    });
}

#[test]
fn call_dynamic() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.dynamic_frame(|global, frame| {
            let func = Module::base(global).function("+")?;
            let arg0 = Value::new(frame, 1u32)?;
            let arg1 = Value::new(frame, 2u32)?;
            let arg2 = Value::new(frame, 3u32)?;
            let arg3 = Value::new(frame, 4u32)?;
            let out = func.call(frame, &mut [arg0, arg1, arg2, arg3])?.unwrap();
            out.cast::<u32>()
        });

        assert_eq!(out.unwrap(), 10);
    });
}

#[test]
fn call_dynamic_output() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.dynamic_frame(|global, frame| {
            let output = frame.output()?;
            let func = Module::base(global).function("+")?;
            let arg0 = Value::new(frame, 1u32)?;
            let arg1 = Value::new(frame, 2u32)?;
            let arg2 = Value::new(frame, 3u32)?;
            let arg3 = Value::new(frame, 4u32)?;
            let out = func
                .with_output(output)
                .call(frame, &mut [arg0, arg1, arg2, arg3])
                .unwrap();
            out.cast::<u32>()
        });

        assert_eq!(out.unwrap(), 10);
    });
}

#[test]
fn call_values() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.frame(5, |global, frame| {
            let func = Module::base(global).function("+")?;
            let args = Values::new(frame, [1u32, 2u32, 3u32, 4u32])?;
            let out = func.call_values(frame, args)?.unwrap();
            out.cast::<u32>()
        });

        assert_eq!(out.unwrap(), 10);
    });
}

#[test]
fn call_values_output() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.frame(5, |global, frame| {
            let output = frame.output()?;
            let func = Module::base(global).function("+")?;
            let args = Values::new(frame, [1u32, 2u32, 3u32, 4u32])?;
            let out = func.with_output(output).call_values(frame, args).unwrap();
            out.cast::<u32>()
        });

        assert_eq!(out.unwrap(), 10);
    });
}

#[test]
fn call_values_dynamic() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.dynamic_frame(|global, frame| {
            let func = Module::base(global).function("+")?;
            let args = Values::new(frame, [1u32, 2u32, 3u32, 4u32])?;
            let out = func.call_values(frame, args)?.unwrap();
            out.cast::<u32>()
        });

        assert_eq!(out.unwrap(), 10);
    });
}

#[test]
fn call_values_dynamic_output() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        let out = jlrs.dynamic_frame(|global, frame| {
            let output = frame.output()?;
            let func = Module::base(global).function("+")?;
            let args = Values::new(frame, [1u32, 2u32, 3u32, 4u32])?;
            let out = func.with_output(output).call_values(frame, args).unwrap();
            out.cast::<u32>()
        });

        assert_eq!(out.unwrap(), 10);
    });
}

#[test]
fn jlrs_extensions_available() {
    JULIA.with(|j| {
        let mut jlrs = j.borrow_mut();

        jlrs.dynamic_frame(|global, frame| {
            let func = Module::base(global).function("+")?;
            assert!(func.attach_stacktrace(frame).is_ok());
            assert!(func.tracing_call(frame).is_ok());

            let o1 = frame.output()?;
            let o2 = frame.output()?;
            assert!(func.with_output(o1).attach_stacktrace(frame).is_ok());
            assert!(func.with_output(o2).tracing_call(frame).is_ok());

            Ok(())
        })
        .unwrap();
    });
}
