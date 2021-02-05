from rofi import Rofi
from requests import get
from ics import Calendar
import arrow


URL = "https://cloud.timeedit.net/chalmers/web/public/ri.ics?sid=3&objects=196836.194&ox=0&p=0.m%2C20210321.x&e=210202&enol=t&ku=34920&k=E0128097C79129968D30F049CAEBC7B9"

def get_timeline():
    c = Calendar(get(URL).text)
    return c.timeline

def get_current():
    """Get all current events"""
    events = list(get_timeline().now())
    if len(events) <= 0:
        return ([])

    return (list(map(gen_event, events)))

def get_next():
    """Get the next event"""
    t = get_timeline()
    now = arrow.utcnow()
    try:
        event = next(t.start_after(now))
    except StopIteration:
        return (None)

    when = event.begin.humanize(locale="sv")
    return ([gen_event(event)])


def gen_event(event):
    begin = event.begin.to('local').format("HH:mm")
    end = event.end.to('local').format("HH:mm")
    time = f"Tid: {begin} - {end}"

    title = f"{event.name}".split(".")[-1]
    if len(title) > 210:
        title = title[0:200]
    
    desc =  f"{event.description}"

    text = title + ", " + time
    return text

r = Rofi()

lessons = get_current()
index, key = r.select('Schedule', lessons)
