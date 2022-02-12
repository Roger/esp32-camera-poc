PoC esp32-camera rust binding
=============================

The bingings are generated using bindgen cli, using the cmake arguments from esp-idf-sys
and including missing paths from esp32 headers, using this header as an input https://github.com/espressif/esp32-camera/blob/master/driver/include/esp_camera.h

The camera works, in can take pictures, the code is converting those images to ascii,
and sending to stdout, using something like espmonitor you can see the ascii images.

Code is partially a translation of examples in https://github.com/espressif/esp32-camera/ from C to Rust

Demo:
-----

[![asciicast](https://asciinema.org/a/VdVWglIPwOHM0yemYC3YJzMSL.svg)](https://asciinema.org/a/VdVWglIPwOHM0yemYC3YJzMSL)

Problems encountered while creating the PoC:
--------------------------------------------

* Need to include Kconfig from esp32-camera in pio-project generated in esp-idf-sys

  This can be solved using `ESP_IDF_GLOB_XX` to include files at build time in the proejct
  (see .cargo/config.toml)

* Add esp32-camera to components in `CMakeLists.txt`

  Partialy solved using `ESP_IDF_GLOB_XX`, but for some reason the linker fails the first clean
  build, doing `touch sdkconfig.defaults` and `cargo build` after failure make this works.
  I'm asuming that the Cmake file is replaced after the build is done, that's why a second
  build fixes this.

* Need to add `lib_deps` to platformio

  This can be done using `ESP_IDF_PIO_CONF_ENV` (see .cargo/config.toml)

* Building in release mode fails compiling num-rational with this error https://github.com/esp-rs/rust/issues/87

  To workaround this, I changed the profile for that library to use what's on debug mode

* The code in ./pio-proj/CMakeLists.txt doesn't know if the build is release/debug, so
  it needs to be changed by hand depending in what type of build you are working on,
  to discover where the component lives inside libdeps

* There's no way to tell `cargo pio espidf menuconfig` to use the Kconfig from esp32-camera,
  so I ended up running this in C project created with platformio and copied by hand the
  config to the root of the repo.

* This is not a problem, just a bit annoying, in the project embuild downloads platformio
  and the dependencies, but then `cargo pio` doesn't detect the installation by default and
  downloads again to your home directory, to save some space in disk I tell cargo-pio to use
  the already downloaded platformio stuff like this: `cargo pio espidf -i $PWD/.embuild/platformio`
