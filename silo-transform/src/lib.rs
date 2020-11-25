#![deny(missing_docs)]

//! silo-transform provides an interface for transforming and exporting
//! sample and patient data. It exports an actor, `TransformActor`, as well as
//! providing various functions and implementations for transforming data.

/// Provides a struct, `MatrixTransformer`, for transforming data into matrices.
pub mod matrix;
