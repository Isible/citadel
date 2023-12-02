cargo test --package api --lib -- tests --nocapture
cargo test --package frontend --lib -- tests --nocapture
cargo test --package middleend --lib -- tests --nocapture
cargo test --package backend --lib -- tests --nocapture
cargo test --package fungus --bin fungus -- tests --nocapture