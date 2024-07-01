use serde::{Deserialize, Serialize};
use candid::CandidType;

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct Event {
    pub title: String,
    pub description: String,
    pub date: String,
    pub time: Option<String>,
}

impl Event {
    pub fn new(title: &str, description: &str, date: &str, time: Option<String>) -> Event {
        Event {
            title: title.to_string(),
            description: description.to_string(),
            date: date.to_string(),
            time,
        }
    }
}

pub struct Calendar {
    events: Vec<Event>,
}

impl Calendar {
    pub fn new() -> Calendar {
        Calendar { events: Vec::new() }
    }

    pub fn get_all(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn get(&self, index: usize) -> Option<&Event> {
        self.events.get(index)
    }

    pub fn add(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn edit(&mut self, index: usize, event: Event) -> bool {
        if index < self.events.len() {
            self.events[index] = event;
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, index: usize) -> bool {
        if index < self.events.len() {
            self.events.remove(index);
            true
        } else {
            false
        }
    }

    pub fn search(&self, start_date: &str, end_date: &str) -> Vec<&Event> {
        self.events.iter().filter(|&event| {
            event.date >= start_date.to_string() && event.date <= end_date.to_string()
        }).collect()
    }
}
