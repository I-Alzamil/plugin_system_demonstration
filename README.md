# plugin_system_demonstration
Complete demonstration of a plugin system that runs rust to rust integration using custom interfaces by utilization dynamic libraries.

## Components
### plugin_interface
Holds the interface (trait) that is shared between plugin and main application.

It holds an important variable to track its version. If a plugin built on an interface version 4 tried to load in an interface version 5 application it will refuse to load. This is done so we avoid access violation errors by reading a plugin built on a different interface with different methods than the main app.

### plugin_a & plugin_b
generic example plugin built on interface version 5.

### plugin_c
An old plugin built using interface version 4.

### main_demonstration
main app which will load plugins and try to use them.

## Usage

1. Build all plugins and main app using

    cargo build -all

2. Run master by 

    cargo run -p main_demonstration


## Credit
Inspired from:

* https://github.com/AndrewGaspar/rust-plugin-example
* https://github.com/luojia65/plugin-system-example