use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
pub struct Light {
    #[serde(rename(serialize = "colorcapabilities", deserialize = "colorcapabilities"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_capabilities: Option<u64>,
    #[serde(rename(serialize = "ctmax", deserialize = "ctmax"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct_max: Option<u64>,
    #[serde(rename(serialize = "ctmin", deserialize = "ctmin"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct_min: Option<u64>,
    #[serde(rename(serialize = "lastannounced", deserialize = "lastannounced"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_announced: Option<String>,
    #[serde(rename(serialize = "lastseen", deserialize = "lastseen"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    pub etag: String,
    #[serde(rename(serialize = "hascolor", deserialize = "hascolor"))]
    pub has_color: bool,
    #[serde(rename(serialize = "manufacturername", deserialize = "manufacturername"))]
    pub manufacturer_name: String,
    pub name: String,
    #[serde(rename(serialize = "modelid", deserialize = "modelid"))]
    pub model_id: String,
    #[serde(rename(serialize = "powerup", deserialize = "powerup"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_up: Option<u8>,
    #[serde(rename(serialize = "swversion", deserialize = "swversion"))]
    pub sw_version: String,
    pub r#type: String,
    pub state: LightState,
    #[serde(rename(serialize = "uniqueid", deserialize = "uniqueid"))]
    pub unique_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[derive(PartialEq)]
pub struct LightState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<AlertMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bri: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<ColorMode>,
    #[serde(rename(serialize = "colorloopspeed", deserialize = "colorloopspeed"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_loop_speed: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat: Option<u8>,
    #[serde(rename(serialize = "transitiontime", deserialize = "transitiontime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xy: Option<[f64; 2]>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[derive(PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AlertMode {
    None,
    Select,
    Lselect,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[derive(PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ColorMode {
    None,
    Hs,
    Xy,
    Ct,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LightEffect {
    None,
    ColorLoop,
}


impl Light {
    pub fn change_brightness(&mut self, delta: i16) -> &mut Self {
        let new_bri = self.state.bri.unwrap_or(0) as i16 + delta;
        self.state.bri = Some(new_bri.max(0).min(u8::MAX as i16) as u8);
        self
    }

    pub fn change_hue(&mut self, delta: i64) -> &mut Self {
        let new_hue = self.state.hue.unwrap_or(0) as i64 + delta;
        self.state.hue = Some(new_hue.max(0).min(u32::MAX as i64) as u32);
        self
    }

    pub fn change_color_temperature(&mut self, delta: i128) -> &mut Self {
        if self.ct_max.is_none() || self.ct_min.is_some() { return self; }
        let new_ct = self.state.ct.unwrap_or(0) as i128 + delta;
        self.state.ct = Some(new_ct.max(self.ct_min.unwrap().into()).min(self.ct_max.unwrap().into()) as u64);
        self
    }
    pub fn on(&mut self, on: bool) -> &mut Self {
        self.state.on = Some(on);
        self
    }
}
