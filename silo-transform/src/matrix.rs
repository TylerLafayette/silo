/// A builder for creating matrix transformers.
pub struct MatrixTransformerBuilder {
    __binary_fields: Vec<String>,
    __int_fields: Vec<String>,
}

impl MatrixTransformerBuilder {
    /// Creates and returns a new MatrixTransformerBuilder.
    fn new() -> Self {
        Self {
            __binary_fields: vec![],
            __int_fields: vec![],
        }
    }

    /// Adds an int field to the matrix.
    fn with_int_field(mut self, field_name: &str) -> Self {
        self.__int_fields.push(field_name.into());
        self
    }

    /// Adds a binary field to the matrix.
    fn with_binary_field(mut self, field_name: &str) -> Self {
        self.__binary_fields.push(field_name.into());
        self
    }

    /// Builds the MatrixTransformer.
    fn build(self) -> MatrixTransformer {
        MatrixTransformer {
            binary_fields: self.__binary_fields,
            int_fields: self.__int_fields,
        }
    }
}

/// Provides methods for transforming samples into matrices.
pub struct MatrixTransformer {
    binary_fields: Vec<String>,
    int_fields: Vec<String>,
}

impl MatrixTransformer {}
