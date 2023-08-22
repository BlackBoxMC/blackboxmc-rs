#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Stores data for lightning striking
#[repr(C)]
pub struct LightningStrikeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum LightningStrikeEventCause<'mc> {
    Command {
        inner: LightningStrikeEventCauseStruct<'mc>,
    },
    Custom {
        inner: LightningStrikeEventCauseStruct<'mc>,
    },
    Spawner {
        inner: LightningStrikeEventCauseStruct<'mc>,
    },
    Trident {
        inner: LightningStrikeEventCauseStruct<'mc>,
    },
    Trap {
        inner: LightningStrikeEventCauseStruct<'mc>,
    },
    Weather {
        inner: LightningStrikeEventCauseStruct<'mc>,
    },
    Unknown {
        inner: LightningStrikeEventCauseStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for LightningStrikeEventCause<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LightningStrikeEventCause::Command { .. } => f.write_str("COMMAND"),
            LightningStrikeEventCause::Custom { .. } => f.write_str("CUSTOM"),
            LightningStrikeEventCause::Spawner { .. } => f.write_str("SPAWNER"),
            LightningStrikeEventCause::Trident { .. } => f.write_str("TRIDENT"),
            LightningStrikeEventCause::Trap { .. } => f.write_str("TRAP"),
            LightningStrikeEventCause::Weather { .. } => f.write_str("WEATHER"),
            LightningStrikeEventCause::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}

