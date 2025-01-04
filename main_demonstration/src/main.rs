// THE VARIABLE BELOW MUST BE USED TO KEEP LIBARARY LOADED
// If the variable holding the library drop, the calls to the plugin from the vector "plugins Vec<Box<dyn Plugin>>" will fail
// Dropping the variable will cause the close() function to fire which will case any subsequent calls to plugin to be invalid
use std::sync::RwLock;
static LIBRARIES: RwLock<Vec<libloading::Library>> = RwLock::new(Vec::new());

// List of libraries to load
// you could use a dir loader instead
// for this demonstration, we just use const for quick testing
#[cfg(target_os = "windows")]
const LIBS: [&str; 3] = [
    "target/debug/plugin_a.dll",
    "target/debug/plugin_b.dll",
    "target/debug/plugin_c.dll"
];
#[cfg(target_os = "linux")]
const LIBS: [&str; 3] = [
    "target/debug/libplugin_a.so",
    "target/debug/libplugin_b.so",
    "target/debug/libplugin_c.so"
];
#[cfg(target_os = "macos")]
const LIBS: [&str; 3] = [
    "target/debug/libplugin_a.dylib",
    "target/debug/libplugin_b.dylib",
    "target/debug/libplugin_c.dylib"
];

fn main() {
    // This is where we store our plugins
    let mut plugins: Vec<Box<dyn plugin_interface::Plugin>> = Vec::new();

    // Try to load all libraries specified above in the const LIBS
    for lib in LIBS {
        LIBRARIES
            .write()
            .unwrap()
            .push(
            unsafe {libloading::Library::new(lib)}
                .expect("Unable to load library")
        );
    }

    // Locate symbol "install_plugin" and push it to plugins vector
    let binding = LIBRARIES.read().unwrap();
    for library in binding.iter() {
        // Load install_plugin symbol
        let install_plugin: libloading::Symbol<extern "Rust" fn(host_interface_version: &str) -> Result<Box<dyn plugin_interface::Plugin>,Box<dyn std::error::Error>>> =
            unsafe{library.get(b"install_plugin")}
            .expect("Unable to load symbol")
        ;
        
        // Create instance for the plugin after doing interface version check
        match install_plugin(plugin_interface::INTERFACE_VERSION) {
            Ok(valid_plugin) => plugins.push(valid_plugin),
            Err(e) => eprintln!("{e}"),
        }
    }
    drop(binding);
    
    // Demonstrate the ability to call functions, get return types, modify internal variables and reading it
    for (counter,plugin) in plugins.iter_mut().enumerate() {
        plugin.say_hello(); // call funcions
        plugin.update_id(&format!("NewID{}",counter+1)); // modify internal variables
        println!("[{}] {} updated id",plugin.get_id(),plugin.get_plugin_name()); // return internal variables
    }
}