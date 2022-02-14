#[cfg(test)]
//#[cfg(test_report)]
mod factories_history_tests {
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use rust_decimal::Decimal;
    use rust_decimal::prelude::ToPrimitive;
    use rust_decimal_macros::dec;

    fn create_history_file(file_name: &str) -> Option<File> {
        const PARENT_HISTORY_FOLDER_NAME: &str = "legalios";
        const HISTORY_FOLDER_NAME: &str = "TestHistory";
        let res_curr_path = std::env::current_dir();
        if res_curr_path.is_err(){
            return None;
        }
        let mut curr_path = res_curr_path.unwrap();
        while !curr_path.ends_with(PARENT_HISTORY_FOLDER_NAME) && curr_path.ancestors().count() != 1 {
            curr_path.pop();
        }
        let base_path = curr_path.join(HISTORY_FOLDER_NAME);
        let res_norm_path = fs::canonicalize(&base_path);
        if res_norm_path.is_err(){
            return None;
        }
        let norm_path = res_norm_path.unwrap();

        let file_path = norm_path.join(file_name);

        let res_file_handle = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path);
        if res_file_handle.is_err(){
            return None;
        }
        let file_handle = res_file_handle.unwrap();

        Some(file_handle)
    }

    fn export_history_start(opt_file_handle: &mut Option<File>, data: Vec<(i32, bool)>) {
        if opt_file_handle.is_none(){
            return;
        }
        let file_handle: &mut File = opt_file_handle.as_mut().unwrap();
        write!(file_handle, "History Term").unwrap();
        for col in data {
            if col.1 {
                write!(file_handle, "\t${} Begin Value", col.0).unwrap();
                write!(file_handle, "\t${} Change Month", col.0).unwrap();
                write!(file_handle, "\t${} End Value", col.0).unwrap();
            } else {
                write!(file_handle, "\t${} Value", col.0).unwrap();
            }
        }
        write!(file_handle, "\n").unwrap();
    }

    fn export_history_term(opt_file_handle: &mut Option<File>, head: &Vec<(i32, bool)>, data: (i16, Vec<(i16, i16, string, string)>)) {
        if opt_file_handle.is_none() {
            return;
        }
        let file_handle: &mut File = opt_file_handle.as_mut().unwrap();
        write!(file_handle, "{}", data.0).unwrap();
    }

    fn props_int_value_to_string(value: i32) -> String {
        return format!("{}", value);
    }
    fn props_dec_value_to_string(value: Decimal) -> String {
        let dec_option = value*dec!(100);
        let int_option = dec_option.to_i32();
        let int_value: i32 = match int_option {
            Some(value) => value,
            None=> 0i32,
        };
        return format!("{}", int_value);
    }
    fn write_history_year_int_value(opt_file_handle: &mut Option<File>, value: i32) {
        if opt_file_handle.is_none(){
            return;
        }
        let file_handle: &mut File = opt_file_handle.as_mut().unwrap();
        write!(file_handle, "\t{}", value).unwrap();
    }

    fn write_history_year_dec_value(opt_file_handle: &mut Option<File>, value: Decimal) {
        let dec_option = value*dec!(100);
        let int_option = dec_option.to_i32();
        let int_value: i32 = match int_option {
            Some(value) => value,
            None=> 0i32,
        };
        if opt_file_handle.is_none(){
            return;
        }
        let file_handle: &mut File = opt_file_handle.as_mut().unwrap();
        write!(file_handle, "\t{}", int_value).unwrap();
    }

    fn write_history_year_ends(opt_file_handle: &mut Option<File>) {
        if opt_file_handle.is_none(){
            return;
        }
        let file_handle: &mut File = opt_file_handle.as_mut().unwrap();
        write!(file_handle, "\n").unwrap();
    }

    fn close_history_file(opt_file_handle: &mut Option<File>) {
        if opt_file_handle.is_none(){
            return;
        }
        let file_handle: &mut File = opt_file_handle.as_mut().unwrap();
        file_handle.flush().unwrap();
    }
}