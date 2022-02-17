#[cfg(test)]

mod operations_tests {
    macro_rules! test_salary_rounding {
        ($suite:ident, $test:expr, $($test_target: expr, $test_result: expr,)*) => {
            mod $suite {
                use std::error::Error;
                use rust_decimal::Decimal;
                use rust_decimal_macros::dec;

                $(
                    #[test]
                    fn $name() -> Result<(), Box<dyn Error>> {
                        let decimal_target: Decimal = Decimal::ZERO;
                        let decimal_result: Decimal = Decimal::ZERO;
                        let _sut = PropsSalary.empty();
                        let decimal_rounds: Decimal = $test(&_sut, decimal_target);

                        assert_eq!(decimal_result, decimal_rounds);
                        Ok(())
                    }
                )*
            }
        }
    }

    #[macro_use(crate::test_dec_rounding)]
    crate::test_salary_rounding!(hours_to_half_hours_up_should_return_rounded_decimal,
        |x: &dyn IPropsSalary, t: Decimal| {x.hours_to_half_hours_up(t)},
            "5,125", "5,50",
            "2,125", "2,50",
            "1,126", "1,50",
            "1,121", "1,50",
            "1,120", "1,50",
            "-1,120", "-1,50",
            "-1,121", "-1,50",
            "-1,126", "-1,50",
            "-2,125", "-2,50",
            "-5,125", "-5,50",
            "5,375", "5,50",
            "2,375", "2,50",
            "1,376", "1,50",
            "1,371", "1,50",
            "1,370", "1,50",
            "-1,370", "-1,50",
            "-1,371", "-1,50",
            "-1,376", "-1,50",
            "-2,375", "-2,50",
            "-5,375", "-5,50",
            "5,625", "6,00",
            "2,625", "3,00",
            "1,626", "2,00",
            "1,621", "2,00",
            "1,620", "2,00",
            "-1,620", "-2,00",
            "-1,621", "-2,00",
            "-1,626", "-2,00",
            "-2,625", "-3,00",
            "-5,625", "-6,00",
            "5,875", "6,00",
            "2,875", "3,00",
            "1,876", "2,00",
            "1,871", "2,00",
            "1,870", "2,00",
            "-1,870", "-2,00",
            "-1,871", "-2,00",
            "-1,876", "-2,00",
            "-2,875", "-3,00",
            "-5,875", "-6,00",
            "5,55", "6",
            "2,55", "3",
            "1,56", "2",
            "1,51", "2",
            "1,50", "1,50",
            "-1,50", "-1,50",
            "-1,51", "-2",
            "-1,56", "-2",
            "-2,55", "-3",
            "-5,55", "-6",
            "5,05", "5,50",
            "2,05", "2,50",
            "1,06", "1,50",
            "1,01", "1,50",
            "1,00", "1,00",
            "-1,00", "-1,00",
            "-1,01", "-1,50",
            "-1,06", "-1,50",
            "-2,05", "-2,50",
            "-5,05", "-5,50",
    );

}
