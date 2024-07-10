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
fn get_event_by_id(id: u64) -> Option<Event> {
    let calendar = CALENDAR.lock().unwrap();
    calendar.get_event_by_id(id).cloned()
}

#[ic_cdk_macros::update]
fn create_event(title: String, description: Option<String>, date: String, time: Option<String>) -> u64 {
    let mut calendar = CALENDAR.lock().unwrap();
    let event = Event::new(calendar.next_id, &title, description.as_deref(), &date, time.as_deref());
    let id = event.id;
    calendar.add(event);
    id
}

#[ic_cdk_macros::update]
fn update_event(id: u64, title: String, description: Option<String>, date: String, time: Option<String>) -> bool {
    let mut calendar = CALENDAR.lock().unwrap();
    let event = Event::new(id, &title, description.as_deref(), &date, time.as_deref());
    calendar.edit(id, event)
}

#[ic_cdk_macros::update]
fn delete_event(id: u64) -> bool {
    let mut calendar = CALENDAR.lock().unwrap();
    calendar.remove(id)
}

#[ic_cdk_macros::query]
fn search_events_in_range(start_date: String, end_date: String) -> Vec<Event> {
    let calendar = CALENDAR.lock().unwrap();
    calendar.search(&start_date, &end_date).into_iter().cloned().collect()
}
