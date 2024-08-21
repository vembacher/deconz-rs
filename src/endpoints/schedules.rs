use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ScheduleRequest {
    name: String,
    description: String,
    status: ScheduleStatus,
    time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ScheduleAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<ScheduleStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<String>,
    #[serde(rename(serialize = "autodelete", deserialize = "autodelete"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_delete: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ScheduleCommand {
    address: String,
    body: HashMap<String, serde_json::Value>,
    method: ScheduleMethod,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "UPPERCASE")]
pub enum ScheduleMethod {
    Put,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "lowercase")]
pub enum ScheduleStatus {
    Enabled,
    Disabled,
}