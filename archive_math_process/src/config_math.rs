use std::time::Duration;

use rsiot::{components::cmp_math::*, executor::Component, message::ValueTime};

use crate::message::*;

const TIME_WINDOW: Duration = Duration::from_millis(100);

pub fn cmp() -> Component<Config<Msg, IntMsg>, Msg> {
    let config = Config {
        fn_input: |msg| -> Option<IntMsg> {
            match msg {
                Msg::MTsdbReader(msg) => match msg {
                    MTsdbReader::AccelX(v) => Some(IntMsg::InputAccelX(v)),
                    MTsdbReader::AccelY(v) => Some(IntMsg::InputAccelY(v)),
                    MTsdbReader::AccelZ(v) => Some(IntMsg::InputAccelZ(v)),
                    MTsdbReader::GyroX(v) => Some(IntMsg::InputGyroX(v)),
                    MTsdbReader::GyroY(v) => Some(IntMsg::InputGyroY(v)),
                    MTsdbReader::GyroZ(v) => Some(IntMsg::InputGyroZ(v)),
                },
                _ => None,
            }
        },
        fn_output: |int_msg| match int_msg {
            IntMsg::InputAccelX(_) => None,
            IntMsg::InputAccelZ(_) => None,
            IntMsg::InputAccelY(_) => None,
            IntMsg::InputGyroX(_) => None,
            IntMsg::InputGyroY(_) => None,
            IntMsg::InputGyroZ(_) => None,
            IntMsg::OutputAccelX(v) => Some(vec![Msg::MMath(MMath::AccelX(v))]),
            IntMsg::OutputAccelY(v) => Some(vec![Msg::MMath(MMath::AccelY(v))]),
            IntMsg::OutputAccelZ(v) => Some(vec![Msg::MMath(MMath::AccelZ(v))]),
            IntMsg::OutputGyroX(v) => Some(vec![Msg::MMath(MMath::GyroX(v))]),
            IntMsg::OutputGyroY(v) => Some(vec![Msg::MMath(MMath::GyroY(v))]),
            IntMsg::OutputGyroZ(v) => Some(vec![Msg::MMath(MMath::GyroZ(v))]),
        },
        algs: vec![
            // Algs::EMA {
            //     kind: EmaKind::Linear,
            //     fn_input_value: |int_msg| match int_msg {
            //         IntMsg::InputAccelX(v) => Some((v.value, v.time)),
            //         _ => None,
            //     },
            //     fn_input_time_window: |_| Some(TIME_WINDOW),
            //     fn_output: |v| {
            //         IntMsg::OutputAccelX(ValueTime {
            //             value: v.ema,
            //             time: v.time,
            //         })
            //     },
            // },
            // Algs::EMA {
            //     kind: EmaKind::Linear,
            //     fn_input_value: |int_msg| match int_msg {
            //         IntMsg::InputAccelY(v) => Some((v.value, v.time)),
            //         _ => None,
            //     },
            //     fn_input_time_window: |_| Some(TIME_WINDOW),
            //     fn_output: |v| {
            //         IntMsg::OutputAccelY(ValueTime {
            //             value: v.ema,
            //             time: v.time,
            //         })
            //     },
            // },
            Algs::SMA {
                fn_input_value: |int_msg| match int_msg {
                    IntMsg::InputAccelX(v) => Some((v.value, v.time)),
                    _ => None,
                },
                fn_input_time_window: |_| Some(TIME_WINDOW),
                fn_output: |v| {
                    IntMsg::OutputAccelX(ValueTime {
                        value: v.sma,
                        time: v.time,
                    })
                },
            },
            Algs::SMA {
                fn_input_value: |int_msg| match int_msg {
                    IntMsg::InputAccelY(v) => Some((v.value, v.time)),
                    _ => None,
                },
                fn_input_time_window: |_| Some(TIME_WINDOW),
                fn_output: |v| {
                    IntMsg::OutputAccelY(ValueTime {
                        value: v.sma,
                        time: v.time,
                    })
                },
            },
        ],
    };

    Cmp::new(config)
}

#[derive(Clone, Copy, Debug)]
pub enum IntMsg {
    InputAccelX(ValueTime),
    InputAccelY(ValueTime),
    InputAccelZ(ValueTime),
    InputGyroX(ValueTime),
    InputGyroY(ValueTime),
    InputGyroZ(ValueTime),
    OutputAccelX(ValueTime),
    OutputAccelY(ValueTime),
    OutputAccelZ(ValueTime),
    OutputGyroX(ValueTime),
    OutputGyroY(ValueTime),
    OutputGyroZ(ValueTime),
}

impl IntMsgBound for IntMsg {}
