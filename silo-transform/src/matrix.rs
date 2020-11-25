use std::collections::HashMap;

/// Specifies different output types for a matrix.
pub enum MatrixOutputType {
    /// Outputs the matrix as a JSON value.
    Json,
    /// Outputs the matrix as a CSV.
    Csv,
    /// Outputs the matrix as a tab-separated file.
    Tsv,
}

/// Contains both int and binary field vectors.
pub struct MatrixTransformerRow {
    binary_fields: HashMap<String, bool>,
    int_fields: HashMap<String, i16>,
}

impl MatrixTransformerRow {
    /// Creates and returns a new MatrixTransformerRow.
    fn new(binary_fields: HashMap<String, bool>, int_fields: HashMap<String, i16>) -> Self {
        Self {
            binary_fields,
            int_fields,
        }
    }
}
/// A builder for creating matrix transformers.
pub struct MatrixTransformerBuilder {
    __binary_fields: Vec<String>,
    __int_fields: Vec<String>,
    __output_type: MatrixOutputType,
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
            output_type: self.__output_type,
        }
    }
}

/// Provides methods for transforming samples into matrices.
pub struct MatrixTransformer {
    binary_fields: Vec<String>,
    int_fields: Vec<String>,
    output_type: MatrixOutputType,
}

impl MatrixTransformer {
    /// Generates and returns a matrix.
    pub fn generate(self, rows: Vec<MatrixTransformerRow>) -> Result<String, std::io::Error> {
        let mut matrix_output = String::from("");

        rows.iter().for_each(|row| match self.output_type {
            // No support yet for types other than TSV.
            // TODO: implement those output types :)
            _ => self.append_tsv_row(&mut matrix_output, &row),
        });

        Ok(matrix_output)
    }

    /// Formats a row as TSV and appends it to the builder with a trailing newline.
    fn append_tsv_row(&self, builder: &mut String, row: &MatrixTransformerRow) {
        self.int_fields
            .iter()
            .for_each(|field_name| match row.int_fields.get(field_name) {
                Some(value) => builder.push_str(&format!("{}\t", value)),
                _ => builder.push_str("NULL\t"),
            });

        self.binary_fields
            .iter()
            .for_each(|field_name| match row.binary_fields.get(field_name) {
                Some(true) => builder.push_str("1\t".into()),
                _ => builder.push_str("0\t"),
            });

        builder.push_str("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_matrix() {
        let transformer = MatrixTransformerBuilder::new()
            .with_int_field("age")
            .with_binary_field("migraine")
            .build();
        let output = transformer.generate(vec![]).unwrap();
        assert!(output == "");
    }

    #[test]
    fn one_element_matrix() {
        let transformer = MatrixTransformerBuilder::new()
            .with_int_field("age")
            .with_binary_field("migraine")
            .output_as(MatrixOutputType::Tsv)
            .build();

        let mut int_fields: HashMap<String, i16> = HashMap::new();
        int_fields.insert("age".into(), 24);

        let mut binary_fields: HashMap<String, bool> = HashMap::new();
        binary_fields.insert("migraine".into(), true);

        let row = MatrixTransformerRow::new(binary_fields, int_fields);

        let output = transformer.generate(vec![row]).unwrap();
        assert!(output == "24\t1\t\n");
    }
}
