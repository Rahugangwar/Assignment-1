mod utility;
mod file_manipulator;
use colored::*;

fn main() {

    //get the arguments(file paths) from terminal
    let cmd_arguments = utility::command_line::arguments::get_arguments();

    //checking if the files exists or not: then getting errors
    let error_strings = file_manipulator::guard::get_error_strings(&cmd_arguments);

    if error_strings.is_empty(){
        
    }

        
}