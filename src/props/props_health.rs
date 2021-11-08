use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::props::props::IProps;
use crate::service::version_id::VersionId;

pub trait IPropsHealth : IProps {
    fn min_monthly_basis(&self) -> i32;
    fn max_annuals_basis(&self) -> i32;
    fn lim_monthly_state(&self) -> i32;
    fn lim_monthly_dis50(&self) -> i32;
    fn factor_compound(&self) -> Decimal;
    fn factor_employee(&self) -> Decimal;
    fn margin_income_emp(&self) -> i32;
    fn margin_income_agr(&self) -> i32;
}

#[derive(Debug, Copy, Clone)]
pub struct PropsHealth{
    version: VersionId,
    min_monthly_basis: i32,
    max_annuals_basis: i32,
    lim_monthly_state: i32,
    lim_monthly_dis50: i32,
    factor_compound: Decimal,
    factor_employee: Decimal,
    margin_income_emp: i32,
    margin_income_agr: i32,
}

#[allow(dead_code)]
impl PropsHealth {
    pub(crate) fn new(_version: VersionId,
                      _min_monthly_basis: i32,
                      _max_annuals_basis: i32,
                      _lim_monthly_state: i32,
                      _lim_monthly_dis50: i32,
                      _factor_compound: Decimal,
                      _factor_employee: Decimal,
                      _margin_income_emp: i32,
                      _margin_income_agr: i32) -> PropsHealth {
        PropsHealth {
            version: _version,
            min_monthly_basis: _min_monthly_basis,
            max_annuals_basis: _max_annuals_basis,
            lim_monthly_state: _lim_monthly_state,
            lim_monthly_dis50: _lim_monthly_dis50,
            factor_compound: _factor_compound,
            factor_employee: _factor_employee,
            margin_income_emp: _margin_income_emp,
            margin_income_agr: _margin_income_agr,
        }
    }
    pub(crate) fn empty() -> PropsHealth {
        PropsHealth {
            version: VersionId::new(),
            min_monthly_basis: 0,
            max_annuals_basis: 0,
            lim_monthly_state: 0,
            lim_monthly_dis50: 0,
            factor_compound: dec!(0),
            factor_employee: dec!(0),
            margin_income_emp: 0,
            margin_income_agr: 0,
        }
    }
}

impl IProps for PropsHealth {
    fn get_version(&self) -> VersionId {
        self.version
    }
}

impl IPropsHealth for PropsHealth {
    fn min_monthly_basis(&self) -> i32 {
        self.min_monthly_basis
    }

    fn max_annuals_basis(&self) -> i32 {
        self.max_annuals_basis
    }

    fn lim_monthly_state(&self) -> i32 {
        self.lim_monthly_state
    }

    fn lim_monthly_dis50(&self) -> i32 {
        self.lim_monthly_dis50
    }

    fn factor_compound(&self) -> Decimal {
        self.factor_compound
    }

    fn factor_employee(&self) -> Decimal {
        self.factor_employee
    }

    fn margin_income_emp(&self) -> i32 {
        self.margin_income_emp
    }

    fn margin_income_agr(&self) -> i32 {
        self.margin_income_agr
    }
}
