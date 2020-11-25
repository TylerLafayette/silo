use std::collections::HashMap;

/// Specifies different output types for a matrix.
pub enum MatrixOutputType {
    Json,
    Csv,
    Tsv,
}

/// A builder for creating matrix transformers.
pub struct MatrixTransformerBuilder {
    __binary_fields: Vec<String>,
    __int_fields: Vec<String>,
    __output_type: MatrixOutputType,
}

/// Contains both int and binary field vectors.
pub struct MatrixTransformerRow {
    binary_fields: HashMap<String, bool>,
    int_fields: HashMap<String, i16>,
}

impl MatrixTransformerBuilder {
    /// Creates and returns a new MatrixTransformerBuilder.
    fn new() -> Self {
        Self {
            __binary_fields: vec![],
            __int_fields: vec![],
            __output_type: MatrixOutputType::Tsv,
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

    /// Sets the output type of the matrix with a MatrixOutputType enum.
    fn output_as(mut self, output_type: MatrixOutputType) -> Self {
        self.__output_type = output_type;
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

impl MatrixTransformer {
    /// Generates and returns a matrix.
    pub fn generate(self, rows: Vec<MatrixTransformerRow>) {}
}
