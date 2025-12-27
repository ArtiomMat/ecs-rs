use super::entity::EntityId;

#[derive(Debug)]
pub enum Error {
    InvalidEntityId(EntityId),
    InvalidWorldComponent(&'static str),
    InvalidEntityComponent(&'static str),
    ComponentAlreadyAdded(&'static str, EntityId),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidEntityId(entity_id) => write!(f, "Entity {} is invalid", entity_id.0),
            Error::InvalidWorldComponent(name) => {
                write!(
                    f,
                    "Component {} was never registered to any entity in the world",
                    name
                )
            }
            Error::InvalidEntityComponent(name, entity_id) => {
                write!(f, "Component {} was never registered to the entity {}", name, entity_id.0)
            }
            Error::ComponentAlreadyAdded(name, entity_id) => {
                write!(
                    f,
                    "Component {} was already added to entity {}",
                    name, entity_id.0
                )
            }
        }
    }
}

impl std::error::Error for Error {}
