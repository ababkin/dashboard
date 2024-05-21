use std::fmt;
use chrono::prelude::*;
use leptos_chartistry::*;
use leptos::*;

pub struct MyData {
    pub decision_timestamp: DateTime<Local>, //i64,
    pub running_avg_snooze: f64,
    pub running_avg_remove: f64,
    pub running_avg_unsubscribe: f64,
    pub running_avg_send_mail_action: f64,
}

// impl MyData {
//     pub fn new(decision_timestamp: DateTime<Local>, running_avg: f64) -> Self {
//         Self { decision_timestamp, running_avg }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventType {
    Snooze,
    Remove,
    Unsubscribe,
    SendEmailAction,
}

impl EventType {
    pub fn to_string(&self) -> String {
        match self.clone() {
            EventType::Snooze => "snooze",
            EventType::Remove => "remove",
            EventType::Unsubscribe => "unsubscribe",
            EventType::SendEmailAction => "send_mail_action",
        }.to_string()
    }
}

// Implementing the Display trait
impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str = self.to_string();
        write!(f, "{}", as_str)
    }
}

pub struct EventTypeSeries {
    data: ReadSignal<Vec<MyData>>,
    color: Colour,
}

impl EventTypeSeries {
    pub fn new(data: ReadSignal<Vec<MyData>>, color: Colour) -> Self {
        Self { data, color }
    }
}