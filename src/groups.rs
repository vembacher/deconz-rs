use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::light::LightEffect;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CreateGroupRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CreateGroupResponse {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGroupResponseWrapper {
    pub success: CreateGroupResponse,
}

#[derive(Serialize, Deserialize)]
pub struct GetAllGroupsEntry {
    #[serde(rename(serialize = "devicemembership", deserialize = "devicemembership"))]
    pub device_membership: Option<Vec<String>>,
    pub name: String,
    pub etag: String,
    pub hidden: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GetGroupsEntry {
    #[serde(rename(serialize = "devicemembership", deserialize = "devicemembership"))]
    pub device_membership: Option<Vec<String>>,
    pub name: String,
    pub etag: String,
    pub hidden: bool,
    pub action: GroupAction,
    pub id: String,
    pub lights: Vec<String>,
    pub lightsequence: Vec<String>,
    pub multideviceids: Vec<String>,
    pub scenes: Vec<SceneEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct SceneEntry {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GroupAction {
    pub on: bool,
    pub bri: u8,
    pub hue: u64,
    pub sat: u8,
    pub ct: u16,
    pub xy: [f64; 2],
    pub effect: LightEffect,
}
