use std::time::Duration;

use rsiot::{components::cmp_timescaledb_reader::*, executor::Component};
use time::macros::datetime;

use super::message::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        connection_string: "postgres://postgres:postgres@localhost:5432/db_data".into(),
        max_connections: 5,
        time_begin: datetime!(2025-08-19 07:17:13.000+03),
        time_end: datetime!(2025-08-19 07:18:18.000+03),
        items: vec![
            ConfigItem {
                entity: "accelerometer",
                attr: "accel_x",
                fn_output: |value| Msg::MTsdbReader(MTsdbReader::AccelX(value)),
            },
            ConfigItem {
                entity: "accelerometer",
                attr: "accel_y",
                fn_output: |value| Msg::MTsdbReader(MTsdbReader::AccelY(value)),
            },
            ConfigItem {
                entity: "accelerometer",
                attr: "accel_z",
                fn_output: |value| Msg::MTsdbReader(MTsdbReader::AccelZ(value)),
            },
            // ConfigItem {
            //     entity: "accelerometer",
            //     attr: "gyro_x",
            //     fn_output: |value| Msg::MTsdbReader(MTsdbReader::GyroX(value)),
            // },
            // ConfigItem {
            //     entity: "accelerometer",
            //     attr: "gyro_y",
            //     fn_output: |value| Msg::MTsdbReader(MTsdbReader::GyroY(value)),
            // },
            // ConfigItem {
            //     entity: "accelerometer",
            //     attr: "gyro_z",
            //     fn_output: |value| Msg::MTsdbReader(MTsdbReader::GyroZ(value)),
            // },
        ],
        delay_between_msgs: Duration::from_micros(20),
        shutdown_delay: Duration::from_secs(5),
    };
    Cmp::new(config)
}
