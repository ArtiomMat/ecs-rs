use super::super::world_backend::WorldBackend;

struct ArchetypeWorld {
    
}

impl WorldBackend for ArchetypeWorld {
    fn add_component<C: 'static>(&mut self, eid: crate::ecs::EntityId, data: C) -> Result<(), crate::ecs::Error> {
        todo!()
    }

    fn remove_component<C: 'static>(&mut self, eid: crate::ecs::EntityId) -> Result<C, crate::ecs::Error> {
        todo!()
    }
}
