use crate::props::props::IProps;
use crate::service::version_id::VersionId;

pub trait IPropsSalary : IProps {
    fn working_shift_week(&self) -> i32;
    fn working_shift_time(&self) -> i32;
    fn min_monthly_wage(&self) -> i32;
    fn min_hourly_wage(&self) -> i32;
}

#[derive(Debug, Copy, Clone)]
pub struct PropsSalary{
    version: VersionId,
    working_shift_week: i32,
    working_shift_time: i32,
    min_monthly_wage: i32,
    min_hourly_wage: i32,
}

#[allow(dead_code)]
impl PropsSalary {
    pub(crate) fn new(_version: VersionId,
                      _working_shift_week: i32,
                      _working_shift_time: i32,
                      _min_monthly_wage: i32,
                      _min_hourly_wage: i32) -> PropsSalary {
        PropsSalary {
            version: _version,
            working_shift_week: _working_shift_week,
            working_shift_time: _working_shift_time,
            min_monthly_wage: _min_monthly_wage,
            min_hourly_wage: _min_hourly_wage,
        }
    }
    pub(crate) fn empty() -> PropsSalary {
        PropsSalary {
            version: VersionId::new(),
            working_shift_week: 0,
            working_shift_time: 0,
            min_monthly_wage: 0,
            min_hourly_wage: 0,
        }
    }
}

impl IProps for PropsSalary {
    fn get_version(&self) -> VersionId {
        self.version
    }
}

impl IPropsSalary for PropsSalary {
    fn working_shift_week(&self) -> i32 {
        self.working_shift_week
    }

    fn working_shift_time(&self) -> i32 {
        self.working_shift_time
    }

    fn min_monthly_wage(&self) -> i32 {
        self.min_monthly_wage
    }

    fn min_hourly_wage(&self) -> i32 {
        self.min_hourly_wage
    }
}
