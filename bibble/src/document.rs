use std::collections::HashMap;

use crate::{Field, FieldValue};

integer_newtype!(DocumentId, "Provides a unique identifier for a document within a collection.");

pub struct Document {
  fields: HashMap<String, Field>
}

impl Document {
  pub fn new() -> Document {
    Document { fields: HashMap::new() }
  }

  pub fn field_value(&self, name: &str) -> Option<&FieldValue> {
    self.fields.get(name).map(|f| f.value())
  }

  pub fn field(&self, name: &str) -> Option<&Field> {
    self.fields.get(name)
  }

  pub fn field_mut(&mut self, name: &str) -> Option<&mut Field> {
    self.fields.get_mut(name)
  }

  pub fn add_field(&mut self, field: Field) {
    self.fields.insert(field.name().to_string(), field);
  }
}