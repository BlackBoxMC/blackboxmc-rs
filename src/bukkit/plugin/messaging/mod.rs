#![allow(deprecated)]
use crate::JNIRaw;
pub enum PluginChannelDirectionEnum {
    Incoming,
    Outgoing,
}
impl std::fmt::Display for PluginChannelDirectionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PluginChannelDirectionEnum::Incoming => f.write_str("INCOMING"),
            PluginChannelDirectionEnum::Outgoing => f.write_str("OUTGOING"),
        }
    }
}
pub struct PluginChannelDirection<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub PluginChannelDirectionEnum,
);
impl<'mc> std::ops::Deref for PluginChannelDirection<'mc> {
    type Target = PluginChannelDirectionEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for PluginChannelDirection<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PluginChannelDirection<'mc> {
    pub const INCOMING: PluginChannelDirectionEnum = PluginChannelDirectionEnum::Incoming;
    pub const OUTGOING: PluginChannelDirectionEnum = PluginChannelDirectionEnum::Outgoing;
    pub fn from_string(str: String) -> std::option::Option<PluginChannelDirectionEnum> {
        match str.as_str() {
            "INCOMING" => Some(PluginChannelDirectionEnum::Incoming),
            "OUTGOING" => Some(PluginChannelDirectionEnum::Outgoing),
            _ => None,
        }
    }
    pub fn value_of(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<
        crate::bukkit::plugin::messaging::PluginChannelDirection<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("org/bukkit/plugin/messaging/PluginChannelDirection")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/plugin/messaging/PluginChannelDirection;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::plugin::messaging::PluginChannelDirection(
                jni,
                raw_obj,
                crate::bukkit::plugin::messaging::PluginChannelDirection::from_string(variant_str)
                    .unwrap(),
            )
        };
        Ok(ret)
    }
}
/// An instantiatable struct that implements PluginMessageListener. Needed for returning it from Java.
pub struct PluginMessageListener<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PluginMessageListener<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PluginMessageListener from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PluginMessageListener") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginMessageListener object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn on_plugin_message_received(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::bukkit::entity::Player<'mc>>,
        _arg2: Vec<i8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "onPluginMessageReceived",
            "(Ljava/lang/String;Lorg/bukkit/entity/Player;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for PluginMessageListener<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct PluginMessageListenerRegistration<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for PluginMessageListenerRegistration<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PluginMessageListenerRegistration<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::plugin::messaging::Messenger<'mc>>,
        arg1: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
        arg2: impl Into<String>,
        arg3: impl Into<crate::bukkit::plugin::messaging::PluginMessageListener<'mc>>,
    ) -> Result<
        crate::bukkit::plugin::messaging::PluginMessageListenerRegistration<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let val_2 = jni::objects::JObject::from(jni.new_string(arg2.into()).unwrap());
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().1.clone()) };
        let cls =
            &jni.find_class("org/bukkit/plugin/messaging/PluginMessageListenerRegistration")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/plugin/messaging/Messenger;Lorg/bukkit/plugin/Plugin;Ljava/lang/String;Lorg/bukkit/plugin/messaging/PluginMessageListener;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        let ret = { crate::bukkit::plugin::messaging::PluginMessageListenerRegistration(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PluginMessageListenerRegistration from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PluginMessageListenerRegistration") {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a PluginMessageListenerRegistration object, got {}",
        name
    )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn is_valid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn channel(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getChannel",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn listener(
        &mut self,
    ) -> Result<
        crate::bukkit::plugin::messaging::PluginMessageListener<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getListener",
            "()Lorg/bukkit/plugin/messaging/PluginMessageListener;",
            &[],
        )?;
        let ret = {
            crate::bukkit::plugin::messaging::PluginMessageListener(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn plugin(
        &mut self,
    ) -> Result<crate::bukkit::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlugin",
            "()Lorg/bukkit/plugin/Plugin;",
            &[],
        )?;
        let ret = {
            crate::bukkit::plugin::Plugin(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
/// An instantiatable struct that implements Messenger. Needed for returning it from Java.
pub struct Messenger<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Messenger<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Messenger from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Messenger") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Messenger object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn is_reserved_channel(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isReservedChannel",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn register_outgoing_plugin_channel(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "registerOutgoingPluginChannel",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn unregister_outgoing_plugin_channel_with_plugin(
        &mut self,
        arg0: std::option::Option<impl Into<crate::bukkit::plugin::Plugin<'mc>>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "unregisterOutgoingPluginChannel",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn register_incoming_plugin_channel(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
        arg2: impl Into<crate::bukkit::plugin::messaging::PluginMessageListener<'mc>>,
    ) -> Result<
        crate::bukkit::plugin::messaging::PluginMessageListenerRegistration<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().1.clone()) };
        let res =
self.jni_ref().call_method(&self.jni_object(),"registerIncomingPluginChannel","(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;Lorg/bukkit/plugin/messaging/PluginMessageListener;)Lorg/bukkit/plugin/messaging/PluginMessageListenerRegistration;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        let ret = {
            crate::bukkit::plugin::messaging::PluginMessageListenerRegistration(
                self.jni_ref(),
                unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
            )
        };
        Ok(ret)
    }
    pub fn unregister_incoming_plugin_channel_with_plugin(
        &mut self,
        arg0: std::option::Option<impl Into<crate::bukkit::plugin::Plugin<'mc>>>,
        arg1: std::option::Option<impl Into<String>>,
        arg2: std::option::Option<
            impl Into<crate::bukkit::plugin::messaging::PluginMessageListener<'mc>>,
        >,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().1.clone()) };
        self.jni_ref().call_method(&self.jni_object(),"unregisterIncomingPluginChannel","(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;Lorg/bukkit/plugin/messaging/PluginMessageListener;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        Ok(())
    }
    pub fn is_registration_valid(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::messaging::PluginMessageListenerRegistration<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isRegistrationValid",
            "(Lorg/bukkit/plugin/messaging/PluginMessageListenerRegistration;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_incoming_channel_registered(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isIncomingChannelRegistered",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_outgoing_channel_registered(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isOutgoingChannelRegistered",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn dispatch_incoming_message(
        &mut self,
        arg0: impl Into<crate::bukkit::entity::Player<'mc>>,
        arg1: impl Into<String>,
        _arg2: Vec<i8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "dispatchIncomingMessage",
            "(Lorg/bukkit/entity/Player;Ljava/lang/String;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for Messenger<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct StandardMessenger<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for StandardMessenger<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> StandardMessenger<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<crate::bukkit::plugin::messaging::StandardMessenger<'mc>, Box<dyn std::error::Error>>
    {
        let cls = &jni.find_class("org/bukkit/plugin/messaging/StandardMessenger")?;
        let res = jni.new_object(cls, "()V", &[])?;
        let ret = { crate::bukkit::plugin::messaging::StandardMessenger(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate StandardMessenger from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("StandardMessenger") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StandardMessenger object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    #[deprecated]
    pub fn validate_and_correct_channel(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "validateAndCorrectChannel",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn is_reserved_channel(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isReservedChannel",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn register_outgoing_plugin_channel(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "registerOutgoingPluginChannel",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn unregister_outgoing_plugin_channel_with_plugin(
        &mut self,
        arg0: std::option::Option<impl Into<crate::bukkit::plugin::Plugin<'mc>>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "unregisterOutgoingPluginChannel",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn register_incoming_plugin_channel(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
        arg2: impl Into<crate::bukkit::plugin::messaging::PluginMessageListener<'mc>>,
    ) -> Result<
        crate::bukkit::plugin::messaging::PluginMessageListenerRegistration<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().1.clone()) };
        let res =
self.jni_ref().call_method(&self.jni_object(),"registerIncomingPluginChannel","(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;Lorg/bukkit/plugin/messaging/PluginMessageListener;)Lorg/bukkit/plugin/messaging/PluginMessageListenerRegistration;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        let ret = {
            crate::bukkit::plugin::messaging::PluginMessageListenerRegistration(
                self.jni_ref(),
                unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
            )
        };
        Ok(ret)
    }
    pub fn unregister_incoming_plugin_channel_with_plugin(
        &mut self,
        arg0: std::option::Option<impl Into<crate::bukkit::plugin::Plugin<'mc>>>,
        arg1: std::option::Option<impl Into<String>>,
        arg2: std::option::Option<
            impl Into<crate::bukkit::plugin::messaging::PluginMessageListener<'mc>>,
        >,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().1.clone()) };
        self.jni_ref().call_method(&self.jni_object(),"unregisterIncomingPluginChannel","(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;Lorg/bukkit/plugin/messaging/PluginMessageListener;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        Ok(())
    }
    pub fn is_registration_valid(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::messaging::PluginMessageListenerRegistration<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isRegistrationValid",
            "(Lorg/bukkit/plugin/messaging/PluginMessageListenerRegistration;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_incoming_channel_registered(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isIncomingChannelRegistered",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_outgoing_channel_registered(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isOutgoingChannelRegistered",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn dispatch_incoming_message(
        &mut self,
        arg0: impl Into<crate::bukkit::entity::Player<'mc>>,
        arg1: impl Into<String>,
        _arg2: Vec<i8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "dispatchIncomingMessage",
            "(Lorg/bukkit/entity/Player;Ljava/lang/String;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    #[deprecated]
    pub fn validate_channel(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("void")?;
        let _res = jni.call_static_method(
            cls,
            "validateChannel",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn validate_plugin_message(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::plugin::messaging::Messenger<'mc>>,
        arg1: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
        arg2: impl Into<String>,
        _arg3: Vec<i8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let val_2 = jni::objects::JObject::from(jni.new_string(arg2.into()).unwrap());
        let cls = &jni.find_class("void")?;
        let _res = jni.call_static_method(cls,"validatePluginMessage",
"(Lorg/bukkit/plugin/messaging/Messenger;Lorg/bukkit/plugin/Plugin;Ljava/lang/String;B)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        Ok(())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::plugin::messaging::Messenger<'mc>> for StandardMessenger<'mc> {
    fn into(self) -> crate::bukkit::plugin::messaging::Messenger<'mc> {
        crate::bukkit::plugin::messaging::Messenger::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements PluginMessageRecipient. Needed for returning it from Java.
pub struct PluginMessageRecipient<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PluginMessageRecipient<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PluginMessageRecipient from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PluginMessageRecipient") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginMessageRecipient object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn send_plugin_message(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
        _arg2: Vec<i8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "sendPluginMessage",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for PluginMessageRecipient<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
