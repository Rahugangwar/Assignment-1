use std::path::Path;
use crate::utility::command_line::arguments::Arguments;
use colored::*;

pub fn get_error_strings(cmd_arguments: &Arguments)-> Vec<String>{

    let mut error_strings = Vec::new();
    
    if !file_exists(&cmd_arguments.emp_data_file_path) {
        error_strings.push(get_error_string(&cmd_arguments.emp_data_file_path, "emloyee data file"));
    }
    
    if !file_exists(&cmd_arguments.dept_data_file_path) {
        error_strings.push(get_error_string(&cmd_arguments.dept_data_file_path, "department data file"));
    }
    
    if !file_exists(&cmd_arguments.salary_data_file_path) {
        error_strings.push(get_error_string(&cmd_arguments.salary_data_file_path, "salary data file"));
    }
    
    if !file_exists(&cmd_arguments.leave_data_file_path) {
        error_strings.push(get_error_string(&cmd_arguments.leave_data_file_path, "leave data file"));
    }
    
    if !file_exists(&cmd_arguments.output_data_file_path) {
        error_strings.push(get_error_string(&cmd_arguments.output_data_file_path, "output data file"));
    }

    error_strings
}

fn file_exists(file_path: &str) -> bool{
    Path::new(file_path).exists()
}

fn get_error_string(file_path: &String, file_type: &str)->String{

    let mut error_string = String::from("The Path given: '");
    error_string.push_str(&file_path);
    error_string.push_str("' does not exist. Please give the correct path for ");
    error_string.push_str(&file_type.bold());

    error_string
}