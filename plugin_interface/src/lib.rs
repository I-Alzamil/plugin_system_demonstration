pub const INTERFACE_VERSION: &str = "5";

pub trait Plugin {
    fn get_plugin_name(&self) -> &str;
    
    fn get_interface_version(&self) -> &str;
    
    fn say_hello(&self);

    fn update_id(&mut self,value: &str);

    fn get_id(&self) -> String;
}