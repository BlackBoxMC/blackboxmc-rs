#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct ServiceRegisterEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ServiceRegisterEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ServiceRegisterEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ServiceRegisterEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/server/ServiceRegisterEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ServiceRegisterEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ServiceRegisterEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        registered_provider: impl Into<crate::plugin::RegisteredServiceProvider<'mc>>,
    ) -> Result<crate::event::server::ServiceRegisterEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/RegisteredServiceProvider;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(registered_provider.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/server/ServiceRegisterEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::server::ServiceRegisterEvent::from_raw(&jni, res)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/server/ServiceRegisterEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.server.ServiceEvent ( ['getHandlers', 'getHandlerList'])

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::server::ServiceEvent<'mc>> for ServiceRegisterEvent<'mc> {
    fn into(self) -> crate::event::server::ServiceEvent<'mc> {
        crate::event::server::ServiceEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ServiceRegisterEvent into crate::event::server::ServiceEvent")
    }
}
#[repr(C)]
pub struct RemoteServerCommandEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for RemoteServerCommandEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RemoteServerCommandEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate RemoteServerCommandEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/server/RemoteServerCommandEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RemoteServerCommandEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> RemoteServerCommandEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        sender: impl Into<crate::command::CommandSender<'mc>>,
        command: impl Into<String>,
    ) -> Result<crate::event::server::RemoteServerCommandEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(command.into())?,
        ));
        let cls = jni.find_class("org/bukkit/event/server/RemoteServerCommandEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::server::RemoteServerCommandEvent::from_raw(&jni, res)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/server/RemoteServerCommandEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.server.ServerCommandEvent ( ['getHandlers', 'getHandlerList'])
    /// Gets the command that the user is attempting to execute from the
    /// console
    pub fn command(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::server::ServerCommandEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::server::ServerCommandEvent = temp_clone.into();
        real.command()
    }
    /// Sets the command that the server will execute
    pub fn set_command(
        &self,
        message: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::server::ServerCommandEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::server::ServerCommandEvent = temp_clone.into();
        real.set_command(message)
    }
    /// Get the command sender.
    pub fn sender(&self) -> Result<crate::command::CommandSender<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::server::ServerCommandEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::server::ServerCommandEvent = temp_clone.into();
        real.sender()
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::server::ServerCommandEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::server::ServerCommandEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::server::ServerCommandEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::server::ServerCommandEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::server::ServerCommandEvent<'mc>> for RemoteServerCommandEvent<'mc> {
    fn into(self) -> crate::event::server::ServerCommandEvent<'mc> {
        crate::event::server::ServerCommandEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting RemoteServerCommandEvent into crate::event::server::ServerCommandEvent")
    }
}
#[repr(C)]
pub struct ServiceUnregisterEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ServiceUnregisterEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ServiceUnregisterEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ServiceUnregisterEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/server/ServiceUnregisterEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ServiceUnregisterEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ServiceUnregisterEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        service_provider: impl Into<crate::plugin::RegisteredServiceProvider<'mc>>,
    ) -> Result<crate::event::server::ServiceUnregisterEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/RegisteredServiceProvider;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(service_provider.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/server/ServiceUnregisterEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::server::ServiceUnregisterEvent::from_raw(&jni, res)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/server/ServiceUnregisterEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.server.ServiceEvent ( ['getHandlers', 'getHandlerList'])

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::server::ServiceEvent<'mc>> for ServiceUnregisterEvent<'mc> {
    fn into(self) -> crate::event::server::ServiceEvent<'mc> {
        crate::event::server::ServiceEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting ServiceUnregisterEvent into crate::event::server::ServiceEvent",
        )
    }
}
pub enum ServerLoadEventLoadType<'mc> {
    Startup {
        inner: ServerLoadEventLoadTypeStruct<'mc>,
    },
    Reload {
        inner: ServerLoadEventLoadTypeStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for ServerLoadEventLoadType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerLoadEventLoadType::Startup { .. } => f.write_str("STARTUP"),
            ServerLoadEventLoadType::Reload { .. } => f.write_str("RELOAD"),
        }
    }
}

