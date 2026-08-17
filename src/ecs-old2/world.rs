use crate::ecs::EntityId;

// TODO: How do I now force World to be queriable?
pub trait CanQuery<T> {
    fn query<F>(&self, f: F) where F: Fn(T);
}

pub trait World {
    fn create_entity(&mut self) -> EntityId;
    fn delete_entity(&mut self, entity_id: EntityId);

    fn add_component<C>(&mut self, entity_id: EntityId, value: C);
    fn remove_component<C>(&mut self, entity_id: EntityId);

    fn query_components<C, F>(&self, f: F)
    where
        Self: CanQuery<C>,
        F: Fn(C),
    {
        CanQuery::<C>::query(self, f);
    }
}

