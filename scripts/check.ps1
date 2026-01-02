cargo fmt --all
if ($LASTEXITCODE -ne 0) { exit 1 }

cargo clippy -- -D warnings
if ($LASTEXITCODE -ne 0) { exit 1 }

cargo test
if ($LASTEXITCODE -ne 0) { exit 1 }

cargo build
if ($LASTEXITCODE -ne 0) { exit 1 }
