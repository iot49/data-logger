# data-logger

Embedded logger for sensor data with history on flash and ble client interface

* server: actually collects the data & communicates over BLE; on nrf52840, rust
  * `cargo run`
* lib: rust code that runs on server (embedded) and host (macos)
* client: displays data; on phone, flutter
  * `flutter run` (make sure phone `A13` is selected as target - see statusbar lower right)
* embed-template: starter for embedded rust project (with ble)
* nus: dev Nordic UART on nrf52840, communicates with client; TODO: merge into server
* host: rust development on host (macos)
* lsm6ds3tr: imu driver (only temperature works), for xiao-sense
* resources: stuff downloaded from web