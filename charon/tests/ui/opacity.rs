//@ charon-args=--include core::option
//@ charon-args=--exclude test_crate::module::dont_translate_body
//@ charon-args=--exclude crate::module::other_mod
//@ charon-args=--include crate::module::other_mod::_
//@ charon-args=--include core::convert::{core::convert::Into<_,_>}::into
//@ charon-args=--include core::convert::num::{core::convert::From<_,_>}
//@ charon-args=--exclude _::exclude_me
// Note: we don't use the `impl Trait for T` syntax above because we can't have spaces in these
// options.

fn foo() {
    let _ = Some(0).is_some();
    let _: u64 = 42u32.into();
}

mod module {
    fn dont_translate_body() {
        println!("aaa")
    }
    mod other_mod {
        fn dont_even_see_me() {}
    }
}

fn exclude_me() {}
