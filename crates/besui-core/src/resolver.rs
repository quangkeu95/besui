use crate::{database::DbConnection, errors::Error};
use besui_config::config::AppConfig;
use once_cell::sync::OnceCell;
use std::sync::Arc;

static DEFAULT_ROOT_RESOLVER: OnceCell<RootResolver> = OnceCell::new();

/**
 * Resolver for all the common services
 *
 * The goal of the resolver is to let consumers construct components without having to know about what their dependencies are.
 *
 *
 */
#[derive(Clone, Debug)]
pub struct RootResolver {
    pub db_conn: DbConnection,
    pub config: Arc<AppConfig>,
}

impl RootResolver {
    /** Init an singleton instance of RootResolver. RootResolver can only be initialized once. */
    pub fn init<F>(init_fn: F) -> anyhow::Result<()>
    where
        F: FnOnce() -> RootResolver,
    {
        match DEFAULT_ROOT_RESOLVER.set(init_fn()) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::ErrorInitializingRootResolver.into()),
        }
    }

    /** Get RootResolver global instance. Return error if RootResolver is not initialized. */
    pub fn get() -> anyhow::Result<&'static Self> {
        match DEFAULT_ROOT_RESOLVER.get() {
            Some(result) => Ok(result),
            None => Err(Error::ErrorRootResolverNotInitialized.into()),
        }
    }
}

#[cfg(test)]
mod root_resolver_test {
    use besui_config::config::get_global_config;
    use claims::{assert_err, assert_err_eq, assert_ok};
    use sea_orm::{DatabaseBackend, MockDatabase};

    use crate::database::DbConnection;

    use super::*;

    #[test]
    fn test_init_root_resolver() {
        let raw_db_conn = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
        let db_conn = DbConnection::try_from(raw_db_conn).unwrap();
        let config = get_global_config();

        let result = RootResolver::init(|| RootResolver {
            db_conn: db_conn.clone(),
            config: config.clone(),
        });
        assert_ok!(result);

        let err_result = RootResolver::init(|| RootResolver {
            db_conn: db_conn.clone(),
            config: config.clone(),
        });
        let err = assert_err!(err_result);
        assert_eq!(
            err.to_string(),
            Error::ErrorInitializingRootResolver.to_string()
        );
    }

    #[test]
    fn test_get_root_resolver() {
        let err = assert_err!(RootResolver::get());
        assert_eq!(
            err.to_string(),
            Error::ErrorRootResolverNotInitialized.to_string()
        );
    }
}
