use leptos::*;
use leptos_chartistry::*;

use crate::types::*;


#[component]
pub fn LineChart(debug: ReadSignal<bool>, data: ReadSignal<Vec<MyData>>) -> impl IntoView {
    // Lines are added to the series
    let series = Series::new(|data: &MyData| data.decision_timestamp)
        .line(Line::new(|data: &MyData| data.running_avg).with_name("running_avg_rate"));
        // .with_x_range(0.0, 10.0)
        // .with_y_range(0.0, 0.001); // TODO make the max dynamic

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