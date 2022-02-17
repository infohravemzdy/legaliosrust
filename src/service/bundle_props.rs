use crate::props::props_health::PropsHealth;
use crate::service::period::{IPeriod, Period};
use crate::props::props_salary::PropsSalary;
use crate::props::props_social::PropsSocial;
use crate::props::props_taxing::PropsTaxing;

pub trait IBundleProps {
    fn period_props(&self) -> Period;
    fn salary_props(&self) -> PropsSalary;
    fn health_props(&self) -> PropsHealth;
    fn social_props(&self) -> PropsSocial;
    fn taxing_props(&self) -> PropsTaxing;

    fn get_period_year(&self) -> i16;
    fn get_period_month(&self) -> i16;
    fn get_period_code(&self) -> i32;
}

pub struct BundleProps {
    period_props: Period,
    salary_props: PropsSalary,
    health_props: PropsHealth,
    social_props: PropsSocial,
    taxing_props: PropsTaxing,
}

impl BundleProps {
    pub(crate) fn new(period: &dyn IPeriod, salary: PropsSalary, health: PropsHealth, social: PropsSocial, taxing: PropsTaxing) -> BundleProps {
        BundleProps {
            period_props: Period::get(period.get_code()),
            salary_props: salary,
            health_props: health,
            social_props: social,
            taxing_props: taxing,
        }
    }
    pub fn empty(period: &dyn IPeriod) -> BundleProps {
        BundleProps {
            period_props: Period::get(period.get_code()),
            salary_props: PropsSalary::empty(),
            health_props: PropsHealth::empty(),
            social_props: PropsSocial::empty(),
            taxing_props: PropsTaxing::empty(),
        }
    }
}

impl IBundleProps for BundleProps {
    fn period_props(&self) -> Period {
        self.period_props
    }
    fn salary_props(&self) -> PropsSalary {
        self.salary_props
    }
    fn health_props(&self) -> PropsHealth {
        self.health_props
    }
    fn social_props(&self) -> PropsSocial {
        self.social_props
    }
    fn taxing_props(&self) -> PropsTaxing {
        self.taxing_props
    }
    fn get_period_year(&self) -> i16 {
        self.period_props.year()
    }
    fn get_period_month(&self) -> i16 {
        self.period_props.month()
    }
    fn get_period_code(&self) -> i32 {
        self.period_props.get_code()
    }
}