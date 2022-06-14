use crate::utility::command_line::arguments::Arguments;
use crate::utility::calculate;
use std::collections::HashMap;
use calamine::{Reader, Xlsx, open_workbook};

pub struct MappedData{
    pub dept_id_dept_name: HashMap<String, String>,
    pub emp_id_salary_status: HashMap<String, String>,
    pub emp_id_leave_status: HashMap<String, i32>
}

pub fn get_mapped_data(cmd_arguments: &Arguments)-> MappedData{

    MappedData{
        dept_id_dept_name: get_dept_id_map_to_dept_name(&cmd_arguments),
        emp_id_salary_status: get_emp_id_map_to_salary_status(&cmd_arguments),
        emp_id_leave_status: get_emp_id_map_to_leave_status(&cmd_arguments),
    }
}

fn get_dept_id_map_to_dept_name(cmd_arguments: &Arguments)-> HashMap<String, String>{
    
    let mut dept_id_dept_name: HashMap<String, String> = HashMap::new();

    let mut excel: Xlsx<_> = open_workbook(&cmd_arguments.dept_data_file_path).expect("Could not open dept-data-file.");
    if let Some(Ok(r)) = excel.worksheet_range(&cmd_arguments.dept_data_sheet_name) {
        for row in r.rows().skip(1) {
            dept_id_dept_name.insert(row[0].to_string(), row[1].to_string());
        }
    }

    dept_id_dept_name

}

pub fn get_emp_id_map_to_salary_status(cmd_arguments: &Arguments)-> HashMap<String, String>{

    let mut emp_id_salary_status: HashMap<String, String> = HashMap::new();

    let mut excel: Xlsx<_> = open_workbook(&cmd_arguments.salary_data_file_path).expect("Could not open the salary-data-file.");
    
    if let Some(Ok(r)) = excel.worksheet_range(&cmd_arguments.salary_data_sheet_name) {
        for row in r.rows().skip(1) {
            if calculate::check_excel_date(row[2].to_string().parse().unwrap()) && "Credited".to_string().eq(&row[4].to_string()){
                println!("{}", row[0]);
                emp_id_salary_status.insert(row[0].to_string(), row[4].to_string());
            }
        }
    }

    emp_id_salary_status

}

pub fn get_emp_id_map_to_leave_status(cmd_arguments: &Arguments)-> HashMap<String, i32>{

    let mut emp_id_leave_status: HashMap<String, i32> = HashMap::new();

    let mut excel: Xlsx<_> = open_workbook(&cmd_arguments.leave_data_file_path).expect("Could not open the leave-data-file.");
    
    if let Some(Ok(r)) = excel.worksheet_range(&cmd_arguments.leave_data_sheet_name) {
        for row in r.rows() {
            let key = row[0].to_string();
            let value = calculate::get_leaves(row[2].to_string().parse().unwrap(), row[3].to_string().parse().unwrap());
            
            if emp_id_leave_status.contains_key(&key){
                *emp_id_leave_status.get_mut(&key).unwrap_or(&mut 0) += value;
            }
            else {
                emp_id_leave_status.insert(key, value);
            }
        }
    }

    emp_id_leave_status
}