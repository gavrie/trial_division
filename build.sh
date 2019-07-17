cargo +nightly rustc --release -- -C link-arg=-undefined -C link-arg=dynamic_lookup 
ln -sf libprimes.dylib target/release/libprimes.so
ln -sf libprimes.so target/release/primes.so
