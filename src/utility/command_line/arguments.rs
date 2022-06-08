extern crate clap;
use clap::{Arg, App};

#[derive(Debug)]
pub struct Arguments {
    pub emp_data_file_path: String,
    pub dept_data_file_path: String,
    pub salary_data_file_path: String,
    pub leave_data_file_path: String,
    pub output_data_file_path: String,
    pub dept_data_sheet_name: String,
    pub salary_data_sheet_name: String,
    pub leave_data_sheet_name: String
}

pub fn get_arguments() -> Arguments{
    let matches = App::new("assignment")
    .version("0.1.0")
    .author("Rahul Gangwar <rahul.gangwar@surya-soft.com>")
    .about("Implementation of assignment-1")
    .arg(Arg::new("e")
             .short('e')
             .required(true)
             .long("emp-data-file-path")
             .takes_value(true)
             .help("A file having details about employee"))
    .arg(Arg::new("d")
             .short('d')
             .required(true)
             .long("dept-data-file-path")
             .takes_value(true)
             .help("A file having details about deparments"))
    .arg(Arg::new("s")
             .short('s')
             .required(true)
             .long("salary-data-file-path")
             .takes_value(true)
             .help("A file having details about salary"))
    .arg(Arg::new("l")
             .short('l')
             .required(true)
             .long("leave-data-file-path")
             .takes_value(true)
             .help("A file having details about leave"))
    .arg(Arg::new("o")
             .short('o')
             .required(true)
             .long("output-file-path")
             .takes_value(true)
             .help("A file having the outputs"))
    .arg(Arg::new("ds")
             .long("department-sheet-name")
             .takes_value(true)
             .default_value("Sheet1")
             .help("name of sheet in the file having the department details"))
    .arg(Arg::new("ss")
             .long("salary-sheet-name")
             .takes_value(true)
             .default_value("Sheet1")
             .help("name of sheet in the file having the salary details"))
    .arg(Arg::new("ls")
             .long("leave-sheet-name")
             .takes_value(true)
             .default_value("Sheet1")
             .help("name of sheet in the file having the leave details"))
    .get_matches();

    //Error strings
    let emp_error = "Please provide employee data file Path.";
    let dept_error = "Please provide dept data file Path.";
    let salary_error = "Please provide salary data file Path.";
    let leave_error = "Please provide leave data file Path.";
    let output_error = "Please provide leave data file Path.";

    //Warnings provided for sheet name
    let salaray_sheet_warning = "Sheet name for salary file is not provided. Taking default vale as 'Sheet-1'.";
    let dept_sheet_warning = "Sheet name for Department file is not provided. Taking default vale as 'Sheet-1'.";
    let leave_sheet_warning = "Sheet name for Leave file is not provided. Taking default vale as 'Sheet-1'.";

    Arguments{
        emp_data_file_path: matches.value_of("e").unwrap_or(emp_error).to_string(),
        dept_data_file_path: matches.value_of("d").unwrap_or(dept_error).to_string(),
        salary_data_file_path: matches.value_of("s").unwrap_or(salary_error).to_string(),
        leave_data_file_path: matches.value_of("l").unwrap_or(leave_error).to_string(),
        output_data_file_path: matches.value_of("o").unwrap_or(output_error).to_string(),
        dept_data_sheet_name: matches.value_of("ds").unwrap_or(salaray_sheet_warning).to_string(),
        salary_data_sheet_name: matches.value_of("ss").unwrap_or(dept_sheet_warning).to_string(),
        leave_data_sheet_name: matches.value_of("ls").unwrap_or(leave_sheet_warning).to_string()
    }
} 