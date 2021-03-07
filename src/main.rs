use rofi;
extern crate web_ical;
use web_ical::Calendar;
use chrono::*;

fn main() {
    let dir_entries = generate_next_days();
    display(dir_entries);
    }

fn display(dir_entries:std::vec::Vec<std::string::String>){
    match rofi::Rofi::new(&dir_entries).prompt("Schedule").run() {
        Ok(_choice) => (),
        Err(rofi::Error::Interrupted) => (),
        Err(e) => println!("Error, {}", e)
    }
}

fn generate_next_days() -> std::vec::Vec<std::string::String> {
    let uri = "https://cloud.timeedit.net/chalmers/web/public/ri.ics?sid=3&objects=196836.194&ox=0&p=0.m%2C20210321.x&e=210202&enol=t&ku=34920&k=E0128097C79129968D30F049CAEBC7B9";
    let icals = Calendar::new(uri);
    let dir_entries = generate_multi_days(icals, 5);
    return dir_entries
}

fn generate_multi_days(icals: Calendar, amount: i64) -> std::vec::Vec<std::string::String> {
    let mut dir_entries: Vec<std::string::String> = Vec::new();
    let day = Local::now();
    for i in 0..amount{
        dir_entries.extend(generate_day(&icals, day + Duration::days(i)));
    }
    return dir_entries
}

fn generate_day(icals: &Calendar, day:chrono::DateTime<chrono::Local>) -> std::vec::Vec<std::string::String> {
    let mut dir_entries: Vec<std::string::String> = Vec::new();

    for ical in &icals.events{
        if day.day() == ical.dtsart.day() {
            let course_id = get_course_id(ical);
            let lesson_type = get_lesson_type(ical);
            let lesson = format!("{} {}-{} {} {}",
                                 ical.dtsart.weekday(),
                                 ical.dtsart.with_timezone(&chrono::Local).format("%H:%M"),
                                 &ical.dtend.with_timezone(&chrono::Local).format("%H:%M"),
                                 &course_id,
                                 &lesson_type);
            dir_entries.push(lesson);
        }
    }
    dir_entries.sort();
    return dir_entries
}

fn get_course_id(ical: &web_ical::Events) -> String {
    let split: Vec<&str> = ical.summary.split(&[',', '.'][..]).collect();
    let tmp = split[split.len() - 2];
    remove_whitespace(tmp)
}

fn get_lesson_type(ical: &web_ical::Events) -> String {
    let split: Vec<&str> = ical.description.split("\\n").collect();
    let tmp = split[split.len() - 2];
    remove_whitespace(tmp)
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
