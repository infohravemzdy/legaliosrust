#[cfg(test)]
mod factories_history_tests {
    use std::error::Error;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use crate::factories::provider_factory::{health_factory, IProviderFactory, ProviderHealthFactory, ProviderSalaryFactory, ProviderSocialFactory, ProviderTaxingFactory, salary_factory, social_factory, taxing_factory};
    use crate::props::props_health::IPropsHealth;
    use crate::props::props_salary::IPropsSalary;
    use crate::props::props_social::IPropsSocial;
    use crate::props::props_taxing::IPropsTaxing;
    use crate::service::period;
    use crate::service::period::{IPeriod, Period};

    //#[cfg(test_history)]
    macro_rules! test_factories_history {
        ($suite:ident, $min_year: expr, $max_year: expr) => {
            mod $suite {
                use std::error::Error;
                use rust_decimal::Decimal;
                use rust_decimal_macros::dec;

                $(
                    #[test]
                    fn $name() -> Result<(), Box<dyn Error>> {
                        let decimal_target: Decimal = dec!(0);
                        let decimal_result: Decimal = dec!(0);
                        let decimal_rounds: Decimal = dec!(0);

                        assert_eq!(decimal_result, decimal_rounds);
                        Ok(())
                    }
                )*
            }
        }
    }

