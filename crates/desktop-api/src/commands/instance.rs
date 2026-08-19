use freeplay_domain::{DomainError, Instance, InstanceId, LoaderType};
use crate::context::AppContext;

#[specta::specta]
pub async fn list_instances(context: &AppContext) -> Result<Vec<Instance>, DomainError> {
    context.instance_service.list_instances().await
}

#[specta::specta]
pub async fn create_instance(
    context: &AppContext,
    name: String,
    game_version: String,
    loader: LoaderType,
) -> Result<Instance, DomainError> {
    context.instance_service.create_instance(name, game_version, loader).await
}

#[specta::specta]
pub async fn launch_instance(
    context: &AppContext,
    instance_id: InstanceId,
) -> Result<Instance, DomainError> {
    context.instance_service.launch_instance(&instance_id).await
}

#[specta::specta]
pub async fn delete_instance(
    context: &AppContext,
    instance_id: InstanceId,
) -> Result<(), DomainError> {
    context.instance_service.delete_instance(&instance_id).await
}
