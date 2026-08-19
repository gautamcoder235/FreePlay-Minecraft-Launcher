use std::sync::Arc;
use freeplay_domain::{DomainError, Instance, InstanceId, LoaderType};
use crate::ports::{MinecraftMetadataPort, StoragePort};

pub struct InstanceService {
    storage: Arc<dyn StoragePort>,
    minecraft: Arc<dyn MinecraftMetadataPort>,
}

impl InstanceService {
    pub fn new(
        storage: Arc<dyn StoragePort>,
        minecraft: Arc<dyn MinecraftMetadataPort>,
    ) -> Self {
        Self { storage, minecraft }
    }

    pub async fn create_instance(
        &self,
        name: String,
        game_version: String,
        loader: LoaderType,
    ) -> Result<Instance, DomainError> {
        let instance = Instance::new(name, game_version, loader)?;
        self.storage.save_instance(&instance).await?;
        Ok(instance)
    }

    pub async fn list_instances(&self) -> Result<Vec<Instance>, DomainError> {
        self.storage.list_instances().await
    }

    pub async fn get_instance(&self, id: &InstanceId) -> Result<Option<Instance>, DomainError> {
        self.storage.get_instance(id).await
    }

    pub async fn launch_instance(&self, id: &InstanceId) -> Result<Instance, DomainError> {
        let mut instance = self
            .storage
            .get_instance(id)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                entity_type: "Instance".to_string(),
                id: id.to_string(),
            })?;

        self.minecraft.prepare_instance_artifacts(&instance).await?;
        self.minecraft.launch_instance(&mut instance).await?;
        self.storage.save_instance(&instance).await?;

        Ok(instance)
    }

    pub async fn delete_instance(&self, id: &InstanceId) -> Result<(), DomainError> {
        self.storage.delete_instance(id).await
    }
}
