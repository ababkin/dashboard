use leptos::*;
use web_sys::{Blob, FileReader, MessageEvent, WebSocket};
use std::sync::{Arc, Mutex};
use arrow::array::{Int64Array, Float64Array};
use serde_json::json;
use chrono::prelude::*;
use leptonic::prelude::*;
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
    // let (snooze_data, set_snooze_data) = create_signal(vec![]);
    // let (remove_data, set_remove_data) = create_signal(vec![]);
    let (data, set_data) = create_signal(vec![]);
    let (event_types, set_event_types) = create_signal(all_events.clone());

    // let datas = vec![EventTypeSeries::new(remove_data, RED), EventTypeSeries::new(snooze_data, GREEN)]

    view! { 
        <div>
            <ChartData set_data />
            <LineChart debug data/>

        </div>
        // <Select
        //     options=vec![EventType::Snooze, EventType::Remove, EventType::Unsubscribe, EventType::SendEmailAction]
        //     search_text_provider=move |o| format!("{o}")
        //     render_option=move |o| format!("{o:?}")
        //     selected=event_types
        //     set_selected=move |v| set_event_types.set(v)
        // />
        <Multiselect
            options=all_events
            search_text_provider=move |o| format!("{o}")
            render_option=move |o| format!("{o:?}")
            selected=event_types
            set_selected=move |v| set_event_types.set(v)
        />
    }

}

