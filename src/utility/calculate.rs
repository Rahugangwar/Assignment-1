use chrono::{Datelike, NaiveDate, Duration};

pub fn check_excel_date(excel_date: i64)->bool{
    let excel_date = get_naive_date(&excel_date);
    let current_date = chrono::Utc::now();
    let is_month_corr = excel_date.month() == current_date.month();
    let is_year_corr = excel_date.year() == current_date.year();
    return  is_month_corr && is_year_corr;
}

pub fn get_naive_date(excel_date: &i64) -> NaiveDate{
    NaiveDate::from_ymd(1900, 1, 1) + Duration::days(excel_date-2)
}

pub fn get_leaves(from: i64, to: i64)->i32{

    let from_date = get_naive_date(&from);
    let to_date = get_naive_date(&to);
    
    if check_excel_date(from) && check_excel_date(to){
        to_date.day() as i32 -from_date.day() as i32 + 1
    }
    else if check_excel_date(from) && !check_excel_date(to) {
        let month_last_dates: [i32; 13] = [0, 31, 30, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        month_last_dates[from_date.month() as usize] - from_date.day() as i32 +1
    }
    else if check_excel_date(to) && !check_excel_date(from) {
        to_date.day() as i32
    }
    else{
        0
    }
}