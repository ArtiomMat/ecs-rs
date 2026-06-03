use super::id_types::{EntityId, ComponentId};
use super::error::Error;
use super::component_storage::{ComponentsStorage, ComponentStorageStrategy};

pub(super) trait WorldBackend {
    // fn is_component_storage_added<C: 'static>(&mut self) -> bool;
    // fn add_component_storage<C: 'static>(&mut self);
    
    /// Adds a component with initial data `data`, tied to `eid`.
    fn add_component<C: 'static>(&mut self, eid: EntityId, data: C) -> Result<(), Error>;
    /// Removesstorage_type: ComponentStorageType a component from `eid`, returns its data.
    fn remove_component<C: 'static>(&mut self, eid: EntityId) -> Result<C, Error>;
}
