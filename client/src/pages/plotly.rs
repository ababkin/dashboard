use plotly::Plot;

use crate::plots::{
    test_plot,
};
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, SubmitEvent};

// use crate::data::*;
use leptos::html::Input;
use leptos::*;
use leptos::{
    component, create_action, create_node_ref, create_resource, create_signal, logging, view, For,
    IntoView, SignalGet, Suspense,
};

#[component]
pub fn Plotly() -> impl IntoView {
    // let (debug, _) = create_signal(false);

    let test_plotted = create_action(|input: &Plot| {
        let input = input.to_owned();
        async move { plotly::bindings::new_plot("TestPlot", &input).await }
    });

    test_plotted.dispatch(test_plot());

    logging::log!("Hola!!!");


    view! {
        // <div>
        //     hello
        // </div>
        // <LineChart debug data=load_data() />
        <Suspense fallback = move || view! {<p>"Loading..."</p>}>
            <div id="TestPlot"></div>
        // <MessengerData data={texts.get()}/>
        </Suspense>

    }
}