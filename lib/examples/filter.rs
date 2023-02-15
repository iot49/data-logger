use logger_lib::state_filter::*;
use logger_lib::state_types::*;


fn main() {
    let mut filter = FilteredStates::new();

    let entity = Entity::new(DeviceType::Gps, 5, Attribute::Latitude);

    for i in 0..10 {
        let x = i as f32;
        filter.update_state(entity, x);
    }

}