impl<'mc> ServerLoadEventLoadType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ServerLoadEventLoadType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/server/ServerLoadEvent/LoadType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/server/ServerLoadEvent/LoadType;",
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
            "STARTUP" => Ok(ServerLoadEventLoadType::Startup {
                inner: ServerLoadEventLoadTypeStruct::from_raw(env, obj)?,
            }),
            "RELOAD" => Ok(ServerLoadEventLoadType::Reload {
                inner: ServerLoadEventLoadTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ServerLoadEventLoadTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ServerLoadEventLoadType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Startup { inner } => inner.0.clone(),
            Self::Reload { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Startup { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Reload { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ServerLoadEventLoadType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ServerLoadEventLoadType from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/server/ServerLoadEvent/LoadType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ServerLoadEventLoadType object, got {}",
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
                "STARTUP" => Ok(ServerLoadEventLoadType::Startup {
                    inner: ServerLoadEventLoadTypeStruct::from_raw(env, obj)?,
                }),
                "RELOAD" => Ok(ServerLoadEventLoadType::Reload {
                    inner: ServerLoadEventLoadTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ServerLoadEventLoadTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ServerLoadEventLoadTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ServerLoadEventLoadTypeStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/server/ServerLoadEvent/LoadType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ServerLoadEventLoadTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ServerLoadEventLoadTypeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::server::ServerLoadEventLoadType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/server/ServerLoadEvent/LoadType;");
        let cls = jni.find_class("org/bukkit/event/server/ServerLoadEvent/LoadType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::server::ServerLoadEventLoadType::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct ServerListPingEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ServerListPingEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ServerListPingEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ServerListPingEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/server/ServerListPingEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ServerListPingEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ServerListPingEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        hostname: impl Into<String>,
        address: jni::objects::JObject<'mc>,
        motd: impl Into<String>,
        num_players: i32,
        max_players: i32,
    ) -> Result<crate::event::server::ServerListPingEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/net/InetAddress;Ljava/lang/String;II)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(hostname.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(address);
        let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(motd.into())?,
        ));
        let val_4 = jni::objects::JValueGen::Int(num_players);
        let val_5 = jni::objects::JValueGen::Int(max_players);
        let cls = jni.find_class("org/bukkit/event/server/ServerListPingEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::server::ServerListPingEvent::from_raw(&jni, res)
    }
    /// Gets the hostname that the player used to connect to the server, or
    /// blank if unknown
    pub fn hostname(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHostname", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Get the address the ping is coming from.
    pub fn address(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/net/InetAddress;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAddress", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    /// Get the message of the day message.
    pub fn motd(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMotd", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Change the message of the day message.
    pub fn set_motd(&self, motd: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(motd.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMotd",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the number of players sent.
    pub fn num_players(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNumPlayers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the maximum number of players sent.
    pub fn max_players(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxPlayers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    #[deprecated]
    /// Gets whether the server needs to send a preview of the chat to the client.
    pub fn should_send_chat_previews(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldSendChatPreviews",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Set the maximum number of players sent.
    pub fn set_max_players(&self, max_players: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(max_players);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxPlayers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the server-icon sent to the client.
    pub fn set_server_icon(
        &self,
        icon: impl Into<crate::util::CachedServerIcon<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/CachedServerIcon;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(icon.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setServerIcon",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/server/ServerListPingEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    /// {@inheritDoc}
    ///
    /// Calling the {@link Iterator#remove()} method will force that particular
    /// player to not be displayed on the player list, decrease the size
    /// returned by {@link #getNumPlayers()}, and will not be returned again by
    /// any new iterator.
    ///
    /// <b>Note:</b> The players here will not be shown in the server info if
    /// {@link Bukkit#getHideOnlinePlayers()} is true.
    pub fn iterator(
        &self,
    ) -> Result<blackboxmc_java::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.server.ServerEvent ( ['getHostname', 'getAddress', 'getMotd', 'setMotd', 'getNumPlayers', 'getMaxPlayers', 'shouldSendChatPreviews', 'setMaxPlayers', 'setServerIcon', 'getHandlers', 'getHandlerList', 'iterator'])

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::server::ServerEvent<'mc>> for ServerListPingEvent<'mc> {
    fn into(self) -> crate::event::server::ServerEvent<'mc> {
        crate::event::server::ServerEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ServerListPingEvent into crate::event::server::ServerEvent")
    }
}
#[repr(C)]
pub struct ServerEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ServerEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ServerEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ServerEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/server/ServerEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ServerEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ServerEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        is_async: std::option::Option<bool>,
    ) -> Result<crate::event::server::ServerEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = is_async {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/server/ServerEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::server::ServerEvent::from_raw(&jni, res)
    }
    // SUPER CLASS: org.bukkit.event.Event ( [])
    /// Convenience method for providing a user-friendly identifier. By
    /// default, it is the event's class's {@linkplain Class#getSimpleName()
    /// simple name}.
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
    }
    /// Any custom event that should not by synchronized with other events must
    /// use the specific constructor. These are the caveats of using an
    /// asynchronous event:
    /// <ul>
    /// <li>The event is never fired from inside code triggered by a
    /// synchronous event. Attempting to do so results in an {@link
    /// java.lang.IllegalStateException}.
    /// <li>However, asynchronous event handlers may fire synchronous or
    /// asynchronous events
    /// <li>The event may be fired multiple times simultaneously and in any
    /// order.
    /// <li>Any newly registered or unregistered handler is ignored after an
    /// event starts execution.
    /// <li>The handlers for this event may block for any length of time.
    /// <li>Some implementations may selectively declare a specific event use
    /// as asynchronous. This behavior should be clearly defined.
    /// <li>Asynchronous calls are not calculated in the plugin timing system.
    /// </ul>
    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Event<'mc>> for ServerEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ServerEvent into crate::event::Event")
    }
}
#[repr(C)]
pub struct ServerLoadEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ServerLoadEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ServerLoadEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ServerLoadEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/server/ServerLoadEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ServerLoadEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ServerLoadEvent<'mc> {
    /// Creates a {@code ServerLoadEvent} with a given loading type.
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        val_type: impl Into<crate::event::server::ServerLoadEventLoadType<'mc>>,
    ) -> Result<crate::event::server::ServerLoadEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/event/server/ServerLoadEvent/LoadType;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/server/ServerLoadEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::server::ServerLoadEvent::from_raw(&jni, res)
    }
    /// Gets the context in which the server was loaded.
    pub fn get_type(
        &self,
    ) -> Result<crate::event::server::ServerLoadEventLoadType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/server/ServerLoadEvent/LoadType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::server::ServerLoadEventLoadType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/server/ServerLoadEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.server.ServerEvent ( ['getType', 'getHandlers', 'getHandlerList'])

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::server::ServerEvent<'mc>> for ServerLoadEvent<'mc> {
    fn into(self) -> crate::event::server::ServerEvent<'mc> {
        crate::event::server::ServerEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ServerLoadEvent into crate::event::server::ServerEvent")
    }
}
#[repr(C)]
pub struct TabCompleteEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TabCompleteEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TabCompleteEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate TabCompleteEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/server/TabCompleteEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TabCompleteEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TabCompleteEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        sender: impl Into<crate::command::CommandSender<'mc>>,
        buffer: impl Into<String>,
        completions: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<crate::event::server::TabCompleteEvent<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(buffer.into())?,
        ));
        let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in completions {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_3,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_3 = jni::objects::JValueGen::Object(raw_val_3);
        let cls = jni.find_class("org/bukkit/event/server/TabCompleteEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::server::TabCompleteEvent::from_raw(&jni, res)
    }
    /// Get the sender completing this command.
    pub fn sender(&self) -> Result<crate::command::CommandSender<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/command/CommandSender;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSender", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::command::CommandSender::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Return the entire buffer which formed the basis of this completion.
    pub fn buffer(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBuffer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// The list of completions which will be offered to the sender, in order.
    /// This list is mutable and reflects what will be offered.
    pub fn completions(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCompletions", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Set the completions offered, overriding any already set.
    pub fn set_completions(
        &self,
        completions: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)V");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in completions {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCompletions",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancelled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancelled.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/server/TabCompleteEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.Event ( ['getSender', 'getBuffer', 'getCompletions', 'setCompletions', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Convenience method for providing a user-friendly identifier. By
    /// default, it is the event's class's {@linkplain Class#getSimpleName()
    /// simple name}.
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
    }
    /// Any custom event that should not by synchronized with other events must
    /// use the specific constructor. These are the caveats of using an
    /// asynchronous event:
    /// <ul>
    /// <li>The event is never fired from inside code triggered by a
    /// synchronous event. Attempting to do so results in an {@link
    /// java.lang.IllegalStateException}.
    /// <li>However, asynchronous event handlers may fire synchronous or
    /// asynchronous events
    /// <li>The event may be fired multiple times simultaneously and in any
    /// order.
    /// <li>Any newly registered or unregistered handler is ignored after an
    /// event starts execution.
    /// <li>The handlers for this event may block for any length of time.
    /// <li>Some implementations may selectively declare a specific event use
    /// as asynchronous. This behavior should be clearly defined.
    /// <li>Asynchronous calls are not calculated in the plugin timing system.
    /// </ul>
    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for TabCompleteEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TabCompleteEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::Event<'mc>> for TabCompleteEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TabCompleteEvent into crate::event::Event")
    }
}
#[repr(C)]
pub struct ServerCommandEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ServerCommandEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ServerCommandEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ServerCommandEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/server/ServerCommandEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ServerCommandEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ServerCommandEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        sender: impl Into<crate::command::CommandSender<'mc>>,
        command: impl Into<String>,
    ) -> Result<crate::event::server::ServerCommandEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(command.into())?,
        ));
        let cls = jni.find_class("org/bukkit/event/server/ServerCommandEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::server::ServerCommandEvent::from_raw(&jni, res)
    }
    /// Gets the command that the user is attempting to execute from the
    /// console
    pub fn command(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCommand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the command that the server will execute
    pub fn set_command(
        &self,
        message: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(message.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCommand",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the command sender.
    pub fn sender(&self) -> Result<crate::command::CommandSender<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/command/CommandSender;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSender", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::command::CommandSender::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/server/ServerCommandEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: org.bukkit.event.server.ServerEvent ( ['getCommand', 'setCommand', 'getSender', 'getHandlers', 'getHandlerList', 'isCancelled', 'setCancelled'])

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for ServerCommandEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ServerCommandEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::server::ServerEvent<'mc>> for ServerCommandEvent<'mc> {
    fn into(self) -> crate::event::server::ServerEvent<'mc> {
        crate::event::server::ServerEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ServerCommandEvent into crate::event::server::ServerEvent")
    }
}
#[repr(C)]
pub struct ServiceEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ServiceEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ServiceEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ServiceEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/server/ServiceEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ServiceEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ServiceEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        provider: impl Into<crate::plugin::RegisteredServiceProvider<'mc>>,
    ) -> Result<crate::event::server::ServiceEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/RegisteredServiceProvider;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(provider.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/server/ServiceEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::server::ServiceEvent::from_raw(&jni, res)
    }
    // SUPER CLASS: org.bukkit.event.server.ServerEvent ( ['getProvider'])

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::server::ServerEvent<'mc>> for ServiceEvent<'mc> {
    fn into(self) -> crate::event::server::ServerEvent<'mc> {
        crate::event::server::ServerEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ServiceEvent into crate::event::server::ServerEvent")
    }
}
#[repr(C)]
pub struct PluginEnableEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PluginEnableEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PluginEnableEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PluginEnableEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/server/PluginEnableEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginEnableEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PluginEnableEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<crate::event::server::PluginEnableEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(plugin.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/server/PluginEnableEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::server::PluginEnableEvent::from_raw(&jni, res)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/server/PluginEnableEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.server.PluginEvent ( ['getHandlers', 'getHandlerList'])
    /// Gets the plugin involved in this event
    pub fn plugin(&self) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::server::PluginEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::server::PluginEvent = temp_clone.into();
        real.plugin()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::server::PluginEvent<'mc>> for PluginEnableEvent<'mc> {
    fn into(self) -> crate::event::server::PluginEvent<'mc> {
        crate::event::server::PluginEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PluginEnableEvent into crate::event::server::PluginEvent")
    }
}
#[repr(C)]
pub struct PluginEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PluginEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PluginEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PluginEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/server/PluginEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PluginEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<crate::event::server::PluginEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(plugin.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/server/PluginEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::server::PluginEvent::from_raw(&jni, res)
    }
    /// Gets the plugin involved in this event
    pub fn plugin(&self) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/plugin/Plugin;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlugin", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::Plugin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.server.ServerEvent ( ['getPlugin'])

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::server::ServerEvent<'mc>> for PluginEvent<'mc> {
    fn into(self) -> crate::event::server::ServerEvent<'mc> {
        crate::event::server::ServerEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PluginEvent into crate::event::server::ServerEvent")
    }
}
#[repr(C)]
pub struct PluginDisableEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PluginDisableEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PluginDisableEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PluginDisableEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/server/PluginDisableEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginDisableEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PluginDisableEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<crate::event::server::PluginDisableEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(plugin.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/server/PluginDisableEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::server::PluginDisableEvent::from_raw(&jni, res)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/server/PluginDisableEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.server.PluginEvent ( ['getHandlers', 'getHandlerList'])
    /// Gets the plugin involved in this event
    pub fn plugin(&self) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::server::PluginEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::server::PluginEvent = temp_clone.into();
        real.plugin()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::server::PluginEvent<'mc>> for PluginDisableEvent<'mc> {
    fn into(self) -> crate::event::server::PluginEvent<'mc> {
        crate::event::server::PluginEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PluginDisableEvent into crate::event::server::PluginEvent")
    }
}
#[repr(C)]
pub struct BroadcastMessageEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BroadcastMessageEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BroadcastMessageEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BroadcastMessageEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/server/BroadcastMessageEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BroadcastMessageEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BroadcastMessageEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        is_async: bool,
        message: impl Into<String>,
        recipients: std::option::Option<impl Into<blackboxmc_java::util::JavaSet<'mc>>>,
    ) -> Result<crate::event::server::BroadcastMessageEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(is_async.into());
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(message.into())?,
        ));
        args.push(val_2);
        if let Some(a) = recipients {
            sig += "Ljava/util/Set;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/server/BroadcastMessageEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::server::BroadcastMessageEvent::from_raw(&jni, res)
    }
    /// Get the message to broadcast.
    pub fn message(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMessage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Set the message to broadcast.
    pub fn set_message(
        &self,
        message: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(message.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMessage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a set of recipients that this chat message will be displayed to.
    ///
    /// The set returned is not guaranteed to be mutable and may auto-populate
    /// on access. Any listener accessing the returned set should be aware that
    /// it may reduce performance for a lazy set implementation.
    ///
    /// Listeners should be aware that modifying the list may throw {@link
    /// UnsupportedOperationException} if the event caller provides an
    /// unmodifiable set.
    pub fn recipients(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRecipients", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancelled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancelled.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/server/BroadcastMessageEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.server.ServerEvent ( ['getMessage', 'setMessage', 'getRecipients', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BroadcastMessageEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BroadcastMessageEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::server::ServerEvent<'mc>> for BroadcastMessageEvent<'mc> {
    fn into(self) -> crate::event::server::ServerEvent<'mc> {
        crate::event::server::ServerEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BroadcastMessageEvent into crate::event::server::ServerEvent")
    }
}
#[repr(C)]
pub struct MapInitializeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MapInitializeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MapInitializeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MapInitializeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/server/MapInitializeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapInitializeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MapInitializeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        map_view: impl Into<crate::map::MapView<'mc>>,
    ) -> Result<crate::event::server::MapInitializeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/map/MapView;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(map_view.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/server/MapInitializeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::server::MapInitializeEvent::from_raw(&jni, res)
    }
    /// Gets the map initialized in this event.
    pub fn map(&self) -> Result<crate::map::MapView<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/map/MapView;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMap", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::map::MapView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/server/MapInitializeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.server.ServerEvent ( ['getMap', 'getHandlers', 'getHandlerList'])

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::server::ServerEvent<'mc>> for MapInitializeEvent<'mc> {
    fn into(self) -> crate::event::server::ServerEvent<'mc> {
        crate::event::server::ServerEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MapInitializeEvent into crate::event::server::ServerEvent")
    }
}
