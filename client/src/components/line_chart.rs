use leptos::*;
use leptos_chartistry::*;
use chrono::prelude::*;

use crate::types::*;


fn get_line(event_type: EventType) -> Line<MyData, f64> {
    const RED: Colour = Colour::from_rgb(255, 0, 0);
    const YELLOW: Colour = Colour::from_rgb(255, 255, 0);
    // const BLACK: Colour = Colour::from_rgb(0, 0, 0);
    const GREEN: Colour = Colour::from_rgb(0, 255, 0);
    const BLUE: Colour = Colour::from_rgb(0, 0, 255);

    match event_type {
        EventType::Snooze =>
            Line::new(|data: &MyData| data.running_avg_snooze).with_name("running_avg_snooze").with_colour(GREEN),
        EventType::Remove =>
            Line::new(|data: &MyData| data.running_avg_remove).with_name("running_avg_remove").with_colour(RED),
        EventType::Unsubscribe =>
            Line::new(|data: &MyData| data.running_avg_unsubscribe).with_name("running_avg_unsubscribe").with_colour(YELLOW),
        EventType::SendEmailAction =>
            Line::new(|data: &MyData| data.running_avg_send_mail_action).with_name("running_avg_send_mail_action").with_colour(BLUE),
    }

}


fn mk_series(lines: Vec<Line<MyData, f64>>) -> Series<MyData, DateTime<Local>, f64> {
    Series::new(|data: &MyData| data.decision_timestamp).lines(lines)
} 

#[component]
pub fn LineChart(debug: ReadSignal<bool>, data: ReadSignal<Vec<MyData>>, event_types: ReadSignal<Vec<EventType>>) -> impl IntoView {

    // let series_state = use_ref(cx, || mk_series(vec![]));

    // create_effect(move |_| {
    //     let new_lines = event_types.get().iter().map(|et| get_line(*et)).collect::<Vec<Line<MyData, f64>>>();
    //     *series_state.borrow_mut() = mk_series(new_lines);
    // });

    // let (series, set_series) = create_signal(mk_series(vec![]));

    // create_effect(move |_| {
    //     let new_lines = event_types.get().iter().map(|et| get_line(*et)).collect::<Vec<Line<MyData, f64>>>();
    //     set_series.set(mk_series(new_lines));
    // });


    // let types = vec!["snooze", "remove", "unsubscribe", "send_mail_action"];
    // let lines = vec![
    //     Line::new(|data: &MyData| data.running_avg_snooze).with_name("running_avg_snooze") .with_colour(GREEN),
    //     Line::new(|data: &MyData| data.running_avg_remove).with_name("running_avg_remove") .with_colour(RED),
    //     Line::new(|data: &MyData| data.running_avg_unsubscribe).with_name("running_avg_unsubscribe") .with_colour(YELLOW),
    //     Line::new(|data: &MyData| data.running_avg_send_mail_action).with_name("running_avg_send_mail_action") .with_colour(BLUE),
    // ];

    
    // Create a signal to store the lines
    // let (series, set_series) = create_signal(mk_series(vec![]));
    // create_effect(move |_| {
    //     let new_lines = event_types.get().into_iter().map(|et| get_line(et)).collect::<Vec<Line<MyData, f64>>>();
    //     set_series.set(mk_series(new_lines))
    // });

    // Series<MyData, DateTime<Local>, f64>>
    // Update the lines whenever event_types changes
    // create_effect(move |_| {
    //     let new_lines = 
    //         event_types.get().into_iter().map(|et| get_line(et)).collect::<Vec<Line<MyData, f64>>>();
    //     // set_lines.set(new_lines);
    //     set_series.set(mk_series(new_lines));
    // });

    // let series = create_memo(move |_| {
    //     let new_lines = 
    //         event_types.get().into_iter().map(|et| get_line(et)).collect::<Vec<Line<MyData, f64>>>();
    //     mk_series(new_lines)
    // });


    // let series = Signal::derive(move || {
    //     let new_lines = event_types.get().iter().map(|et| get_line(*et)).collect::<Vec<Line<MyData, f64>>>();
    //     mk_series(new_lines)
    // });

    let lines = Signal::derive(move || {
        event_types.get().iter().map(|et| get_line(*et)).collect::<Vec<Line<MyData, f64>>>()
    });

    // let lines = 
    //     event_types.get().into_iter().map(|et| get_line(et)).collect::<Vec<Line<MyData, f64>>>();

    // let series = Series::new(|data: &MyData| data.decision_timestamp)
    //     .lines(lines.get());
        // .with_x_range(0.0, 10.0)
        // .with_y_range(0.0, 0.001); // TODO make the max dynamic

    // Axis
    let x_periods = Timestamps::from_periods(Period::all());
    let x_ticks = TickLabels::from_generator(x_periods.clone());
    let y_ticks = TickLabels::aligned_floats();

    view! {
        <div style="width: 100%; background: white !important;">
            <Chart
                aspect_ratio=AspectRatio::from_outer_height(1500.0, 3.0)
                debug=debug
                series=move || mk_series(lines.get())
                // series={move || Signal::derive(move || mk_series(lines.get())).get()}
                data=data

                top=RotatedLabel::middle("Avg Hourly Rate")
                left=TickLabels::aligned_floats()
                // bottom=Legend::end()
                bottom=vec![x_ticks.clone().into_edge(),
                            // RotatedLabel::middle("This demo shows most of the available options. Edit things below...").into_edge(),
                ]

                inner=[
                    AxisMarker::horizontal_zero().into_inner(),
                    AxisMarker::left_edge().into_inner(),
                    XGridLine::from_ticks(x_ticks).into_inner(),
                    YGridLine::from_ticks(y_ticks).into_inner(),
                    XGuideLine::over_data().into_inner(),
                    YGuideLine::over_mouse().into_inner(),
                ]
                tooltip=Tooltip::left_cursor().show_x_ticks(false)
            />
        </div>
    }
}