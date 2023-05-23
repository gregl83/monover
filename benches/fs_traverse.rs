#![feature(test)]

extern crate test;
use test::Bencher;

use std::env;
use std::path::PathBuf;

mod utils;
use utils::TempDir;


struct DeepFileTree {
    root: PathBuf,
    // depth: usize,
    // width: usize,
}

impl DeepFileTree {
    fn setup() -> Self {
        let alpha_file_name = "alpha";
        let alpha_file_contents = "alpha-body".as_bytes();
        let bravo_file_name = "bravo";
        let bravo_file_contents = "bravo-body".as_bytes();
        let charlie_file_name = "charlie";
        let charlie_file_contents = "charlie-body".as_bytes();
        let one_file_name = "1";
        let one_file_contents = "1-body".as_bytes();
        let nine_file_name = "9";
        let nine_file_contents = "9-body".as_bytes();

        let dir = TempDir::new("bench_hashes_directory_files").unwrap();
        dir.new_file(alpha_file_name, alpha_file_contents).unwrap();
        dir.new_file(bravo_file_name, bravo_file_contents).unwrap();
        dir.new_file(charlie_file_name, charlie_file_contents).unwrap();
        dir.new_file(one_file_name, one_file_contents).unwrap();
        dir.new_file(nine_file_name, nine_file_contents).unwrap();

        DeepFileTree {
            root: dir.path().as_os_str().to_str().unwrap(),
        }
    }

    fn teardown(&self) {
        // Tear down your shared resource here
    }
}


use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::{Arc, Mutex};


fn benchmark_walkdir_traverse(deep_file_tree: &DeepFileTree) {
    paq::hash_source(deep_file_tree.root, false)
}

fn benchmark_threadpool_traverse(deep_file_tree: &DeepFileTree) {
    paq::hash_source(deep_file_tree.root, false)
}

fn custom_benchmarks(c: &mut Criterion) {
    let shared_resource = Arc::new(Mutex::new(SharedResource::setup()));

    {
        let shared_resource = Arc::clone(&shared_resource);
        c.bench_function("walkdir_traverse", move |b| {
            b.iter(|| {
                let shared_resource = shared_resource.lock().unwrap();
                benchmark_walkdir_traverse(&shared_resource)
            })
        });
    }

    {
        let shared_resource = Arc::clone(&shared_resource);
        c.bench_function("threadpool_traverse", move |b| {
            b.iter(|| {
                let shared_resource = shared_resource.lock().unwrap();
                benchmark_threadpool_traverse(&shared_resource)
            })
        });
    }

    shared_resource.lock().unwrap().teardown();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = custom_benchmarks
}
criterion_main!(benches);





