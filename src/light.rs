use std::u64;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Light {
    #[serde(rename(serialize = "colorcapabilities", deserialize = "colorcapabilities"))]
    pub color_capabilities: u64,
    #[serde(rename(serialize = "ctmax", deserialize = "ctmax"))]
    pub ct_max: u64,
    #[serde(rename(serialize = "ctmin", deserialize = "ctmin"))]
    pub ct_min: u64,
    #[serde(rename(serialize = "lastannounced", deserialize = "lastannounced"))]
    pub last_announced: Option<String>,
    #[serde(rename(serialize = "lastseen", deserialize = "lastseen"))]
    pub last_seen: String,
    pub etag: String,
    #[serde(rename(serialize = "hascolor", deserialize = "hascolor"))]
    pub has_color: bool,
    #[serde(rename(serialize = "manufacturername", deserialize = "manufacturername"))]
    pub manufacturer_name: String,
    pub name: String,
    #[serde(rename(serialize = "modelid", deserialize = "modelid"))]
    pub model_id: String,
    #[serde(rename(serialize = "powerup", deserialize = "powerup"))]
    pub power_up: Option<u8>,
    #[serde(rename(serialize = "swversion", deserialize = "swversion"))]
    pub sw_version: String,
    pub r#type: String,
    pub state: LightState,
    #[serde(rename(serialize = "uniqueid", deserialize = "uniqueid"))]
    pub unique_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum AlertMode {
    None,
    Select,
    Lselect,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ColorMode {
    None,
    Hs,
    Xy,
    Ct,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LightEffect {
    None,
    ColorLoop,
}

#[derive(Error, Debug)]
pub enum LightStateError {
    #[error("Invalid value for field, not in range {from}-{to}")]
    ImpossibleDelta {
        from: isize,
        to: isize,
        delta: isize,
    },
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
        let new_ct = self.state.ct.unwrap_or(0) as i128 + delta;
        self.state.ct = Some(new_ct.max(self.ct_min.into()).min(self.ct_max.into()) as u64);
        self
    }
    pub fn on(&mut self, on: bool) -> &mut Self {
        self.state.on = Some(on);
        self
    }
}


#[cfg(test)]
mod light_tests {
    use std::str::FromStr;
    use std::time::Duration;
    use tokio::time::sleep;
    use crate::connection::{DeconzConnection, RequestResponse};

    #[tokio::test]
    async fn test_set_light() {
        let url = url::Url::from_str("http://192.168.0.166:80/").unwrap();

        let connection = DeconzConnection::new(url, "D453E7BAF8".to_string()).unwrap();
        let mut lights = connection.get_all_lights().await.unwrap();
        println!("{:?}", lights);
        let mut ceiling = lights.remove("1").unwrap();
        let mut small = lights.remove("2").unwrap();

        ceiling.change_brightness(-254).on(true);
        small.change_brightness(-254).on(true);

        ceiling.state.on = Some(true);
        small.state.on = Some(true);

        ceiling.state.ct = Some(ceiling.ct_min);
        small.state.ct = Some(small.ct_min);

        connection.set_light_state("1", &ceiling.state).await.unwrap();
        connection.set_light_state("2", &small.state).await.unwrap();
        for _ in 0..254 {
            sleep(Duration::from_millis(200)).await;


            for (id, light) in [("1", &mut ceiling), ("2", &mut small)] {
                light
                    .change_brightness(1)
                    .change_color_temperature(1);
                match connection.set_light_state(id, &light.state).await {
                    Ok(response) => {
                        response.iter().for_each(|att| {
                            match att {
                                RequestResponse::Error { address, description, r#type } => {
                                    eprintln!("{address} failed with '{description}' (code: {:?})", r#type)
                                }
                                RequestResponse::Success(success) => {
                                    println!("{success:?}");
                                }
                            }
                        })
                    }
                    Err(err) => {
                        eprintln!("{err}");
                    }
                }
            }
        }
    }
}