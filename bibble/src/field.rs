#[derive(Debug, PartialEq, Eq)]
pub enum FieldValue {
  Str(String)
}

impl From<&str> for FieldValue {
    fn from(s: &str) -> Self {
      FieldValue::Str(s.to_string())
    }
}

impl From<String> for FieldValue {
    fn from(s: String) -> Self {
      FieldValue::Str(s)
    }
}

pub struct Field {
  name: String,
  value: FieldValue
}

impl Field {
  pub fn new(name: impl Into<String>, value: impl Into<FieldValue>) -> Field {
    Field { name: name.into(), value: value.into() }
  }

  pub fn name(&self) -> &str { &self.name }
  pub fn value(&self) -> &FieldValue { &self.value }
}