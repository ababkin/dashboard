use leptos::signal_prelude::*;

pub struct MyData {
    pub x: f64,
    pub y1: f64,
    pub y2: f64,
}

impl MyData {
    fn new(x: f64, y1: f64, y2: f64) -> Self {
        Self { x, y1, y2 }
    }
}

pub fn load_data() -> Signal<Vec<MyData>> {
    Signal::derive(|| {
        vec![
            MyData::new(0.0, 1.0, 0.0),
            MyData::new(1.0, 3.0, 1.0),
            MyData::new(2.0, 5.0, 2.5),
            MyData::new(3.0, 5.5, 3.0),
            MyData::new(4.0, 5.0, 3.0),
            MyData::new(5.0, 2.5, 4.0),
            MyData::new(6.0, 2.25, 9.0),
            MyData::new(7.0, 3.0, 5.0),
            MyData::new(8.0, 7.0, 3.5),
            MyData::new(9.0, 8.5, 3.2),
            MyData::new(10.0, 10.0, 3.0),
        ]
    })
}