use crate::components::charts::DateValuePoint;
use chrono::{Duration, Local};

#[cfg(not(feature = "ssr"))]
use rand::Rng;

pub fn calendar_heatmap_data() -> Vec<(String, f64)> {
    vec![
        ("2026-01-01".into(), 4210.0),
        ("2026-01-03".into(), 8123.0),
        ("2026-01-05".into(), 6750.0),
        ("2026-01-08".into(), 9340.0),
        ("2026-01-10".into(), 2860.0),
        ("2026-01-12".into(), 7312.0),
        ("2026-01-15".into(), 9980.0),
        ("2026-01-18".into(), 4521.0),
        ("2026-01-20".into(), 6145.0),
        ("2026-01-23".into(), 8470.0),
        ("2026-01-26".into(), 2150.0),
        ("2026-01-29".into(), 7034.0),
        ("2026-02-01".into(), 3890.0),
        ("2026-02-03".into(), 5480.0),
        ("2026-02-05".into(), 9210.0),
        ("2026-02-08".into(), 6675.0),
        ("2026-02-10".into(), 4820.0),
        ("2026-02-12".into(), 10320.0),
        ("2026-02-15".into(), 7560.0),
        ("2026-02-18".into(), 3200.0),
        ("2026-02-20".into(), 8445.0),
        ("2026-02-22".into(), 9120.0),
        ("2026-02-25".into(), 5780.0),
        ("2026-02-28".into(), 6420.0),
        ("2026-03-01".into(), 7120.0),
        ("2026-03-03".into(), 9850.0),
        ("2026-03-05".into(), 4360.0),
        ("2026-03-08".into(), 8750.0),
        ("2026-03-10".into(), 2920.0),
        ("2026-03-12".into(), 5640.0),
        ("2026-03-15".into(), 11040.0),
        ("2026-03-18".into(), 6870.0),
        ("2026-03-20".into(), 4555.0),
        ("2026-03-23".into(), 8090.0),
        ("2026-03-26".into(), 9735.0),
        ("2026-03-29".into(), 6150.0),
        ("2026-03-31".into(), 7425.0),
    ]
}

pub fn chart_data() -> Vec<DateValuePoint> {
    let start = Local::now();

    #[cfg(feature = "ssr")]
    {
        (0..10)
            .map(|i| DateValuePoint {
                date: start - Duration::days(i),
                value: 30.0 + (i as f64 * 15.0),
            })
            .collect()
    }

    #[cfg(not(feature = "ssr"))]
    {
        let mut rng = rand::thread_rng();

        (0..10)
            .map(|i| DateValuePoint {
                date: start - Duration::days(i),
                value: (rng.gen_range(0.0_f64..=100.0_f64) * 10.0).round() / 10.0,
            })
            .collect()
    }
}
