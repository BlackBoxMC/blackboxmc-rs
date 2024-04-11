#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct Messenger<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Messenger<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Messenger<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Messenger from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/messaging/Messenger")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Messenger object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> Messenger<'mc> {
	pub fn is_reserved_channel(&self,channel: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isReservedChannel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn register_outgoing_plugin_channel(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,channel: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"registerOutgoingPluginChannel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn unregister_outgoing_plugin_channel_with_plugin(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,channel: std::option::Option<impl Into<String>>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/plugin/Plugin;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = channel {
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(a.into())?));
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"unregisterOutgoingPluginChannel",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn register_incoming_plugin_channel(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,channel: impl Into<String>,listener: impl Into<crate::plugin::messaging::PluginMessageListener<'mc>>) 
-> Result<crate::plugin::messaging::PluginMessageListenerRegistration<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;Lorg/bukkit/plugin/messaging/PluginMessageListener;)Lcrate::plugin::messaging::PluginMessageListenerRegistration;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(listener.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"registerIncomingPluginChannel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::messaging::PluginMessageListenerRegistration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn unregister_incoming_plugin_channel_with_plugin(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,channel: std::option::Option<impl Into<String>>,listener: std::option::Option<impl Into<crate::plugin::messaging::PluginMessageListener<'mc>>>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/plugin/Plugin;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = channel {
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(a.into())?));
args.push(val_2);
}
if let Some(a) = listener {
sig += "Lorg/bukkit/plugin/messaging/PluginMessageListener;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"unregisterIncomingPluginChannel",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn get_outgoing_channels(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)Lblackboxmc_java::util::Set;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getOutgoingChannels",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_incoming_channels(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)Lblackboxmc_java::util::Set;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getIncomingChannels",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_incoming_channel_registrations_with_channel(&self,channel: impl Into<String>) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
args.push(val_1);
sig += ")Ljava/util/Set;";
let res = self.jni_ref().call_method(&self.jni_object(),"getIncomingChannelRegistrations",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_incoming_channel_registrations_with_plugin(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,channel: std::option::Option<impl Into<String>>) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/plugin/Plugin;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = channel {
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(a.into())?));
args.push(val_2);
}
sig += ")Ljava/util/Set;";
let res = self.jni_ref().call_method(&self.jni_object(),"getIncomingChannelRegistrations",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_registration_valid(&self,registration: impl Into<crate::plugin::messaging::PluginMessageListenerRegistration<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/messaging/PluginMessageListenerRegistration;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(registration.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"isRegistrationValid",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_incoming_channel_registered(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,channel: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isIncomingChannelRegistered",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_outgoing_channel_registered(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,channel: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isOutgoingChannelRegistered",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn dispatch_incoming_message(&self,source: impl Into<crate::entity::Player<'mc>>,channel: impl Into<String>,message: i8) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Ljava/lang/String;B)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(source.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
let val_3 = jni::objects::JValueGen::Byte(message);
let res = self.jni_ref().call_method(&self.jni_object(),"dispatchIncomingMessage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
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
pub struct PluginMessageListener<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginMessageListener<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginMessageListener<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginMessageListener from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/messaging/PluginMessageListener")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginMessageListener object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginMessageListener<'mc> {
	pub fn on_plugin_message_received(&self,channel: impl Into<String>,player: impl Into<crate::entity::Player<'mc>>,message: i8) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Lorg/bukkit/entity/Player;B)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Byte(message);
let res = self.jni_ref().call_method(&self.jni_object(),"onPluginMessageReceived",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
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
pub struct PluginMessageRecipient<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginMessageRecipient<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginMessageRecipient<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginMessageRecipient from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/messaging/PluginMessageRecipient")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginMessageRecipient object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginMessageRecipient<'mc> {
	pub fn send_plugin_message(&self,source: impl Into<crate::plugin::Plugin<'mc>>,channel: impl Into<String>,message: i8) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;B)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(source.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
let val_3 = jni::objects::JValueGen::Byte(message);
let res = self.jni_ref().call_method(&self.jni_object(),"sendPluginMessage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn listening_plugin_channels(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getListeningPluginChannels",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct ChannelNotRegisteredException<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ChannelNotRegisteredException<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ChannelNotRegisteredException<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ChannelNotRegisteredException from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/messaging/ChannelNotRegisteredException")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ChannelNotRegisteredException object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ChannelNotRegisteredException<'mc> {
	pub fn new_with_channel(jni: &blackboxmc_general::SharedJNIEnv<'mc>,channel: std::option::Option<impl Into<String>>) 
-> Result<crate::plugin::messaging::ChannelNotRegisteredException<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = channel {
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(a.into())?));
args.push(val_1);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/plugin/messaging/ChannelNotRegisteredException"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::messaging::ChannelNotRegisteredException::from_raw(&jni,res
)}
// SUPER CLASS: RuntimeException

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct ReservedChannelException<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ReservedChannelException<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ReservedChannelException<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ReservedChannelException from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/messaging/ReservedChannelException")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ReservedChannelException object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ReservedChannelException<'mc> {
	pub fn new_with_name(jni: &blackboxmc_general::SharedJNIEnv<'mc>,name: std::option::Option<impl Into<String>>) 
-> Result<crate::plugin::messaging::ReservedChannelException<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = name {
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(a.into())?));
args.push(val_1);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/plugin/messaging/ReservedChannelException"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::messaging::ReservedChannelException::from_raw(&jni,res
)}
// SUPER CLASS: RuntimeException

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct ChannelNameTooLongException<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ChannelNameTooLongException<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ChannelNameTooLongException<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ChannelNameTooLongException from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/messaging/ChannelNameTooLongException")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ChannelNameTooLongException object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ChannelNameTooLongException<'mc> {
	pub fn new_with_channel(jni: &blackboxmc_general::SharedJNIEnv<'mc>,channel: std::option::Option<impl Into<String>>) 
-> Result<crate::plugin::messaging::ChannelNameTooLongException<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = channel {
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(a.into())?));
args.push(val_1);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/plugin/messaging/ChannelNameTooLongException"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::messaging::ChannelNameTooLongException::from_raw(&jni,res
)}
// SUPER CLASS: RuntimeException

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct MessageTooLargeException<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for MessageTooLargeException<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for MessageTooLargeException<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MessageTooLargeException from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/messaging/MessageTooLargeException")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MessageTooLargeException object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MessageTooLargeException<'mc> {
	pub fn new_with_length(jni: &blackboxmc_general::SharedJNIEnv<'mc>,length: std::option::Option<i32>) 
-> Result<crate::plugin::messaging::MessageTooLargeException<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = length {
sig += "I";
let val_1 = jni::objects::JValueGen::Int(a);
args.push(val_1);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/plugin/messaging/MessageTooLargeException"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::messaging::MessageTooLargeException::from_raw(&jni,res
)}
	pub fn new_with_msg(jni: &blackboxmc_general::SharedJNIEnv<'mc>,msg: std::option::Option<impl Into<String>>) 
-> Result<crate::plugin::messaging::MessageTooLargeException<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = msg {
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(a.into())?));
args.push(val_1);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/plugin/messaging/MessageTooLargeException"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::messaging::MessageTooLargeException::from_raw(&jni,res
)}
// SUPER CLASS: RuntimeException

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub enum PluginChannelDirection<'mc> {
	Incoming {inner: PluginChannelDirectionStruct<'mc>},
	Outgoing {inner: PluginChannelDirectionStruct<'mc>},
}
impl<'mc> std::fmt::Display for PluginChannelDirection<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PluginChannelDirection::Incoming { .. } => f.write_str("INCOMING"),
           PluginChannelDirection::Outgoing { .. } => f.write_str("OUTGOING"),
       }
   }
}

        impl<'mc> PluginChannelDirection<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PluginChannelDirection<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/plugin/messaging/PluginChannelDirection");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/plugin/messaging/PluginChannelDirection;",
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
                    
