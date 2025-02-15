pub trait ValidateV1<Value> {
    type Error;

    fn validate_v1(&self, value: &Value) -> Vec<Self::Error>;
}
