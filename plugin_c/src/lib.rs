use plugin_interface_old::Plugin;

const PLUGIN_NAME: &str = "PluginC";
const INTERFACE_VERSION: &str = "4";

#[no_mangle]
pub extern "Rust" fn install_plugin(host_interface_version: &str) -> Result<Box<dyn Plugin>,Box<dyn std::error::Error>> {
    match host_interface_version == INTERFACE_VERSION {
        true => Ok(Box::new(PluginC::new())),
        false => Err(format!("[!] Error: Failed to load the {} due to incompatible interface version",PLUGIN_NAME).into()),
    }
}

pub struct PluginC {
    id: String,
}

impl PluginC {
    fn new() -> PluginC {
        let id = format!("{:08x}", rand::random::<u32>());
        println!("[{}] {} Created instance!",id,PLUGIN_NAME);
        PluginC { id }
    }
}

impl Plugin for PluginC {
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
}

impl Drop for PluginC {
    fn drop(&mut self) {
        println!("[{}] {} Destroyed instance!",self.id,PLUGIN_NAME);
    }
}