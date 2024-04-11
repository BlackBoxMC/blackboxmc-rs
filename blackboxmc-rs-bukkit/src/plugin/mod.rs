#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct InvalidDescriptionException<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for InvalidDescriptionException<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for InvalidDescriptionException<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate InvalidDescriptionException from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/InvalidDescriptionException")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a InvalidDescriptionException object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> InvalidDescriptionException<'mc> {
	pub fn new_with_message(jni: &blackboxmc_general::SharedJNIEnv<'mc>,message: std::option::Option<impl Into<String>>) 
-> Result<crate::plugin::InvalidDescriptionException<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = message {
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(a.into())?));
args.push(val_1);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/plugin/InvalidDescriptionException"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::InvalidDescriptionException::from_raw(&jni,res
)}
// SUPER CLASS: Exception

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct TimedRegisteredListener<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for TimedRegisteredListener<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for TimedRegisteredListener<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TimedRegisteredListener from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/TimedRegisteredListener")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TimedRegisteredListener object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TimedRegisteredListener<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,plugin_listener: impl Into<crate::event::Listener<'mc>>,event_executor: impl Into<crate::plugin::EventExecutor<'mc>>,event_priority: impl Into<crate::event::EventPriority<'mc>>,registered_plugin: impl Into<crate::plugin::Plugin<'mc>>,listen_cancelled: bool) 
-> Result<crate::plugin::TimedRegisteredListener<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Listener;Lorg/bukkit/plugin/EventExecutor;Lorg/bukkit/event/EventPriority;Lorg/bukkit/plugin/Plugin;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin_listener.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(event_executor.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(event_priority.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(registered_plugin.into().jni_object().clone())});
let val_5 = jni::objects::JValueGen::Bool(listen_cancelled.into());
let cls = jni.find_class("org/bukkit/plugin/TimedRegisteredListener"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::TimedRegisteredListener::from_raw(&jni,res
)}
	pub fn call_event(&self,event: impl Into<crate::event::Event<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Event;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(event.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"callEvent",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn reset(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"reset",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn count(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCount",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn total_time(&self) 
-> Result<i64, Box<dyn std::error::Error>>

{let sig = String::from("()Li64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTotalTime",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.j()?
)}
	pub fn event_class(&self) 
-> Result<Option<jni::objects::JClass<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JClass;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEventClass",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)
)}
	pub fn has_multiple(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"hasMultiple",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
// SUPER CLASS: RegisteredListener

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::plugin::RegisteredListener<'mc>> for TimedRegisteredListener<'mc>{

fn into(self) -> crate::plugin::RegisteredListener<'mc> {

crate::plugin::RegisteredListener::from_raw(&self.jni_ref(), self.1).expect("Error converting TimedRegisteredListener into crate::plugin::RegisteredListener")

   }
}
#[repr(C)]
pub struct IllegalPluginAccessException<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for IllegalPluginAccessException<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for IllegalPluginAccessException<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate IllegalPluginAccessException from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/IllegalPluginAccessException")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a IllegalPluginAccessException object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> IllegalPluginAccessException<'mc> {
	pub fn new_with_msg(jni: &blackboxmc_general::SharedJNIEnv<'mc>,msg: std::option::Option<impl Into<String>>) 
-> Result<crate::plugin::IllegalPluginAccessException<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = msg {
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(a.into())?));
args.push(val_1);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/plugin/IllegalPluginAccessException"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::IllegalPluginAccessException::from_raw(&jni,res
)}
// SUPER CLASS: RuntimeException

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct AuthorNagException<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for AuthorNagException<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for AuthorNagException<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate AuthorNagException from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/AuthorNagException")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a AuthorNagException object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> AuthorNagException<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,message: impl Into<String>) 
-> Result<crate::plugin::AuthorNagException<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(message.into())?));
let cls = jni.find_class("org/bukkit/plugin/AuthorNagException"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::AuthorNagException::from_raw(&jni,res
)}
	pub fn message(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMessage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
// SUPER CLASS: RuntimeException

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct EventExecutor<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for EventExecutor<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for EventExecutor<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EventExecutor from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/EventExecutor")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EventExecutor object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> EventExecutor<'mc> {
	pub fn execute(&self,listener: impl Into<crate::event::Listener<'mc>>,event: impl Into<crate::event::Event<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Listener;Lorg/bukkit/event/Event;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(listener.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(event.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"execute",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PluginLogger<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginLogger<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginLogger<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginLogger from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginLogger")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginLogger object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginLogger<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,context: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<crate::plugin::PluginLogger<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(context.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/plugin/PluginLogger"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::PluginLogger::from_raw(&jni,res
)}
	pub fn log(&self,log_record: impl Into<blackboxmc_java::util::logging::JavaLogRecord<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/logging/LogRecord;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(log_record.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"log",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
// SUPER CLASS: Logger

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<blackboxmc_java::util::logging::JavaLogger<'mc>> for PluginLogger<'mc>{

fn into(self) -> blackboxmc_java::util::logging::JavaLogger<'mc> {

blackboxmc_java::util::logging::JavaLogger::from_raw(&self.jni_ref(), self.1).expect("Error converting PluginLogger into blackboxmc_java::util::logging::JavaLogger")

   }
}
#[repr(C)]
pub struct UnknownDependencyException<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for UnknownDependencyException<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for UnknownDependencyException<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate UnknownDependencyException from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/UnknownDependencyException")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a UnknownDependencyException object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> UnknownDependencyException<'mc> {
	pub fn new_with_message(jni: &blackboxmc_general::SharedJNIEnv<'mc>,message: std::option::Option<impl Into<String>>) 
-> Result<crate::plugin::UnknownDependencyException<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = message {
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(a.into())?));
args.push(val_1);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/plugin/UnknownDependencyException"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::UnknownDependencyException::from_raw(&jni,res
)}
// SUPER CLASS: RuntimeException

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct ServicesManager<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ServicesManager<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ServicesManager<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ServicesManager from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/ServicesManager")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ServicesManager object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ServicesManager<'mc> {
	pub fn register(&self,service: jni::objects::JClass<'mc>,provider: jni::objects::JObject<'mc>,plugin: impl Into<crate::plugin::Plugin<'mc>>,priority: impl Into<crate::plugin::ServicePriority<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Class;Ljava/lang/Object;Lorg/bukkit/plugin/Plugin;Lorg/bukkit/plugin/ServicePriority;)L();");
let val_1 = jni::objects::JValueGen::Object(service.into());
let val_2 = jni::objects::JValueGen::Object(provider);
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(priority.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"register",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn unregister_all(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"unregisterAll",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn unregister_with_service(&self,service: jni::objects::JClass<'mc>,provider: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/Class;";
let val_1 = jni::objects::JValueGen::Object(service.into());
args.push(val_1);
if let Some(a) = provider {
sig += "Ljava/lang/Object;";
let val_2 = jni::objects::JValueGen::Object(a);
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"unregister",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn load(&self,service: jni::objects::JClass<'mc>) 
-> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Class;)Ljni::objects::JObject;");
let val_1 = jni::objects::JValueGen::Object(service.into());
let res = self.jni_ref().call_method(&self.jni_object(),"load",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
res.l()?
)
)}
	pub fn get_registration(&self,service: jni::objects::JClass<'mc>) 
-> Result<Option<crate::plugin::RegisteredServiceProvider<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Class;)Lcrate::plugin::RegisteredServiceProvider;");
let val_1 = jni::objects::JValueGen::Object(service.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getRegistration",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::plugin::RegisteredServiceProvider::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn get_registrations_with_service(&self,service: jni::objects::JClass<'mc>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/Class;";
let val_1 = jni::objects::JValueGen::Object(service.into());
args.push(val_1);
sig += ")Ljava/util/Collection;";
let res = self.jni_ref().call_method(&self.jni_object(),"getRegistrations",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn known_services(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getKnownServices",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn is_provided_for(&self,service: jni::objects::JClass<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Class;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(service.into());
let res = self.jni_ref().call_method(&self.jni_object(),"isProvidedFor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub enum PluginLoadOrder<'mc> {
	Startup {inner: PluginLoadOrderStruct<'mc>},
	Postworld {inner: PluginLoadOrderStruct<'mc>},
}
impl<'mc> std::fmt::Display for PluginLoadOrder<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PluginLoadOrder::Startup { .. } => f.write_str("STARTUP"),
           PluginLoadOrder::Postworld { .. } => f.write_str("POSTWORLD"),
       }
   }
}

        impl<'mc> PluginLoadOrder<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PluginLoadOrder<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/plugin/PluginLoadOrder");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/plugin/PluginLoadOrder;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"STARTUP" => Ok(PluginLoadOrder::Startup { inner: PluginLoadOrderStruct::from_raw(env,obj)?}),
"POSTWORLD" => Ok(PluginLoadOrder::Postworld { inner: PluginLoadOrderStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PluginLoadOrderStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginLoadOrder<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Startup { inner } => inner.0.clone(),
Self::Postworld { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Startup { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Postworld { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginLoadOrder<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginLoadOrder from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginLoadOrder")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginLoadOrder object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "STARTUP" => Ok(PluginLoadOrder::Startup { inner: PluginLoadOrderStruct::from_raw(env,obj)?}),"POSTWORLD" => Ok(PluginLoadOrder::Postworld { inner: PluginLoadOrderStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PluginLoadOrderStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginLoadOrderStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginLoadOrderStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginLoadOrder")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginLoadOrderStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginLoadOrderStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::plugin::PluginLoadOrder<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::plugin::PluginLoadOrder;");
let cls = jni.find_class("org/bukkit/plugin/PluginLoadOrder"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::plugin::PluginLoadOrder::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct Plugin<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Plugin<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Plugin<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Plugin from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/Plugin")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Plugin object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> Plugin<'mc> {
/// Return one of the extendable classes that BlackBox supports, based on the value given.
///
/// ## Safety
/// - It returns a Java Object that you must then cast into the proper object via JNI. You are responsible for the checks yourself.
/// - This function is specific to the BlackboxPlugin class supplied within the plugin, and will error out if you pass a regular JavaPlugin.
pub unsafe fn new_extendable(
    &self,
    address: i32,
    class_name: impl Into<String>
        + std::convert::AsRef<str>
        + std::convert::AsRef<str>
        + std::convert::AsRef<str>,
    name: impl Into<String> + std::convert::AsRef<str> + std::convert::AsRef<str>,
    lib_name: impl Into<String> + std::convert::AsRef<str> + std::convert::AsRef<str>,
) -> Result<jni::objects::JObject, Box<dyn std::error::Error>> {
    let obj = self.jni_ref().call_method(
        &self.1,
        "newExtendable",
        "(ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)Ljava/lang/Object;",
        vec![
            jni::objects::JValueGen::Int(address),
            jni::objects::JValueGen::from(jni::objects::JObject::from(
                self.jni_ref().new_string(class_name).unwrap(),
            )),
            jni::objects::JValueGen::from(jni::objects::JObject::from(
                self.jni_ref().new_string(name).unwrap(),
            )),
            jni::objects::JValueGen::from(jni::objects::JObject::from(
                self.jni_ref().new_string(lib_name).unwrap(),
            )),
            #[cfg(target_arch = "wasm32")]
            jni::objects::JValueGen::Bool(true.into()),
            #[cfg(not(target_arch = "wasm32"))]
            jni::objects::JValueGen::Bool(false.into()),
        ],
    );
    let obj = self.jni_ref().translate_error(obj)?;
    Ok(jni::objects::JObject::from_raw(*obj.l()?))
}
pub fn from_extendable(
    env: &blackboxmc_general::SharedJNIEnv<'mc>,
    plugin: &'mc crate::plugin::Plugin,
    address: i32,
    lib_name: String,
    name: String,
) -> Result<Self, Box<dyn std::error::Error>> {
    let obj = unsafe { plugin.new_extendable(address, "Plugin", name, lib_name) }?;
    Self::from_raw(env, obj)
}
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
	pub fn reload_config(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"reloadConfig",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
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
	pub fn on_disable(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"onDisable",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn on_load(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"onLoad",sig.as_str(),vec![]);
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
	pub fn logger(&self) 
-> Result<blackboxmc_java::util::logging::JavaLogger<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::logging::Logger;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLogger",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::logging::JavaLogger::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn on_tab_complete(&self,sender: impl Into<crate::command::CommandSender<'mc>>,command: impl Into<crate::command::Command<'mc>>,label: impl Into<String>,val_args: impl Into<String>) 
-> Result<Option<Vec<jni::objects::JObject<'mc>>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Lorg/bukkit/command/Command;Ljava/lang/String;Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(command.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(label.into())?));
let val_4 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"onTabComplete",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
Some(
new_vec
)
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::command::TabExecutor<'mc>> for Plugin<'mc>{

fn into(self) -> crate::command::TabExecutor<'mc> {

crate::command::TabExecutor::from_raw(&self.jni_ref(), self.1).expect("Error converting Plugin into crate::command::TabExecutor")

   }
}
#[repr(C)]
pub struct PluginDescriptionFile<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginDescriptionFile<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginDescriptionFile<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginDescriptionFile from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginDescriptionFile")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginDescriptionFile object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginDescriptionFile<'mc> {
	pub fn new_with_plugin_name(jni: &blackboxmc_general::SharedJNIEnv<'mc>,plugin_name: impl Into<String>,plugin_version: std::option::Option<impl Into<String>>,main_class: std::option::Option<impl Into<String>>) 
-> Result<crate::plugin::PluginDescriptionFile<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(plugin_name.into())?));
args.push(val_1);
if let Some(a) = plugin_version {
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(a.into())?));
args.push(val_2);
}
if let Some(a) = main_class {
sig += "Ljava/lang/String;";
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(a.into())?));
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/plugin/PluginDescriptionFile"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::PluginDescriptionFile::from_raw(&jni,res
)}
	pub fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn provides(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getProvides",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn version(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getVersion",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn main(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMain",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn description(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDescription",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
	pub fn load(&self) 
-> Result<crate::plugin::PluginLoadOrder<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::plugin::PluginLoadOrder;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLoad",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::PluginLoadOrder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn authors(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAuthors",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn contributors(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getContributors",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn website(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getWebsite",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
	pub fn depend(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDepend",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn soft_depend(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSoftDepend",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn load_before(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLoadBefore",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn prefix(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPrefix",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
	pub fn commands(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCommands",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn permissions(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPermissions",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn permission_default(&self) 
-> Result<crate::permissions::PermissionDefault<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::permissions::PermissionDefault;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPermissionDefault",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::permissions::PermissionDefault::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn awareness(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAwareness",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn full_name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFullName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn apiversion(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAPIVersion",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
	pub fn libraries(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLibraries",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
#[deprecated]

	pub fn class_loader_of(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getClassLoaderOf",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
	pub fn raw_name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRawName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub enum PluginAwarenessFlags<'mc> {
	Utf8 {inner: PluginAwarenessFlagsStruct<'mc>},
}
impl<'mc> std::fmt::Display for PluginAwarenessFlags<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PluginAwarenessFlags::Utf8 { .. } => f.write_str("UTF8"),
       }
   }
}

        impl<'mc> PluginAwarenessFlags<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PluginAwarenessFlags<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/plugin/PluginAwareness/Flags");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/plugin/PluginAwareness/Flags;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"UTF8" => Ok(PluginAwarenessFlags::Utf8 { inner: PluginAwarenessFlagsStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PluginAwarenessFlagsStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginAwarenessFlags<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Utf8 { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Utf8 { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginAwarenessFlags<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginAwarenessFlags from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginAwareness/Flags")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginAwarenessFlags object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "UTF8" => Ok(PluginAwarenessFlags::Utf8 { inner: PluginAwarenessFlagsStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PluginAwarenessFlagsStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginAwarenessFlagsStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginAwarenessFlagsStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginAwareness/Flags")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginAwarenessFlagsStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginAwarenessFlagsStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::plugin::PluginAwarenessFlags<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::plugin::PluginAwarenessFlags;");
let cls = jni.find_class("org/bukkit/plugin/PluginAwareness/Flags"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::plugin::PluginAwarenessFlags::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PluginLoader<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginLoader<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginLoader<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginLoader from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginLoader")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginLoader object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginLoader<'mc> {
pub fn from_extendable(
    env: &blackboxmc_general::SharedJNIEnv<'mc>,
    plugin: &'mc crate::plugin::Plugin,
    address: i32,
    lib_name: String,
    name: String,
) -> Result<Self, Box<dyn std::error::Error>> {
    let obj = unsafe { plugin.new_extendable(address, "PluginLoader", name, lib_name) }?;
    Self::from_raw(env, obj)
}
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PluginAwareness<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginAwareness<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginAwareness<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginAwareness from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginAwareness")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginAwareness object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginAwareness<'mc> {

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PluginManager<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginManager<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginManager<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginManager from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginManager")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginManager object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginManager<'mc> {
	pub fn register_interface(&self,loader: jni::objects::JClass<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Class;)L();");
let val_1 = jni::objects::JValueGen::Object(loader.into());
let res = self.jni_ref().call_method(&self.jni_object(),"registerInterface",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn get_plugin(&self,name: impl Into<String>) 
-> Result<Option<crate::plugin::Plugin<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lcrate::plugin::Plugin;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getPlugin",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn plugins(&self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::plugin::Plugin;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlugins",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_plugin_enabled_with_plugin(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/plugin/Plugin;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_1);
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"isPluginEnabled",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn disable_plugins(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"disablePlugins",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn clear_plugins(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"clearPlugins",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn call_event(&self,event: impl Into<crate::event::Event<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Event;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(event.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"callEvent",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn register_events(&self,listener: impl Into<crate::event::Listener<'mc>>,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Listener;Lorg/bukkit/plugin/Plugin;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(listener.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"registerEvents",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn register_event_with_event(&self,event: jni::objects::JClass<'mc>,listener: impl Into<crate::event::Listener<'mc>>,priority: impl Into<crate::event::EventPriority<'mc>>,executor: impl Into<crate::plugin::EventExecutor<'mc>>,plugin: impl Into<crate::plugin::Plugin<'mc>>,ignore_cancelled: std::option::Option<bool>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/Class;";
let val_1 = jni::objects::JValueGen::Object(event.into());
args.push(val_1);
sig += "Lorg/bukkit/event/Listener;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(listener.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/event/EventPriority;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(priority.into().jni_object().clone())});
args.push(val_3);
sig += "Lorg/bukkit/plugin/EventExecutor;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(executor.into().jni_object().clone())});
args.push(val_4);
sig += "Lorg/bukkit/plugin/Plugin;";
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_5);
if let Some(a) = ignore_cancelled {
sig += "Z";
let val_6 = jni::objects::JValueGen::Bool(a.into());
args.push(val_6);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"registerEvent",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
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
	pub fn get_permission(&self,name: impl Into<String>) 
-> Result<Option<crate::permissions::Permission<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lcrate::permissions::Permission;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getPermission",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::permissions::Permission::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn add_permission(&self,perm: impl Into<crate::permissions::Permission<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/permissions/Permission;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(perm.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addPermission",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn remove_permission_with_name(&self,name: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
args.push(val_1);
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"removePermission",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn get_default_permissions(&self,op: bool) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Z)Lblackboxmc_java::util::Set;");
let val_1 = jni::objects::JValueGen::Bool(op.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultPermissions",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn recalculate_permission_defaults(&self,perm: impl Into<crate::permissions::Permission<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/permissions/Permission;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(perm.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"recalculatePermissionDefaults",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn subscribe_to_permission(&self,permission: impl Into<String>,permissible: impl Into<crate::permissions::Permissible<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Lorg/bukkit/permissions/Permissible;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(permission.into())?));
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(permissible.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"subscribeToPermission",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn unsubscribe_from_permission(&self,permission: impl Into<String>,permissible: impl Into<crate::permissions::Permissible<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Lorg/bukkit/permissions/Permissible;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(permission.into())?));
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(permissible.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"unsubscribeFromPermission",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn get_permission_subscriptions(&self,permission: impl Into<String>) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lblackboxmc_java::util::Set;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(permission.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getPermissionSubscriptions",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn subscribe_to_default_perms(&self,op: bool,permissible: impl Into<crate::permissions::Permissible<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(ZLorg/bukkit/permissions/Permissible;)L();");
let val_1 = jni::objects::JValueGen::Bool(op.into());
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(permissible.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"subscribeToDefaultPerms",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn unsubscribe_from_default_perms(&self,op: bool,permissible: impl Into<crate::permissions::Permissible<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(ZLorg/bukkit/permissions/Permissible;)L();");
let val_1 = jni::objects::JValueGen::Bool(op.into());
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(permissible.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"unsubscribeFromDefaultPerms",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn get_default_perm_subscriptions(&self,op: bool) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Z)Lblackboxmc_java::util::Set;");
let val_1 = jni::objects::JValueGen::Bool(op.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultPermSubscriptions",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn permissions(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPermissions",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn use_timings(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"useTimings",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PluginBase<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginBase<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginBase<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginBase from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginBase")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginBase object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginBase<'mc> {
pub fn from_extendable(
    env: &blackboxmc_general::SharedJNIEnv<'mc>,
    plugin: &'mc crate::plugin::Plugin,
    address: i32,
    lib_name: String,
    name: String,
) -> Result<Self, Box<dyn std::error::Error>> {
    let obj = unsafe { plugin.new_extendable(address, "PluginBase", name, lib_name) }?;
    Self::from_raw(env, obj)
}
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::plugin::PluginBase<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/plugin/PluginBase"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::PluginBase::from_raw(&jni,res
)}
	pub fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
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
	pub fn reload_config(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"reloadConfig",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
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
	pub fn on_disable(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"onDisable",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn on_load(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"onLoad",sig.as_str(),vec![]);
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
	pub fn logger(&self) 
-> Result<blackboxmc_java::util::logging::JavaLogger<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::logging::Logger;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLogger",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::logging::JavaLogger::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn on_tab_complete(&self,sender: impl Into<crate::command::CommandSender<'mc>>,command: impl Into<crate::command::Command<'mc>>,label: impl Into<String>,val_args: impl Into<String>) 
-> Result<Option<Vec<jni::objects::JObject<'mc>>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Lorg/bukkit/command/Command;Ljava/lang/String;Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(command.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(label.into())?));
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
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::plugin::Plugin<'mc>> for PluginBase<'mc>{

fn into(self) -> crate::plugin::Plugin<'mc> {

crate::plugin::Plugin::from_raw(&self.jni_ref(), self.1).expect("Error converting PluginBase into crate::plugin::Plugin")

   }
}
pub enum ServicePriority<'mc> {
	Lowest {inner: ServicePriorityStruct<'mc>},
	Low {inner: ServicePriorityStruct<'mc>},
	Normal {inner: ServicePriorityStruct<'mc>},
	High {inner: ServicePriorityStruct<'mc>},
	Highest {inner: ServicePriorityStruct<'mc>},
}
impl<'mc> std::fmt::Display for ServicePriority<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           ServicePriority::Lowest { .. } => f.write_str("Lowest"),
           ServicePriority::Low { .. } => f.write_str("Low"),
           ServicePriority::Normal { .. } => f.write_str("Normal"),
           ServicePriority::High { .. } => f.write_str("High"),
           ServicePriority::Highest { .. } => f.write_str("Highest"),
       }
   }
}

        impl<'mc> ServicePriority<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<ServicePriority<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/plugin/ServicePriority");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/plugin/ServicePriority;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"Lowest" => Ok(ServicePriority::Lowest { inner: ServicePriorityStruct::from_raw(env,obj)?}),
"Low" => Ok(ServicePriority::Low { inner: ServicePriorityStruct::from_raw(env,obj)?}),
"Normal" => Ok(ServicePriority::Normal { inner: ServicePriorityStruct::from_raw(env,obj)?}),
"High" => Ok(ServicePriority::High { inner: ServicePriorityStruct::from_raw(env,obj)?}),
"Highest" => Ok(ServicePriority::Highest { inner: ServicePriorityStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct ServicePriorityStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ServicePriority<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Lowest { inner } => inner.0.clone(),
Self::Low { inner } => inner.0.clone(),
Self::Normal { inner } => inner.0.clone(),
Self::High { inner } => inner.0.clone(),
Self::Highest { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Lowest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Low { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Normal { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::High { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Highest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for ServicePriority<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ServicePriority from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/ServicePriority")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ServicePriority object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "Lowest" => Ok(ServicePriority::Lowest { inner: ServicePriorityStruct::from_raw(env,obj)?}),"Low" => Ok(ServicePriority::Low { inner: ServicePriorityStruct::from_raw(env,obj)?}),"Normal" => Ok(ServicePriority::Normal { inner: ServicePriorityStruct::from_raw(env,obj)?}),"High" => Ok(ServicePriority::High { inner: ServicePriorityStruct::from_raw(env,obj)?}),"Highest" => Ok(ServicePriority::Highest { inner: ServicePriorityStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for ServicePriorityStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ServicePriorityStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ServicePriorityStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/ServicePriority")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ServicePriorityStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ServicePriorityStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::plugin::ServicePriority<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::plugin::ServicePriority;");
let cls = jni.find_class("org/bukkit/plugin/ServicePriority"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::plugin::ServicePriority::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct InvalidPluginException<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for InvalidPluginException<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for InvalidPluginException<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate InvalidPluginException from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/InvalidPluginException")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a InvalidPluginException object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> InvalidPluginException<'mc> {
// SUPER CLASS: Exception

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct RegisteredListener<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RegisteredListener<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RegisteredListener<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RegisteredListener from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/RegisteredListener")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RegisteredListener object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RegisteredListener<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,listener: impl Into<crate::event::Listener<'mc>>,executor: impl Into<crate::plugin::EventExecutor<'mc>>,priority: impl Into<crate::event::EventPriority<'mc>>,plugin: impl Into<crate::plugin::Plugin<'mc>>,ignore_cancelled: bool) 
-> Result<crate::plugin::RegisteredListener<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Listener;Lorg/bukkit/plugin/EventExecutor;Lorg/bukkit/event/EventPriority;Lorg/bukkit/plugin/Plugin;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(listener.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(executor.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(priority.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_5 = jni::objects::JValueGen::Bool(ignore_cancelled.into());
let cls = jni.find_class("org/bukkit/plugin/RegisteredListener"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::RegisteredListener::from_raw(&jni,res
)}
	pub fn listener(&self) 
-> Result<crate::event::Listener<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::Listener;");
let res = self.jni_ref().call_method(&self.jni_object(),"getListener",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::Listener::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn plugin(&self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::plugin::Plugin;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlugin",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn priority(&self) 
-> Result<crate::event::EventPriority<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::EventPriority;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPriority",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventPriority::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn call_event(&self,event: impl Into<crate::event::Event<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Event;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(event.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"callEvent",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_ignoring_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isIgnoringCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct RegisteredServiceProvider<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RegisteredServiceProvider<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RegisteredServiceProvider<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RegisteredServiceProvider from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/RegisteredServiceProvider")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RegisteredServiceProvider object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RegisteredServiceProvider<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,service: jni::objects::JClass<'mc>,provider: jni::objects::JObject<'mc>,priority: impl Into<crate::plugin::ServicePriority<'mc>>,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<crate::plugin::RegisteredServiceProvider<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Class;Ljava/lang/Object;Lorg/bukkit/plugin/ServicePriority;Lorg/bukkit/plugin/Plugin;)V");
let val_1 = jni::objects::JValueGen::Object(service.into());
let val_2 = jni::objects::JValueGen::Object(provider);
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(priority.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/plugin/RegisteredServiceProvider"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::RegisteredServiceProvider::from_raw(&jni,res
)}
	pub fn service(&self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JClass;");
let res = self.jni_ref().call_method(&self.jni_object(),"getService",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
	pub fn plugin(&self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::plugin::Plugin;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlugin",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn provider(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JObject;");
let res = self.jni_ref().call_method(&self.jni_object(),"getProvider",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
	pub fn priority(&self) 
-> Result<crate::plugin::ServicePriority<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::plugin::ServicePriority;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPriority",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::ServicePriority::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn compare_to(&self,other: impl Into<crate::plugin::RegisteredServiceProvider<'mc>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/RegisteredServiceProvider;)Li32;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"compareTo",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub mod java;
pub mod messaging;
