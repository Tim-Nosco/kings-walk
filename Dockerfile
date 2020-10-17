from rustlang/rust:nightly

workdir /src
copy . /src
run cargo check && cargo test
