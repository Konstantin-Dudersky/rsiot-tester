use serde::{Deserialize, Serialize};

use rsiot::message::{MsgDataBound, MsgKey, ValueTime};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    MMath(MMath),
    MTsdbReader(MTsdbReader),
}

impl MsgDataBound for Msg {}

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum MTsdbReader {
    AccelX(ValueTime),
    AccelY(ValueTime),
    AccelZ(ValueTime),
    GyroX(ValueTime),
    GyroY(ValueTime),
    GyroZ(ValueTime),
}

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum MMath {
    AccelX(ValueTime),
    AccelY(ValueTime),
    AccelZ(ValueTime),
    GyroX(ValueTime),
    GyroY(ValueTime),
    GyroZ(ValueTime),
}
