#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/plugin/mod.rs*/

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
    
impl<'mc> InvalidDescriptionExceptionTrait<'mc> for InvalidDescriptionException<'mc> {}
pub trait InvalidDescriptionExceptionTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> TimedRegisteredListenerTrait<'mc> for TimedRegisteredListener<'mc> {}
pub trait TimedRegisteredListenerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,plugin_listener: impl Into<crate::event::Listener<'mc>>,event_executor: impl Into<crate::plugin::EventExecutor<'mc>>,event_priority: impl Into<crate::event::EventPriority<'mc>>,registered_plugin: impl Into<crate::plugin::Plugin<'mc>>,listen_cancelled: bool) 
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

	fn call_event(&self,event: impl Into<crate::event::Event<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Event;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(event.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"callEvent",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Resets the call count and total time for this listener
	fn reset(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"reset",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the total times this listener has been called
	fn count(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getCount",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the total time calls to this listener have taken
	fn total_time(&self) 
-> Result<i64, Box<dyn std::error::Error>>

{let sig = String::from("()J");
let res = self.jni_ref().call_method(&self.jni_object(),"getTotalTime",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.j()?
)}
/// Gets the class of the events this listener handled. If it handled
/// multiple classes of event, the closest shared superclass will be
/// returned, such that for any event this listener has handled,
/// <code>this.getEventClass().isAssignableFrom(event.getClass())</code>
/// and no class <code>this.getEventClass().isAssignableFrom(clazz)
/// {@literal && this.getEventClass() != clazz &&}
/// event.getClass().isAssignableFrom(clazz)</code> for all handled events.
	fn event_class(&self) 
-> Result<Option<jni::objects::JClass<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/Class;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEventClass",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)
)}
/// Gets whether this listener has handled multiple events, such that for
/// some two events, <code>eventA.getClass() != eventB.getClass()</code>.
	fn has_multiple(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasMultiple",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::plugin::RegisteredListener<'mc>> for TimedRegisteredListener<'mc>{

fn into(self) -> crate::plugin::RegisteredListener<'mc> {

crate::plugin::RegisteredListener::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting TimedRegisteredListener into crate::plugin::RegisteredListener")

   }
}
impl<'mc> crate::plugin::RegisteredListenerTrait<'mc> for TimedRegisteredListener<'mc> {}
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
    
impl<'mc> IllegalPluginAccessExceptionTrait<'mc> for IllegalPluginAccessException<'mc> {}
pub trait IllegalPluginAccessExceptionTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Constructs an instance of <code>IllegalPluginAccessException</code>
/// with the specified detail message.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,msg: std::option::Option<impl Into<String>>) 
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

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> AuthorNagExceptionTrait<'mc> for AuthorNagException<'mc> {}
pub trait AuthorNagExceptionTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Constructs a new AuthorNagException based on the given Exception
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,message: impl Into<String>) 
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

	fn message(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMessage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> EventExecutorTrait<'mc> for EventExecutor<'mc> {}
pub trait EventExecutorTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn execute(&self,listener: impl Into<crate::event::Listener<'mc>>,event: impl Into<crate::event::Event<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Listener;Lorg/bukkit/event/Event;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(listener.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(event.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"execute",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> PluginLoggerTrait<'mc> for PluginLogger<'mc> {}
pub trait PluginLoggerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Creates a new PluginLogger that extracts the name from a plugin.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,context: impl Into<crate::plugin::Plugin<'mc>>) 
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

	fn log(&self,log_record: impl Into<blackboxmc_java::util::logging::JavaLogRecord<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/logging/LogRecord;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(log_record.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"log",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<blackboxmc_java::util::logging::JavaLogger<'mc>> for PluginLogger<'mc>{

fn into(self) -> blackboxmc_java::util::logging::JavaLogger<'mc> {

blackboxmc_java::util::logging::JavaLogger::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PluginLogger into blackboxmc_java::util::logging::JavaLogger")

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
    
impl<'mc> UnknownDependencyExceptionTrait<'mc> for UnknownDependencyException<'mc> {}
pub trait UnknownDependencyExceptionTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> ServicesManagerTrait<'mc> for ServicesManager<'mc> {}
pub trait ServicesManagerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Register a provider of a service.
	fn register(&self,service: jni::objects::JClass<'mc>,provider: jni::objects::JObject<'mc>,plugin: impl Into<crate::plugin::Plugin<'mc>>,priority: impl Into<crate::plugin::ServicePriority<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Class;LT;Lorg/bukkit/plugin/Plugin;Lorg/bukkit/plugin/ServicePriority;)V");
let val_1 = jni::objects::JValueGen::Object(service.into());
let val_2 = jni::objects::JValueGen::Object(provider);
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(priority.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"register",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Unregister all the providers registered by a particular plugin.
	fn unregister_all(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"unregisterAll",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Unregister a particular provider for a particular service.
	fn unregister(&self,service: jni::objects::JClass<'mc>,provider: std::option::Option<jni::objects::JObject<'mc>>) 
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
/// Queries for a provider. This may return null if no provider has been
/// registered for a service. The highest priority provider is returned.
	fn load(&self,service: jni::objects::JClass<'mc>) 
-> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Class;)LT;");
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
/// Get a list of known services. A service is known if it has registered
/// providers for it.
	fn known_services(&self) 
-> Result<Vec<jni::objects::JClass<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Collection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getKnownServices",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(unsafe {jni::objects::JClass::from_raw(*obj)})
};
Ok(
new_vec
)}
/// Returns whether a provider has been registered for a service. Do not
/// check this first only to call <code>load(service)</code> later, as that
/// would be a non-thread safe situation.
	fn is_provided_for(&self,service: jni::objects::JClass<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Class;)Z");
let val_1 = jni::objects::JValueGen::Object(service.into());
let res = self.jni_ref().call_method(&self.jni_object(),"isProvidedFor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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

        impl<'mc> PluginLoadOrderTrait<'mc> for PluginLoadOrder<'mc> {}
        
        pub trait PluginLoadOrderTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
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

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::plugin::PluginLoadOrder<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/plugin/PluginLoadOrder;");
let cls = jni.find_class("org/bukkit/plugin/PluginLoadOrder"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::plugin::PluginLoadOrder::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> PluginTrait<'mc> for Plugin<'mc> {}
pub trait PluginTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns the plugin.yaml file containing the details for this plugin
	fn description(&self) 
-> Result<crate::plugin::PluginDescriptionFile<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/plugin/PluginDescriptionFile;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDescription",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::PluginDescriptionFile::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets a {@link FileConfiguration} for this plugin, read through
/// "config.yml"
/// 
/// If there is a default config.yml embedded in this plugin, it will be
/// provided as a default for this Configuration.
	fn config(&self) 
-> Result<crate::configuration::file::FileConfiguration<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/configuration/file/FileConfiguration;");
let res = self.jni_ref().call_method(&self.jni_object(),"getConfig",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::configuration::file::FileConfiguration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Saves the {@link FileConfiguration} retrievable by {@link #getConfig()}.
	fn save_config(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"saveConfig",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Saves the raw contents of the default config.yml file to the location
/// retrievable by {@link #getConfig()}.
/// 
/// This should fail silently if the config.yml already exists.
	fn save_default_config(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"saveDefaultConfig",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Saves the raw contents of any resource embedded with a plugin's .jar
/// file assuming it can be found using {@link #getResource(String)}.
/// 
/// The resource is saved into the plugin's data folder using the same
/// hierarchy as the .jar file (subdirectories are preserved).
	fn save_resource(&self,resource_path: impl Into<String>,replace: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Z)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(resource_path.into())?));
let val_2 = jni::objects::JValueGen::Bool(replace.into());
let res = self.jni_ref().call_method(&self.jni_object(),"saveResource",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Discards any data in {@link #getConfig()} and reloads from disk.
	fn reload_config(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"reloadConfig",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the associated PluginLoader responsible for this plugin
	fn plugin_loader(&self) 
-> Result<crate::plugin::PluginLoader<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/plugin/PluginLoader;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPluginLoader",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::PluginLoader::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns the Server instance currently running this plugin
	fn server(&self) 
-> Result<crate::Server<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Server;");
let res = self.jni_ref().call_method(&self.jni_object(),"getServer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Server::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns a value indicating whether or not this plugin is currently
/// enabled
	fn is_enabled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isEnabled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Called when this plugin is disabled
	fn on_disable(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"onDisable",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Called after a plugin is loaded but before it has been enabled.
/// 
/// When multiple plugins are loaded, the onLoad() for all plugins is
/// called before any onEnable() is called.
	fn on_load(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"onLoad",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Called when this plugin is enabled
	fn on_enable(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"onEnable",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Simple boolean if we can still nag to the logs about things
	fn is_naggable(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isNaggable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set naggable state
	fn set_naggable(&self,can_nag: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(can_nag.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setNaggable",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets a {@link ChunkGenerator} for use in a default world, as specified
/// in the server configuration
	fn get_default_world_generator(&self,world_name: impl Into<String>,id: impl Into<String>) 
-> Result<Option<crate::generator::ChunkGenerator<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/lang/String;)Lorg/bukkit/generator/ChunkGenerator;");
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
/// Gets a {@link BiomeProvider} for use in a default world, as specified
/// in the server configuration
	fn get_default_biome_provider(&self,world_name: impl Into<String>,id: impl Into<String>) 
-> Result<Option<crate::generator::BiomeProvider<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/lang/String;)Lorg/bukkit/generator/BiomeProvider;");
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
/// Returns the plugin logger associated with this server's logger. The
/// returned logger automatically tags all log messages with the plugin's
/// name.
	fn logger(&self) 
-> Result<blackboxmc_java::util::logging::JavaLogger<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/logging/Logger;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLogger",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::logging::JavaLogger::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns the name of the plugin.
/// 
/// This should return the bare name of the plugin and should be used for
/// comparison.
	fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::command::TabExecutor<'mc>> for Plugin<'mc>{

fn into(self) -> crate::command::TabExecutor<'mc> {

crate::command::TabExecutor::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Plugin into crate::command::TabExecutor")

   }
}
impl<'mc> crate::command::TabExecutorTrait<'mc> for Plugin<'mc> {}
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
    
impl<'mc> PluginDescriptionFileTrait<'mc> for PluginDescriptionFile<'mc> {}
pub trait PluginDescriptionFileTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Creates a new PluginDescriptionFile with the given detailed
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,plugin_name: impl Into<String>,plugin_version: std::option::Option<impl Into<String>>,main_class: std::option::Option<impl Into<String>>) 
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
/// Gives the name of the plugin. This name is a unique identifier for
/// plugins.
/// <ul>
/// <li>Must consist of all alphanumeric characters, underscores, hyphon,
/// and period (a-z,A-Z,0-9, _.-). Any other character will cause the
/// plugin.yml to fail loading.
/// <li>Used to determine the name of the plugin's data folder. Data
/// folders are placed in the ./plugins/ directory by default, but this
/// behavior should not be relied on. {@link Plugin#getDataFolder()}
/// should be used to reference the data folder.
/// <li>It is good practice to name your jar the same as this, for example
/// 'MyPlugin.jar'.
/// <li>Case sensitive.
/// <li>The is the token referenced in {@link #getDepend()}, {@link
/// #getSoftDepend()}, and {@link #getLoadBefore()}.
/// <li>Using spaces in the plugin's name is deprecated.
/// </ul>
/// 
/// In the plugin.yml, this entry is named <code>name</code>.
/// 
/// Example:<blockquote><pre>name: MyPlugin</pre></blockquote>
	fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gives the list of other plugin APIs which this plugin provides.
/// These are usable for other plugins to depend on.
/// <ul>
/// <li>Must consist of all alphanumeric characters, underscores, hyphon,
/// and period (a-z,A-Z,0-9, _.-). Any other character will cause the
/// plugin.yml to fail loading.
/// <li>A different plugin providing the same one or using it as their name
/// will not result in the plugin to fail loading.
/// <li>Case sensitive.
/// <li>An entry of this list can be referenced in {@link #getDepend()},
/// {@link #getSoftDepend()}, and {@link #getLoadBefore()}.
/// <li><code>provides</code> must be in <a
/// href="https://en.wikipedia.org/wiki/YAML#Lists">YAML list
/// format</a>.
/// </ul>
/// 
/// In the plugin.yml, this entry is named <code>provides</code>.
/// 
/// Example:
/// <blockquote><pre>provides:
/// - OtherPluginName
/// - OldPluginName</pre></blockquote>
	fn provides(&self) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getProvides",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
new_vec
)}
/// Gives the version of the plugin.
/// <ul>
/// <li>Version is an arbitrary string, however the most common format is
/// MajorRelease.MinorRelease.Build (eg: 1.4.1).
/// <li>Typically you will increment this every time you release a new
/// feature or bug fix.
/// <li>Displayed when a user types <code>/version PluginName</code>
/// </ul>
/// 
/// In the plugin.yml, this entry is named <code>version</code>.
/// 
/// Example:<blockquote><pre>version: 1.4.1</pre></blockquote>
	fn version(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getVersion",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gives the fully qualified name of the main class for a plugin. The
/// format should follow the {@link ClassLoader#loadClass(String)} syntax
/// to successfully be resolved at runtime. For most plugins, this is the
/// class that extends {@link JavaPlugin}.
/// <ul>
/// <li>This must contain the full namespace including the class file
/// itself.
/// <li>If your namespace is <code>org.bukkit.plugin</code>, and your class
/// file is called <code>MyPlugin</code> then this must be
/// <code>org.bukkit.plugin.MyPlugin</code>
/// <li>No plugin can use <code>org.bukkit.</code> as a base package for
/// <b>any class</b>, including the main class.
/// </ul>
/// 
/// In the plugin.yml, this entry is named <code>main</code>.
/// 
/// Example:
/// <blockquote><pre>main: org.bukkit.plugin.MyPlugin</pre></blockquote>
	fn main(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMain",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gives a human-friendly description of the functionality the plugin
/// provides.
/// <ul>
/// <li>The description can have multiple lines.
/// <li>Displayed when a user types <code>/version PluginName</code>
/// </ul>
/// 
/// In the plugin.yml, this entry is named <code>description</code>.
/// 
/// Example:
/// <blockquote><pre>description: This plugin is so 31337. You can set yourself on fire.</pre></blockquote>
	fn description(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDescription",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Gives the phase of server startup that the plugin should be loaded.
/// <ul>
/// <li>Possible values are in {@link PluginLoadOrder}.
/// <li>Defaults to {@link PluginLoadOrder#POSTWORLD}.
/// <li>Certain caveats apply to each phase.
/// <li>When different, {@link #getDepend()}, {@link #getSoftDepend()}, and
/// {@link #getLoadBefore()} become relative in order loaded per-phase.
/// If a plugin loads at <code>STARTUP</code>, but a dependency loads
/// at <code>POSTWORLD</code>, the dependency will not be loaded before
/// the plugin is loaded.
/// </ul>
/// 
/// In the plugin.yml, this entry is named <code>load</code>.
/// 
/// Example:<blockquote><pre>load: STARTUP</pre></blockquote>
	fn load(&self) 
-> Result<crate::plugin::PluginLoadOrder<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/plugin/PluginLoadOrder;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLoad",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::PluginLoadOrder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gives the list of authors for the plugin.
/// <ul>
/// <li>Gives credit to the developer.
/// <li>Used in some server error messages to provide helpful feedback on
/// who to contact when an error occurs.
/// <li>A SpigotMC forum handle or email address is recommended.
/// <li>Is displayed when a user types <code>/version PluginName</code>
/// <li><code>authors</code> must be in <a
/// href="https://en.wikipedia.org/wiki/YAML#Lists">YAML list
/// format</a>.
/// </ul>
/// 
/// In the plugin.yml, this has two entries, <code>author</code> and
/// <code>authors</code>.
/// 
/// Single author example:
/// <blockquote><pre>author: CaptainInflamo</pre></blockquote>
/// Multiple author example:
/// <blockquote><pre>authors: [Cogito, verrier, EvilSeph]</pre></blockquote>
/// When both are specified, author will be the first entry in the list, so
/// this example:
/// <blockquote><pre>author: Grum
/// authors:
/// - feildmaster
/// - amaranth</pre></blockquote>
/// Is equivilant to this example:
/// <pre>authors: [Grum, feildmaster, aramanth]</pre>
	fn authors(&self) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAuthors",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
new_vec
)}
/// Gives the list of contributors for the plugin.
/// <ul>
/// <li>Gives credit to those that have contributed to the plugin, though
/// not enough so to warrant authorship.
/// <li>Unlike {@link #getAuthors()}, contributors will not be mentioned in
/// server error messages as a means of contact.
/// <li>A SpigotMC forum handle or email address is recommended.
/// <li>Is displayed when a user types <code>/version PluginName</code>
/// <li><code>contributors</code> must be in <a
/// href="https://en.wikipedia.org/wiki/YAML#Lists">YAML list
/// format</a>.
/// </ul>
/// 
/// Example:
/// <blockquote><pre>authors: [Choco, md_5]</pre></blockquote>
	fn contributors(&self) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getContributors",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
new_vec
)}
/// Gives the plugin's or plugin's author's website.
/// <ul>
/// <li>A link to the Curse page that includes documentation and downloads
/// is highly recommended.
/// <li>Displayed when a user types <code>/version PluginName</code>
/// </ul>
/// 
/// In the plugin.yml, this entry is named <code>website</code>.
/// 
/// Example:
/// <blockquote><pre>website: http://www.curse.com/server-mods/minecraft/myplugin</pre></blockquote>
	fn website(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getWebsite",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Gives a list of other plugins that the plugin requires.
/// <ul>
/// <li>Use the value in the {@link #getName()} of the target plugin to
/// specify the dependency.
/// <li>If any plugin listed here is not found, your plugin will fail to
/// load at startup.
/// <li>If multiple plugins list each other in <code>depend</code>,
/// creating a network with no individual plugin does not list another
/// plugin in the <a
/// href=https://en.wikipedia.org/wiki/Circular_dependency>network</a>,
/// all plugins in that network will fail.
/// <li><code>depend</code> must be in <a
/// href="https://en.wikipedia.org/wiki/YAML#Lists">YAML list
/// format</a>.
/// </ul>
/// 
/// In the plugin.yml, this entry is named <code>depend</code>.
/// 
/// Example:
/// <blockquote><pre>depend:
/// - OnePlugin
/// - AnotherPlugin</pre></blockquote>
	fn depend(&self) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDepend",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
new_vec
)}
/// Gives a list of other plugins that the plugin requires for full
/// functionality. The {@link PluginManager} will make best effort to treat
/// all entries here as if they were a {@link #getDepend() dependency}, but
/// will never fail because of one of these entries.
/// <ul>
/// <li>Use the value in the {@link #getName()} of the target plugin to
/// specify the dependency.
/// <li>When an unresolvable plugin is listed, it will be ignored and does
/// not affect load order.
/// <li>When a circular dependency occurs (a network of plugins depending
/// or soft-dependending each other), it will arbitrarily choose a
/// plugin that can be resolved when ignoring soft-dependencies.
/// <li><code>softdepend</code> must be in <a
/// href="https://en.wikipedia.org/wiki/YAML#Lists">YAML list
/// format</a>.
/// </ul>
/// 
/// In the plugin.yml, this entry is named <code>softdepend</code>.
/// 
/// Example:
/// <blockquote><pre>softdepend: [OnePlugin, AnotherPlugin]</pre></blockquote>
	fn soft_depend(&self) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSoftDepend",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
new_vec
)}
/// Gets the list of plugins that should consider this plugin a
/// soft-dependency.
/// <ul>
/// <li>Use the value in the {@link #getName()} of the target plugin to
/// specify the dependency.
/// <li>The plugin should load before any other plugins listed here.
/// <li>Specifying another plugin here is strictly equivalent to having the
/// specified plugin's {@link #getSoftDepend()} include {@link
/// #getName() this plugin}.
/// <li><code>loadbefore</code> must be in <a
/// href="https://en.wikipedia.org/wiki/YAML#Lists">YAML list
/// format</a>.
/// </ul>
/// 
/// In the plugin.yml, this entry is named <code>loadbefore</code>.
/// 
/// Example:
/// <blockquote><pre>loadbefore:
/// - OnePlugin
/// - AnotherPlugin</pre></blockquote>
	fn load_before(&self) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLoadBefore",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
new_vec
)}
/// Gives the token to prefix plugin-specific logging messages with.
/// <ul>
/// <li>This includes all messages using {@link Plugin#getLogger()}.
/// <li>If not specified, the server uses the plugin's {@link #getName()
/// name}.
/// <li>This should clearly indicate what plugin is being logged.
/// </ul>
/// 
/// In the plugin.yml, this entry is named <code>prefix</code>.
/// 
/// Example:<blockquote><pre>prefix: ex-why-zee</pre></blockquote>
	fn prefix(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPrefix",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Gives the map of command-name to command-properties. Each entry in this
/// map corresponds to a single command and the respective values are the
/// properties of the command. Each property, <i>with the exception of
/// aliases</i>, can be defined at runtime using methods in {@link
/// PluginCommand} and are defined here only as a convenience.
/// <table border=1>
/// <caption>The command section's description</caption>
/// <tr>
/// <th>Node</th>
/// <th>Method</th>
/// <th>Type</th>
/// <th>Description</th>
/// <th>Example</th>
/// </tr><tr>
/// <td><code>description</code></td>
/// <td>{@link PluginCommand#setDescription(String)}</td>
/// <td>String</td>
/// <td>A user-friendly description for a command. It is useful for
/// documentation purposes as well as in-game help.</td>
/// <td><blockquote><pre>description: Set yourself on fire</pre></blockquote></td>
/// </tr><tr>
/// <td><code>aliases</code></td>
/// <td>{@link PluginCommand#setAliases(List)}</td>
/// <td>String or <a
/// href="https://en.wikipedia.org/wiki/YAML#Lists">List</a> of
/// strings</td>
/// <td>Alternative command names, with special usefulness for commands
/// that are already registered. <i>Aliases are not effective when
/// defined at runtime,</i> so the plugin description file is the
/// only way to have them properly defined.
/// 
/// Note: Command aliases may not have a colon in them.</td>
/// <td>Single alias format:
/// <blockquote><pre>aliases: combust_me</pre></blockquote> or
/// multiple alias format:
/// <blockquote><pre>aliases: [combust_me, combustMe]</pre></blockquote></td>
/// </tr><tr>
/// <td><code>permission</code></td>
/// <td>{@link PluginCommand#setPermission(String)}</td>
/// <td>String</td>
/// <td>The name of the {@link Permission} required to use the command.
/// A user without the permission will receive the specified
/// message (see {@linkplain
/// PluginCommand#setPermissionMessage(String) below}), or a
/// standard one if no specific message is defined. Without the
/// permission node, no {@link
/// PluginCommand#setExecutor(CommandExecutor) CommandExecutor} or
/// {@link PluginCommand#setTabCompleter(TabCompleter)} will be called.</td>
/// <td><blockquote><pre>permission: inferno.flagrate</pre></blockquote></td>
/// </tr><tr>
/// <td><code>permission-message</code></td>
/// <td>{@link PluginCommand#setPermissionMessage(String)}</td>
/// <td>String</td>
/// <td><ul>
/// <li>Displayed to a player that attempts to use a command, but
/// does not have the required permission. See {@link
/// PluginCommand#getPermission() above}.
/// <li>&lt;permission&gt; is a macro that is replaced with the
/// permission node required to use the command.
/// <li>Using empty quotes is a valid way to indicate nothing
/// should be displayed to a player.
/// </ul></td>
/// <td><blockquote><pre>permission-message: You do not have /&lt;permission&gt;</pre></blockquote></td>
/// </tr><tr>
/// <td><code>usage</code></td>
/// <td>{@link PluginCommand#setUsage(String)}</td>
/// <td>String</td>
/// <td>This message is displayed to a player when the {@link
/// PluginCommand#setExecutor(CommandExecutor)} {@linkplain
/// CommandExecutor#onCommand(CommandSender, Command, String, String[]) returns false}.
/// &lt;command&gt; is a macro that is replaced the command issued.</td>
/// <td><blockquote><pre>usage: Syntax error! Perhaps you meant /&lt;command&gt; PlayerName?</pre></blockquote>
/// It is worth noting that to use a colon in a yaml, like
/// <code>`usage: Usage: /god [player]'</code>, you need to
/// <a href="http://yaml.org/spec/current.html#id2503232">surround
/// the message with double-quote</a>:
/// <blockquote><pre>usage: "Usage: /god [player]"</pre></blockquote></td>
/// </tr>
/// </table>
/// The commands are structured as a hiearchy of <a
/// href="http://yaml.org/spec/current.html#id2502325">nested mappings</a>.
/// The primary (top-level, no intendentation) node is
/// `<code>commands</code>', while each individual command name is
/// indented, indicating it maps to some value (in our case, the
/// properties of the table above).
/// 
/// Here is an example bringing together the piecemeal examples above, as
/// well as few more definitions:<blockquote><pre>
/// commands:
/// flagrate:
/// description: Set yourself on fire.
/// aliases: [combust_me, combustMe]
/// permission: inferno.flagrate
/// permission-message: You do not have /&lt;permission&gt;
/// usage: Syntax error! Perhaps you meant /&lt;command&gt; PlayerName?
/// burningdeaths:
/// description: List how many times you have died by fire.
/// aliases:
/// - burning_deaths
/// - burningDeaths
/// permission: inferno.burningdeaths
/// usage: |
/// /&lt;command&gt; [player]
/// Example: /&lt;command&gt; - see how many times you have burned to death
/// Example: /&lt;command&gt; CaptainIce - see how many times CaptainIce has burned to death
/// # The next command has no description, aliases, etc. defined, but is still valid
/// # Having an empty declaration is useful for defining the description, permission, and messages from a configuration dynamically
/// apocalypse:
/// </pre></blockquote>
/// Note: Command names may not have a colon in their name.
	fn commands(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCommands",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gives the list of permissions the plugin will register at runtime,
/// immediately proceding enabling. The format for defining permissions is
/// a map from permission name to properties. To represent a map without
/// any specific property, empty <a
/// href="http://yaml.org/spec/current.html#id2502702">curly-braces</a> (
/// <code>&#123;&#125;</code> ) may be used (as a null value is not
/// accepted, unlike the {@link #getCommands() commands} above).
/// 
/// A list of optional properties for permissions:
/// <table border=1>
/// <caption>The permission section's description</caption>
/// <tr>
/// <th>Node</th>
/// <th>Description</th>
/// <th>Example</th>
/// </tr><tr>
/// <td><code>description</code></td>
/// <td>Plaintext (user-friendly) description of what the permission
/// is for.</td>
/// <td><blockquote><pre>description: Allows you to set yourself on fire</pre></blockquote></td>
/// </tr><tr>
/// <td><code>default</code></td>
/// <td>The default state for the permission, as defined by {@link
/// Permission#getDefault()}. If not defined, it will be set to
/// the value of {@link PluginDescriptionFile#getPermissionDefault()}.
/// 
/// For reference:<ul>
/// <li><code>true</code> - Represents a positive assignment to
/// {@link Permissible permissibles}.
/// <li><code>false</code> - Represents no assignment to {@link
/// Permissible permissibles}.
/// <li><code>op</code> - Represents a positive assignment to
/// {@link Permissible#isOp() operator permissibles}.
/// <li><code>notop</code> - Represents a positive assignment to
/// {@link Permissible#isOp() non-operator permissibiles}.
/// </ul></td>
/// <td><blockquote><pre>default: true</pre></blockquote></td>
/// </tr><tr>
/// <td><code>children</code></td>
/// <td>Allows other permissions to be set as a {@linkplain
/// Permission#getChildren() relation} to the parent permission.
/// When a parent permissions is assigned, child permissions are
/// respectively assigned as well.
/// <ul>
/// <li>When a parent permission is assigned negatively, child
/// permissions are assigned based on an inversion of their
/// association.
/// <li>When a parent permission is assigned positively, child
/// permissions are assigned based on their association.
/// </ul>
/// 
/// Child permissions may be defined in a number of ways:<ul>
/// <li>Children may be defined as a <a
/// href="https://en.wikipedia.org/wiki/YAML#Lists">list</a> of
/// names. Using a list will treat all children associated
/// positively to their parent.
/// <li>Children may be defined as a map. Each permission name maps
/// to either a boolean (representing the association), or a
/// nested permission definition (just as another permission).
/// Using a nested definition treats the child as a positive
/// association.
/// <li>A nested permission definition must be a map of these same
/// properties. To define a valid nested permission without
/// defining any specific property, empty curly-braces (
/// <code>&#123;&#125;</code> ) must be used.
/// <li>A nested permission may carry it's own nested permissions
/// as children, as they may also have nested permissions, and
/// so forth. There is no direct limit to how deep the
/// permission tree is defined.
/// </ul></td>
/// <td>As a list:
/// <blockquote><pre>children: [inferno.flagrate, inferno.burningdeaths]</pre></blockquote>
/// Or as a mapping:
/// <blockquote><pre>children:
/// inferno.flagrate: true
/// inferno.burningdeaths: true</pre></blockquote>
/// An additional example showing basic nested values can be seen
/// <a href="doc-files/permissions-example_plugin.yml">here</a>.
/// </td>
/// </tr>
/// </table>
/// The permissions are structured as a hiearchy of <a
/// href="http://yaml.org/spec/current.html#id2502325">nested mappings</a>.
/// The primary (top-level, no intendentation) node is
/// `<code>permissions</code>', while each individual permission name is
/// indented, indicating it maps to some value (in our case, the
/// properties of the table above).
/// 
/// Here is an example using some of the properties:<blockquote><pre>
/// permissions:
/// inferno.*:
/// description: Gives access to all Inferno commands
/// children:
/// inferno.flagrate: true
/// inferno.burningdeaths: true
/// inferno.flagate:
/// description: Allows you to ignite yourself
/// default: true
/// inferno.burningdeaths:
/// description: Allows you to see how many times you have burned to death
/// default: true
/// </pre></blockquote>
/// Another example, with nested definitions, can be found <a
/// href="doc-files/permissions-example_plugin.yml">here</a>.
	fn permissions(&self) 
-> Result<Vec<crate::permissions::Permission<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPermissions",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::permissions::Permission::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Gives the default {@link Permission#getDefault() default} state of
/// {@link #getPermissions() permissions} registered for the plugin.
/// <ul>
/// <li>If not specified, it will be {@link PermissionDefault#OP}.
/// <li>It is matched using {@link PermissionDefault#getByName(String)}
/// <li>It only affects permissions that do not define the
/// <code>default</code> node.
/// <li>It may be any value in {@link PermissionDefault}.
/// </ul>
/// 
/// In the plugin.yml, this entry is named <code>default-permission</code>.
/// 
/// Example:<blockquote><pre>default-permission: NOT_OP</pre></blockquote>
	fn permission_default(&self) 
-> Result<crate::permissions::PermissionDefault<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/permissions/PermissionDefault;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPermissionDefault",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::permissions::PermissionDefault::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gives a set of every {@link PluginAwareness} for a plugin. An awareness
/// dictates something that a plugin developer acknowledges when the plugin
/// is compiled. Some implementions may define extra awarenesses that are
/// not included in the API. Any unrecognized
/// awareness (one unsupported or in a future version) will cause a dummy
/// object to be created instead of failing.
/// <ul>
/// <li>Currently only supports the enumerated values in {@link
/// PluginAwareness.Flags}.
/// <li>Each awareness starts the identifier with bang-at
/// (<code>!@</code>).
/// <li>Unrecognized (future / unimplemented) entries are quietly replaced
/// by a generic object that implements PluginAwareness.
/// <li>A type of awareness must be defined by the runtime and acknowledged
/// by the API, effectively discluding any derived type from any
/// plugin's classpath.
/// <li><code>awareness</code> must be in <a
/// href="https://en.wikipedia.org/wiki/YAML#Lists">YAML list
/// format</a>.
/// </ul>
/// 
/// In the plugin.yml, this entry is named <code>awareness</code>.
/// 
/// Example:<blockquote><pre>awareness:
/// - !@UTF8</pre></blockquote>
/// 
/// <b>Note:</b> Although unknown versions of some future awareness are
/// gracefully substituted, previous versions of Bukkit (ones prior to the
/// first implementation of awareness) will fail to load a plugin that
/// defines any awareness.
	fn awareness(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAwareness",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns the name of a plugin, including the version. This method is
/// provided for convenience; it uses the {@link #getName()} and {@link
/// #getVersion()} entries.
	fn full_name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFullName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gives the API version which this plugin is designed to support. No
/// specific format is guaranteed.
/// <ul>
/// <li>Refer to release notes for supported API versions.
/// </ul>
/// 
/// In the plugin.yml, this entry is named <code>api-version</code>.
/// 
/// Example:<blockquote><pre>api-version: 1.13</pre></blockquote>
	fn apiversion(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAPIVersion",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Gets the libraries this plugin requires. This is a preview feature.
/// <ul>
/// <li>Libraries must be GAV specifiers and are loaded from Maven Central.
/// </ul>
/// 
/// Example:<blockquote><pre>libraries:
/// - com.squareup.okhttp3:okhttp:4.9.0</pre></blockquote>
	fn libraries(&self) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLibraries",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
new_vec
)}
#[deprecated]

	fn class_loader_of(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getClassLoaderOf",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}

	fn raw_name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRawName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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

        impl<'mc> PluginAwarenessFlagsTrait<'mc> for PluginAwarenessFlags<'mc> {}
        
        pub trait PluginAwarenessFlagsTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
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

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::plugin::PluginAwarenessFlags<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/plugin/PluginAwareness/Flags;");
let cls = jni.find_class("org/bukkit/plugin/PluginAwareness/Flags"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::plugin::PluginAwarenessFlags::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> PluginLoaderTrait<'mc> for PluginLoader<'mc> {}
pub trait PluginLoaderTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns a list of all filename filters expected by this PluginLoader
	fn plugin_file_filters(&self) 
-> Result<blackboxmc_java::util::regex::JavaPattern<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/regex/Pattern;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPluginFileFilters",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::regex::JavaPattern::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Enables the specified plugin
/// 
/// Attempting to enable a plugin that is already enabled will have no
/// effect
	fn enable_plugin(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"enablePlugin",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Disables the specified plugin
/// 
/// Attempting to disable a plugin that is not enabled will have no effect
	fn disable_plugin(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"disablePlugin",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> PluginAwarenessTrait<'mc> for PluginAwareness<'mc> {}
pub trait PluginAwarenessTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> PluginManagerTrait<'mc> for PluginManager<'mc> {}
pub trait PluginManagerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Registers the specified plugin loader
	fn register_interface(&self,loader: jni::objects::JClass<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Class;)V");
let val_1 = jni::objects::JValueGen::Object(loader.into());
let res = self.jni_ref().call_method(&self.jni_object(),"registerInterface",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks if the given plugin is loaded and returns it when applicable
/// 
/// Please note that the name of the plugin is case-sensitive
	fn get_plugin(&self,name: impl Into<String>) 
-> Result<Option<crate::plugin::Plugin<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/plugin/Plugin;");
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
/// Gets a list of all currently loaded plugins
	fn plugins(&self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/plugin/Plugin;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlugins",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Checks if the given plugin is enabled or not
	fn is_plugin_enabled(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
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
/// Disables all the loaded plugins
	fn disable_plugins(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"disablePlugins",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Disables and removes all plugins
	fn clear_plugins(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"clearPlugins",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Calls an event with the given details
	fn call_event(&self,event: impl Into<crate::event::Event<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Event;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(event.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"callEvent",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Registers all the events in the given listener class
	fn register_events(&self,listener: impl Into<crate::event::Listener<'mc>>,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Listener;Lorg/bukkit/plugin/Plugin;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(listener.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"registerEvents",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Registers the specified executor to the given event class
	fn register_event(&self,event: jni::objects::JClass<'mc>,listener: impl Into<crate::event::Listener<'mc>>,priority: impl Into<crate::event::EventPriority<'mc>>,executor: impl Into<crate::plugin::EventExecutor<'mc>>,plugin: impl Into<crate::plugin::Plugin<'mc>>,ignore_cancelled: std::option::Option<bool>) 
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
/// Enables the specified plugin
/// 
/// Attempting to enable a plugin that is already enabled will have no
/// effect
	fn enable_plugin(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"enablePlugin",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Disables the specified plugin
/// 
/// Attempting to disable a plugin that is not enabled will have no effect
	fn disable_plugin(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"disablePlugin",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets a {@link Permission} from its fully qualified name
	fn get_permission(&self,name: impl Into<String>) 
-> Result<Option<crate::permissions::Permission<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/permissions/Permission;");
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
/// Adds a {@link Permission} to this plugin manager.
/// 
/// If a permission is already defined with the given name of the new
/// permission, an exception will be thrown.
	fn add_permission(&self,perm: impl Into<crate::permissions::Permission<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/permissions/Permission;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(perm.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addPermission",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Removes a {@link Permission} registration from this plugin manager.
/// 
/// If the specified permission does not exist in this plugin manager,
/// nothing will happen.
/// 
/// Removing a permission registration will <b>not</b> remove the
/// permission from any {@link Permissible}s that have it.
	fn remove_permission(&self,name: impl Into<String>) 
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
/// Gets the default permissions for the given op status
	fn get_default_permissions(&self,op: bool) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Z)Ljava/util/Set;");
let val_1 = jni::objects::JValueGen::Bool(op.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultPermissions",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Recalculates the defaults for the given {@link Permission}.
/// 
/// This will have no effect if the specified permission is not registered
/// here.
	fn recalculate_permission_defaults(&self,perm: impl Into<crate::permissions::Permission<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/permissions/Permission;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(perm.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"recalculatePermissionDefaults",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Subscribes the given Permissible for information about the requested
/// Permission, by name.
/// 
/// If the specified Permission changes in any form, the Permissible will
/// be asked to recalculate.
	fn subscribe_to_permission(&self,permission: impl Into<String>,permissible: impl Into<crate::permissions::Permissible<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Lorg/bukkit/permissions/Permissible;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(permission.into())?));
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(permissible.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"subscribeToPermission",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Unsubscribes the given Permissible for information about the requested
/// Permission, by name.
	fn unsubscribe_from_permission(&self,permission: impl Into<String>,permissible: impl Into<crate::permissions::Permissible<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Lorg/bukkit/permissions/Permissible;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(permission.into())?));
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(permissible.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"unsubscribeFromPermission",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets a set containing all subscribed {@link Permissible}s to the given
/// permission, by name
	fn get_permission_subscriptions(&self,permission: impl Into<String>) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Ljava/util/Set;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(permission.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getPermissionSubscriptions",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Subscribes to the given Default permissions by operator status
/// 
/// If the specified defaults change in any form, the Permissible will be
/// asked to recalculate.
	fn subscribe_to_default_perms(&self,op: bool,permissible: impl Into<crate::permissions::Permissible<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(ZLorg/bukkit/permissions/Permissible;)V");
let val_1 = jni::objects::JValueGen::Bool(op.into());
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(permissible.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"subscribeToDefaultPerms",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Unsubscribes from the given Default permissions by operator status
	fn unsubscribe_from_default_perms(&self,op: bool,permissible: impl Into<crate::permissions::Permissible<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(ZLorg/bukkit/permissions/Permissible;)V");
let val_1 = jni::objects::JValueGen::Bool(op.into());
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(permissible.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"unsubscribeFromDefaultPerms",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets a set containing all subscribed {@link Permissible}s to the given
/// default list, by op status
	fn get_default_perm_subscriptions(&self,op: bool) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Z)Ljava/util/Set;");
let val_1 = jni::objects::JValueGen::Bool(op.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultPermSubscriptions",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets a set of all registered permissions.
/// 
/// This set is a copy and will not be modified live.
	fn permissions(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPermissions",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns whether or not timing code should be used for event calls
	fn use_timings(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"useTimings",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> PluginBaseTrait<'mc> for PluginBase<'mc> {}
pub trait PluginBaseTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::plugin::PluginBase<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/plugin/PluginBase"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::PluginBase::from_raw(&jni,res
)}

	fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::plugin::Plugin<'mc>> for PluginBase<'mc>{

fn into(self) -> crate::plugin::Plugin<'mc> {

crate::plugin::Plugin::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PluginBase into crate::plugin::Plugin")

   }
}
impl<'mc> crate::plugin::PluginTrait<'mc> for PluginBase<'mc> {}
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

        impl<'mc> ServicePriorityTrait<'mc> for ServicePriority<'mc> {}
        
        pub trait ServicePriorityTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
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

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::plugin::ServicePriority<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/plugin/ServicePriority;");
let cls = jni.find_class("org/bukkit/plugin/ServicePriority"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::plugin::ServicePriority::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> InvalidPluginExceptionTrait<'mc> for InvalidPluginException<'mc> {}
pub trait InvalidPluginExceptionTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> RegisteredListenerTrait<'mc> for RegisteredListener<'mc> {}
pub trait RegisteredListenerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,listener: impl Into<crate::event::Listener<'mc>>,executor: impl Into<crate::plugin::EventExecutor<'mc>>,priority: impl Into<crate::event::EventPriority<'mc>>,plugin: impl Into<crate::plugin::Plugin<'mc>>,ignore_cancelled: bool) 
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
/// Gets the listener for this registration
	fn listener(&self) 
-> Result<crate::event::Listener<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/Listener;");
let res = self.jni_ref().call_method(&self.jni_object(),"getListener",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::Listener::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the plugin for this registration
	fn plugin(&self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/plugin/Plugin;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlugin",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the priority for this registration
	fn priority(&self) 
-> Result<crate::event::EventPriority<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/EventPriority;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPriority",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventPriority::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Calls the event executor
	fn call_event(&self,event: impl Into<crate::event::Event<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Event;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(event.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"callEvent",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Whether this listener accepts cancelled events
	fn is_ignoring_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isIgnoringCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub mod java;
pub mod messaging;
