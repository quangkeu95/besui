use crate::{
    errors::Error,
    persistence::{SharedConnection, SharedPersistence},
};
use besui_config::AppConfig;
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
#[derive(Clone)]
pub struct RootResolver {
    pub persistence: SharedPersistence,
    pub connection_pool: SharedConnection,
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
    use crate::persistence::MockPersistence;
    use claims::{assert_err, assert_err_eq, assert_ok};
    use sea_orm::{DatabaseBackend, MockDatabase};
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_init_root_resolver() {
        let persistence = Arc::new(MockPersistence::new());
        let raw_db_conn = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
        let db_conn = Arc::new(raw_db_conn);

        let result = RootResolver::init(|| RootResolver {
            persistence: persistence.clone(),
            connection_pool: db_conn.clone(),
        });
        assert_ok!(result);

        let err_result = RootResolver::init(|| RootResolver {
            persistence: persistence.clone(),
            connection_pool: db_conn.clone(),
        });
        let err = assert_err!(err_result);
        assert_eq!(
            err.to_string(),
            Error::ErrorInitializingRootResolver.to_string()
        );
    }

    #[test]
    fn test_get_root_resolver() {
        let root_resolver = RootResolver::get();
        let err = assert_err!(root_resolver);
        assert_eq!(
            err.to_string(),
            Error::ErrorRootResolverNotInitialized.to_string()
        );
    }
}
