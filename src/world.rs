pub trait Query<'w>: Sized {
    fn get<W: World>(world: &'w W) -> Option<Self>;
}

pub trait World {
    fn query<'a, T>(&self, system: impl Fn(T))
    where
        T: Query<'a>;
}
