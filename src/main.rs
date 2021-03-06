use rofi;
extern crate web_ical;
use web_ical::Calendar;
use chrono;

fn main() {
    let uri = "https://cloud.timeedit.net/chalmers/web/public/ri.ics?sid=3&objects=196836.194&ox=0&p=0.m%2C20210321.x&e=210202&enol=t&ku=34920&k=E0128097C79129968D30F049CAEBC7B9";
    let icals = Calendar::new(uri);
    let day = chrono::offset::Local::now();
    let mut dir_entries: Vec<std::string::String> = generate_day(icals, day);
    dir_entries.sort();
    display(dir_entries);
    }

fn display(dir_entries:std::vec::Vec<std::string::String>){
    match rofi::Rofi::new(&dir_entries).prompt("Schedule").run() {
        Ok(_choice) => (),
        Err(rofi::Error::Interrupted) => (),
        Err(e) => println!("Error, {}", e)
    }
}

fn generate_day(icals: Calendar, day:chrono::DateTime<chrono::Local>) -> std::vec::Vec<std::string::String> {
    let mut dir_entries: Vec<std::string::String> = Vec::new();

    for ical in &icals.events{
        if day.format("%F").to_string() == ical.dtsart.format("%F").to_string(){
                let temp = &ical.summary;
                if temp != ""{
                let split = temp.split(". ");
                let mut tmp = "";
                for i in split {
                    tmp = &i;
                }
                let lesson = ical.dtsart.with_timezone(&chrono::Local).format("%H:%M").to_string() + "-" + &ical.dtend.with_timezone(&chrono::Local).format("%H:%M").to_string() + " " + &tmp.to_string();
                dir_entries.push(lesson);
            }
        }
    }
    return dir_entries
}
