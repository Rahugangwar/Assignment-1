use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::prelude::*;
use std::io::LineWriter;
use crate::utility::command_line::arguments::Arguments;
use crate::utility::string_manipulator;
use super::data::MappedData;

pub fn put_data_in_file(cmd_arguments: Arguments, mapped_data:MappedData){
    
    let emp_data = get_emp_data(&cmd_arguments.emp_data_file_path);
    write_emp_data(&cmd_arguments.output_data_file_path, emp_data, mapped_data);
}

fn get_emp_data(emp_file: &str)->Vec<String> {

    let mut emp_data = Vec::new();
    if let Ok(lines) = read_lines(emp_file) {
        for line in lines.skip(1) {
            if let Ok(file_line) = line {
                emp_data.push(file_line);
            }
        }
    }
    else{
        eprintln!("Could not get the data from employee data file.");
    }

    emp_data

}

fn write_emp_data(op_file: &str, emp_data: Vec<String>, mapped_data: MappedData){
    
    let file = File::create(op_file).expect("could not make the output file.");
    let mut file = LineWriter::new(file);

    file.write(b"Emp ID~#~Emp Name~#~Dept Title~#~Mobile No~#~Email~#~Salary Status~#~On Leave\n").unwrap();

    for line in emp_data{
        let op_line = string_manipulator::get_op_data_string(line, &mapped_data);
        file.write(&op_line.as_bytes()).unwrap();
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}