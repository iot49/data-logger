/// Filter (downsample & compress) state updates, keep table of most recent state
 
#[cfg(feature = "defmt")]
use defmt::*;
// use std::prelude::v1::*;
use std::collections::BTreeMap;
use super::state_types::{Entity, Value};
use super::timestamp::Timestamp;

type StatesMap = BTreeMap<Entity, Filter>;

// #[cfg_attr(feature = "defmt", derive(Format))]
pub struct FilteredStates(StatesMap);


impl FilteredStates {

    pub fn new() -> Self {
        Self(StatesMap::new())
    }

    pub fn update_state(&mut self, entity: Entity, x: f32) {
        if let Some(state) = self.0.get_mut(&entity) {
            state.update_state(x);
      } else {
            // Register new Entity
            // TODO: determine wc=1/tau and abs_tol from Attribute
            let wc: f32 = 10.0;
            let abs_tol = 0.5;
            let state = Filter::new(x, wc, abs_tol);
            self.0.insert(entity, state);
        };
    }

    pub fn value(&self, entity: &Entity) -> f32 {
        let state = self.0.get(entity).unwrap();
        state.value
    }

    pub fn timestamp(&self, entity: &Entity) -> Timestamp {
        let state = self.0.get(entity).unwrap();
        state.timestamp
    }

}


#[cfg_attr(feature = "defmt", derive(Format))]
struct Filter {
    /// current (i.e. last updated value and timestamp)
    value: f32,
    timestamp: Timestamp,
    /// filter state: y[k-1], x[k-1], t[k-1]
    yk1: f32, xk1: f32,  tk1: Timestamp,
    /// filter parameters; time_constant same unit as timestamp [sec]
    wc: f32,
    abs_tol: f32

}

impl Filter {
    fn new(value: Value, wc: f32, abs_tol: f32) -> Self {
        let t = Timestamp::now();
        Self {
            value: value,
            timestamp: t,
            yk1: value, xk1: value, tk1: t,
            wc: wc,
            abs_tol: abs_tol
        }
    }

    fn update_state(&mut self, x: f32) {
        let ts = Timestamp::now();
        let value = self.value;
        if abs(value-x) > self.abs_tol {
            // Dramatic change, ignore filter
            self.value = x;
            self.timestamp = ts;
            // filter state
            self.yk1 = x;
            self.xk1 = x;
            self.tk1 = ts;
        } else {
            // 1st Order bilinear LPF
            #[allow(non_snake_case)]
            let T = ts - self.tk1;
            let tr = T*self.wc;
            let y = (2.0-tr)*self.yk1 + tr*(x+self.xk1);
            let y = y / (2.0+tr);
            // update state
            self.yk1 = y;
            self.xk1 = x;
            self.tk1 = ts;
            // Update necessary?
            if abs(y-self.value) > self.abs_tol {
                #[cfg(feature = "defmt")]
                info!("--- update state {} -> {} -> {}", self, self.value, y);
                self.value = y;
                self.timestamp = ts;
            }
        }
    }

}


fn abs(x: f32) -> f32 {
    if x>=0.0 {
        x
    } else {
        -x
    }
}

