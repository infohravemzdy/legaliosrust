pub trait IParticyResult : Copy {
    fn particy_code(&self) -> i32;
    fn result_basis(&self) -> i32;
    fn result_value(&self) -> i32;
    fn set_result_value(&mut self, value: i32) -> i32;
}