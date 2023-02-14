/// Filter (downsample & compress) state updates, keep table of most recent state
 
#[cfg(feature = "defmt")]
use defmt::*;
use heapless::LinearMap as HeaplessMap;
use super::state_types::{Entity, Value};
use super::timestamp::Timestamp;

const CAPACITY: usize = 64;
type StatesMap = HeaplessMap<Entity, Filter, CAPACITY>;

pub struct FilteredStates {
    states: StatesMap,
}


#[cfg_attr(feature = "defmt", derive(Format))]
impl FilteredStates {

    pub fn new() -> Self {
        Self {
            states: StatesMap::new()
        }
    }

    pub fn update_state(&mut self, entity: Entity, x: f32) {
        if let Some(state) = self.states.get_mut(&entity) {
            state.update_state(x);
      } else {
            // Register new Entity
            // TODO: determine time_constant and abs_tol from Attribute
            let time_constant = 0.1;
            let abs_tol = 0.5;
            let state = Filter::new(x, time_constant, abs_tol);
            if self.states.insert(entity, state).is_err() {

            }
        };
    }

    pub fn value(&self, entity: &Entity) -> f32 {
        let state = self.states.get(entity).unwrap();
        state.value
    }

    pub fn timestamp(&self, entity: &Entity) -> Timestamp {
        let state = self.states.get(entity).unwrap();
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
    tau: f32,
    abs_tol: f32

}

impl Filter {
    fn new(value: Value, tau: f32, abs_tol: f32) -> Self {
        let t = Timestamp::now();
        Self {
            value: value,
            timestamp: t,
            yk1: value, xk1: value, tk1: t,
            tau: tau,
            abs_tol: abs_tol
        }
    }

    fn update_state(&mut self, x: f32) {
        let ts = Timestamp::now();
        // Current value
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
            let tr = T/self.tau;
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

