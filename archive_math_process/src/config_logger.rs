use rsiot::{components::cmp_logger::*, message::Message};

use crate::message::*;

pub fn cmp() -> rsiot::executor::Component<Config<Msg>, Msg> {
    let config = Config {
        level: Level::INFO,
        // fn_input: |msg: Message<Msg>| {
        //     let text = msg.serialize_data()?;
        //     Ok(Some(text))
        // },
        fn_input: |msg: Message<Msg>| {
            return Ok(None);

            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };

            let text = match msg {
                Msg::MMath(v) => format!("Math: {:?}", v),
                Msg::MTsdbReader(_) => return Ok(None),
            };
            Ok(Some(text))
        },
    };

    Cmp::new(config)
}
