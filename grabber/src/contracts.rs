use ethers::abi::{Contract, Event, ParamType, Token};
use ethers::types::Bytes;
use indexmap::IndexMap;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct EventParams {
    pub topics: Option<IndexMap<String, ParamType>>,
    pub data: Option<IndexMap<String, ParamType>>,
}

#[allow(dead_code)]
pub async fn decode_data_with_params(
    data: &Bytes,
    data_params: IndexMap<String, ParamType>,
) -> Vec<Token> {
    ethers::abi::decode(
        &data_params.into_values().collect::<Vec<ParamType>>(),
        data.as_ref(),
    )
    .unwrap()
}

pub fn find_event(abi: &Contract, param_name: impl Into<String>) -> EventParams {
    let all_params = contract_get_params(abi, param_name).unwrap();

    let mut event_params = EventParams {
        topics: None,
        data: None,
    };
    let mut event_topics = IndexMap::with_capacity(10);
    let mut event_data = IndexMap::with_capacity(10);

    for param in all_params {
        if param.1 .1 {
            event_topics.insert(param.0, param.1 .0);
        } else {
            event_data.insert(param.0, param.1 .0);
        }
    }
    if !event_topics.is_empty() {
        event_params.topics = Some(event_topics);
    }
    if !event_data.is_empty() {
        event_params.data = Some(event_data);
    }

    event_params
}

fn contract_get_params(
    abi: &Contract,
    param_name: impl Into<String>,
) -> Option<IndexMap<String, (ParamType, bool)>> {
    let param_name = param_name.into();
    let mut params = IndexMap::with_capacity(10);
    let events: BTreeMap<String, Vec<Event>> = abi.events.clone();
    let found_events = match events.get(&*param_name) {
        None => return None,
        Some(v) => v,
    };
    for event in found_events {
        for input in event.clone().inputs {
            params.insert(input.name, (input.kind, input.indexed));
        }
    }
    Some(params)
}
