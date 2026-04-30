pub const fn short_or_default(short: &'static str, default: &'static str) -> &'static str {
    if cfg!(feature = "short_names") {
        short
    } else {
        default
    }
}
