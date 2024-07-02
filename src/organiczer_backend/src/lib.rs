mod calendar;

use calendar::{Calendar, Event};
use std::sync::Mutex;
use lazy_static::lazy_static;
use candid::{CandidType, Principal};

lazy_static! {
    static ref CALENDAR: Mutex<Calendar> = Mutex::new(Calendar::new());
}

#[ic_cdk_macros::query]
fn get_all_events() -> Vec<Event> {
    let calendar = CALENDAR.lock().unwrap();
    calendar.get_all().clone()
}

#[ic_cdk_macros::query]
fn get_event_by_index(index: u64) -> Option<Event> {
    let calendar = CALENDAR.lock().unwrap();
    calendar.get(index as usize).cloned()
}

#[ic_cdk_macros::update]
fn create_event(title: String, description: Option<String>, date: String, time: Option<String>) -> bool {
    let mut calendar = CALENDAR.lock().unwrap();
    let event = Event::new(&title, &description.unwrap_or_else(|| "".to_string()), &date, time);
    calendar.add(event);
    true
}

#[ic_cdk_macros::update]
fn update_event(index: u64, title: String, description: Option<String>, date: String, time: Option<String>) -> bool {
    let mut calendar = CALENDAR.lock().unwrap();
    let event = Event::new(&title, &description.unwrap_or_else(|| "".to_string()), &date, time);
    calendar.edit(index as usize, event)
}

#[ic_cdk_macros::update]
fn delete_event(index: u64) -> bool {
    let mut calendar = CALENDAR.lock().unwrap();
    calendar.remove(index as usize)
}

#[ic_cdk_macros::query]
fn search_events_in_range(start_date: String, end_date: String) -> Vec<Event> {
    let calendar = CALENDAR.lock().unwrap();
    calendar.search(&start_date, &end_date).into_iter().cloned().collect()
}
