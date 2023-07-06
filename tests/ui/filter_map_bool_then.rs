//@run-rustfix
//@aux-build:proc_macros.rs:proc-macro
#![allow(clippy::clone_on_copy, clippy::unnecessary_lazy_evaluations, unused)]
#![warn(clippy::filter_map_bool_then)]

#[macro_use]
extern crate proc_macros;

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    v.clone().into_iter().filter_map(|i| (i % 2 == 0).then(|| i + 1));
    v.clone()
        .into_iter()
        .filter_map(|i| -> Option<_> { (i % 2 == 0).then(|| i + 1) });
    v.clone()
        .into_iter()
        .filter(|&i| i != 1000)
        .filter_map(|i| (i % 2 == 0).then(|| i + 1));
    v.iter()
        .copied()
        .filter(|&i| i != 1000)
        .filter_map(|i| (i.clone() % 2 == 0).then(|| i + 1));
    // Do not lint
    external! {
        let v = vec![1, 2, 3, 4, 5, 6];
        v.clone().into_iter().filter_map(|i| (i % 2 == 0).then(|| i + 1));
    }
    with_span! {
        span
        let v = vec![1, 2, 3, 4, 5, 6];
        v.clone().into_iter().filter_map(|i| (i % 2 == 0).then(|| i + 1));
    }
}
