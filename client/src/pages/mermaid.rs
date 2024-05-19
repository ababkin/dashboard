// use futuresdr_types::FlowgraphDescription;
use leptos::html::div;
use leptos::html::pre;
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "mermaid.init")]
    pub fn mermaid_render();
}

fn graph() -> String {
    "graph TD;
    A[Start] --> B{Is it sunny?};
    B -- Yes --> C[Go to the park];
    B -- No --> D[Stay home];
    C --> E{Bring a picnic?};
    E -- Yes --> F[Pack a picnic];
    E -- No --> G[Don't pack a picnic];
    F --> H[Enjoy the day!];
    G --> H;
    D --> I[Do indoor activities];
    H --> I[End of day];".to_string()
}

#[component]
pub fn Mermaid() -> impl IntoView {
    div().on_mount(|_| mermaid_render()).child(
        pre()
            .classes("mermaid")
            .inner_html(graph()),
    )
}