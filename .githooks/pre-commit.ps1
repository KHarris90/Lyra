# PowerShell pre-commit hook for Lyra
$ErrorActionPreference = "Stop"

Write-Host "[pre-commit] cargo fmt --all"
cargo fmt --all | Out-Host

Write-Host "[pre-commit] cargo clippy (workspace)"
cargo clippy --workspace --all-targets --all-features -- -D warnings -D clippy::all -D clippy::pedantic -A clippy::module_name_repetitions -A clippy::missing_errors_doc -A clippy::missing_panics_doc | Out-Host

Write-Host "[pre-commit] cargo test (workspace)"
cargo test --workspace | Out-Host