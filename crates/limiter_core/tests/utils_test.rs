use limiter_core::utils::time::timestamp;

#[cfg(test)]
mod tests {
    use super::timestamp;

    #[test]
    fn timestamp_returns_non_zero_value() {
        let ts = timestamp();
        assert!(ts > 0);
    }

    #[test]
    fn timestamp_is_non_decreasing() {
        let first = timestamp();
        let second = timestamp();

        assert!(second >= first);
    }
}
