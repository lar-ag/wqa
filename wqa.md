# `Water quality monitoring station`

@font:`PragmataPro`

*Move it to gui readme*
[Building a fast Electron app with Rust](https://keminglabs.com/blog/building-a-fast-electron-app-with-rust/)
[Hello Rust](https://github.com/hello-rust)
 `umweltanalytik` ist ein Teilbereich der chemischen Analytik und beschäftigt sich mit der qualitativen und quantitativen Untersuchung von Stoffen in der Umwelt. Umweltkompartimente Luft (einschließlich Innenraumluft), Boden und **Wasser** und können sowohl einzelne Stoffe als auch Summenparameter umfassen. 

[Wiki](https://de.wikipedia.org/wiki/Umweltanalytik)

## Summenparameter in der Wasseranalytik

- Biochemischer Sauerstoffbedarf (BSB)
- Permanganat-Index (PI)
- Chemischer Sauerstoffbedarf (CSB, gelegentlich noch CSV - Chemischer Sauerstoffverbrauch, engl. Chemical Oxygen Demand, COD)
- Gesamter Organischer Kohlenstoff (Total Organic Carbon, TOC)
- Gesamter gelöster organischer Kohlenstoff (Dissolved Organic Carbon, DOC)
- Leitfähigkeit

***Gruppenparameter?***

- Adsorbierbare organisch gebundene Halogene, AOX (X steht für Halogene)
- Extrahierbare organisch gebundene Halogene, EOX
- Ausblasbare (flüchtige) organisch gebundene Halogene (POX, von „purgable“)

Anwendung in `Water Quality Monitoring Station` sollte die Umstieg von C auf Rust erleichtern.



## Betribsablauf

```bob
          ①                                 ②                                    ③
                       ┌─────────────────────────────────────────┐ ┌───────────────────────────────┐
                       │      ┌────────────────────────┐         │ │                 ┌─────────┐   │
     ┌─────────────┐   │  ┌───┴─────────────┐  ┌───────┴───┐     │ │┌───────────┐ ┌──┤`display`│   │
 O───┤`calibration`├───┼──┤ `measurement`   ├──┤`replicate`├─────┼─┼┤`transmit` ├─┤  └─────────┘   │
     ╰─────────────┘   │  └───────┬─────────┘  └─────┬─────┘     │ │╰───────────┘ │  ┌─────────┐   │
                       │          └──────────────────┘           │ │              └──┤`extern` │   │
                       │*statistic*                              │ │ *output*        └─────────┘   │
                       ╰─────────────────────────────────────────╯ └───────────────────────────────┘

```

[Documentation](https://docs.lar.io/umweltanalytic)

## Einfuhrung

*Wichtige Informnationen*
[GitHub](https://github.com/rust-embedded)
[RISCV](https://github.com/riscv-rust)
Die Kopie `rust-embedded`
> Device driver developers should consider building on top of the
> [embedded-hal](https://crates.io/crates/embedded-hal) traits rather than directly coupling to this library. An implementation of those generic traits for Linux can be found in
> [linux-embedded-hal](https://crates.io/crates/linux-embedded-hal) which, at present, uses this crate as the backend for I2C.

Die Kopie `rust-i2dev`

> The Rust `i2cdev` crate seeks to provide full access to the Linux i2cdev
> driver interface in Rust without the need to wrap any C code or directly make
> low-level system calls.  The documentation for the i2cdev interace can
> be found at https://www.kernel.org/doc/Documentation/i2c/dev-interface and
> in the [lm-sensors projects](http://www.lm-sensors.org/).

The source includes an example of using the library to talk to a Wii
Nunchuck (which has an i2c interface).
[Go View the Example](https://github.com/rust-embedded/rust-i2cdev/blob/master/examples/nunchuck.rs).

Here's a real quick example showing the guts of how you create
device and start talking to it... 

```rust,no_run,skeptic-template
extern crate i2cdev;

use std::thread;
use std::time::Duration;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const NUNCHUCK_SLAVE_ADDR: u16 = 0x52;

// real code should probably not use unwrap()
fn i2cfun() -> Result<(), LinuxI2CError> {
    let mut dev = LinuxI2CDevice::new("/dev/i2c-1", NUNCHUCK_SLAVE_ADDR)?;

    // init sequence
    dev.smbus_write_byte_data(0xF0, 0x55)?;
    dev.smbus_write_byte_data(0xFB, 0x00)?;
    thread::sleep(Duration::from_millis(100));

    loop {
        let mut buf: [u8; 6] = [0; 6];
        dev.smbus_write_byte(0x00).unwrap();
        thread::sleep(Duration::from_millis(10));
        dev.read(&mut buf).unwrap();
        println!("Reading: {:?}", buf);
    }
}
```

In addition to the Read/Write traits, the following methods are
available via the [I2CDevice trait](https://rust-embedded.github.io/rust-i2cdev/i2cdev/core/trait.I2CDevice.html).


## Cross Compiling

Most likely, the machine you are running on is not your development
machine (although it could be).  In those cases, you will need to
cross-compile.  See https://github.com/japaric/rust-cross for pointers.
