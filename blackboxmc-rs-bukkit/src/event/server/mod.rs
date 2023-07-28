#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub struct RemoteServerCommandEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for RemoteServerCommandEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RemoteServerCommandEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate RemoteServerCommandEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "RemoteServerCommandEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a RemoteServerCommandEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::command::CommandSender<'mc>>,arg1: impl Into<&'mc String>) 
-> Result<crate::event::server::RemoteServerCommandEvent<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
let cls = &jni.find_class("org/bukkit/event/server/RemoteServerCommandEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/command/CommandSender;Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::server::RemoteServerCommandEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn command(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCommand","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_command(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setCommand","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_1 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn sender(&mut self) 
-> Result<crate::command::CommandSender<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSender","()Lorg/bukkit/command/CommandSender;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::command::CommandSender::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::server::ServerCommandEvent<'mc>> for RemoteServerCommandEvent<'mc>{
   fn into(self) -> crate::event::server::ServerCommandEvent<'mc> {
       crate::event::server::ServerCommandEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PluginEnableEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PluginEnableEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PluginEnableEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PluginEnableEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PluginEnableEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PluginEnableEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>) 
-> Result<crate::event::server::PluginEnableEvent<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/server/PluginEnableEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/plugin/Plugin;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::event::server::PluginEnableEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn plugin(&mut self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlugin","()Lorg/bukkit/plugin/Plugin;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::server::PluginEvent<'mc>> for PluginEnableEvent<'mc>{
   fn into(self) -> crate::event::server::PluginEvent<'mc> {
       crate::event::server::PluginEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ServerCommandEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ServerCommandEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ServerCommandEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ServerCommandEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ServerCommandEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ServerCommandEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::command::CommandSender<'mc>>,arg1: impl Into<&'mc String>) 
-> Result<crate::event::server::ServerCommandEvent<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
let cls = &jni.find_class("org/bukkit/event/server/ServerCommandEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/command/CommandSender;Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::server::ServerCommandEvent::from_raw(&jni,res
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn command(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCommand","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_command(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setCommand","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_1 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn sender(&mut self) 
-> Result<crate::command::CommandSender<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSender","()Lorg/bukkit/command/CommandSender;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::command::CommandSender::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for ServerCommandEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::server::ServerEvent<'mc>> for ServerCommandEvent<'mc>{
   fn into(self) -> crate::event::server::ServerEvent<'mc> {
       crate::event::server::ServerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ServiceUnregisterEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ServiceUnregisterEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ServiceUnregisterEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ServiceUnregisterEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ServiceUnregisterEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ServiceUnregisterEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::plugin::RegisteredServiceProvider<'mc, dyn JNIRaw<'mc>>>) 
-> Result<crate::event::server::ServiceUnregisterEvent<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/server/ServiceUnregisterEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/plugin/RegisteredServiceProvider;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::event::server::ServiceUnregisterEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn provider(&mut self) 
-> Result<crate::plugin::RegisteredServiceProvider<'mc, dyn JNIRaw<'mc>>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getProvider","()Lorg/bukkit/plugin/RegisteredServiceProvider;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::RegisteredServiceProvider::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::server::ServiceEvent<'mc>> for ServiceUnregisterEvent<'mc>{
   fn into(self) -> crate::event::server::ServiceEvent<'mc> {
       crate::event::server::ServiceEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct BroadcastMessageEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BroadcastMessageEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BroadcastMessageEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate BroadcastMessageEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "BroadcastMessageEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a BroadcastMessageEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: bool,arg1: std::option::Option<impl Into<&'mc String>>,arg2: std::option::Option<impl Into<&'mc blackboxmc_java::JavaSet<'mc, orgCommandSender>>>) 
-> Result<crate::event::server::BroadcastMessageEvent<'mc>, Box<dyn std::error::Error>>

{// 1
let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/server/BroadcastMessageEvent")?;
let res = jni.new_object(cls,
"(ZLjava/lang/String;Ljava/util/Set;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::server::BroadcastMessageEvent::from_raw(&jni,res
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn message(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMessage","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_message(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setMessage","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_1 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn recipients(&mut self) 
-> Result<blackboxmc_java::JavaSet<'mc, orgCommandSender>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRecipients","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BroadcastMessageEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::server::ServerEvent<'mc>> for BroadcastMessageEvent<'mc>{
   fn into(self) -> crate::event::server::ServerEvent<'mc> {
       crate::event::server::ServerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ServerEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ServerEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ServerEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ServerEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ServerEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ServerEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<bool>) 
-> Result<crate::event::server::ServerEvent<'mc>, Box<dyn std::error::Error>>

{// -1
let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
let cls = &jni.find_class("org/bukkit/event/server/ServerEvent")?;
let res = jni.new_object(cls,
"(Z)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::event::server::ServerEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Event<'mc>> for ServerEvent<'mc>{
   fn into(self) -> crate::event::Event<'mc> {
       crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct TabCompleteEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for TabCompleteEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TabCompleteEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate TabCompleteEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "TabCompleteEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a TabCompleteEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::command::CommandSender<'mc>>,arg1: impl Into<&'mc String>,arg2: impl Into<&'mc blackboxmc_java::JavaList<'mc, String>>) 
-> Result<crate::event::server::TabCompleteEvent<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/server/TabCompleteEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/util/List;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::server::TabCompleteEvent::from_raw(&jni,res
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn buffer(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBuffer","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_1 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn sender(&mut self) 
-> Result<crate::command::CommandSender<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSender","()Lorg/bukkit/command/CommandSender;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::command::CommandSender::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn completions(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, String>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCompletions","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_completions(&mut self,arg0: impl Into<&'mc blackboxmc_java::JavaList<'mc, String>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setCompletions","(Ljava/util/List;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for TabCompleteEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::Event<'mc>> for TabCompleteEvent<'mc>{
   fn into(self) -> crate::event::Event<'mc> {
       crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PluginDisableEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PluginDisableEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PluginDisableEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PluginDisableEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PluginDisableEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PluginDisableEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>) 
-> Result<crate::event::server::PluginDisableEvent<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/server/PluginDisableEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/plugin/Plugin;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::event::server::PluginDisableEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn plugin(&mut self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlugin","()Lorg/bukkit/plugin/Plugin;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::server::PluginEvent<'mc>> for PluginDisableEvent<'mc>{
   fn into(self) -> crate::event::server::PluginEvent<'mc> {
       crate::event::server::PluginEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct MapInitializeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MapInitializeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapInitializeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate MapInitializeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "MapInitializeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a MapInitializeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::map::MapView<'mc>>) 
-> Result<crate::event::server::MapInitializeEvent<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/server/MapInitializeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/map/MapView;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::event::server::MapInitializeEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map(&mut self) 
-> Result<crate::map::MapView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMap","()Lorg/bukkit/map/MapView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::map::MapView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::server::ServerEvent<'mc>> for MapInitializeEvent<'mc>{
   fn into(self) -> crate::event::server::ServerEvent<'mc> {
       crate::event::server::ServerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ServiceEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ServiceEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ServiceEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ServiceEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ServiceEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ServiceEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::plugin::RegisteredServiceProvider<'mc, dyn JNIRaw<'mc>>>) 
-> Result<crate::event::server::ServiceEvent<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/server/ServiceEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/plugin/RegisteredServiceProvider;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::event::server::ServiceEvent::from_raw(&jni,res
)}
	pub fn provider(&mut self) 
-> Result<crate::plugin::RegisteredServiceProvider<'mc, dyn JNIRaw<'mc>>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getProvider","()Lorg/bukkit/plugin/RegisteredServiceProvider;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::RegisteredServiceProvider::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::server::ServerEvent<'mc>> for ServiceEvent<'mc>{
   fn into(self) -> crate::event::server::ServerEvent<'mc> {
       crate::event::server::ServerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ServerLoadEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct ServerLoadEventLoadType<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ServerLoadEventLoadType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ServerLoadEventLoadType<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ServerLoadEventLoadType from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ServerLoadEventLoadType")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ServerLoadEventLoadType object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value_of_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<T, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Enum")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
T::from_raw(&jni,obj
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"name","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn describe_constable(&mut self) 
-> Result<blackboxmc_java::JavaOptional<'mc, javaEnumEnumDesc<E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"describeConstable","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn declaring_class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDeclaringClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn ordinal(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"ordinal","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ServerLoadEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ServerLoadEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ServerLoadEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ServerLoadEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ServerLoadEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::event::server::ServerLoadEventLoadType<'mc>>) 
-> Result<crate::event::server::ServerLoadEvent<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/server/ServerLoadEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/event/server/ServerLoadEvent$LoadType;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::event::server::ServerLoadEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_type(&mut self) 
-> Result<crate::event::server::ServerLoadEventLoadType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getType","()Lorg/bukkit/event/server/ServerLoadEvent$LoadType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::server::ServerLoadEventLoadType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::server::ServerEvent<'mc>> for ServerLoadEvent<'mc>{
   fn into(self) -> crate::event::server::ServerEvent<'mc> {
       crate::event::server::ServerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ServiceRegisterEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ServiceRegisterEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ServiceRegisterEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ServiceRegisterEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ServiceRegisterEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ServiceRegisterEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::plugin::RegisteredServiceProvider<'mc, dyn JNIRaw<'mc>>>) 
-> Result<crate::event::server::ServiceRegisterEvent<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/server/ServiceRegisterEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/plugin/RegisteredServiceProvider;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::event::server::ServiceRegisterEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn provider(&mut self) 
-> Result<crate::plugin::RegisteredServiceProvider<'mc, dyn JNIRaw<'mc>>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getProvider","()Lorg/bukkit/plugin/RegisteredServiceProvider;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::RegisteredServiceProvider::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::server::ServiceEvent<'mc>> for ServiceRegisterEvent<'mc>{
   fn into(self) -> crate::event::server::ServiceEvent<'mc> {
       crate::event::server::ServiceEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PluginEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PluginEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PluginEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PluginEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PluginEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PluginEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>) 
-> Result<crate::event::server::PluginEvent<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/server/PluginEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/plugin/Plugin;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::event::server::PluginEvent::from_raw(&jni,res
)}
	pub fn plugin(&mut self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlugin","()Lorg/bukkit/plugin/Plugin;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::server::ServerEvent<'mc>> for PluginEvent<'mc>{
   fn into(self) -> crate::event::server::ServerEvent<'mc> {
       crate::event::server::ServerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ServerListPingEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ServerListPingEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ServerListPingEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ServerListPingEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ServerListPingEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ServerListPingEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn iterator(&mut self) 
-> Result<blackboxmc_java::JavaIterator<'mc, orgPlayer>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn max_players(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMaxPlayers","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_max_players(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setMaxPlayers","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
#[deprecated]
	pub fn should_send_chat_previews(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"shouldSendChatPreviews","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn motd(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMotd","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_motd(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setMotd","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn hostname(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHostname","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn num_players(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNumPlayers","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_server_icon(&mut self,arg0: impl Into<&'mc crate::util::CachedServerIcon<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setServerIcon","(Lorg/bukkit/util/CachedServerIcon;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn spliterator(&mut self) 
-> Result<blackboxmc_java::JavaSpliterator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn for_each(&mut self,arg0: impl Into<&'mc blackboxmc_java::function::JavaConsumer<'mc, T>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"forEach","(Ljava/util/function/Consumer;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::server::ServerEvent<'mc>> for ServerListPingEvent<'mc>{
   fn into(self) -> crate::event::server::ServerEvent<'mc> {
       crate::event::server::ServerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
