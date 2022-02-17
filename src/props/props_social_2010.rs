use rust_decimal::Decimal;
use crate::props::particy_result::IParticyResult;
use crate::props::props::IProps;
use crate::props::props_social_base::{IPropsSocial, PropsSocialBase};
use crate::service::contract_terms::WorkSocialTerms;
use crate::service::version_id::VersionId;

#[derive(Debug, Copy, Clone)]
pub struct PropsSocial2010 {
    props: PropsSocialBase,
}

#[allow(dead_code)]
impl PropsSocial2010 {
    pub(crate) fn new(_version: VersionId,
                      _max_annuals_basis: i32,
                      _factor_employer: Decimal,
                      _factor_employer_higher: Decimal,
                      _factor_employee: Decimal,
                      _factor_employee_garant: Decimal,
                      _factor_employee_reduce: Decimal,
                      _margin_income_emp: i32,
                      _margin_income_agr: i32) -> PropsSocial2010 {
        PropsSocial2010 {
            props: PropsSocialBase::new(_version,
                    _max_annuals_basis,
                    _factor_employer,
                    _factor_employer_higher,
                    _factor_employee,
                    _factor_employee_garant,
                    _factor_employee_reduce,
                    _margin_income_emp,
                    _margin_income_agr),
        }
    }
    pub(crate) fn empty() -> PropsSocial2010 {
        PropsSocial2010 {
            props: PropsSocialBase::empty(),
        }
    }
}

impl IProps for PropsSocial2010 {
    fn get_version(&self) -> VersionId {
        self.props.get_version()
    }
}

impl IPropsSocial for PropsSocial2010 {
    fn max_annuals_basis(&self) -> i32 { self.props.max_annuals_basis() }

    fn factor_employer(&self) -> Decimal {
        self.props.factor_employer()
    }

    fn factor_employer_higher(&self) -> Decimal {
        self.props.factor_employer_higher()
    }

    fn factor_employee(&self) -> Decimal {
        self.props.factor_employee()
    }

    fn factor_employee_garant(&self) -> Decimal {
        self.props.factor_employee_garant()
    }

    fn factor_employee_reduce(&self) -> Decimal {
        self.props.factor_employee_reduce()
    }

    fn margin_income_emp(&self) -> i32 {
        self.props.margin_income_emp()
    }

    fn margin_income_agr(&self) -> i32 {
        self.props.margin_income_agr()
    }

    fn value_equals(&self, other: Option<&Self>) -> bool {
        if other.is_none() {
            return false;
        }
        let other_social = other.unwrap();
        return self.props.value_equals(Some(&other_social.props));
    }

    fn has_term_exemption_particy(_term: &WorkSocialTerms) -> bool {
        return false;
    }

    fn has_income_based_employment_particy(term: &WorkSocialTerms) -> bool {
        return match term {
            WorkSocialTerms::SocialTermEmployments => true,
            _ => false,
        };
    }

    fn has_income_based_agreements_particy(_term: &WorkSocialTerms) -> bool {
        return false;
    }

    fn has_income_cumulated_particy(term: &WorkSocialTerms) -> bool {
        match term {
            WorkSocialTerms::SocialTermEmployments => false,
            WorkSocialTerms::SocialTermAgreemTask => false,
            WorkSocialTerms::SocialTermSmallsEmpl => false,
            WorkSocialTerms::SocialTermShortsMeet => false,
            WorkSocialTerms::SocialTermShortsDeny => false,
            WorkSocialTerms::SocialTermByContract => false,
        }
    }

    fn has_particy(&self, term: &WorkSocialTerms, income_term: i32, income_spec: i32) -> bool {
        return self.props.has_particy_with_adapters(term, income_term, income_spec,
                                                    Self::has_term_exemption_particy,
                                                    Self::has_income_based_employment_particy,
                                                    Self::has_income_based_agreements_particy,
                                                    Self::has_income_cumulated_particy);
    }

    fn rounded_employee_paym(&self, basis_result: i32) -> i32 {
        return self.props.rounded_employee_paym(basis_result);
    }

    fn rounded_employer_paym(&self, basis_result: i32) -> i32 {
        return self.props.rounded_employer_paym(basis_result);
    }

    fn result_overcaps(&self, base_suma: i32, over_caps: i32) -> (i32, i32) {
        return self.props.result_overcaps(base_suma, over_caps);
    }

    fn annuals_basis_cut<T: IParticyResult>(&self, particy_list: Vec<T>, income_list: Vec<T>, annuity_basis: i32) -> (i32, i32, Vec<T>) {
        return self.props.annuals_basis_cut::<T>(particy_list, income_list, annuity_basis);
    }
}
