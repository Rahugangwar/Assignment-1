use std::collections::HashMap;
use crate::file_manipulator::data::MappedData;

pub fn get_op_data_string(emp_data_line: String, mapped_data: &MappedData) -> String{

    let splitted_line = get_splitted_line(&emp_data_line, "|");

    let mut op_line: String = get_emp_id(&splitted_line).to_string();
    op_line += "~#~";

    op_line += get_emp_name(&splitted_line);
    op_line += "~#~";

    op_line += get_dept_title(&splitted_line, &mapped_data.dept_id_dept_name);
    op_line += "~#~";

    op_line += get_mobile_no(&splitted_line);
    op_line += "~#~";
    
    op_line += get_email(&splitted_line);
    op_line += "~#~";
    
    op_line += get_salary_status(&splitted_line, &mapped_data.emp_id_salary_status);
    op_line += "~#~";

    op_line += &get_leaves(&splitted_line, &mapped_data.emp_id_leave_status);

    op_line += "\n";

    op_line
}

fn get_splitted_line<'a>(line: &'a String, delimeter: &'a str) -> Vec<&'a str>{
    
    let sp:Vec<&str> = line.split(delimeter).collect();
    return sp    

}

fn get_emp_id<'a>(splitted_line: &'a Vec<&str>) -> &'a str{
    splitted_line[0]
}

fn get_emp_name<'a>(splitted_line: &'a Vec<&str>) -> &'a str{
    splitted_line[1]
}

fn get_dept_title<'a>(splitted_line: &'a Vec<&str>, dept_id_dept_name: &'a HashMap<std::string::String, std::string::String>) -> &'a str{
    
    let mut dept_title = "Not found";
    
    if dept_id_dept_name.contains_key(splitted_line[2]){
        dept_title = &dept_id_dept_name[splitted_line[2]];
    }
    
    dept_title
}

fn get_mobile_no<'a>(splitted_line: &'a Vec<&str>) -> &'a str{
    splitted_line[3]
}

fn get_email<'a>(splitted_line: &'a Vec<&str>) -> &'a str{
    splitted_line[4]
}

fn get_salary_status<'a>(splitted_line: &'a Vec<&str>, emp_id_salary_status: &'a HashMap<std::string::String, std::string::String>) -> &'a str{
    
    let mut salary_status = "Not Credited";
    
    if emp_id_salary_status.contains_key(splitted_line[0]){
        salary_status = &emp_id_salary_status[splitted_line[0]];
    }
    
    salary_status
}

fn get_leaves<'a>(splitted_line: &'a Vec<&str>, emp_id_leave_status: &HashMap<String, i32>) -> String{
    
    let mut leaves = "0".to_string();
    
    if emp_id_leave_status.contains_key(splitted_line[0]){
        leaves = emp_id_leave_status.get(splitted_line[0]).unwrap().to_string();
    }
    
    leaves
}