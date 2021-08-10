use crate::error::Result;

pub trait Aggregate {}

pub trait Event<A: Aggregate> {
    fn apply(&self, aggregate: &mut A);
}

pub trait Command<A: Aggregate, E: Event<A>> {
    fn handle(self, aggregate: &A) -> Result<Vec<E>>;
}
