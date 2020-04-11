//! The main goal behind `jlrs` is to provide a simple and safe interface to the Julia C API.
//! Currently this crate has only been tested on Linux, if you try to use it on another OS it will
//! likely fail to generate the bindings to Julia. This crate is currently tested with Julia 
//! v1.4.0.
//!
//! # Generating the bindings
//! This crate depends on `jl-sys` which contains the raw bindings to the Julia C API, these are
//! generated by `bindgen`. The recommended way to install Julia is to download the binaries from
//! the official website, which is distributed in an archive containing a directory called
//! `julia-x.y.z`. This directory contains several other directories, including a `bin` directory
//! containing the `julia` executable.
//!
//! In order to ensure the `julia.h` header file can be found, you have to set the `JL_PATH`
//! environment variable to `/path/to/julia-x.y.z`. Similarly, in order to load `libjulia.so` you
//! must add `/path/to/julia-x.y.z/lib` to the `LD_LIBRARY_PATH` environment variable. If they can
//! be found at the standard locations, e.g. because you've installed Julia through your package
//! manager, this is not necessary and things should build without setting the `JL_PATH`
//! environment variable.
//!
//! # Using this crate
//! The first thing you should do is `use` the [`prelude`]-module with an asterisk, this will
//! bring all the structs and traits you're likely to need in scope. Before you can use Julia it
//! must first be initialized. You do this by calling [`Julia::init`], this method forces you to 
//! pick a `stack size`. You will learn how to choose this value soon. Note that this method can 
//! only be called once, if you drop [`Julia`] you won't be able to create a new one and have to 
//! restart the entire program.
//!
//! You can call [`Julia::include`] to include your own Julia code and either [`Julia::frame`] or 
//! [`Julia::dynamic_frame`] to interact with Julia. If you want to create arrays with more than 
//! three dimensions or borrow arrays with more than one, `jlrs.jl` must be icluded. You can find 
//! this file in the root of this crate's github repository. This is necessary because this 
//! functionality currently depends on some Julia code defined in that file.
//!
//! The other two methods, [`Julia::frame`] and [`Julia::dynamic_frame`], take a closure that
//! provides you with a [`StaticFrame`] and a [`DynamicFrame`] respectively. Both types implement
//! the [`Frame`] trait. These frames are used to create new values, access Julia modules and
//! their functions, call functions, and copy data back to Rust. Additionally, frames can be
//! nested; you're free to mix static and dynamic frames. The main reason things work this way is
//! that it ensures that all active values are protected from being freed by Julia's garbage
//! collector. Each frame takes at least two slots on the stack whose size was chosen when you
//! initialized Julia, plus an additional one for each value you create and function you call. A
//! [`StaticFrame`] forces you to choose the number of slots that will be available, while a
//! [`DynamicFrame`] grows dynamically. The slots that were used are reclaimed when the frame goes
//! out of scope.
//!
//! In order to call a Julia function, you'll need two things: a function to call, and arguments
//! to call it with. You can acquire the function through the module that defines it with
//! [`Module::function`]; [`Module::base`] and [`Module::core`] provide access to Julia's `Base`
//! and `Core` module respectively, while everything you include through [`Julia::include`] is
//! made available relative to the `Main` module which you can access by calling [`Module::main`].
//!
//! Most Julia data is represented by a [`Value`]. Basic data types like numbers, booleans, and
//! strings can be created through [`Value::new`] and several methods exist to create an
//! n-dimensional array. Julia functions, their arguments and their results are all `Value`s. All
//! `Value`s can be called as functions, whether this will succeed depends on the value actually
//! being a function. You can copy data from Julia to Rust by calling [`Value::try_unbox`].
//!
//! As a simple example, let's create two values and add them:
//!
//! ```no_run
//! # use jlrs::prelude::*;
//! # fn main() {
//! let mut julia = unsafe { Julia::init(16).unwrap() };
//! julia.dynamic_frame(|frame| {
//!     // Create the two arguments
//!     let i = Value::new(frame, 2u64)?;
//!     let j = Value::new(frame, 1u32)?;
//!
//!     // We can find the addition-function in the base module
//!     let func = Module::base(frame).function("+")?;
//!
//!     // Call the function and unbox the result
//!     let output = func.call2(frame, i, j)?;
//!     output.try_unbox::<u64>()
//! }).unwrap();
//! # }
//! ```
//!
//! You can also do this with a static frame:
//! 
//! ```no_run
//! # use jlrs::prelude::*;
//! # fn main() {
//! let mut julia = unsafe { Julia::init(16).unwrap() };
//! // Three slots; two for the inputs and one for the output.
//! julia.frame(3, |frame| {
//!     // Create the two arguments
//!     let i = Value::new(frame, 2u64)?;
//!     let j = Value::new(frame, 1u32)?;
//!
//!     // We can find the addition-function in the base module
//!     let func = Module::base(frame).function("+")?;
//!
//!     // Call the function and unbox the result
//!     let output = func.call2(frame, i, j)?;
//!     output.try_unbox::<u64>()
//! }).unwrap();
//! # }
//! ```
//!
//! For more examples, you can take a look at this crate's integration tests.
//! 
//! # Lifetimes
//! While reading the documentation for this crate, you will see that a lot of lifetimes are used.
//! Most of these lifetimes have a specific meaning:
//! 
//! - `'base` is the lifetime of a frame created through [`Julia::frame`] or 
//! [`Julia::static_frame`]. This lifetime prevents you from using Julia data outside of a frame,
//! even if that data is a global value in Julia.
//! 
//! - `'frame` is the lifetime of an arbitrary frame; in the base frame it will be the same as 
//! `'base`. This lifetime prevents you from using Julia data after the frame that protects it from
//! garbage collection goes out of scope.
//! 
//! - `'data` is the lifetime of the data that Julia borrows from Rust. This lifetime prevents you
//! from mutably aliasing data and trying to use it after the borrowed data is dropped.
//! 
//! - `'output` is the lifetime of the frame that created the output. This lifetime ensures that 
//! when Julia data is protected by an older frame this data can be used until that frame goes out
//! of scope.
//! 
//! # Limitations
//! Calling Julia is entirely single-threaded. You won't be able to use [`Julia`] from
//! another thread and while Julia is doing stuff you won't be able to interact with it.
//!
//! [`prelude`]: prelude/index.html
//! [`Julia`]: struct.Julia.html
//! [`Julia::init`]: struct.Julia.html#method.init
//! [`Julia::include`]: struct.Julia.html#method.include
//! [`Julia::frame`]: struct.Julia.html#method.frame
//! [`Julia::dynamic_frame`]: struct.Julia.html#method.dynamic_frame
//! [`StaticFrame`]: frame/struct.StaticFrame.html
//! [`DynamicFrame`]: frame/struct.DynamicFrame.html
//! [`Frame`]: traits/trait.Frame.html
//! [`Module::function`]: module/struct.Module.html#method.function
//! [`Module::base`]: module/struct.Module.html#method.base
//! [`Module::core`]: module/struct.Module.html#method.core
//! [`Module::main`]: module/struct.Module.html#method.main
//! [`Value`]: value/struct.Value.html
//! [`Value::new`]: value/struct.Value.html#method.new
//! [`Value::try_unbox`]: value/struct.Value.html#method.try_unbox

