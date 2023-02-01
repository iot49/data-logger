use defmt::*;
use crate::comm::{StateBus, StateSub};

#[embassy_executor::task]
pub async fn main_task(comm: &'static StateBus) {
    let mut subs: StateSub = comm.subscriber().unwrap();

    loop {
        let e = subs.next_message_pure().await;
        info!{"LOG: {}", e};
    }
}
