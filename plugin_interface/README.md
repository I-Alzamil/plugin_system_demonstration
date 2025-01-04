# plugin_interface
Holds the interface (trait) that is shared between plugin and main application.

It holds an important variable to track its version. If a plugin built on an interface version 4 tried to load in an interface version 5 application it will refuse to load. This is done so we avoid access violation errors by reading a plugin built on a different interface with different methods than the main app.