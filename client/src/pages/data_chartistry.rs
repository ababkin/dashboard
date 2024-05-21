use leptos::*;
use leptos_chartistry::*;
// use leptos_server_signal::create_server_signal;
use leptos_server_signal::*;
use web_sys::{Blob, FileReader, MessageEvent, WebSocket};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use arrow::ipc::reader::StreamReader;
use arrow::array::{Int64Array, Float64Array};
use serde_json::json;
use js_sys::Uint8Array;
use std::io::Cursor;
use std::rc::Rc;
use std::cell::RefCell;
use chrono::prelude::*;
use std::str::FromStr;


pub struct MyData {
    pub decision_timestamp: DateTime<Local>, //i64,
    pub running_avg: f64,
}

impl MyData {
    fn new(decision_timestamp: DateTime<Local>, running_avg: f64) -> Self {
        Self { decision_timestamp, running_avg }
    }
}

#[component]
pub fn DataChartistry() -> impl IntoView {
    let (debug, _) = create_signal(false);
    let (data, set_data) = create_signal(vec![]);

    // Create WebSocket connection
    let ws = Rc::new(RefCell::new(WebSocket::new("ws://localhost:5000/ws").unwrap()));
    let ws_clone = ws.clone();
    let set_data_clone = set_data.clone();

    // On message event
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Ok(blob) = e.data().dyn_into::<Blob>() {
            let file_reader = FileReader::new().unwrap();
            let fr_c = file_reader.clone();
            let set_data = set_data_clone.clone();
            let onloadend_cb = Closure::once(Box::new(move || {
                let array = Uint8Array::new(&fr_c.result().unwrap());
                let arrow_data = array.to_vec();

                // Deserialize Arrow IPC format data
                let cursor = Cursor::new(arrow_data);
                let mut reader = StreamReader::try_new(cursor, None).unwrap();

                let mut data_vec = vec![];
                while let Some(Ok(batch)) = reader.next() {
                    let column_timestamp = batch.column(0).as_any().downcast_ref::<Int64Array>().unwrap();
                    let column_avg = batch.column(1).as_any().downcast_ref::<Float64Array>().unwrap();

                    for i in 0..batch.num_rows() {
                        data_vec.push(MyData::new(
                            DateTime::from_timestamp_nanos(column_timestamp.value(i)).with_timezone(&Local),
                            column_avg.value(i),
                        ));
                    }
                }

                set_data.set(data_vec);
            }) as Box<dyn FnMut()>);
            file_reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
            file_reader.read_as_array_buffer(&blob).unwrap();
            onloadend_cb.forget();
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    ws.borrow().set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    // On open event: send a message to request data
    let onopen_callback = Closure::wrap(Box::new(move || {
        ws_clone.borrow().send_with_str("Requesting data").unwrap();
    }) as Box<dyn FnMut()>);
    ws.borrow().set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    view! { 
        <div>
            <h1>"Data from WebSocket"</h1>
            <LineChart debug data=data />
        </div>
    }
}



#[component]
pub fn LineChart(debug: ReadSignal<bool>, data: ReadSignal<Vec<MyData>>) -> impl IntoView {
    // Lines are added to the series
    let series = Series::new(|data: &MyData| data.decision_timestamp)
        .line(Line::new(|data: &MyData| data.running_avg).with_name("running_avg_rate"))
        // .with_x_range(0.0, 10.0)
        .with_y_range(0.0, 0.001); // TODO make the max dynamic

    // Axis
    let x_periods = Timestamps::from_periods(Period::all());
    let x_ticks = TickLabels::from_generator(x_periods.clone());
    let y_ticks = TickLabels::aligned_floats();

    let (min_x, max_x) = (series.min_x, series.max_x);
    let (min_y, max_y) = (series.min_y, series.max_y);
    let series_colours = series.colours;
    let series_len = series.len();


    view! {
        <div style="width: 100%; background: white !important;">
            <Chart
                aspect_ratio=AspectRatio::from_outer_height(1500.0, 3.0)
                debug=debug
                series=series
                data=data

                top=RotatedLabel::middle("Avg Rate")
                left=TickLabels::aligned_floats()
                // bottom=Legend::end()
                bottom=vec![x_ticks.clone().into_edge(),
                            // RotatedLabel::middle("This demo shows most of the available options. Edit things below...").into_edge(),
                ]

                // inner=inner.get().into_inner()
                inner=[
                    AxisMarker::horizontal_zero().into_inner(),
                    AxisMarker::left_edge().into_inner(),
                    XGridLine::from_ticks(x_ticks).into_inner(),
                    YGridLine::from_ticks(y_ticks).into_inner(),
                    XGuideLine::over_data().into_inner(),
                    YGuideLine::over_mouse().into_inner(),
                    // AxisMarker::left_edge().into_inner(),
                    // AxisMarker::bottom_edge().into_inner(),
                    // XGridLine::default().into_inner(),
                    // YGridLine::default().into_inner(),
                    // YGuideLine::over_mouse().into_inner(),
                    // XGuideLine::over_data().into_inner(),
                ]
                tooltip=Tooltip::left_cursor().show_x_ticks(false)
            />
        </div>
    }
}

// #[component]
// pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
//     let series = Series::new(|data: &MyData| data.x)
//         .line(|data: &MyData| data.y1)
//         .line(|data: &MyData| data.y2)
//         .with_x_range(0.0, 10.0)
//         .with_y_range(-10.0, 10.0);
//     view! {
//         <Chart
//             aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
//             debug=debug
//             series=series
//             data=data

//             inner=vec![
//                 // Axis markers run along the edge of an axis, usually along the edge
//                 AxisMarker::bottom_edge().into_inner(),
//                 // However they can also be placed at zero (if shown)
//                 AxisMarker::horizontal_zero().into_inner(),
//                 // Or at the top edge if that makes sense for your chart
//                 AxisMarker::top_edge().into_inner(),
//                 // We can also remove embellishments (the arrow) from the marker
//                 AxisMarker::left_edge().with_arrow(false).into_inner(),
//             ]
//         />
//     }
// }