impl<'mc> LightningStrikeEventCause<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<LightningStrikeEventCause<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/weather/LightningStrikeEvent$Cause");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/weather/LightningStrikeEvent$Cause;",
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
            "COMMAND" => Ok(LightningStrikeEventCause::Command {
                inner: LightningStrikeEventCauseStruct::from_raw(env, obj)?,
            }),
            "CUSTOM" => Ok(LightningStrikeEventCause::Custom {
                inner: LightningStrikeEventCauseStruct::from_raw(env, obj)?,
            }),
            "SPAWNER" => Ok(LightningStrikeEventCause::Spawner {
                inner: LightningStrikeEventCauseStruct::from_raw(env, obj)?,
            }),
            "TRIDENT" => Ok(LightningStrikeEventCause::Trident {
                inner: LightningStrikeEventCauseStruct::from_raw(env, obj)?,
            }),
            "TRAP" => Ok(LightningStrikeEventCause::Trap {
                inner: LightningStrikeEventCauseStruct::from_raw(env, obj)?,
            }),
            "WEATHER" => Ok(LightningStrikeEventCause::Weather {
                inner: LightningStrikeEventCauseStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(LightningStrikeEventCause::Unknown {
                inner: LightningStrikeEventCauseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct LightningStrikeEventCauseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LightningStrikeEventCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Command { inner } => inner.0.clone(),
            Self::Custom { inner } => inner.0.clone(),
            Self::Spawner { inner } => inner.0.clone(),
            Self::Trident { inner } => inner.0.clone(),
            Self::Trap { inner } => inner.0.clone(),
            Self::Weather { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Command { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Spawner { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Trident { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Trap { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Weather { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LightningStrikeEventCause<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate LightningStrikeEventCause from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/weather/LightningStrikeEvent$Cause")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LightningStrikeEventCause object, got {}",
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
                "COMMAND" => Ok(LightningStrikeEventCause::Command {
                    inner: LightningStrikeEventCauseStruct::from_raw(env, obj)?,
                }),
                "CUSTOM" => Ok(LightningStrikeEventCause::Custom {
                    inner: LightningStrikeEventCauseStruct::from_raw(env, obj)?,
                }),
                "SPAWNER" => Ok(LightningStrikeEventCause::Spawner {
                    inner: LightningStrikeEventCauseStruct::from_raw(env, obj)?,
                }),
                "TRIDENT" => Ok(LightningStrikeEventCause::Trident {
                    inner: LightningStrikeEventCauseStruct::from_raw(env, obj)?,
                }),
                "TRAP" => Ok(LightningStrikeEventCause::Trap {
                    inner: LightningStrikeEventCauseStruct::from_raw(env, obj)?,
                }),
                "WEATHER" => Ok(LightningStrikeEventCause::Weather {
                    inner: LightningStrikeEventCauseStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(LightningStrikeEventCause::Unknown {
                    inner: LightningStrikeEventCauseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for LightningStrikeEventCauseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LightningStrikeEventCauseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate LightningStrikeEventCauseStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/weather/LightningStrikeEvent$Cause")?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a LightningStrikeEventCauseStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LightningStrikeEventCauseStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        Vec<crate::event::weather::LightningStrikeEventCause<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/weather/LightningStrikeEvent$Cause;");
        let cls = jni.find_class("org/bukkit/event/weather/LightningStrikeEvent$Cause");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::event::weather::LightningStrikeEventCause::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for LightningStrikeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LightningStrikeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LightningStrikeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/weather/LightningStrikeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LightningStrikeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LightningStrikeEvent<'mc> {
    pub fn new_with_world(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::World<'mc>>,
        arg1: impl Into<crate::entity::LightningStrike<'mc>>,
        arg2: std::option::Option<impl Into<crate::event::weather::LightningStrikeEventCause<'mc>>>,
    ) -> Result<crate::event::weather::LightningStrikeEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/World;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/LightningStrike;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/event/weather/LightningStrikeEvent$Cause;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/weather/LightningStrikeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::weather::LightningStrikeEvent::from_raw(&jni, res)
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

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
    /// Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn lightning(
        &self,
    ) -> Result<crate::entity::LightningStrike<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LightningStrike;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLightning", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LightningStrike::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn cause(
        &self,
    ) -> Result<crate::event::weather::LightningStrikeEventCause<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/weather/LightningStrikeEvent$Cause;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCause", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::weather::LightningStrikeEventCause::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: WeatherEvent
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::weather::WeatherEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::weather::WeatherEvent = temp_clone.into();
        real.world()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
    }
    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for LightningStrikeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LightningStrikeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::weather::WeatherEvent<'mc>> for LightningStrikeEvent<'mc> {
    fn into(self) -> crate::event::weather::WeatherEvent<'mc> {
        crate::event::weather::WeatherEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting LightningStrikeEvent into crate::event::weather::WeatherEvent",
        )
    }
}
/// Stores data for thunder state changing in a world
#[repr(C)]
pub struct ThunderChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ThunderChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ThunderChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ThunderChangeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/weather/ThunderChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ThunderChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ThunderChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::World<'mc>>,
        arg1: bool,
    ) -> Result<crate::event::weather::ThunderChangeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let cls = jni.find_class("org/bukkit/event/weather/ThunderChangeEvent");
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
        crate::event::weather::ThunderChangeEvent::from_raw(&jni, res)
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

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
    /// Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn to_thunder_state(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toThunderState", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: WeatherEvent
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::weather::WeatherEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::weather::WeatherEvent = temp_clone.into();
        real.world()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
    }
    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for ThunderChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ThunderChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::weather::WeatherEvent<'mc>> for ThunderChangeEvent<'mc> {
    fn into(self) -> crate::event::weather::WeatherEvent<'mc> {
        crate::event::weather::WeatherEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ThunderChangeEvent into crate::event::weather::WeatherEvent")
    }
}
/// Stores data for weather changing in a world
#[repr(C)]
pub struct WeatherChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for WeatherChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for WeatherChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate WeatherChangeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/weather/WeatherChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WeatherChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> WeatherChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::World<'mc>>,
        arg1: bool,
    ) -> Result<crate::event::weather::WeatherChangeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let cls = jni.find_class("org/bukkit/event/weather/WeatherChangeEvent");
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
        crate::event::weather::WeatherChangeEvent::from_raw(&jni, res)
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

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
    /// Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn to_weather_state(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toWeatherState", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: WeatherEvent
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::weather::WeatherEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::weather::WeatherEvent = temp_clone.into();
        real.world()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
    }
    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for WeatherChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WeatherChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::weather::WeatherEvent<'mc>> for WeatherChangeEvent<'mc> {
    fn into(self) -> crate::event::weather::WeatherEvent<'mc> {
        crate::event::weather::WeatherEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WeatherChangeEvent into crate::event::weather::WeatherEvent")
    }
}
/// Represents a Weather-related event
#[repr(C)]
pub struct WeatherEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for WeatherEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for WeatherEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate WeatherEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/weather/WeatherEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WeatherEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> WeatherEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::World<'mc>>,
    ) -> Result<crate::event::weather::WeatherEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/weather/WeatherEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::weather::WeatherEvent::from_raw(&jni, res)
    }

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: Event
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
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
    }
    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Event<'mc>> for WeatherEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WeatherEvent into crate::event::Event")
    }
}
