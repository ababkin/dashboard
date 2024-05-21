use leptos::*;
use web_sys::{Blob, FileReader, MessageEvent, WebSocket};
use std::sync::{Arc, Mutex};
use arrow::array::{Int64Array, Float64Array};
use serde_json::json;
use chrono::prelude::*;
use leptonic::prelude::*;

use crate::types::*;
use crate::components::{
    chart_data::*,
    line_chart::*,
};


pub fn DataChartistry() -> impl IntoView {
    
    let (debug, _) = create_signal(false);
    let (data, set_data) = create_signal(vec![]);
    let (event_type, set_event_type) = create_signal(EventType::Remove);

    view! { 
        <div>
            <h1>"Data from WebSocket"</h1>
            <ChartData set_data event_type />
            <LineChart debug data=data />
        </div>
        <Select
            options=vec![EventType::Snooze, EventType::Remove, EventType::Unsubscribe, EventType::SendEmailAction]
            search_text_provider=move |o| format!("{o}")
            render_option=move |o| format!("{o:?}")
            selected=event_type
            set_selected=move |v| set_event_type.set(v)
        />
    }

}