"INCOMING" => Ok(PluginChannelDirection::Incoming { inner: PluginChannelDirectionStruct::from_raw(env,obj)?}),
"OUTGOING" => Ok(PluginChannelDirection::Outgoing { inner: PluginChannelDirectionStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PluginChannelDirectionStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginChannelDirection<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Incoming { inner } => inner.0.clone(),
Self::Outgoing { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Incoming { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Outgoing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginChannelDirection<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginChannelDirection from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/messaging/PluginChannelDirection")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginChannelDirection object, got {}",
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
                    "INCOMING" => Ok(PluginChannelDirection::Incoming { inner: PluginChannelDirectionStruct::from_raw(env,obj)?}),"OUTGOING" => Ok(PluginChannelDirection::Outgoing { inner: PluginChannelDirectionStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PluginChannelDirectionStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginChannelDirectionStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginChannelDirectionStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/messaging/PluginChannelDirection")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginChannelDirectionStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginChannelDirectionStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::plugin::messaging::PluginChannelDirection<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::plugin::messaging::PluginChannelDirection;");
let cls = jni.find_class("org/bukkit/plugin/messaging/PluginChannelDirection"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::plugin::messaging::PluginChannelDirection::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PluginMessageListenerRegistration<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginMessageListenerRegistration<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginMessageListenerRegistration<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginMessageListenerRegistration from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/messaging/PluginMessageListenerRegistration")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginMessageListenerRegistration object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginMessageListenerRegistration<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,messenger: impl Into<crate::plugin::messaging::Messenger<'mc>>,plugin: impl Into<crate::plugin::Plugin<'mc>>,channel: impl Into<String>,listener: impl Into<crate::plugin::messaging::PluginMessageListener<'mc>>) 
-> Result<crate::plugin::messaging::PluginMessageListenerRegistration<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/messaging/Messenger;Lorg/bukkit/plugin/Plugin;Ljava/lang/String;Lorg/bukkit/plugin/messaging/PluginMessageListener;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(messenger.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(channel.into())?));
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(listener.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/plugin/messaging/PluginMessageListenerRegistration"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::messaging::PluginMessageListenerRegistration::from_raw(&jni,res
)}
	pub fn channel(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChannel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn listener(&self) 
-> Result<crate::plugin::messaging::PluginMessageListener<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::plugin::messaging::PluginMessageListener;");
let res = self.jni_ref().call_method(&self.jni_object(),"getListener",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::messaging::PluginMessageListener::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn plugin(&self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::plugin::Plugin;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlugin",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_valid(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isValid",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
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
	pub fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
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
#[repr(C)]
pub struct StandardMessenger<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for StandardMessenger<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for StandardMessenger<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate StandardMessenger from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/messaging/StandardMessenger")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a StandardMessenger object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> StandardMessenger<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::plugin::messaging::StandardMessenger<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/plugin/messaging/StandardMessenger"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::plugin::messaging::StandardMessenger::from_raw(&jni,res
)}
	pub fn is_reserved_channel(&self,channel: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isReservedChannel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn register_outgoing_plugin_channel(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,channel: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"registerOutgoingPluginChannel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn unregister_outgoing_plugin_channel_with_plugin(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,channel: std::option::Option<impl Into<String>>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/plugin/Plugin;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = channel {
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(a.into())?));
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"unregisterOutgoingPluginChannel",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn register_incoming_plugin_channel(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,channel: impl Into<String>,listener: impl Into<crate::plugin::messaging::PluginMessageListener<'mc>>) 
-> Result<crate::plugin::messaging::PluginMessageListenerRegistration<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;Lorg/bukkit/plugin/messaging/PluginMessageListener;)Lcrate::plugin::messaging::PluginMessageListenerRegistration;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(listener.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"registerIncomingPluginChannel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::messaging::PluginMessageListenerRegistration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn unregister_incoming_plugin_channel_with_plugin(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,channel: std::option::Option<impl Into<String>>,listener: std::option::Option<impl Into<crate::plugin::messaging::PluginMessageListener<'mc>>>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/plugin/Plugin;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = channel {
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(a.into())?));
args.push(val_2);
}
if let Some(a) = listener {
sig += "Lorg/bukkit/plugin/messaging/PluginMessageListener;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"unregisterIncomingPluginChannel",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn get_outgoing_channels(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)Lblackboxmc_java::util::Set;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getOutgoingChannels",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_incoming_channels(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)Lblackboxmc_java::util::Set;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getIncomingChannels",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_incoming_channel_registrations_with_channel(&self,channel: impl Into<String>) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
args.push(val_1);
sig += ")Ljava/util/Set;";
let res = self.jni_ref().call_method(&self.jni_object(),"getIncomingChannelRegistrations",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_incoming_channel_registrations_with_plugin(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,channel: std::option::Option<impl Into<String>>) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/plugin/Plugin;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = channel {
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(a.into())?));
args.push(val_2);
}
sig += ")Ljava/util/Set;";
let res = self.jni_ref().call_method(&self.jni_object(),"getIncomingChannelRegistrations",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_registration_valid(&self,registration: impl Into<crate::plugin::messaging::PluginMessageListenerRegistration<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/messaging/PluginMessageListenerRegistration;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(registration.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"isRegistrationValid",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_incoming_channel_registered(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,channel: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isIncomingChannelRegistered",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_outgoing_channel_registered(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,channel: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isOutgoingChannelRegistered",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn dispatch_incoming_message(&self,source: impl Into<crate::entity::Player<'mc>>,channel: impl Into<String>,message: i8) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Ljava/lang/String;B)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(source.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(channel.into())?));
let val_3 = jni::objects::JValueGen::Byte(message);
let res = self.jni_ref().call_method(&self.jni_object(),"dispatchIncomingMessage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]

	pub fn validate_channel(jni: &blackboxmc_general::SharedJNIEnv<'mc>,channel: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(channel.into())?));
let cls = jni.find_class("org/bukkit/plugin/messaging/StandardMessenger"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"validateChannel",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
jni.translate_error(res)?;
Ok(
()
)}
#[deprecated]

	pub fn validate_and_correct_channel(jni: &blackboxmc_general::SharedJNIEnv<'mc>,channel: impl Into<String>) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LString;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(channel.into())?));
let cls = jni.find_class("org/bukkit/plugin/messaging/StandardMessenger"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"validateAndCorrectChannel",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
jni.get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn validate_plugin_message(jni: &blackboxmc_general::SharedJNIEnv<'mc>,messenger: impl Into<crate::plugin::messaging::Messenger<'mc>>,source: impl Into<crate::plugin::Plugin<'mc>>,channel: impl Into<String>,message: i8) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/messaging/Messenger;Lorg/bukkit/plugin/Plugin;Ljava/lang/String;B)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(messenger.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(source.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(channel.into())?));
let val_4 = jni::objects::JValueGen::Byte(message);
let cls = jni.find_class("org/bukkit/plugin/messaging/StandardMessenger"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"validatePluginMessage",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
jni.translate_error(res)?;
Ok(
()
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::plugin::messaging::Messenger<'mc>> for StandardMessenger<'mc>{

fn into(self) -> crate::plugin::messaging::Messenger<'mc> {

crate::plugin::messaging::Messenger::from_raw(&self.jni_ref(), self.1).expect("Error converting StandardMessenger into crate::plugin::messaging::Messenger")

   }
}
