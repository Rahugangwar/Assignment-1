mod utility;
mod file_manipulator;
use colored::*;

fn main() {

    //get the arguments(file paths) from terminal
    let cmd_arguments = utility::command_line::arguments::get_arguments();

    //checking if the files exists or not:
    //getting error strings
    let error_strings = file_manipulator::guard::get_error_strings(&cmd_arguments);

    if error_strings.is_empty(){

        //prepare mapped data in order to get information from other files.
        let mapped_data = file_manipulator::data::get_mapped_data(&cmd_arguments);

        //finally write the data to the output file.
        file_manipulator::read_write::put_data_in_file(cmd_arguments, mapped_data);

        //A message that data ha been written correctly.
        println!("{}","Sucessfully written the output".green());
    }
    else{
        for error in error_strings{
            eprintln!("{}", error.red());
        }
    }

    // for (key, values) in EmpID_leaveStatus.iter(){
    //     println!("{}: {}", key, values);
    // }
}