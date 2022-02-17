use crate::service::bundle_props::{BundleProps};
use crate::factories::provider_factory::{salary_factory, health_factory, social_factory, taxing_factory, IProviderFactory, ProviderSalaryFactory, ProviderHealthFactory, ProviderSocialFactory, ProviderTaxingFactory};
use crate::props::props::IProps;
use crate::props::props_health::{PropsHealth};
use crate::props::props_salary::{PropsSalary};
use crate::props::props_social::PropsSocial;
use crate::props::props_taxing::{PropsTaxing};
use crate::service::period::IPeriod;

pub(crate) trait  IBundleBuilder {
    fn get_bundle(&self, _period: &dyn IPeriod) -> Option<BundleProps>;
}

pub(crate) struct BundleBuilder {
    salary_factory: ProviderSalaryFactory,
    health_factory: ProviderHealthFactory,
    social_factory: ProviderSocialFactory,
    taxing_factory: ProviderTaxingFactory,
}

#[allow(dead_code)]
impl BundleBuilder {
    pub fn new() -> BundleBuilder {
        BundleBuilder {
            salary_factory: salary_factory(),
            health_factory: health_factory(),
            social_factory: social_factory(),
            taxing_factory: taxing_factory(),
        }
    }

    fn get_salary_props(&self, _period: &dyn IPeriod) -> PropsSalary {
        self.salary_factory.get_props(_period)
    }

    fn get_health_props(&self, _period: &dyn IPeriod) -> PropsHealth {
        self.health_factory.get_props(_period)
    }

    fn get_social_props(&self, _period: &dyn IPeriod) -> PropsSocial {
        self.social_factory.get_props(_period)
    }

    fn get_taxing_props(&self, _period: &dyn IPeriod) -> PropsTaxing {
        self.taxing_factory.get_props(_period)
    }
    fn is_null_or_empty(props: &dyn IProps) -> bool {
        const MIN_VERSION: i16 = 2010;
        let version = props.get_version().value;
        version == 0 || version < MIN_VERSION
     }
    fn is_valid_bundle(salary: &dyn IProps, health: &dyn IProps, social: &dyn IProps, taxing: &dyn IProps) -> bool {
        if BundleBuilder::is_null_or_empty(salary) {
             return false;
        }
        if BundleBuilder::is_null_or_empty(health) {
            return false;
        }
        if BundleBuilder::is_null_or_empty(social) {
            return false;
        }
        if BundleBuilder::is_null_or_empty(taxing) {
            return false;
        }
        true
    }
}

impl IBundleBuilder for BundleBuilder {
    fn get_bundle(&self, _period: &dyn IPeriod) -> Option<BundleProps> {
        let salary= self.get_salary_props(_period);
        let health= self.get_health_props(_period);
        let social= self.get_social_props(_period);
        let taxing= self.get_taxing_props(_period);

        if BundleBuilder::is_valid_bundle(&salary, &health, &social, &taxing) {
            return Some(BundleProps::new(_period, salary, health, social, taxing));
        }
        None
    }
}