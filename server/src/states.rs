use defmt::*;
use logger_lib::state_filter::FilteredStates;
use crate::comm::Comm;

#[embassy_executor::task]
pub async fn main_task(comm: &'static Comm) {
    info!("states task running");
    let mut states = FilteredStates::new();

    loop {
        let (entity, value) = comm.state_bus.recv().await;
        info!("got {} = {}", entity, value);
        states.update_state(entity, value);
        let filtered_value = states.value(&entity);
        let timestamp = states.timestamp(&entity);
        info!("entity {} = {}", entity, filtered_value);
    }

}