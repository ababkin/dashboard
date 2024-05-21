use leptos::*;
use web_sys::{Blob, FileReader, MessageEvent, WebSocket};
use std::sync::{Arc, Mutex};
use arrow::array::{Int64Array, Float64Array};
use serde_json::json;
use chrono::prelude::*;
use leptos_chartistry::*;


use crate::types::*;
use crate::components::{
    chart_data::*,
    line_chart::*,
};


pub fn EventStats() -> impl IntoView {

    let all_events = vec![
        EventType::Snooze,
        EventType::Remove,
        EventType::Unsubscribe,
        EventType::SendEmailAction,
    ];
    
    let (debug, _) = create_signal(false);
    let (data, set_data) = create_signal(vec![]);
    let (event_types, set_event_types) = create_signal(all_events.clone());

    view! { 
        <div>
            <ChartData set_data />
            <LineChart debug data/>

        </div>
    }

}

