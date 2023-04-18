// External uses
use serde::Deserialize;

// Local uses
use crate::envy_load;

/// Used database configuration.
#[derive(Default, Debug, Deserialize, Clone, PartialEq)]
pub struct DBConfig {
    /// Amount of open connections to the database held by server in the pool.
    pub pool_size: usize,
    /// Database URL.
    pub url: String,
}

impl DBConfig {
    pub fn from_env() -> Self {
        envy_load!("database", "DATABASE_")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configs::test_utils::set_env;

    fn expected_config() -> DBConfig {
        DBConfig {
            pool_size: 10,
            url: "postgres://postgres@localhost/plasma".into(),
        }
    }

    #[test]
    fn from_env() {
        let config = r#"
            DATABASE_URL="postgres://postgres@localhost/plasma"
            DATABASE_POOL_SIZE="10"
        "#;
        set_env(config);

        let actual = DBConfig::from_env();
        assert_eq!(actual, expected_config());
    }
}
