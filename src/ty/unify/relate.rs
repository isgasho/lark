use crate::ty::intern::{Intern, Untern};
use crate::ty::unify::UnificationTable;
use crate::ty::AsInferVar;
use crate::ty::Generic;
use crate::ty::InferVar;
use crate::ty::Predicate;
use crate::ty::Region;
use crate::ty::Ty;
use crate::ty::{Base, BaseData};
use crate::ty::{Generics, GenericsData};
use crate::ty::{Perm, PermData};
use std::convert::TryFrom;

mod base_eq;
mod spine;
use self::spine::InstantiateSpine;

enum Direction {
    LessThan,
    GreaterThan,
    Equal,
}

struct Relate<'me> {
    unify: &'me mut UnificationTable,
    predicates: Vec<Predicate>,
}

struct Error;

impl Relate<'me> {
    fn intern<D: Intern>(&self, data: D) -> D::Key {
        self.unify.intern.intern(data)
    }

    fn untern<K: Untern>(&self, key: K) -> K::Data {
        self.unify.intern.untern(key)
    }
}
