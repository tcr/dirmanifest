#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! ignore = "0.4"
//! num_cpus = "1.0"
//! ```

extern crate ignore;
extern crate num_cpus;

use ignore::WalkBuilder;

fn main() {
    println!("num_cpus {:?}", num_cpus);
    WalkBuilder::new("/")
        .hidden(false)
        .ignore(false)
        .threads(num_cpus::get())
        .build_parallel()
        .run(|| Box::new(move |entry_o| {
            let _ = entry_o.map(|e| {
                e.path().file_name().map(|os_str| {
                    println!("{}", os_str.to_string_lossy());
                });
            });
            ignore::WalkState::Continue
        }));
}