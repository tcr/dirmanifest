extern crate ignore;
extern crate num_cpus;
extern crate csv;
extern crate crossbeam_channel;
extern crate failure;

use ignore::WalkBuilder;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam_channel::unbounded;
use std::io;
use failure::Error;

fn main() {
    // let root = "/Volumes/Backup 1/";
    let root = "/Users/trim";
    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(false)
        .ignore(false);
    
    let (tx, rx) = unbounded::<Option<(String,u64)>>();
    
    ::std::thread::spawn(move || -> Result<(), Error> {
        let mut wtr = csv::Writer::from_writer(io::stdout());

        wtr.write_record(&["Path", "Size"])?;
        while let Some(Some(item)) = rx.recv() {
            wtr.serialize(item)?;
        }

        wtr.flush()?;
        Ok(())
    });

    let count = Arc::new(AtomicUsize::new(0));
    let parallel_walker = builder
        .threads(num_cpus::get())
        .build_parallel();

    parallel_walker.run(|| {
        let count = count.clone();
        let tx = tx.clone();
        Box::new(move |entry_o| {
            match entry_o {
                Ok(e) => {
                    let a = count.fetch_add(1, Ordering::Relaxed);
                    let outpath = e.path().to_string_lossy().chars().skip(root.len()).collect::<String>();
                    let size = ::std::fs::metadata(e.path()).map(|x| x.len()).unwrap_or(0);
                    // println!("{},{}", size, outpath);
                    tx.send(Some((outpath, size)));
                }
                Err(_) => {}
            }
            ignore::WalkState::Continue
        })
    });

    tx.send(None);
    
    eprintln!("file count: {}", count.load(Ordering::Relaxed));
    // eprintln!("hi");
}