    pub(crate) fn test_factories_history_test(min_year: i16, max_year: i16) -> Result<(), Box<dyn Error>> {
        let health_min_monthly_basis:i32            = 101;
        let health_max_annuals_basis:i32            = 102;
        let health_lim_monthly_state:i32            = 103;
        let health_lim_monthly_dis50:i32            = 104;
        let health_factor_compound:i32              = 105;
        let health_factor_employee:i32              = 106;
        let health_margin_income_emp:i32            = 107;
        let health_margin_income_agr:i32            = 108;
;
        let salary_working_shift_week:i32           = 201;
        let salary_working_shift_time:i32           = 202;
        let salary_min_monthly_wage:i32             = 203;
        let salary_min_hourly_wage:i32              = 204;
;
        let social_max_annuals_basis:i32            = 301;
        let social_factor_employer:i32              = 302;
        let social_factor_employer_higher:i32       = 303;
        let social_factor_employee:i32              = 304;
        let social_factor_employee_garant:i32       = 305;
        let social_factor_employee_reduce:i32       = 306;
        let social_margin_income_emp:i32            = 307;
        let social_margin_income_agr:i32            = 308;
;
        let taxing_allowance_payer:i32              = 401;
        let taxing_allowance_disab_1st:i32          = 402;
        let taxing_allowance_disab_2nd:i32          = 403;
        let taxing_allowance_disab_3rd:i32          = 404;
        let taxing_allowance_study:i32              = 405;
        let taxing_allowance_child_1st:i32          = 406;
        let taxing_allowance_child_2nd:i32          = 407;
        let taxing_allowance_child_3rd:i32          = 408;
        let taxing_factor_advances:i32              = 409;
        let taxing_factor_withhold:i32              = 410;
        let taxing_factor_solidary:i32              = 411;
        let taxing_factor_taxrate2:i32              = 412;
        let taxing_min_amount_of_taxbonus:i32       = 413;
        let taxing_max_amount_of_taxbonus:i32       = 414;
        let taxing_margin_income_of_taxbonus:i32    = 415;
        let taxing_margin_income_of_rounding:i32    = 416;
        let taxing_margin_income_of_withhold:i32    = 417;
        let taxing_margin_income_of_solidary:i32    = 418;
        let taxing_margin_income_of_taxrate2:i32    = 419;
        let taxing_margin_income_of_wht_emp:i32     = 420;
        let taxing_margin_income_of_wht_agr:i32     = 421;

        let _sut_salary : ProviderSalaryFactory = salary_factory();
        let _sut_health : ProviderHealthFactory = health_factory();
        let _sut_social : ProviderSocialFactory = social_factory();
        let _sut_taxing : ProviderTaxingFactory = taxing_factory();


        let mut file = create_report_file(format!("history_{}_{}.xls", min_year, max_year));
        write_report_head(&mut file);

        let mut header_data:Vec<(i32, bool)> = vec![];
        for exp_year in min_year..max_year {
            let mut year_with_changes = false;

            let test_period = period::Period::get_with_year_month(exp_year, 1);

            let test_salary_prop = _sutSalary.get_props(testPeriod);
            let test_health_prop = _sutHealth.get_props(testPeriod);
            let test_social_prop = _sutSocial.get_props(testPeriod);
            let test_taxing_prop = _sutTaxing.get_props(testPeriod);

            for test_month in 2i16..12i16 {
                let next_period = period::Period::get_with_year_month(test_year, test_month);

                let test_salary_next = _sutSalary.get_props(next_period);
                let test_health_next = _sutHealth.get_props(next_period);
                let test_social_next = _sutSocial.get_props(next_period);
                let test_taxing_next = _sutTaxing.get_props(next_period);

                if test_salary_next.value_equals(test_salary_prop) == false {
                    year_with_changes = true;
                }
                if test_health_next.value_equals(test_health_prop) == false {
                    year_with_changes = true;
                }
                if test_social_next.value_equals(test_social_prop) == false {
                    year_with_changes = true;
                }
                if test_taxing_next.value_equals(test_taxing_prop) == false {
                    year_with_changes = true;
                }
                test_salary_prop = test_salary_next;
                test_health_prop = test_health_next;
                test_social_prop = test_social_next;
                test_taxing_prop = test_taxing_next;
            }
            header_data.push((test_year, year_with_changes));
        }
        export_history_start(test_protokol, header_data);

        let mut vect_health_min_monthly_basis: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_health_max_annuals_basis: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_health_lim_monthly_state: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_health_lim_monthly_dis50: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_health_factor_compound: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_health_factor_employee: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_health_margin_income_emp: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_health_margin_income_agr: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_salary_working_shift_week: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_salary_working_shift_time: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_salary_min_monthly_wage: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_salary_min_hourly_wage: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_max_annuals_basis: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_factor_employer: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_factor_employer_higher: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_factor_employee: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_factor_employee_garant: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_factor_employee_reduce: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_margin_income_emp: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_margin_income_agr: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_payer: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_disab_1st: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_disab_2nd: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_disab_3rd: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_study: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_child_1st: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_child_2nd: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_child_3rd: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_factor_advances: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_factor_withhold: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_factor_solidary: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_factor_taxrate2: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_min_amount_of_taxbonus: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_max_amount_of_taxbonus: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_margin_income_of_taxbonus: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_margin_income_of_rounding: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_margin_income_of_withhold: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_margin_income_of_solidary: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_margin_income_of_taxrate2: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_margin_income_of_wht_emp: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_margin_income_of_wht_agr: Vec<(i16, i16, String, String)> = vec![];

        let table_data: Vec<(i32, Vec<(i16, i16, String, String)>)> = vec![
            (health_min_monthly_basis, vect_health_min_monthly_basis),
            (health_max_annuals_basis, vect_health_max_annuals_basis),
            (health_lim_monthly_state, vect_health_lim_monthly_state),
            (health_lim_monthly_dis50, vect_health_lim_monthly_dis50),
            (health_factor_compound, vect_health_factor_compound),
            (health_factor_employee, vect_health_factor_employee),
            (health_margin_income_emp, vect_health_margin_income_emp),
            (health_margin_income_agr, vect_health_margin_income_agr),
            (salary_working_shift_week, vect_salary_working_shift_week),
            (salary_working_shift_time, vect_salary_working_shift_time),
            (salary_min_monthly_wage, vect_salary_min_monthly_wage),
            (salary_min_hourly_wage, vect_salary_min_hourly_wage),
            (social_max_annuals_basis, vect_social_max_annuals_basis),
            (social_factor_employer, vect_social_factor_employer),
            (social_factor_employer_higher, vect_social_factor_employer_higher),
            (social_factor_employee, vect_social_factor_employee),
            (social_factor_employee_garant, vect_social_factor_employee_garant),
            (social_factor_employee_reduce, vect_social_factor_employee_reduce),
            (social_margin_income_emp, vect_social_margin_income_emp),
            (social_margin_income_agr, vect_social_margin_income_agr),
            (taxing_allowance_payer, vect_taxing_allowance_payer),
            (taxing_allowance_disab_1st, vect_taxing_allowance_disab_1st),
            (taxing_allowance_disab_2nd, vect_taxing_allowance_disab_2nd),
            (taxing_allowance_disab_3rd, vect_taxing_allowance_disab_3rd),
            (taxing_allowance_study, vect_taxing_allowance_study),
            (taxing_allowance_child_1st, vect_taxing_allowance_child_1st),
            (taxing_allowance_child_2nd, vect_taxing_allowance_child_2nd),
            (taxing_allowance_child_3rd, vect_taxing_allowance_child_3rd),
            (taxing_factor_advances, vect_taxing_factor_advances),
            (taxing_factor_withhold, vect_taxing_factor_withhold),
            (taxing_factor_solidary, vect_taxing_factor_solidary),
            (taxing_factor_taxrate2, vect_taxing_factor_taxrate2),
            (taxing_min_amount_of_taxbonus, vect_taxing_min_amount_of_taxbonus),
            (taxing_max_amount_of_taxbonus, vect_taxing_max_amount_of_taxbonus),
            (taxing_margin_income_of_taxbonus, vect_taxing_margin_income_of_taxbonus),
            (taxing_margin_income_of_rounding, vect_taxing_margin_income_of_rounding),
            (taxing_margin_income_of_withhold, vect_taxing_margin_income_of_withhold),
            (taxing_margin_income_of_solidary, vect_taxing_margin_income_of_solidary),
            (taxing_margin_income_of_taxrate2, vect_taxing_margin_income_of_taxrate2),
            (taxing_margin_income_of_wht_emp, vect_taxing_margin_income_of_wht_emp),
            (taxing_margin_income_of_wht_agr, vect_taxing_margin_income_of_wht_agr),
        ];
        for test_year in min_year..max_year {
            let mut mes_health_min_monthly_basis = 0;
            let mut mes_health_max_annuals_basis = 0;
            let mut mes_health_lim_monthly_state = 0;
            let mut mes_health_lim_monthly_dis50 = 0;
            let mut mes_health_factor_compound = 0;
            let mut mes_health_factor_employee = 0;
            let mut mes_health_margin_income_emp = 0;
            let mut mes_health_margin_income_agr = 0;
            let mut mes_salary_working_shift_week = 0;
            let mut mes_salary_working_shift_time = 0;
            let mut mes_salary_min_monthly_wage = 0;
            let mut mes_salary_min_hourly_wage = 0;
            let mut mes_social_max_annuals_basis = 0;
            let mut mes_social_factor_employer = 0;
            let mut mes_social_factor_employer_higher = 0;
            let mut mes_social_factor_employee = 0;
            let mut mes_social_factor_employee_garant = 0;
            let mut mes_social_factor_employee_reduce = 0;
            let mut mes_social_margin_income_emp = 0;
            let mut mes_social_margin_income_agr = 0;
            let mut mes_taxing_allowance_payer = 0;
            let mut mes_taxing_allowance_disab_1st = 0;
            let mut mes_taxing_allowance_disab_2nd = 0;
            let mut mes_taxing_allowance_disab_3rd = 0;
            let mut mes_taxing_allowance_study = 0;
            let mut mes_taxing_allowance_child_1st = 0;
            let mut mes_taxing_allowance_child_2nd = 0;
            let mut mes_taxing_allowance_child_3rd = 0;
            let mut mes_taxing_factor_advances = 0;
            let mut mes_taxing_factor_withhold = 0;
            let mut mes_taxing_factor_solidary = 0;
            let mut mes_taxing_factor_taxrate2 = 0;
            let mut mes_taxing_min_amount_of_taxbonus = 0;
            let mut mes_taxing_max_amount_of_taxbonus = 0;
            let mut mes_taxing_margin_income_of_taxbonus = 0;
            let mut mes_taxing_margin_income_of_rounding = 0;
            let mut mes_taxing_margin_income_of_withhold = 0;
            let mut mes_taxing_margin_income_of_solidary = 0;
            let mut mes_taxing_margin_income_of_taxrate2 = 0;
            let mut mes_taxing_margin_income_of_wht_emp = 0;
            let mut mes_taxing_margin_income_of_wht_agr = 0;

            let test_period: &dyn IPeriod = &period::Period::get_with_year_month(test_year, 1);

            let mut test_salary_prop = _sut_salary.get_props(test_period);
            let mut test_health_prop = _sut_health.get_props(test_period);
            let mut test_social_prop = _sut_social.get_props(test_period);
            let mut test_taxing_prop = _sut_taxing.get_props(test_period);

            let jan_health_min_monthly_basis = props_int_value_to_string(test_health_prop.min_monthly_basis());
            let jan_health_max_annuals_basis = props_int_value_to_string(test_health_prop.max_annuals_basis());
            let jan_health_lim_monthly_state = props_int_value_to_string(test_health_prop.lim_monthly_state());
            let jan_health_lim_monthly_dis50 = props_int_value_to_string(test_health_prop.lim_monthly_dis50());
            let jan_health_factor_compound = props_dec_value_to_string(test_health_prop.factor_compound());
            let jan_health_factor_employee = props_dec_value_to_string(test_health_prop.factor_employee());
            let jan_health_margin_income_emp = props_int_value_to_string(test_health_prop.margin_income_emp());
            let jan_health_margin_income_agr = props_int_value_to_string(test_health_prop.margin_income_agr());
            let jan_salary_working_shift_week = props_int_value_to_string(test_salary_prop.working_shift_week());
            let jan_salary_working_shift_time = props_int_value_to_string(test_salary_prop.working_shift_time());
            let jan_salary_min_monthly_wage = props_int_value_to_string(test_salary_prop.min_monthly_wage());
            let jan_salary_min_hourly_wage = props_int_value_to_string(test_salary_prop.min_hourly_wage());
            let jan_social_max_annuals_basis = props_int_value_to_string(test_social_prop.max_annuals_basis());
            let jan_social_factor_employer = props_dec_value_to_string(test_social_prop.factor_employer());
            let jan_social_factor_employer_higher = props_dec_value_to_string(test_social_prop.factor_employer_higher());
            let jan_social_factor_employee = props_dec_value_to_string(test_social_prop.factor_employee());
            let jan_social_factor_employee_garant = props_dec_value_to_string(test_social_prop.factor_employee_garant());
            let jan_social_factor_employee_reduce = props_dec_value_to_string(test_social_prop.factor_employee_reduce());
            let jan_social_margin_income_emp = props_int_value_to_string(test_social_prop.margin_income_emp());
            let jan_social_margin_income_agr = props_int_value_to_string(test_social_prop.margin_income_agr());
            let jan_taxing_allowance_payer = props_int_value_to_string(test_taxing_prop.allowance_payer());
            let jan_taxing_allowance_disab_1st = props_int_value_to_string(test_taxing_prop.allowance_disab1st());
            let jan_taxing_allowance_disab_2nd = props_int_value_to_string(test_taxing_prop.allowance_disab2nd());
            let jan_taxing_allowance_disab_3rd = props_int_value_to_string(test_taxing_prop.allowance_disab3rd());
            let jan_taxing_allowance_study = props_int_value_to_string(test_taxing_prop.allowance_study());
            let jan_taxing_allowance_child_1st = props_int_value_to_string(test_taxing_prop.allowance_child1st());
            let jan_taxing_allowance_child_2nd = props_int_value_to_string(test_taxing_prop.allowance_child2nd());
            let jan_taxing_allowance_child_3rd = props_int_value_to_string(test_taxing_prop.allowance_child3rd());
            let jan_taxing_factor_advances = props_dec_value_to_string(test_taxing_prop.factor_advances());
            let jan_taxing_factor_withhold = props_dec_value_to_string(test_taxing_prop.factor_withhold());
            let jan_taxing_factor_solidary = props_dec_value_to_string(test_taxing_prop.factor_solidary());
            let jan_taxing_factor_taxrate2 = props_dec_value_to_string(test_taxing_prop.factor_taxrate2());
            let jan_taxing_min_amount_of_taxbonus = props_int_value_to_string(test_taxing_prop.min_amount_of_tax_bonus());
            let jan_taxing_max_amount_of_taxbonus = props_int_value_to_string(test_taxing_prop.max_amount_of_tax_bonus());
            let jan_taxing_margin_income_of_taxbonus = props_int_value_to_string(test_taxing_prop.margin_income_of_tax_bonus());
            let jan_taxing_margin_income_of_rounding = props_int_value_to_string(test_taxing_prop.margin_income_of_rounding());
            let jan_taxing_margin_income_of_withhold = props_int_value_to_string(test_taxing_prop.margin_income_of_withhold());
            let jan_taxing_margin_income_of_solidary = props_int_value_to_string(test_taxing_prop.margin_income_of_solidary());
            let jan_taxing_margin_income_of_taxrate2 = props_int_value_to_string(test_taxing_prop.margin_income_of_taxrate2());
            let jan_taxing_margin_income_of_wht_emp = props_int_value_to_string(test_taxing_prop.margin_income_of_wth_emp());
            let jan_taxing_margin_income_of_wht_agr = props_int_value_to_string(test_taxing_prop.margin_income_of_wth_agr());

            for test_month in 2i16..12i16 {
                let next_period: &dyn IPeriod  = &period::Period::get_with_year_month(test_year, test_month);
                
                let test_salary_next = _sut_salary.get_props(next_period);
                let test_health_next = _sut_health.get_props(next_period);
                let test_social_next = _sut_social.get_props(next_period);
                let test_taxing_next = _sut_taxing.get_props(next_period);

                if test_health_next.min_monthly_basis().eq(testHealthProp.min_monthly_basis())==false { mes_health_min_monthly_basis = test_month; }
                if test_health_next.max_annuals_basis().eq(testHealthProp.max_annuals_basis())==false { mes_health_max_annuals_basis = test_month; }
                if test_health_next.lim_monthly_state().eq(testHealthProp.lim_monthly_state())==false { mes_health_lim_monthly_state = test_month; }
                if test_health_next.lim_monthly_dis50().eq(testHealthProp.lim_monthly_dis50())==false { mes_health_lim_monthly_dis50 = test_month; }
                if test_health_next.factor_compound().eq(testHealthProp.factor_compound())==false { mes_health_factor_compound = test_month;}
                if test_health_next.factor_employee().eq(testHealthProp.factor_employee())==false { mes_health_factor_employee = test_month; }
                if test_health_next.margin_income_emp().eq(testHealthProp.margin_income_emp())==false { mes_health_margin_income_emp = test_month; }
                if test_health_next.margin_income_agr().eq(testHealthProp.margin_income_agr())==false { mes_health_margin_income_agr = test_month; }
                if test_salary_next.working_shift_week().eq(testSalaryProp.working_shift_week())==false { mes_salary_working_shift_week = test_month; }
                if test_salary_next.working_shift_time().eq(testSalaryProp.working_shift_time())==false { mes_salary_working_shift_time = test_month; }
                if test_salary_next.min_monthly_wage().eq(testSalaryProp.min_monthly_wage())==false { mes_salary_min_monthly_wage = test_month; }
                if test_salary_next.min_hourly_wage().eq(testSalaryProp.min_hourly_wage())==false { mes_salary_min_hourly_wage = test_month; }
                if test_social_next.max_annuals_basis().eq(testSocialProp.max_annuals_basis())==false { mes_social_max_annuals_basis = test_month; }
                if test_social_next.factor_employer().eq(testSocialProp.factor_employer())==false { mes_social_factor_employer = test_month; }
                if test_social_next.factor_employer_higher().eq(testSocialProp.factor_employer_higher())==false { mes_social_factor_employer_higher = test_month; }
                if test_social_next.factor_employee().eq(testSocialProp.factor_employee())==false { mes_social_factor_employee = test_month; }
                if test_social_next.factor_employee_garant().eq(testSocialProp.factor_employee_garant())==false { mes_social_factor_employee_reduce = test_month; }
                if test_social_next.factor_employee_reduce().eq(testSocialProp.factor_employee_reduce())==false { mes_social_factor_employee_garant = test_month; }
                if test_social_next.margin_income_emp().eq(testSocialProp.margin_income_emp())==false { mes_social_margin_income_emp = test_month; }
                if test_social_next.margin_income_agr().eq(testSocialProp.margin_income_agr())==false { mes_social_margin_income_agr = test_month; }
                if test_taxing_next.allowance_payer().eq(testTaxingProp.allowance_payer())==false { mes_taxing_allowance_payer = test_month; }
                if test_taxing_next.allowance_disab1st().eq(testTaxingProp.allowance_disab1st())==false { mes_taxing_allowance_disab_1st = test_month; }
                if test_taxing_next.allowance_disab2nd().eq(testTaxingProp.allowance_disab2nd())==false { mes_taxing_allowance_disab_2nd = test_month; }
                if test_taxing_next.allowance_disab3rd().eq(testTaxingProp.allowance_disab3rd())==false { mes_taxing_allowance_disab_3rd = test_month; }
                if test_taxing_next.allowance_study().eq(testTaxingProp.allowance_study())==false { mes_taxing_allowance_study = test_month; }
                if test_taxing_next.allowance_child1st().eq(testTaxingProp.allowance_child1st())==false { mes_taxing_allowance_child_1st = test_month; }
                if test_taxing_next.allowance_child2nd().eq(testTaxingProp.allowance_child2nd())==false { mes_taxing_allowance_child_2nd = test_month; }
                if test_taxing_next.allowance_child3rd().eq(testTaxingProp.allowance_child3rd())==false { mes_taxing_allowance_child_3rd = test_month; }
                if test_taxing_next.factor_advances().eq(testTaxingProp.factor_advances())==false { mes_taxing_factor_advances = test_month; }
                if test_taxing_next.factor_withhold().eq(testTaxingProp.factor_withhold())==false { mes_taxing_factor_withhold = test_month; }
                if test_taxing_next.factor_solidary().eq(testTaxingProp.factor_solidary())==false { mes_taxing_factor_solidary = test_month; }
                if test_taxing_next.factor_taxrate2().eq(testTaxingProp.factor_taxrate2())==false { mes_taxing_factor_taxrate2 = test_month; }
                if test_taxing_next.min_amount_of_tax_bonus().eq(testTaxingProp.min_amount_of_tax_bonus())==false { mes_taxing_min_amount_of_taxbonus = test_month; }
                if test_taxing_next.max_amount_of_tax_bonus().eq(testTaxingProp.max_amount_of_tax_bonus())==false { mes_taxing_max_amount_of_taxbonus = test_month; }
                if test_taxing_next.margin_income_of_tax_bonus().eq(testTaxingProp.margin_income_of_tax_bonus())==false { mes_taxing_margin_income_of_taxbonus = test_month; }
                if test_taxing_next.margin_income_of_rounding().eq(testTaxingProp.margin_income_of_rounding())==false { mes_taxing_margin_income_of_rounding = test_month; }
                if test_taxing_next.margin_income_of_withhold().eq(testTaxingProp.margin_income_of_withhold())==false { mes_taxing_margin_income_of_withhold = test_month; }
                if test_taxing_next.margin_income_of_solidary().eq(testTaxingProp.margin_income_of_solidary())==false { mes_taxing_margin_income_of_solidary = test_month; }
                if test_taxing_next.margin_income_of_taxrate2().eq(testTaxingProp.margin_income_of_taxrate2())==false { mes_taxing_margin_income_of_taxrate2 = test_month; }
                if test_taxing_next.margin_income_of_wth_emp().eq(testTaxingProp.margin_income_of_wth_emp())==false { mes_taxing_margin_income_of_wht_emp = test_month; }
                if test_taxing_next.margin_income_of_wth_agr().eq(testTaxingProp.margin_income_of_wth_agr())==false { mes_taxing_margin_income_of_wht_agr = test_month; }

                test_salary_prop = test_salary_next;
                test_health_prop = test_health_next;
                test_social_prop = test_social_next;
                test_taxing_prop = test_taxing_next;
            }
            vect_health_min_monthly_basis.push((test_year, mes_health_min_monthly_basis, jan_health_min_monthly_basis, props_int_value_to_string(test_health_prop.min_monthly_basis())));
            vect_health_max_annuals_basis.push((test_year, mes_health_max_annuals_basis, jan_health_max_annuals_basis, props_int_value_to_string(test_health_prop.max_annuals_basis())));
            vect_health_lim_monthly_state.push((test_year, mes_health_lim_monthly_state, jan_health_lim_monthly_state, props_int_value_to_string(test_health_prop.lim_monthly_state())));
            vect_health_lim_monthly_dis50.push((test_year, mes_health_lim_monthly_dis50, jan_health_lim_monthly_dis50, props_int_value_to_string(test_health_prop.lim_monthly_dis50())));
            vect_health_factor_compound.push((test_year, mes_health_factor_compound, jan_health_factor_compound, props_dec_value_to_string(test_health_prop.factor_compound())));
            vect_health_factor_employee.push((test_year, mes_health_factor_employee, jan_health_factor_employee, props_dec_value_to_string(test_health_prop.factor_employee())));
            vect_health_margin_income_emp.push((test_year, mes_health_margin_income_emp, jan_health_margin_income_emp, props_int_value_to_string(test_health_prop.margin_income_emp())));
            vect_health_margin_income_agr.push((test_year, mes_health_margin_income_agr, jan_health_margin_income_agr, props_int_value_to_string(test_health_prop.margin_income_agr())));
            vect_salary_working_shift_week.push((test_year, mes_salary_working_shift_week, jan_salary_working_shift_week, props_int_value_to_string(test_salary_prop.working_shift_week())));
            vect_salary_working_shift_time.push((test_year, mes_salary_working_shift_time, jan_salary_working_shift_time, props_int_value_to_string(test_salary_prop.working_shift_time())));
            vect_salary_min_monthly_wage.push((test_year, mes_salary_min_monthly_wage, jan_salary_min_monthly_wage, props_int_value_to_string(test_salary_prop.min_monthly_wage())));
            vect_salary_min_hourly_wage.push((test_year, mes_salary_min_hourly_wage, jan_salary_min_hourly_wage, props_int_value_to_string(test_salary_prop.min_hourly_wage())));
            vect_social_max_annuals_basis.push((test_year, mes_social_max_annuals_basis, jan_social_max_annuals_basis, props_int_value_to_string(test_social_prop.max_annuals_basis())));
            vect_social_factor_employer.push((test_year, mes_social_factor_employer, jan_social_factor_employer, props_dec_value_to_string(test_social_prop.factor_employer())));
            vect_social_factor_employer_higher.push((test_year, mes_social_factor_employer_higher, jan_social_factor_employer_higher, props_dec_value_to_string(test_social_prop.factor_employer_higher())));
            vect_social_factor_employee.push((test_year, mes_social_factor_employee, jan_social_factor_employee, props_dec_value_to_string(test_social_prop.factor_employee())));
            vect_social_factor_employee_garant.push((test_year, mes_social_factor_employee_garant, jan_social_factor_employee_garant, props_dec_value_to_string(test_social_prop.factor_employee_garant())));
            vect_social_factor_employee_reduce.push((test_year, mes_social_factor_employee_reduce, jan_social_factor_employee_reduce, props_dec_value_to_string(test_social_prop.factor_employee_reduce())));
            vect_social_margin_income_emp.push((test_year, mes_social_margin_income_emp, jan_social_margin_income_emp, props_int_value_to_string(test_social_prop.margin_income_emp())));
            vect_social_margin_income_agr.push((test_year, mes_social_margin_income_agr, jan_social_margin_income_agr, props_int_value_to_string(test_social_prop.margin_income_agr())));
            vect_taxing_allowance_payer.push((test_year, mes_taxing_allowance_payer, jan_taxing_allowance_payer, props_int_value_to_string(test_taxing_prop.allowance_payer())));
            vect_taxing_allowance_disab_1st.push((test_year, mes_taxing_allowance_disab_1st, jan_taxing_allowance_disab_1st, props_int_value_to_string(test_taxing_prop.allowance_disab1st())));
            vect_taxing_allowance_disab_2nd.push((test_year, mes_taxing_allowance_disab_2nd, jan_taxing_allowance_disab_2nd, props_int_value_to_string(test_taxing_prop.allowance_disab2nd())));
            vect_taxing_allowance_disab_3rd.push((test_year, mes_taxing_allowance_disab_3rd, jan_taxing_allowance_disab_3rd, props_int_value_to_string(test_taxing_prop.allowance_disab3rd())));
            vect_taxing_allowance_study.push((test_year, mes_taxing_allowance_study, jan_taxing_allowance_study, props_int_value_to_string(test_taxing_prop.allowance_study())));
            vect_taxing_allowance_child_1st.push((test_year, mes_taxing_allowance_child_1st, jan_taxing_allowance_child_1st, props_int_value_to_string(test_taxing_prop.allowance_child1st())));
            vect_taxing_allowance_child_2nd.push((test_year, mes_taxing_allowance_child_2nd, jan_taxing_allowance_child_2nd, props_int_value_to_string(test_taxing_prop.allowance_child2nd())));
            vect_taxing_allowance_child_3rd.push((test_year, mes_taxing_allowance_child_3rd, jan_taxing_allowance_child_3rd, props_int_value_to_string(test_taxing_prop.allowance_child3rd())));
            vect_taxing_factor_advances.push((test_year, mes_taxing_factor_advances, jan_taxing_factor_advances, props_dec_value_to_string(test_taxing_prop.factor_advances())));
            vect_taxing_factor_withhold.push((test_year, mes_taxing_factor_withhold, jan_taxing_factor_withhold, props_dec_value_to_string(test_taxing_prop.factor_withhold())));
            vect_taxing_factor_solidary.push((test_year, mes_taxing_factor_solidary, jan_taxing_factor_solidary, props_dec_value_to_string(test_taxing_prop.factor_solidary())));
            vect_taxing_factor_taxrate2.push((test_year, mes_taxing_factor_taxrate2, jan_taxing_factor_taxrate2, props_dec_value_to_string(test_taxing_prop.factor_taxrate2())));
            vect_taxing_min_amount_of_taxbonus.push((test_year, mes_taxing_min_amount_of_taxbonus, jan_taxing_min_amount_of_taxbonus, props_int_value_to_string(test_taxing_prop.min_amount_of_tax_bonus())));
            vect_taxing_max_amount_of_taxbonus.push((test_year, mes_taxing_max_amount_of_taxbonus, jan_taxing_max_amount_of_taxbonus, props_int_value_to_string(test_taxing_prop.max_amount_of_tax_bonus())));
            vect_taxing_margin_income_of_taxbonus.push((test_year, mes_taxing_margin_income_of_taxbonus, jan_taxing_margin_income_of_taxbonus, props_int_value_to_string(test_taxing_prop.margin_income_of_tax_bonus())));
            vect_taxing_margin_income_of_rounding.push((test_year, mes_taxing_margin_income_of_rounding, jan_taxing_margin_income_of_rounding, props_int_value_to_string(test_taxing_prop.margin_income_of_rounding())));
            vect_taxing_margin_income_of_withhold.push((test_year, mes_taxing_margin_income_of_withhold, jan_taxing_margin_income_of_withhold, props_int_value_to_string(test_taxing_prop.margin_income_of_withhold())));
            vect_taxing_margin_income_of_solidary.push((test_year, mes_taxing_margin_income_of_solidary, jan_taxing_margin_income_of_solidary, props_int_value_to_string(test_taxing_prop.margin_income_of_solidary())));
            vect_taxing_margin_income_of_taxrate2.push((test_year, mes_taxing_margin_income_of_taxrate2, jan_taxing_margin_income_of_taxrate2, props_int_value_to_string(test_taxing_prop.margin_income_of_taxrate2())));
            vect_taxing_margin_income_of_wht_emp.push((test_year, mes_taxing_margin_income_of_wht_emp, jan_taxing_margin_income_of_wht_emp, props_int_value_to_string(test_taxing_prop.margin_income_of_wth_emp())));
            vect_taxing_margin_income_of_wht_agr.push((test_year, mes_taxing_margin_income_of_wht_agr, jan_taxing_margin_income_of_wht_agr, props_int_value_to_string(test_taxing_prop.margin_income_of_wth_agr())));
        }
        for data in table_data {
            export_history_term(test_protokol, &header_data, data);
        }
        Ok(())
    }

    #[macro_use(crate::test_factories_history)]
    crate::test_factories_history!(get_props_should_export_history, 2010, 2022);
}
