use lift::*;

pub type Xor<L,R> = Result<R,L>;

pub enum Ior<L, R> {
    Left(L),
    Right(R),
    Both(L,R)
}

lift2right!(Ior);

pub enum Lazy<A> {
    Lazy(Box<Fn() -> A>)
}

impl<A> Lazy<A> {
    pub fn apply(&self) -> A {
        match *self {
            Lazy::Lazy(ref f) => f()
        }
    }
}

pub enum TailRec<A> {
    Pure(A),
    Suspend(Lazy<A>),
    FlatMap(Box<TailRec<A>>, Fn(A)->Box<TailRec<B>>)
}
