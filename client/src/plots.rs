use std::collections::HashMap;

use plotly::common::Title;
use plotly::{Bar, Histogram, Layout, Plot, Scatter};


pub fn test_plot() -> Plot {
    let mut plot = Plot::new();
    let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
    plot.add_trace(trace);

    // let mut msg_plot = Plot::new();

    // for name in messages_count.keys() {
    //     msg_plot.add_trace(
    //         Bar::new(
    //             ["Messages"].to_vec(),
    //             [messages_count[&name.clone()]].to_vec(),
    //         )
    //         .name(name),
    //     )
    // }

    // let msg_layout = Layout::new().title(Title::new("Messages per participants"));

    // msg_plot.set_layout(msg_layout);
    // return msg_plot;

    plot
}