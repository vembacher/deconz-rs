use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct RuleRequest {
    actions: Vec<Action>,
    conditions: Vec<Condition>,
    name: String,
    periodic: u128,
    status: RuleStatus,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct RuleResponse {
    actions: Vec<Action>,
    conditions: Vec<Condition>,
    name: String,
    periodic: u128,
    status: RuleStatus,
    created: String,
    etag: String,
    #[serde(rename(serialize = "lasttriggered", deserialize = "lasttriggered"))]
    last_triggered: String,
    owner: String,
    #[serde(rename(serialize = "timestriggered", deserialize = "timestriggered"))]
    times_triggered: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Action {
    address: String,
    body: HashMap<String, serde_json::Value>,
    method: ActionMethod,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "lowercase")]
pub enum RuleStatus {
    Enabled,
    Disabled,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "UPPERCASE")]
pub enum ActionMethod {
    Put,
    Post,
    Delete,
    Bind,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Condition {}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Operator {
    #[serde(rename(serialize = "eq", deserialize = "eq"))]
    Equals,
    #[serde(rename(serialize = "gt", deserialize = "gt"))]
    GreaterThan,
    #[serde(rename(serialize = "lt", deserialize = "lt"))]
    LessThan,
    #[serde(rename(serialize = "dx", deserialize = "dx"))]
    OnChange,
}

