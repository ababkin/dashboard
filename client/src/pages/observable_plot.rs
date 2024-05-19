use leptos::html::*;
use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Node};
use js_sys::{Function, Object, Reflect};
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use serde_json::json;
use web_sys::console;


#[derive(Serialize)]
struct RectYProps {
    length: i32,
}

#[derive(Serialize)]
struct BinXProps {
    y: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &Plottable);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_jsval(s: &JsValue);

    #[wasm_bindgen(js_namespace = Plot)]
    type Plot;
    #[derive(Debug)]
    type Plottable;

    #[wasm_bindgen(js_namespace = Plot, js_name = rectY)]
    fn rect_y(props: &JsValue, p: &Plottable) -> Plottable;

    #[wasm_bindgen(js_namespace = Plot, js_name = binX)]
    fn bin_x(props: &JsValue, p: &JsValue) -> Plottable;

    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;

    #[wasm_bindgen(method)]
    fn plot(this: &Plottable) -> Node;
}


#[component]
pub fn ObservablePlot() -> impl IntoView {
    
    div().on_mount(move |el| 

        {
            let math = js_sys::global().unchecked_into::<Object>();
            let random_fn = Reflect::get(&math, &JsValue::from_str("Math"))
                .unwrap()
                .unchecked_into::<Object>();
            let random_fn = Reflect::get(&random_fn, &JsValue::from_str("random"))
                .unwrap()
                .unchecked_into::<Function>();
            
            let rand = Object::new();
        
            Reflect::set(&rand, &JsValue::from_str("x"), &random_fn).unwrap();


            let bin_x_props = to_value(&BinXProps{ y: "count".to_string() }).unwrap();
            // log_jsval(&bin_x_props);
            // log_jsval(&rand);
            let bin_x = Plot::bin_x(&bin_x_props, &rand);
            // log(&bin_x);

            let rect_y_props = to_value(&RectYProps { length: 10000 }).unwrap();
            let plottable = Plot::rect_y(&rect_y_props, &bin_x);

            // log(&plottable);
            let plt = plottable.plot(); 
            el.append_child(&plt).unwrap();
        }
    )
}
