use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::endpoints::light::LightState;

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Scene {
    lights: Vec<String>,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct SceneAttributes {
    lights: Vec<LightState>,
    name: String,
    #[serde(rename(serialize = "state", deserialize = "state"))]
    __state: Option<u128>,
}