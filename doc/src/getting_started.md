# Getting Started

## Create Documentation

```bash
cd /workspace/doc
mdbook build
```

Build and open in browser:

```bash
mdbook serve --open
```

## Overview

* [Embassy](https://embassy.dev)
* Debug probe vs UF2

## Hardware

The examples use an [XIOA ble sense](https://www.seeedstudio.com/Seeed-XIAO-BLE-Sense-nRF52840-p-5253.html), but, with suitable modification, should also work with other [nRF 52840](https://www.nordicsemi.com/Products/nRF52840) boards.

### Components:

* [nRF52840](https://files.seeedstudio.com/wiki/XIAO-BLE/Nano_BLE_MCU-nRF52840_PS_v1.1.pdf)
* [IMU LSM6DS3TR-C](https://files.seeedstudio.com/wiki/XIAO-BLE/ST_LSM6DS3TR_Datasheet.pdf)
  * [Driver](https://crates.io/crates/lsm6ds33)
  * [Usage (bogus)](https://lib.rs/crates/ism330dhcx)
* [Microphone](https://files.seeedstudio.com/wiki/XIAO-BLE/mic-MSM261D3526H1CPM-ENG.pdf)
* [Flash 2MBytes (P25Q16H)](https://files.seeedstudio.com/wiki/github_weiruanexample/Flash_P25Q16H-UXH-IR_Datasheet.pdf)
* [Battery Charger](https://files.seeedstudio.com/wiki/XIAO-BLE/BQ25101.pdf)
* [Pad Positioning](figures/xiao-pad-positioning)

### Pins

![XIAO Pin Map](figures/xiao-pins.png)

#### Internal:

| Function      | Pin   | Comment |
|---------------|-------|---------|
| LED_RED       | P0_26 | LEDs |
| LED_GREEN     | P0_30 | |
| LED_BLUE      | P0_06 | |
| IMU_PWR       | P1_08 | IMU |
| IMU_SCL       | P0_27 | |
| IMU_SDA       | P0_07 | |
| IMU_INT       | P0_11 | |
| MIC_PWR       | P1_10 | Microphone |
| MIC_CLK       | P1_00 | |
| MIC_DATA      | P0_16 | |
| BATT_EN       | P0_14 | Battery Charger |
| VBATT         | P0_31 | |
| CHARGE_STATUS | P0_17 | |
| CHARGE_RATE   | P0_13 | |

### Schematic Diagrams

* [Processor](figures/xiao-schematic-processor.pdf)
* [Peripherals](figures/xiao-schematic-peripherals.pdf)

<img src="figures/xiao-schematic-processor.pdf" alt="drawing" width="5000"/>
<img src="figures/xiao-schematic-peripherals.pdf" alt="drawing" width="5000"/>


## Download

```bash
git clone https://github.com/iot49/rust-nrf.git
cd rust-nrf
git clone https://github.com/embassy-rs/embassy.git
git clone https://github.com/embassy-rs/nrf-softdevice.git
cd examples
./uf2 blink3
```

Copy `target/blink3.uf2` to mcu.

Device will blink 3-color pattern and send log messages to usb port. Display with:

```bash
picocom /dev/tty.usbmodem14101
```

## Documentation

This documentation is at `doc/book/index.html`.

Build cargo docs:

```bash
RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo +nightly doc --document-private-items
```

Located at `examples/target/doc` and `examples/target/thumb3em-none-eabihf/doc`.


## Build

Create `uf2` binary in `target/uf2`:

```bash
./uf2.sh blink3
```

## Object Size

#### Summary

```bash
$ cargo size --bin blink3 --release
    Finished release [optimized] target(s) in 1.26s
   text    data     bss     dec     hex filename
  35376    1176    2968   39520    9a60 blink3
```

* Flash: *text* bytes
* RAM: *data* + *bss* bytes (rest used for stack)
* *Note:* Logging to USB adds about 20kB flash.

#### Detailed

```bash
$ cargo size --release -- -A   
    Finished release [optimized] target(s) in 1.07s
blink3  :
section               size        addr
.vector_table          256     0x27000
.text                29988     0x27100
.rodata               5132     0x2e630
.data                 1176  0x20002000
.gnu.sgstubs             0     0x2fee0
.bss                  1944  0x20002498
.uninit               1024  0x20002c30
.defmt                  86         0x0
.ARM.attributes         58         0x0
.debug_frame         27776         0x0
.debug_loc             109         0x0
.debug_abbrev         3143         0x0
.debug_info         152326         0x0
.debug_aranges        8640         0x0
.debug_ranges       108704         0x0
.debug_str          263228         0x0
.debug_pubnames      90371         0x0
.debug_pubtypes        467         0x0
.debug_line         155381         0x0
.comment                19         0x0
Total               849828
```

[Additional Commands](https://github.com/rust-embedded/cargo-binutils)

## Flash

The XIAO (as well as many other boards e.g. from [Adafruit](https://www.adafruit.com/product/4062)) comes with an [UF2 Bootloader](https://github.com/microsoft/uf2) that permits flashing the application *without* extra hardware (debug probe, e.g. [J-Link](https://www.adafruit.com/product/3571)).

Connect the XIAO (or other UF2 board) to the computer via USB and double press the reset button twice in rapid succession. A new drive appears on the computer. The file `INFO_UF2.TXT` contains information about the bootloader, and, in particular, the SoftDevice loaded. Update `memory.x` if it differs from the one used by the template and update the base address (0x27000) below.

Also change the "family" in the command below if you are using a different processor. If in doubt, procure a "working" uf2 (e.g. [CircuitPython]() binary). Download [uf2conv.py](https://github.com/microsoft/uf2/blob/master/utils/uf2conv.py) and run `./uf2conv.py -i UF2FILE`. The family ID is listed in the output.

## Probe-rs & Defmt

### Flash Softdevice

[Instructions](https://github.com/embassy-rs/nrf-softdevice)

```bash
cargo install probe-rs-cli
probe-rs-cli erase --chip nrf52840
probe-rs-cli download --chip nrf52840 --format hex s140_nrf52_7.3.0_softdevice.hex
```

### Run

```bash
cd nrf/nrf-softdevice/examples && DEFMT_LOG=info cargo run --release --bin ble_advertise
cd nrf/nrf-softdevice/examples && DEFMT_LOG=info cargo run --release --bin ble_bond_peripheral
```

```bash
DEFMT_LOG=info cargo run --release
```