pub mod array;
pub mod error;
pub mod frame;
mod stack;
pub mod module;
pub mod prelude;
pub mod traits;
pub mod value;

use error::{JlrsError, JlrsResult};
use frame::{DynamicFrame, StaticFrame};
use jl_sys::{jl_atexit_hook, jl_init};
use stack::{Dynamic, RawStack, StackView, Static};
use module::Module;
use std::mem::ManuallyDrop;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use value::Value;

static INIT: AtomicBool = AtomicBool::new(false);

/// This struct can be created only once during the lifetime of your program. You
/// must create it with [`Julia::init`] before you can do anything related to Julia.
///
/// [`Julia::init`]: struct.Julia.html#method.init
pub struct Julia {
    stack: RawStack,
}

impl Julia {
    /// Initializes Julia, this function can only be called once. If you call it a second time it
    /// will return an error. If this struct is dropped, you will need to restart your program to
    /// be able to call Julia code again.
    ///
    /// You have to choose a stack size when calling this function. This will be the total number
    /// of slots that will be available on the GC stack. One of these slots will alwas be in use.
    ///
    /// This function is unsafe because this crate provides you with a way to execute arbitrary
    /// Julia code which can't be checked for correctness. 
    #[cfg_attr(tarpaulin, skip)]
    pub unsafe fn init(stack_size: usize) -> JlrsResult<Self> {
        if INIT.swap(true, Ordering::SeqCst) {
            return Err(JlrsError::AlreadyInitialized.into());
        }

        jl_init();

        Ok(Julia {
            stack: RawStack::new(stack_size),
        })
    }

    /// Change the stack size to `stack_size`.
    pub fn set_stack_size(&mut self, stack_size: usize) {
        unsafe { self.stack = RawStack::new(stack_size) }
    }

