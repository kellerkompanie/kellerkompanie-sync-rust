# kellerkompanie-sync-rust

## Prerequisites
1. Install Rust (including `cargo`), see https://www.rust-lang.org/ for more information.
2. Clone repository `git clone https://github.com/kellerkompanie/kellerkompanie-sync-rust.git`

## How to build
1. `cd kellerkompanie-sync-rust/kellerkompanie-sync`
2. `cargo build`

## How to run
1. Create a script, e.g., `run-kekosync.sh` with the following contents:
```bash
cd kellerkompanie-sync-rust/kellerkompanie-sync
git pull
cargo run
```
2. Make the script executable using `chmod +x run-kekosync.sh`
3. Run script by invoking `./run-kekosync.sh`