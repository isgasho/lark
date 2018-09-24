use crate::ty::intern::{Interners, TyInterners};
use crate::ty::unify::UnificationTable;
use crate::ty::Predicate;
use crate::ty::Ty;

mod repr_eq;
mod spine;
mod test;

impl UnificationTable {
    crate fn ty_repr_eq(&mut self, ty1: Ty, ty2: Ty) -> Result<Vec<Predicate>, Error> {
        let mut relate = Relate {
            unify: self,
            predicates: vec![],
        };
        // FIXME transaction
        relate.ty_repr_eq(ty1, ty2)?;
        Ok(relate.predicates)
    }
}

enum Direction {
    LessThan,
    GreaterThan,
    Equal,
}

struct Relate<'me> {
    unify: &'me mut UnificationTable,
    predicates: Vec<Predicate>,
}

#[derive(Copy, Clone, Debug)]
crate struct Error;

impl Interners for Relate<'_> {
    fn interners(&self) -> &TyInterners {
        self.unify.interners()
    }
}
