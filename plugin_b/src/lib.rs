use plugin_interface::Plugin;

const PLUGIN_NAME: &str = "PluginB";
const INTERFACE_VERSION: &str = "5";

#[no_mangle]
pub extern "Rust" fn install_plugin(host_interface_version: &str) -> Result<Box<dyn Plugin>,Box<dyn std::error::Error>> {
    match host_interface_version == INTERFACE_VERSION {
        true => Ok(Box::new(PluginB::new())),
        false => Err(format!("[!] Error: Failed to load the {} due to incompatible interface version",PLUGIN_NAME).into()),
    }
}

pub struct PluginB {
    id: String,
}

impl PluginB {
    fn new() -> PluginB {
        let id = format!("{:08x}", rand::random::<u32>());
        println!("[{}] {} Created instance!",id,PLUGIN_NAME);
        PluginB { id }
    }
}

impl Plugin for PluginB {
    fn get_plugin_name(&self) -> &str {
        PLUGIN_NAME
    }
    
    fn get_interface_version(&self) -> &str {
        INTERFACE_VERSION
    }
    
    fn say_hello(&self) {
        println!("[{}] {} Hello from plugin!",self.id,PLUGIN_NAME);
    }

    fn update_id(&mut self,value: &str) {
        self.id = value.to_string();
    }
    
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl Drop for PluginB {
    fn drop(&mut self) {
        println!("[{}] {} Destroyed instance!",self.id,PLUGIN_NAME);
    }
}