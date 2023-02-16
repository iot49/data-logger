use super::msg::Msg;


struct Comm;

impl Comm {

    pub fn publish_state(&self, device: Device, attr: Attribute, value: Value) {
        let entity = Entity { device: device, attr: attr };
        if self.state_bus.try_send((entity, value)).is_err() {
            self.log_str("Dropped state update (buffer full)");
        }
    }

    pub fn log(msg: String) {
        
    }

    pub async fn rx() -> Msg {   
    }

    pub async fn tx(msg: Msg) {  
    }

}