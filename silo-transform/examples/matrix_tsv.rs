use silo_transform::matrix::*;
use std::collections::HashMap;

fn main() {
    let transformer = MatrixTransformerBuilder::new()
        .output_as(MatrixOutputType::Tsv)
        .with_header(true)
        .with_int_field("age")
        .with_int_field("length_of_stay")
        .with_binary_field("sex")
        .with_binary_field("abdominal_pain")
        .with_binary_field("cough")
        .build();

    let mut int_fields: HashMap<String, i16> = HashMap::new();
    int_fields.insert("sex".into(), 0);
    int_fields.insert("length_of_stay".into(), 123);
    int_fields.insert("age".into(), 24);

    let mut binary_fields: HashMap<String, bool> = HashMap::new();
    binary_fields.insert("abdominal_pain".into(), true);
    binary_fields.insert("cough".into(), true);

    let patient = MatrixTransformerRow::new(binary_fields, int_fields);

    let output = transformer.generate(vec![patient.clone(), patient]);

    println!("{}", output.unwrap());
}
