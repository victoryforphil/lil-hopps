use std::{collections::BTreeSet, sync::{Arc, Mutex}};

use tracing::{debug, error};
use victory_commander::system::System;
use victory_data_store::{database::DataView, topics::TopicKey};
use victory_wtf::Timespan;


use crate::mavlink::{core::{QuadLinkCore, QuadlinkCoreHandle}, messages::QuadMessageRx};


pub struct QuadlinkSystem{
    mavlink: QuadlinkCoreHandle,
}

impl QuadlinkSystem{
    pub fn new(mavlink: QuadlinkCoreHandle) -> Self{
        Self{mavlink}
    }

    pub fn new_from_connection_string(connection_string: &str) -> Result<Self, anyhow::Error>{
        let mavlink = QuadLinkCore::new(connection_string)?;
        Ok(Self{mavlink: Arc::new(Mutex::new(mavlink))})
    }

    fn proccess_message(&mut self, msg: QuadMessageRx, data_view: &mut DataView){

        match msg{
            QuadMessageRx::ParamValue(p_key, p_value) => {
                debug!("RX Got param update: {} = {}", p_key, p_value);
                let topic_key = TopicKey::from_str(&format!("params/{}", p_key));
                data_view.add_latest(&topic_key, p_value);
            }
        };
    }
}   

impl System for QuadlinkSystem{
    fn init(&mut self) {
        let mut mavlink = self.mavlink.lock().unwrap();
        mavlink.start_thread().unwrap();
    }

    fn get_subscribed_topics(&self) -> std::collections::BTreeSet<TopicKey> {
        let topics = BTreeSet::new();
      
        topics
    }

    fn execute<'a>(&mut self, _: &'a DataView, _: Timespan) -> DataView {
        let mut output = DataView::new();

       let mut msgs = vec![];
       {
            let mavlink = self.mavlink.lock().unwrap();
            msgs = mavlink.recv().unwrap();
       }
        for msg in msgs{
            self.proccess_message(msg, &mut output);
        }
        output
    }

    fn cleanup(&mut self) {

    }
}