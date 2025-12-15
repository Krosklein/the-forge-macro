#[cfg(test)]
mod tests {
    use crate::config::AppConfig;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();

        assert_eq!(config.potion_key, "3");
        assert_eq!(config.time_key, 15);
        assert!(config.window_pos.is_none());
    }
}
