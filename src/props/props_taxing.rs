use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::props::props::IProps;
use crate::service::version_id::VersionId;

pub trait IPropsTaxing : IProps {
    fn allowance_payer(&self) -> i32;
    fn allowance_disab1st(&self) -> i32;
    fn allowance_disab2nd(&self) -> i32;
    fn allowance_disab3rd(&self) -> i32;
    fn allowance_study(&self) -> i32;
    fn allowance_child1st(&self) -> i32;
    fn allowance_child2nd(&self) -> i32;
    fn allowance_child3rd(&self) -> i32;
    fn factor_advances(&self) -> Decimal;
    fn factor_withhold(&self) -> Decimal;
    fn factor_solitary(&self) -> Decimal;
    fn min_amount_of_tax_bonus(&self) -> i32;
    fn max_amount_of_tax_bonus(&self) -> i32;
    fn margin_income_of_tax_bonus(&self) -> i32;
    fn margin_income_of_rounding(&self) -> i32;
    fn margin_income_of_withhold(&self) -> i32;
    fn margin_income_of_solitary(&self) -> i32;
    fn margin_income_of_wth_emp(&self) -> i32;
    fn margin_income_of_wth_agr(&self) -> i32;
}

#[derive(Debug, Copy, Clone)]
pub struct PropsTaxing{
    version: VersionId,
    allowance_payer: i32,
    allowance_disab1st: i32,
    allowance_disab2nd: i32,
    allowance_disab3rd: i32,
    allowance_study: i32,
    allowance_child1st: i32,
    allowance_child2nd: i32,
    allowance_child3rd: i32,
    factor_advances: Decimal,
    factor_withhold: Decimal,
    factor_solitary: Decimal,
    min_amount_of_tax_bonus: i32,
    max_amount_of_tax_bonus: i32,
    margin_income_of_tax_bonus: i32,
    margin_income_of_rounding: i32,
    margin_income_of_withhold: i32,
    margin_income_of_solitary: i32,
    margin_income_of_wth_emp: i32,
    margin_income_of_wth_agr: i32
}

#[allow(dead_code)]
impl PropsTaxing {
    pub(crate) fn new(_version: VersionId,
                      _allowance_payer: i32,
                      _allowance_disab1st: i32,
                      _allowance_disab2nd: i32,
                      _allowance_disab3rd: i32,
                      _allowance_study: i32,
                      _allowance_child1st: i32,
                      _allowance_child2nd: i32,
                      _allowance_child3rd: i32,
                      _factor_advances: Decimal,
                      _factor_withhold: Decimal,
                      _factor_solitary: Decimal,
                      _min_amount_of_tax_bonus: i32,
                      _max_amount_of_tax_bonus: i32,
                      _margin_income_of_tax_bonus: i32,
                      _margin_income_of_rounding: i32,
                      _margin_income_of_withhold: i32,
                      _margin_income_of_solitary: i32,
                      _margin_income_of_wth_emp: i32,
                      _margin_income_of_wth_agr: i32) -> PropsTaxing {
        PropsTaxing {
            version: _version,
            allowance_payer: _allowance_payer,
            allowance_disab1st: _allowance_disab1st,
            allowance_disab2nd: _allowance_disab2nd,
            allowance_disab3rd: _allowance_disab3rd,
            allowance_study: _allowance_study,
            allowance_child1st: _allowance_child1st,
            allowance_child2nd: _allowance_child2nd,
            allowance_child3rd: _allowance_child3rd,
            factor_advances: _factor_advances,
            factor_withhold: _factor_withhold,
            factor_solitary: _factor_solitary,
            min_amount_of_tax_bonus: _min_amount_of_tax_bonus,
            max_amount_of_tax_bonus: _max_amount_of_tax_bonus,
            margin_income_of_tax_bonus: _margin_income_of_tax_bonus,
            margin_income_of_rounding: _margin_income_of_rounding,
            margin_income_of_withhold: _margin_income_of_withhold,
            margin_income_of_solitary: _margin_income_of_solitary,
            margin_income_of_wth_emp: _margin_income_of_wth_emp,
            margin_income_of_wth_agr: _margin_income_of_wth_agr
        }
    }
    pub(crate) fn empty() -> PropsTaxing {
        PropsTaxing {
            version: VersionId::new(),
            allowance_payer: 0,
            allowance_disab1st: 0,
            allowance_disab2nd: 0,
            allowance_disab3rd: 0,
            allowance_study: 0,
            allowance_child1st: 0,
            allowance_child2nd: 0,
            allowance_child3rd: 0,
            factor_advances: dec!(0),
            factor_withhold: dec!(0),
            factor_solitary: dec!(0),
            min_amount_of_tax_bonus: 0,
            max_amount_of_tax_bonus: 0,
            margin_income_of_tax_bonus: 0,
            margin_income_of_rounding: 0,
            margin_income_of_withhold: 0,
            margin_income_of_solitary: 0,
            margin_income_of_wth_emp: 0,
            margin_income_of_wth_agr: 0
        }
    }
}

impl IProps for PropsTaxing {
    fn get_version(&self) -> VersionId {
        self.version
    }
}

impl IPropsTaxing for PropsTaxing {
    fn allowance_payer(&self) -> i32 {
        self.allowance_payer
    }

    fn allowance_disab1st(&self) -> i32 {
        self.allowance_disab1st
    }

    fn allowance_disab2nd(&self) -> i32 {
        self.allowance_disab2nd
    }

    fn allowance_disab3rd(&self) -> i32 {
        self.allowance_disab3rd
    }

    fn allowance_study(&self) -> i32 {
        self.allowance_study
    }

    fn allowance_child1st(&self) -> i32 {
        self.allowance_child1st
    }

    fn allowance_child2nd(&self) -> i32 {
        self.allowance_child2nd
    }

    fn allowance_child3rd(&self) -> i32 {
        self.allowance_child3rd
    }

    fn factor_advances(&self) -> Decimal {
        self.factor_advances
    }

    fn factor_withhold(&self) -> Decimal {
        self.factor_withhold
    }

    fn factor_solitary(&self) -> Decimal {
        self.factor_solitary
    }

    fn min_amount_of_tax_bonus(&self) -> i32 {
        self.min_amount_of_tax_bonus
    }

    fn max_amount_of_tax_bonus(&self) -> i32 {
        self.max_amount_of_tax_bonus
    }

    fn margin_income_of_tax_bonus(&self) -> i32 {
        self.margin_income_of_tax_bonus
    }

    fn margin_income_of_rounding(&self) -> i32 {
        self.margin_income_of_rounding
    }

    fn margin_income_of_withhold(&self) -> i32 {
        self.margin_income_of_withhold
    }

    fn margin_income_of_solitary(&self) -> i32 {
        self.margin_income_of_solitary
    }

    fn margin_income_of_wth_emp(&self) -> i32 {
        self.margin_income_of_wth_emp
    }

    fn margin_income_of_wth_agr(&self) -> i32 {
        self.margin_income_of_wth_agr
    }
}

