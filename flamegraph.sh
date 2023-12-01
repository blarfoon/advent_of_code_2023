#!/bin/sh

# cargo flamegraph --profile flamegraph --root --package day-$1 --bin part$2 -o flamegraphs/day-$1--part$2.svg
cargo flamegraph --profile flamegraph --root --package day-$1 --bin part$2 -o flamegraphs/day-$1--part$2.svg