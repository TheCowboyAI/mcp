#![feature(test)]
extern crate test;

use nix_inspector_mcp::SystemAnalyzer;
use test::Bencher;

#[bench]
fn bench_system_analyzer_creation(b: &mut Bencher) {
    b.iter(|| {
        SystemAnalyzer::new().expect("Failed to create system analyzer")
    });
} 