use std::time::Duration;

use rsiot::{components::cmp_timescaledb::*, executor::Component};

use crate::message::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        connection_string: "postgres://postgres:postgres@localhost:5432/db_data".into(),
        max_connections: 5,
        table_name: "math",
        send_period: Duration::from_millis(2000),
        fn_input: |msg: &Msg| match msg {
            Msg::MMath(msg) => match msg {
                MMath::AccelX(v) => Some(vec![Row::new_with_ts(
                    "accelerometer",
                    "accel_x",
                    v.value,
                    v.time,
                )]),
                MMath::AccelY(v) => Some(vec![Row::new_with_ts(
                    "accelerometer",
                    "accel_y",
                    v.value,
                    v.time,
                )]),
                MMath::AccelZ(v) => Some(vec![Row::new_with_ts(
                    "accelerometer",
                    "accel_z",
                    v.value,
                    v.time,
                )]),
                MMath::GyroX(v) => Some(vec![Row::new_with_ts(
                    "accelerometer",
                    "gyro_x",
                    v.value,
                    v.time,
                )]),
                MMath::GyroY(v) => Some(vec![Row::new_with_ts(
                    "accelerometer",
                    "gyro_y",
                    v.value,
                    v.time,
                )]),
                MMath::GyroZ(v) => Some(vec![Row::new_with_ts(
                    "accelerometer",
                    "gyro_z",
                    v.value,
                    v.time,
                )]),
            },
            Msg::MTsdbReader(_) => None,
        },
        delete_before_write: true,
    };

    Cmp::new(config)
}
