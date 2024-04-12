use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, Mutex},
};

use bumpalo::Bump;

fn main() {
    const_default(2);
    striiiiings();
    stack_vs_heap();
    marker_traits();
    hygienic_macros();
}

// Rust propaganda talk
// Agenda
// 1. const by default
// 2. string types
// 3. stack vs. heap and why it matters
// 4. Marker traits (Copy, Clone, Send, Sync)
// 5. hygienic macros

fn const_default(needle: u32) -> bool {
    let haystack = vec![1, 2, 3, 4, 5];
    let mut found = false;
    for i in haystack {
        if i == needle {
            found = true;
            break;
        }
    }

    found
}

// https://i.redd.it/jsh156txovp91.png
fn striiiiings() {
    // This is an owned string
    let mut owned = String::new();

    // So is this
    let owned2 = "owned string".to_owned();

    let str = "This is a string slice, aka &'static str";

    let path: PathBuf = "/this/is/a/path".to_string().into();

    let cstr = c"This is a nul-terminated string";

    // You can get the inner bytes for all of them
    owned.as_bytes();
    str.as_bytes();
    path.as_os_str().as_encoded_bytes();
    cstr.to_bytes();

    // You can only change strings you own
    owned.push('A');

    match str {
        "not matching" => {}
        _ => {}
    }

    // You need to convert to slice to compare/match against an owned string
    match owned.as_str() {
        "A" => {
            println!("yay")
        }
        _ => (),
    }
}

// https://en.wikipedia.org/wiki/Stack-based_memory_allocation#/media/File:ProgramCallStack2_en.svg
// fn stack_vs_heap<'a>() -> &'a mut i32 {
fn stack_vs_heap() {
    let stack_value = 42;
    let heap_value = Box::new(42);
    let rc_value = Rc::new(42);
    let arc_value = Arc::new(42);
    let arc_mut_value = Arc::new(Mutex::new(42));
    let arena = Bump::new();
    let arena_value = arena.alloc(42);

    // drop(arena);

    let handle = {
        let arc_value = arc_value.clone();
        let arc_mut_value = arc_mut_value.clone();
        std::thread::spawn(move || {
            let res = stack_value;
            // let res = heap_value;
            // let res = rc_value;
            let res = arc_value;
            // *arc_value = 43;
            // let res = arc_mut_value;
            *arc_mut_value.lock().unwrap() = 43;
            // *arena_value = 43;
        })
    };

    handle.join().unwrap();

    println!(
        "stack_vs_heap: {stack_value} {heap_value} {rc_value} {arc_value} {arc_mut_value} {arena_value}",
        arc_mut_value = arc_mut_value.lock().unwrap()
    );

    // arena_value
}

/// Notable marker traits:
/// Copy: small types that can be copied with a simple memcpy (think integers, bools, small enums, small structs)
/// Clone: values that can be copied, but are expensive or requires extra code to copy
///
/// And now the hard ones:
/// Send: A type is Send if it is safe to send it to another thread.
/// Sync: A type is Sync if it is safe to share between threads (T is Sync if and only if &T is Send).
///
/// Other useful traits from the standard library
/// Display: values that can be printed in format strings: "{value}"
/// Debug: values that can be debug printed in format strings: "{value:?}"
///
/// PartialEq: values that can be compared with ==
/// PartialOrd: values that can be compared with <
/// Eq and Ord: the same but doesn't like floats
///
/// From<T>/Into<T>: can always be converted from/into the type T
///
/// TryFrom<T>/TryInto<T>: can sometimes be converted from/into the type T
///
fn marker_traits() {
    // integers are copy, clone, display, debug, send and sync
    let copy_value = 42;

    // vectors are clone, debug, send and sync
    let clone_value = vec![u8::MAX; 8];

    // drop(copy_value);
    // drop(clone_value);

    // rc are neither send nor sync
    let rc_val = Rc::new(42);

    // Re
    let refcell_val = RefCell::new(42);
    let refcell_ref = &refcell_val;

    let handle = std::thread::spawn(move || {
        // drop(clone_value);
        // refcell_val.borrow();
        // refcell_ref.borrow();
    });
    handle.join().unwrap();

    println!("marker_traits: {copy_value} {clone_value:02x?}");
    // println!("marker_traits: {rc_val} {refcell_val:?}");
}

fn hygienic_macros() {
    macro_rules! do_impl {
        ($foo:ident) => {
            pub struct $foo {
                field_1: i32,
                field_2: i32,
            }

            impl $foo {
                fn some_method(&self) -> i32 {
                    self.field_1
                }
            }
        };
    }

    do_impl!(Foo);
    do_impl!(Bar);
    do_impl!(Baz);
}
