#[cfg(test)]

mod operations_tests {
    macro_rules! test_dec_rounding {
        ($suite:ident, $test:expr, $($test_target: expr, $test_result: expr,)*) => {
            mod $suite {
                use std::error::Error;
                use rust_decimal::Decimal;
                use rust_decimal_macros::dec;

                $(
                    #[test]
                    fn $name() -> Result<(), Box<dyn Error>> {
                        let decimal_target: Decimal = dec!(0);
                        let decimal_result: Decimal = dec!(0);
                        let decimal_rounds: Decimal = $test(decimal_target);

                        assert_eq!(decimal_result, decimal_rounds);
                        Ok(())
                    }
                )*
            }
        }
    }

    #[macro_use(crate::test_dec_rounding)]
    crate::test_dec_rounding!(dec_round_up_should_return_rounded_decimal,
            |t: decimal| {OperationsRound.dec_round_up(t)},
            "5,5", "6",
            "2,5", "3",
            "1,6", "2",
            "1,1", "2",
            "1,0", "1",
            "-1,0", "-1",
            "-1,1", "-2",
            "-1,6", "-2",
            "-2,5", "-3",
            "-5,5", "-6",
    );
}
