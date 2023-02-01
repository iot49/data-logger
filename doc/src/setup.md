# Setup

* [template](https://github.com/titanclass/embassy-start)
* Softdevice:
  * [instructions](https://github.com/embassy-rs/nrf-softdevice)
  * erase flash & download softdevice:
    * nrf52840 (xenon, etc):
      * `probe-rs-cli erase --chip nrf52840 && probe-rs-cli download --chip nrf52840 --format hex  ../resources/s140_nrf52_7.3.0_softdevice.hex`
    * microbit-v2:
      * `probe-rs-cli erase --chip nrf52833 && probe-rs-cli download --chip nrf52833 --format hex  ../resources/s140_nrf52_7.3.0_softdevice.hex`
* run:
  * nrf52840 (xenon, etc):
    * `cargo run`
  * microbit-v2:
    * `cargo run --target thumbv7em-none-eabihf --features "microbit-v2" --no-default-features`
  