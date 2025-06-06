// Copyright 2025 Accenture.
//
// SPDX-License-Identifier: Apache-2.0

use core::hash::{BuildHasher, Hasher};
use core::ops::Range;
use core::time::Duration;
use feo_tracing::{instrument, span, Level};
use std::collections::hash_map::RandomState;
use std::thread;
use tracing::level_filters::LevelFilter;
use tracing::{event, info};

fn main() {
    feo_tracing::init(LevelFilter::TRACE);

    iteration(10);

    // Spawn some threads that will generate traces
    (0..4).for_each(|n| {
        drop(thread::spawn(move || loop {
            iteration(n);
            sleep_rand_millis(20..50);
        }))
    });

    thread::park();
}

#[instrument(name = "iteration")]
fn iteration(n: u32) {
    // Create an event
    event!(Level::DEBUG, thread = format!("{}", n));

    // Create a span
    let now = format!(
        "{:?}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
    );
    let span = span!(Level::INFO, "hello tracing", now = now).entered();

    sleep_rand_millis(100..200);

    // Event that is part of the span
    event!(parent: &span, Level::DEBUG, now=now);

    info!(name: "Something happened", data="here");

    sleep_rand_millis(100..200);

    // Call a instrumented fn
    whooha();

    sleep_rand_millis(200..300);

    // Create a child span
    {
        let _inner = span!(Level::INFO, "inner", info = "inner span").entered();
        sleep_rand_millis(200..300);
    }

    sleep_rand_millis(100..130);
}

#[instrument]
fn whooha() {
    thread::sleep(Duration::from_secs(rand() % 2));
}

#[instrument(level = "trace")]
fn rand() -> u64 {
    RandomState::new().build_hasher().finish()
}

#[instrument(level = "trace")]
fn sleep_rand_millis(range: Range<u64>) {
    thread::sleep(Duration::from_millis(
        rand() % (range.end - range.start) + range.start,
    ));
}
