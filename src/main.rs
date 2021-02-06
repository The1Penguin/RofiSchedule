use rofi;
extern crate web_ical;
use web_ical::Calendar;
use chrono;

fn main() {
    let uri = "https://cloud.timeedit.net/chalmers/web/public/ri.ics?sid=3&objects=196836.194&ox=0&p=0.m%2C20210321.x&e=210202&enol=t&ku=34920&k=E0128097C79129968D30F049CAEBC7B9";
    let icals = Calendar::new(uri);
    
    let mut dir_entries: Vec<std::string::String> = Vec::new();
    let day = chrono::offset::Local::now();
    for ical in &icals.events{
        if day == ical.dtsart{
            let temp = &ical.summary;
            let mut tmp = "";
            let split = temp.split(". ");
            for i in split {
                tmp = &i;
            }
            let lesson = ical.dtsart.with_timezone(&chrono::Local).format("%H:%M").to_string() + "-" + &ical.dtend.with_timezone(&chrono::Local).format("%H:%M").to_string() + " " + &tmp.to_string();
            dir_entries.push(lesson);
        }
    }
    
    dir_entries.sort();
    
    
    match rofi::Rofi::new(&dir_entries).run() {
        Ok(choice) => println!("Choice: {}", choice),
        Err(rofi::Error::Interrupted) => println!("Interrupted"),
        Err(e) => println!("Error: {}", e)
    }
}


