use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::props::props::IProps;
use crate::service::version_id::VersionId;

pub trait IPropsSocial : IProps {
    fn max_annuals_basis(&self) -> i32;
    fn factor_employer(&self) -> Decimal;
    fn factor_employer_higher(&self) -> Decimal;
    fn factor_employee(&self) -> Decimal;
    fn factor_employee_garant(&self) -> Decimal;
    fn factor_employee_reduce(&self) -> Decimal;
    fn margin_income_emp(&self) -> i32;
    fn margin_income_agr(&self) -> i32;
}

#[derive(Debug, Copy, Clone)]
pub struct PropsSocial{
    version: VersionId,
    max_annuals_basis: i32,
    factor_employer: Decimal,
    factor_employer_higher: Decimal,
    factor_employee: Decimal,
    factor_employee_garant: Decimal,
    factor_employee_reduce: Decimal,
    margin_income_emp: i32,
    margin_income_agr: i32,
}

#[allow(dead_code)]
impl PropsSocial {
    pub(crate) fn new(_version: VersionId,
                      _max_annuals_basis: i32,
                      _factor_employer: Decimal,
                      _factor_employer_higher: Decimal,
                      _factor_employee: Decimal,
                      _factor_employee_garant: Decimal,
                      _factor_employee_reduce: Decimal,
                      _margin_income_emp: i32,
                      _margin_income_agr: i32) -> PropsSocial {
        PropsSocial {
            version: _version,
            max_annuals_basis: _max_annuals_basis,
            factor_employer: _factor_employer,
            factor_employer_higher: _factor_employer_higher,
            factor_employee: _factor_employee,
            factor_employee_garant: _factor_employee_garant,
            factor_employee_reduce: _factor_employee_reduce,
            margin_income_emp: _margin_income_emp,
            margin_income_agr: _margin_income_agr,
        }
    }
    pub(crate) fn empty() -> PropsSocial {
        PropsSocial {
            version: VersionId::new(),
            max_annuals_basis: 0,
            factor_employer: dec!(0),
            factor_employer_higher: dec!(0),
            factor_employee: dec!(0),
            factor_employee_garant: dec!(0),
            factor_employee_reduce: dec!(0),
            margin_income_emp: 0,
            margin_income_agr: 0,
        }
    }
}

impl IProps for PropsSocial {
    fn get_version(&self) -> VersionId {
        self.version
    }
}

impl IPropsSocial for PropsSocial {
    fn max_annuals_basis(&self) -> i32 {
        self.max_annuals_basis
    }

    fn factor_employer(&self) -> Decimal {
        self.factor_employer
    }

    fn factor_employer_higher(&self) -> Decimal {
        self.factor_employer_higher
    }

    fn factor_employee(&self) -> Decimal {
        self.factor_employee
    }

    fn factor_employee_garant(&self) -> Decimal {
        self.factor_employee_garant
    }

    fn factor_employee_reduce(&self) -> Decimal {
        self.factor_employee_reduce
    }

    fn margin_income_emp(&self) -> i32 {
        self.margin_income_emp
    }

    fn margin_income_agr(&self) -> i32 {
        self.margin_income_agr
    }
}

