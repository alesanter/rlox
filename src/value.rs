#[derive(Debug, Clone, Copy)]
pub struct Value(pub f64);

impl ToString for Value {
    fn to_string(&self) -> String {
        return format!("{:}", self.0);
    }
}

#[derive(Debug)]
pub struct Constant(pub f64);

impl From<f64> for Constant {
    fn from(value: f64) -> Self {
        Constant(value)
    }
}
