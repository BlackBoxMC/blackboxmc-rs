#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct JavaPlugin<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for JavaPlugin<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for JavaPlugin<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate JavaPlugin from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/java/JavaPlugin")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a JavaPlugin object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> JavaPlugin<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::plugin::java::JavaPlugin<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/plugin/java/Plugin"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::java::JavaPlugin::from_raw(&jni,res
)}
	pub fn plugin_loader(&self) 
-> Result<crate::plugin::PluginLoader<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::plugin::PluginLoader;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPluginLoader",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::PluginLoader::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn server(&self) 
-> Result<crate::Server<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::Server;");
let res = self.jni_ref().call_method(&self.jni_object(),"getServer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Server::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_enabled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isEnabled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn description(&self) 
-> Result<crate::plugin::PluginDescriptionFile<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::plugin::PluginDescriptionFile;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDescription",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::PluginDescriptionFile::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn config(&self) 
-> Result<crate::configuration::file::FileConfiguration<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::file::FileConfiguration;");
let res = self.jni_ref().call_method(&self.jni_object(),"getConfig",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::configuration::file::FileConfiguration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn reload_config(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"reloadConfig",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn save_config(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"saveConfig",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn save_default_config(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"saveDefaultConfig",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn save_resource(&self,resource_path: impl Into<String>,replace: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Z)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(resource_path.into())?));
let val_2 = jni::objects::JValueGen::Bool(replace.into());
let res = self.jni_ref().call_method(&self.jni_object(),"saveResource",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn on_command(&self,sender: impl Into<crate::command::CommandSender<'mc>>,command: impl Into<crate::command::Command<'mc>>,label: impl Into<String>,val_args: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Lorg/bukkit/command/Command;Ljava/lang/String;Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(command.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(label.into())?));
let val_4 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"onCommand",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn on_tab_complete(&self,sender: impl Into<crate::command::CommandSender<'mc>>,command: impl Into<crate::command::Command<'mc>>,alias: impl Into<String>,val_args: impl Into<String>) 
-> Result<Option<Vec<jni::objects::JObject<'mc>>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Lorg/bukkit/command/Command;Ljava/lang/String;Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(command.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(alias.into())?));
let val_4 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"onTabComplete",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
Some(
new_vec
)
)}
	pub fn get_command(&self,name: impl Into<String>) 
-> Result<Option<crate::command::PluginCommand<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lcrate::command::PluginCommand;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getCommand",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::command::PluginCommand::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn on_load(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"onLoad",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn on_disable(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"onDisable",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn on_enable(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"onEnable",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn get_default_world_generator(&self,world_name: impl Into<String>,id: impl Into<String>) 
-> Result<Option<crate::generator::ChunkGenerator<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/lang/String;)Lcrate::generator::ChunkGenerator;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(world_name.into())?));
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(id.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultWorldGenerator",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::generator::ChunkGenerator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn get_default_biome_provider(&self,world_name: impl Into<String>,id: impl Into<String>) 
-> Result<Option<crate::generator::BiomeProvider<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/lang/String;)Lcrate::generator::BiomeProvider;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(world_name.into())?));
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(id.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultBiomeProvider",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::generator::BiomeProvider::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_naggable(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isNaggable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_naggable(&self,can_nag: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(can_nag.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setNaggable",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn logger(&self) 
-> Result<blackboxmc_java::util::logging::JavaLogger<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::logging::Logger;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLogger",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::logging::JavaLogger::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[doc(hidden)]
	pub fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn get_plugin(jni: &blackboxmc_general::SharedJNIEnv<'mc>,clazz: jni::objects::JClass<'mc>) 
-> Result<crate::plugin::java::JavaPlugin<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Class;)Lcrate::plugin::java::Plugin;");
let val_1 = jni::objects::JValueGen::Object(clazz.into());
let cls = jni.find_class("org/bukkit/plugin/java/Plugin"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getPlugin",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::plugin::java::JavaPlugin::from_raw(&jni,obj
)}
	pub fn get_providing_plugin(jni: &blackboxmc_general::SharedJNIEnv<'mc>,clazz: jni::objects::JClass<'mc>) 
-> Result<crate::plugin::java::JavaPlugin<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Class;)Lcrate::plugin::java::Plugin;");
let val_1 = jni::objects::JValueGen::Object(clazz.into());
let cls = jni.find_class("org/bukkit/plugin/java/Plugin"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getProvidingPlugin",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::plugin::java::JavaPlugin::from_raw(&jni,obj
)}
// SUPER CLASS: PluginBase

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for JavaPlugin<'mc> {
            fn to_string(&self) -> String {
                match &self.internal_to_string() {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling JavaPlugin.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::plugin::PluginBase<'mc>> for JavaPlugin<'mc>{

fn into(self) -> crate::plugin::PluginBase<'mc> {

crate::plugin::PluginBase::from_raw(&self.jni_ref(), self.1).expect("Error converting JavaPlugin into crate::plugin::PluginBase")

   }
}
#[repr(C)]
pub struct JavaPluginLoader<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for JavaPluginLoader<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for JavaPluginLoader<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate JavaPluginLoader from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/java/JavaPluginLoader")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a JavaPluginLoader object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> JavaPluginLoader<'mc> {
#[deprecated]

	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,instance: impl Into<crate::Server<'mc>>) 
-> Result<crate::plugin::java::JavaPluginLoader<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Server;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(instance.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/plugin/java/PluginLoader"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::java::JavaPluginLoader::from_raw(&jni,res
)}
	pub fn plugin_file_filters(&self) 
-> Result<blackboxmc_java::util::regex::JavaPattern<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::regex::Pattern;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPluginFileFilters",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::regex::JavaPattern::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn create_registered_listeners(&self,listener: impl Into<crate::event::Listener<'mc>>,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Listener;Lorg/bukkit/plugin/Plugin;)Lblackboxmc_java::util::Map;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(listener.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"createRegisteredListeners",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn enable_plugin(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"enablePlugin",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn disable_plugin(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"disablePlugin",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::plugin::PluginLoader<'mc>> for JavaPluginLoader<'mc>{

fn into(self) -> crate::plugin::PluginLoader<'mc> {

crate::plugin::PluginLoader::from_raw(&self.jni_ref(), self.1).expect("Error converting JavaPluginLoader into crate::plugin::PluginLoader")

   }
}
