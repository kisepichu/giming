use domain::{entity::Workspace, error::Error};
use mockall::automock;

use crate::service_error::ServiceError;

#[automock]
pub trait WorkspaceRepository<E: Error + 'static> {
    fn exists(&self, contest_id: &str) -> Result<bool, ServiceError<E>>;
    #[allow(clippy::needless_lifetimes)] // need for automock
    fn create<'p>(&self, contest_id: &str, workspace: Workspace<'p>)
        -> Result<(), ServiceError<E>>;
}