    /// Returns the current stack size.
    pub fn stack_size(&self) -> usize {
        self.stack.size()
    }

    #[doc(hidden)]
    // DO NOT USE THIS. It's an awful, memory-leaking, workaround to make integration testing
    // easier
    pub unsafe fn testing_instance() -> ManuallyDrop<Julia> {
        let mut rt = ManuallyDrop::new(Julia {
            stack: RawStack::new(32),
        });

        if INIT.swap(true, Ordering::SeqCst) {
            return rt;
        }

        jl_init();
        rt.include("jlrs.jl").unwrap();
        rt
    }

    /// Calls `include` in the `Main` module in Julia, which executes the file's contents in that
    /// module. This has the same effect as calling `include` in the Julia REPL.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use jlrs::prelude::*;
    /// # fn main() {
    /// let mut julia = unsafe { Julia::init(16).unwrap() };
    /// julia.include("jlrs.jl").unwrap();
    /// # }
    /// ```
    pub fn include<P: AsRef<Path>>(&mut self, path: P) -> JlrsResult<()> {
        if path.as_ref().exists() {
            return self.frame(3, |frame| {
                let path_jl_str = Value::new(frame, path.as_ref().to_string_lossy())?;
                let include_func = Module::main(frame).function("include")?;
                let _ = include_func.call1(frame, path_jl_str)?;
                Ok(())
            });
        }

        Err(JlrsError::IncludeNotFound(path.as_ref().to_string_lossy().into()).into())
    }

    /// Create a [`StaticFrame`] that can hold `capacity` values, and call the given closure.
    /// Returns the result of this closure, or an error if the new frame can't be created because
    /// there's not enough space on the GC stack. The number of required slots on the stack is
    /// `capacity + 2`.
    ///
    /// Every output and value you create inside the closure using the [`StaticFrame`], either
    /// directly or through calling a [`Value`], will reduce the available capacity of the
    /// [`StaticFrame`] by 1.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use jlrs::prelude::*;
    /// # fn main() {
    /// # let mut julia = unsafe { Julia::init(16).unwrap() };
    /// julia.frame(2, |frame| {
    ///     let _i = Value::new(frame, 2u64)?;
    ///     let _j = Value::new(frame, 1u32)?;
    ///     Ok(())
    /// }).unwrap();
    /// # }
    /// ```
    ///
    /// [`StaticFrame`]: ../frame/struct.StaticFrame.html
    /// [`Value`]: ../value/struct.Value.html
    pub fn frame<T, F>(&mut self, capacity: usize, func: F) -> JlrsResult<T>
    where
        F: FnOnce(&mut StaticFrame<'_, '_>) -> JlrsResult<T>,
    {
        unsafe {
            let mut view = StackView::<Static>::new(self.stack.as_mut());
            let frame_idx = view.new_frame(capacity)?;
            let mut scope = frame::Scope;
            let mut frame = StaticFrame::with_capacity(frame_idx, capacity, view, &mut scope);
            func(&mut frame)
        }
    }

    /// Create a [`DynamicFrame`] and call the given closure. Returns the result of this closure,
    /// or an error if the new frame can't be created because the stack is too small. The number
    /// of required slots on the stack is 2.
    ///
    /// Every output and value you create inside the closure using the [`DynamicFrame`], either
    /// directly or through calling a [`Value`], will occupy a single slot on the GC stack.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use jlrs::prelude::*;
    /// # fn main() {
    /// # let mut julia = unsafe { Julia::init(16).unwrap() };
    /// julia.dynamic_frame(|frame| {
    ///     let _i = Value::new(frame, 2u64)?;
    ///     let _j = Value::new(frame, 1u32)?;
    ///     Ok(())
    /// }).unwrap();
    /// # }
    /// ```
    ///
    /// [`DynamicFrame`]: ../frame/struct.DynamicFrame.html
    /// [`Value`]: ../value/struct.Value.html
    pub fn dynamic_frame<T, F>(&mut self, func: F) -> JlrsResult<T>
    where
        F: FnOnce(&mut DynamicFrame<'_, '_>) -> JlrsResult<T>,
    {
        unsafe {
            let mut view = StackView::<Dynamic>::new(self.stack.as_mut());
            let frame_idx = view.new_frame()?;
            let mut scope = frame::Scope;
            let mut frame = DynamicFrame::new(frame_idx, view, &mut scope);
            func(&mut frame)
        }
    }
}

impl Drop for Julia {
    #[cfg_attr(tarpaulin, skip)]
    fn drop(&mut self) {
        unsafe {
            jl_atexit_hook(0);
        }
    }
}
