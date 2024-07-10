use serde::{Deserialize, Serialize};
use candid::CandidType;

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct Event {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub date: String,
    pub time: Option<String>,
}

impl Event {
    pub fn new(id: u64, title: &str, description: Option<&str>, date: &str, time: Option<&str>) -> Event {
        Event {
            id,
            title: title.to_string(),
            description: description.map(|s| s.to_string()),
            date: date.to_string(),
            time: time.map(|s| s.to_string()),
        }
    }
}

pub struct Calendar {
    events: Vec<Event>,
    pub next_id: u64,
}

impl Calendar {
    pub fn new() -> Calendar {
        Calendar { events: Vec::new(), next_id: 1 }
    }

    pub fn get_all(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn get_event_by_id(&self, id: u64) -> Option<&Event> {
        self.events.iter().find(|event| event.id == id)
    }

    pub fn add(&mut self, mut event: Event) {
        event.id = self.next_id;
        self.next_id += 1;
        self.events.push(event);
    }

    pub fn edit(&mut self, id: u64, event: Event) -> bool {
        if let Some(index) = self.events.iter().position(|e| e.id == id) {
            self.events[index] = event;
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, id: u64) -> bool {
        if let Some(index) = self.events.iter().position(|e| e.id == id) {
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
