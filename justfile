# Run everything
all: test-all build-all stress-test-built smoke-test-built
    @echo ""
    @echo "===================================================="
    @echo "✅ SUCCESS: All Latinga checks passed successfully!"
    @echo "===================================================="

# --- Testing ---

test-native:
    cargo test --all-features

test-wasm:
    wasm-pack test --node --features wasm
    wasm-pack test --chrome --headless --features wasm

test-all: test-native test-wasm

# --- Building ---

build-native:
    cargo build --release --features cli

build-wasm:
    wasm-pack build --target web --out-dir web/pkg --features wasm

build-all: build-native build-wasm

# --- Utilities ---

clean:
    cargo clean
    rm -rf web/pkg

# --- Benchmarks and stress tests ---

# Run official Criterion benchmarks (Slow, high-precision)
bench:
    cargo bench

# Helper to run a shorter, faster version for quick checks
smoke-test-built:
    @echo "🧪 Running smoke test..."
    @# We use zsh -c to get your specific shell's 'time' formatting
    @python3 -c "print('Oʻzbekiston - kelajagi buyuk davlat! Maʼno va mantiq.\n' * 1000000)" | zsh -c 'time ./target/release/latinga -c="-baseline5" > /dev/null'
    @echo "✅ Smoke test complete."
smoke-test: build-native smoke-test-built

# Full stress test
stress-test-built:
    @echo "🔥 Running 1,000,000 line stress test..."
    @python3 -c "print('Oʻzbekiston - kelajagi buyuk davlat! Maʼno va mantiq.\n' * 1000000)" | zsh -c '/usr/bin/time -v ./target/release/latinga -c="-baseline5" > /dev/null'
    @echo "✅ Stress test complete."
stress-test: build-native stress-test-built
