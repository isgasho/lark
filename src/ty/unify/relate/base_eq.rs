//! This module contains functions that assert "base equality",
//! which means that two types have equal "base types" information but
//! which does not relate their permissions.

use crate::ty::intern::{Intern, Untern};
use crate::ty::unify::relate::spine::InstantiateSpine;
use crate::ty::unify::relate::Error;
use crate::ty::unify::relate::Relate;
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

impl Relate<'me> {
    /// Checks that two types are "base equal", which
    /// means that their bases are deeply equal but which
    /// says nothing about their permissions.
    crate fn ty_base_eq(&mut self, ty1: Ty, ty2: Ty) -> Result<(), Error> {
        let Ty {
            perm: _,
            base: base1,
        } = ty1;

        let Ty {
            perm: _,
            base: base2,
        } = ty2;

        self.base_eq(base1, base2)
    }

    /// No matter what the "variance" is, for two types
    /// to be related, their bases must be equal. So
    /// for example `p1 String` and `p2 Vec` can never
    /// be related, but `p1 String` and `p2 String` can be.
    fn base_eq(&mut self, base1: Base, base2: Base) -> Result<(), Error> {
        match (
            self.unify.shallow_resolve_data(base1),
            self.unify.shallow_resolve_data(base2),
        ) {
            (Ok(data1), Ok(data2)) => self.base_data_eq(base1, data1, base2, data2),

            (Ok(data1), Err(var2)) => self.base_var_data_eq(var2, data1),

            (Err(var1), Ok(data2)) => self.base_var_data_eq(var1, data2),

            (Err(var1), Err(var2)) => {
                self.predicates.push(Predicate::BaseEq(base1, base2));
                Ok(())
            }
        }
    }

    fn generic_base_eq(&mut self, generic1: Generic, generic2: Generic) -> Result<(), Error> {
        match (generic1, generic2) {
            (Generic::Ty(ty1), Generic::Ty(ty2)) => self.ty_base_eq(ty1, ty2),
        }
    }

    fn base_var_data_eq(&mut self, var1: InferVar, data2: BaseData) -> Result<(), Error> {
        assert!(self.unify.probe(var1).is_none());
        assert!(data2.as_infer_var().is_none());
        let new_spine_data = data2.instantiate_spine(self);
        let new_spine = self.intern(new_spine_data);
        self.unify.bind_unbound_var_to_value(var1, new_spine);
        Ok(())
    }

    fn base_data_eq(
        &mut self,
        _base1: Base,
        data1: BaseData,
        _base2: Base,
        data2: BaseData,
    ) -> Result<(), Error> {
        if data1.kind != data2.kind {
            return Err(Error);
        }

        let generics_data1 = self.untern(data1.generics);
        let generics_data2 = self.untern(data2.generics);
        assert_eq!(generics_data1.len(), generics_data2.len());

        for (generic1, generic2) in generics_data1.iter().zip(&generics_data2) {
            self.generic_base_eq(generic1, generic2);
        }

        Ok(())
    }
}
