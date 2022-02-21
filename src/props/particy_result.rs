pub trait IParticyResult : Copy {
    fn particy_code(&self) -> i32;
    fn result_basis(&self) -> i32;
    fn result_value(&self) -> i32;
    fn set_result_value(&mut self, value: i32) -> i32;
}

#[derive(Debug, Copy, Clone)]
pub struct ParticyResult {
    particy_code: i32,
    result_basis: i32,
    result_value: i32,
}

#[allow(dead_code)]
impl ParticyResult {
    pub(crate) fn new(_particy_code: i32,
                      _result_basis: i32,
                      _result_value: i32) -> ParticyResult {
        ParticyResult {
            particy_code: _particy_code,
            result_basis: _result_basis,
            result_value: _result_value,
        }
    }
    pub(crate) fn empty() -> ParticyResult {
        ParticyResult {
            particy_code: 0,
            result_basis: 0,
            result_value: 0,
        }
    }
}

impl IParticyResult for ParticyResult {
    fn particy_code(&self) -> i32 {
        self.particy_code
    }

    fn result_basis(&self) -> i32 {
        self.result_basis
    }

    fn result_value(&self) -> i32 {
        self.result_value
    }

    fn set_result_value(&mut self, value: i32) -> i32 {
        self.result_value = value;
        return self.result_value;
    }
}
