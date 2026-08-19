use crate::ecs::World;

/// Literally represents a parameter passed to a system.
///
/// A system function must only have `SystemParam` bound
/// parameters.
///
/// E.g. `Query`, `Resource`, `Local`, etc. all implement
/// [`SystemParam`].
pub trait SystemParam<'w> {
    fn fetch(world: &'w World) -> Self;
}
