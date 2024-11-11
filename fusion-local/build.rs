#[cfg(windows)]
fn main() {
    static_vcruntime::metabuild();
}

#[cfg(not(windows))]
fn main() {
    // For non-Windows targets, do nothing
}

