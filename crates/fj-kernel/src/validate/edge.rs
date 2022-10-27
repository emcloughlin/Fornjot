use std::convert::Infallible;

use crate::{
    objects::{GlobalEdge, HalfEdge},
    storage::Store,
};

use super::{Validate2, ValidationConfig};

impl Validate2 for HalfEdge {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Validate2 for GlobalEdge {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
