use crate::data::*;
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

#[component]
pub fn DataChartistry() -> impl IntoView {
    let (debug, _) = create_signal(false);

    // leptos_server_signal::provide_websocket("ws://localhost:5000/ws").unwrap();

    // Create server signal
    // let event = create_server_signal::<WsEvent>("counter");
    let (data, set_data) = create_signal(vec![]);

    // Create WebSocket connection
    // let ws = WebSocket::new("ws://localhost:5000/ws").unwrap();
    let ws = Rc::new(RefCell::new(WebSocket::new("ws://localhost:5000/ws").unwrap()));
    let ws_clone = ws.clone();
    let set_data = set_data.clone();

    // On message event
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Ok(blob) = e.data().dyn_into::<Blob>() {
            let file_reader = FileReader::new().unwrap();
            let fr_c = file_reader.clone();
            let set_data = set_data.clone();
            let onloadend_cb = Closure::once(Box::new(move || {
                let array = Uint8Array::new(&fr_c.result().unwrap());
                let arrow_data = array.to_vec();

                // Deserialize Arrow IPC format data
                let cursor = Cursor::new(arrow_data);
                let mut reader = StreamReader::try_new(cursor, None).unwrap();

                let mut data_vec = vec![];
                while let Some(Ok(batch)) = reader.next() {
                    let column_a = batch.column(0).as_any().downcast_ref::<Int64Array>().unwrap();
                    let column_b = batch.column(1).as_any().downcast_ref::<Float64Array>().unwrap();

                    for i in 0..batch.num_rows() {
                        data_vec.push(json!({
                            "a": column_a.value(i),
                            "b": column_b.value(i)
                        }));
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


    // Display the data
    let display_data = move || {
        data.get().iter().map(|d| {
            let a = d["a"].as_i64().unwrap();
            let b = d["b"].as_f64().unwrap();
            view! {
                <p>{format!("a: {}, b: {}", a, b)}</p>
            }
        }).collect::<Vec<_>>()
    };

    view! { 
        <div>
            <h1>"Data from WebSocket"</h1>
            {move || display_data()}
            // {display_data()}
        </div>
    }

    // view! {
    //     <LineChart debug data=load_data() />
    // }
}

#[component]
pub fn LineChart(debug: ReadSignal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    // Lines are added to the series
    let series = Series::new(|data: &MyData| data.x)
        .line(Line::new(|data: &MyData| data.y1).with_name("butterflies"))
        .line(Line::new(|data: &MyData| data.y2).with_name("dragonflies"));
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data

            // Decorate our chart
            top=RotatedLabel::middle("My garden")
            left=TickLabels::aligned_floats()
            bottom=Legend::end()
            inner=[
                // Standard set of inner layout options
                AxisMarker::left_edge().into_inner(),
                AxisMarker::bottom_edge().into_inner(),
                XGridLine::default().into_inner(),
                YGridLine::default().into_inner(),
                YGuideLine::over_mouse().into_inner(),
                XGuideLine::over_data().into_inner(),
            ]
            tooltip=Tooltip::left_cursor().show_x_ticks(false)
        />
    }
}