#![feature(test)]

use mimalloc_rust::*;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
