use leptos::*;
use leptos_chartistry::*;
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
use wasm_bindgen::prelude::*;
use web_sys::console;
use flate2::read::GzDecoder;

use crate::types::*;

pub fn log(message: &str) {
    console::log_1(&message.into());
}

#[component]
pub fn ChartData(set_data: WriteSignal<Vec<MyData>>) -> impl IntoView {
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
                let compressed_data = array.to_vec();

                // Decompress the data
                let mut decoder = GzDecoder::new(&compressed_data[..]);
                let mut decompressed_data = Vec::new();
                std::io::copy(&mut decoder, &mut decompressed_data).unwrap();

                // Deserialize Arrow IPC format data
                let cursor = Cursor::new(decompressed_data);
                let mut reader = StreamReader::try_new(cursor, None).unwrap();

                let mut data_vec = vec![];
                while let Some(Ok(batch)) = reader.next() {
                    let column_timestamp = batch.column(0).as_any().downcast_ref::<Int64Array>().unwrap();
                    let column_avg_snooze = batch.column(1).as_any().downcast_ref::<Float64Array>().unwrap();
                    let column_avg_remove = batch.column(2).as_any().downcast_ref::<Float64Array>().unwrap();
                    let column_avg_unsubscribe = batch.column(3).as_any().downcast_ref::<Float64Array>().unwrap();
                    let column_avg_send_mail_action = batch.column(4).as_any().downcast_ref::<Float64Array>().unwrap();

                    for i in 0..batch.num_rows() {
                        data_vec.push(MyData {
                            decision_timestamp: DateTime::from_timestamp_nanos(column_timestamp.value(i)).with_timezone(&Local),
                            running_avg_snooze: column_avg_snooze.value(i),
                            running_avg_remove: column_avg_remove.value(i),
                            running_avg_unsubscribe: column_avg_unsubscribe.value(i),
                            running_avg_send_mail_action: column_avg_send_mail_action.value(i),
                        });
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
        ws_clone.borrow().send_with_str("give me data").unwrap();
    }) as Box<dyn FnMut()>);
    ws.borrow().set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    // Reactive effect to send message whenever event_type changes
    // create_effect(move |_| {
    //     let event_type_value = event_type;

    //     match ws.borrow().send_with_str(&event_type_value.to_string()) {
    //         Ok(()) => (),
    //         Err(e) => log(&format!("error: {:?}", e)),
    //     }
    // });

    view! { 
        <div>
        </div>
    }
}