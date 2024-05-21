use leptos::*;
use leptos_chartistry::*;

use crate::types::*;


#[component]
pub fn LineChart(debug: ReadSignal<bool>, data: ReadSignal<Vec<MyData>>) -> impl IntoView {

    const RED: Colour = Colour::from_rgb(255, 0, 0);
    const YELLOW: Colour = Colour::from_rgb(255, 255, 0);
    // const BLACK: Colour = Colour::from_rgb(0, 0, 0);
    const GREEN: Colour = Colour::from_rgb(0, 255, 0);
    const BLUE: Colour = Colour::from_rgb(0, 0, 255);
    let sea_green: Colour = "#20b2aa".parse().unwrap();

    // let types = vec!["snooze", "remove", "unsubscribe", "send_mail_action"];
    let lines = vec![
        Line::new(|data: &MyData| data.running_avg_snooze).with_name("running_avg_snooze") .with_colour(GREEN),
        Line::new(|data: &MyData| data.running_avg_remove).with_name("running_avg_remove") .with_colour(RED),
        Line::new(|data: &MyData| data.running_avg_unsubscribe).with_name("running_avg_unsubscribe") .with_colour(YELLOW),
        Line::new(|data: &MyData| data.running_avg_send_mail_action).with_name("running_avg_send_mail_action") .with_colour(BLUE),
    ];

    let series = Series::new(|data: &MyData| data.decision_timestamp)
        .lines(lines);
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
                series
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