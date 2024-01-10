cargo build --workspace --release;
time bash ./run_all.sh >> time.txt;
time bash ./run_all_parallel.sh >> time.txt;