﻿use rust_decimal::Decimal;
use crate::props::props::IProps;
use crate::props::props_taxing::{PropsTaxing};
use crate::providers::history_const_taxing::HistoryConstTaxing;
use crate::providers::period_2019::history_const_taxing_2019::{HistoryConstTaxing2019, HistoryConstTaxing2019var05};
use crate::providers::props_provider::IPropsProvider;
use crate::service::period::IPeriod;
use crate::service::version_id::VersionId;

pub(crate) struct ProviderTaxing2019 {
    version: VersionId
}

#[allow(dead_code)]
impl ProviderTaxing2019 {
    pub(crate) fn new() -> ProviderTaxing2019 {
        ProviderTaxing2019 {
            version: VersionId::get(HistoryConstTaxing2019::VERSION_CODE)
        }
    }
    fn allowance_payer(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2019::ALLOWANCE_PAYER
    }

    fn allowance_disab1st(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2019::ALLOWANCE_DISAB_1ST
    }

    fn allowance_disab2nd(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2019::ALLOWANCE_DISAB_2ND
    }

    fn allowance_disab3rd(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2019::ALLOWANCE_DISAB_3RD
    }

    fn allowance_study(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2019::ALLOWANCE_STUDY
    }

    fn allowance_child1st(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2019::ALLOWANCE_CHILD_1ST
    }

    fn allowance_child2nd(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2019::ALLOWANCE_CHILD_2ND
    }

    fn allowance_child3rd(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2019::ALLOWANCE_CHILD_3RD
    }

    fn factor_advances(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstTaxing2019::FACTOR_ADVANCES
    }

    fn factor_withhold(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstTaxing2019::FACTOR_WITHHOLD
    }

    fn factor_solitary(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstTaxing2019::FACTOR_SOLITARY
    }

    fn min_amount_of_tax_bonus(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2019::MIN_AMOUNT_OF_TAXBONUS
    }

    fn max_amount_of_tax_bonus(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2019::MAX_AMOUNT_OF_TAXBONUS
    }

    fn margin_income_of_tax_bonus(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2019::MARGIN_INCOME_OF_TAXBONUS
    }

    fn margin_income_of_rounding(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2019::MARGIN_INCOME_OF_ROUNDING
    }

    fn margin_income_of_withhold(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2019::MARGIN_INCOME_OF_WITHHOLD
    }

    fn margin_income_of_solitary(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2019::MARGIN_INCOME_OF_SOLITARY
    }

    fn margin_income_of_wth_emp(&self, _period: &dyn IPeriod) -> i32 {
        if _period.is_period_greater_or_equal_than(2019, 5) {
            return HistoryConstTaxing2019var05::MARGIN_INCOME_OF_WHT_EMP;
        }
        HistoryConstTaxing2019::MARGIN_INCOME_OF_WHT_EMP
    }

    fn margin_income_of_wth_agr(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2019::MARGIN_INCOME_OF_WHT_AGR
    }
}

impl IProps for ProviderTaxing2019 {
    fn get_version(&self) -> VersionId {
        self.version
    }
}

impl IPropsProvider<PropsTaxing> for ProviderTaxing2019 {
    fn get_version(&self) -> VersionId {
        self.version
    }
    fn get_props(&self, _period: &dyn IPeriod) -> PropsTaxing {
        PropsTaxing::new(self.version,
                         self.allowance_payer(_period),
                         self.allowance_disab1st(_period),
                         self.allowance_disab2nd(_period),
                         self.allowance_disab3rd(_period),
                         self.allowance_study(_period),
                         self.allowance_child1st(_period),
                         self.allowance_child2nd(_period),
                         self.allowance_child3rd(_period),
                         self.factor_advances(_period),
                         self.factor_withhold(_period),
                         self.factor_solitary(_period),
                         self.min_amount_of_tax_bonus(_period),
                         self.max_amount_of_tax_bonus(_period),
                         self.margin_income_of_tax_bonus(_period),
                         self.margin_income_of_rounding(_period),
                         self.margin_income_of_withhold(_period),
                         self.margin_income_of_solitary(_period),
                         self.margin_income_of_wth_emp(_period),
                         self.margin_income_of_wth_agr(_period))
    }
}