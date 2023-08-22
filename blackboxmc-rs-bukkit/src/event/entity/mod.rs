#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub enum EntityUnleashEventUnleashReason<'mc> {
    HolderGone {
        inner: EntityUnleashEventUnleashReasonStruct<'mc>,
    },
    PlayerUnleash {
        inner: EntityUnleashEventUnleashReasonStruct<'mc>,
    },
    Distance {
        inner: EntityUnleashEventUnleashReasonStruct<'mc>,
    },
    Unknown {
        inner: EntityUnleashEventUnleashReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityUnleashEventUnleashReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityUnleashEventUnleashReason::HolderGone { .. } => f.write_str("HOLDER_GONE"),
            EntityUnleashEventUnleashReason::PlayerUnleash { .. } => f.write_str("PLAYER_UNLEASH"),
            EntityUnleashEventUnleashReason::Distance { .. } => f.write_str("DISTANCE"),
            EntityUnleashEventUnleashReason::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}

impl<'mc> EntityUnleashEventUnleashReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityUnleashEventUnleashReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityUnleashEvent$UnleashReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityUnleashEvent$UnleashReason;",
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
            "HOLDER_GONE" => Ok(EntityUnleashEventUnleashReason::HolderGone {
                inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
            }),
            "PLAYER_UNLEASH" => Ok(EntityUnleashEventUnleashReason::PlayerUnleash {
                inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
            }),
            "DISTANCE" => Ok(EntityUnleashEventUnleashReason::Distance {
                inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(EntityUnleashEventUnleashReason::Unknown {
                inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityUnleashEventUnleashReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityUnleashEventUnleashReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::HolderGone { inner } => inner.0.clone(),
            Self::PlayerUnleash { inner } => inner.0.clone(),
            Self::Distance { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::HolderGone { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PlayerUnleash { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Distance { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityUnleashEventUnleashReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityUnleashEventUnleashReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityUnleashEvent$UnleashReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityUnleashEventUnleashReason object, got {}",
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
                "HOLDER_GONE" => Ok(EntityUnleashEventUnleashReason::HolderGone {
                    inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
                }),
                "PLAYER_UNLEASH" => Ok(EntityUnleashEventUnleashReason::PlayerUnleash {
                    inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
                }),
                "DISTANCE" => Ok(EntityUnleashEventUnleashReason::Distance {
                    inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(EntityUnleashEventUnleashReason::Unknown {
                    inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityUnleashEventUnleashReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityUnleashEventUnleashReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityUnleashEventUnleashReasonStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityUnleashEvent$UnleashReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityUnleashEventUnleashReasonStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityUnleashEventUnleashReasonStruct<'mc> {
    //Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum PowerCause<'mc> {
    Lightning { inner: PowerCauseStruct<'mc> },
    SetOn { inner: PowerCauseStruct<'mc> },
    SetOff { inner: PowerCauseStruct<'mc> },
}
impl<'mc> std::fmt::Display for PowerCause<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PowerCause::Lightning { .. } => f.write_str("LIGHTNING"),
            PowerCause::SetOn { .. } => f.write_str("SET_ON"),
            PowerCause::SetOff { .. } => f.write_str("SET_OFF"),
        }
    }
}

impl<'mc> PowerCause<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<PowerCause<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/PowerCause");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/PowerCause;",
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
            "LIGHTNING" => Ok(PowerCause::Lightning {
                inner: PowerCauseStruct::from_raw(env, obj)?,
            }),
            "SET_ON" => Ok(PowerCause::SetOn {
                inner: PowerCauseStruct::from_raw(env, obj)?,
            }),
            "SET_OFF" => Ok(PowerCause::SetOff {
                inner: PowerCauseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct PowerCauseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PowerCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Lightning { inner } => inner.0.clone(),
            Self::SetOn { inner } => inner.0.clone(),
            Self::SetOff { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Lightning { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SetOn { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SetOff { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PowerCause<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PowerCause from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/PowerCause")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PowerCause object, got {}",
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
                "LIGHTNING" => Ok(PowerCause::Lightning {
                    inner: PowerCauseStruct::from_raw(env, obj)?,
                }),
                "SET_ON" => Ok(PowerCause::SetOn {
                    inner: PowerCauseStruct::from_raw(env, obj)?,
                }),
                "SET_OFF" => Ok(PowerCause::SetOff {
                    inner: PowerCauseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for PowerCauseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PowerCauseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PowerCauseStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/PowerCause")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PowerCauseStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PowerCauseStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Called when an entity explodes
#[repr(C)]
pub struct EntityExplodeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityExplodeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityExplodeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityExplodeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityExplodeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityExplodeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityExplodeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::Location<'mc>>,
        arg2: Vec<impl Into<crate::event::entity::EntityExplodeEvent<'mc>>>,
        arg3: f32,
    ) -> Result<crate::event::entity::EntityExplodeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/Location;Ljava/util/List;F)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg2 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            jni.call_method(
                &raw_val_3,
                "add",
                "(Lorg/bukkit/event/entity/crate::event::entity::EntityExplodeEvent)V",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_3 = jni::objects::JValueGen::Object(raw_val_3);
        let val_4 = jni::objects::JValueGen::Float(arg3);
        let cls = jni.find_class("org/bukkit/event/entity/EntityExplodeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityExplodeEvent::from_raw(&jni, res)
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
    /// Sets the percentage of blocks to drop from this explosion
    pub fn set_yield(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setYield",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_yield(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getYield", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
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

    pub fn block_list(&self) -> Result<Vec<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "blockList", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::block::Block::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityExplodeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityExplodeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityExplodeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityExplodeEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when a non-player entity is about to teleport because it is in contact with a portal.
/// <p>For players see <a href="../player/PlayerPortalEvent.html" title="class in org.bukkit.event.player"><code>PlayerPortalEvent</code></a></p>
#[repr(C)]
pub struct EntityPortalEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPortalEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPortalEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityPortalEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityPortalEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPortalEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPortalEvent<'mc> {
    pub fn new_with_entity(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::Location<'mc>>,
        arg2: impl Into<crate::Location<'mc>>,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::event::entity::EntityPortalEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/Location;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/Location;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        args.push(val_3);
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityPortalEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityPortalEvent::from_raw(&jni, res)
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
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    /// Set the Block radius to search in for available portals.
    pub fn set_search_radius(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSearchRadius",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn search_radius(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSearchRadius", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //EntityTeleportEvent
    //crate::event::entity::EntityTeleportEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
    }
    pub fn from(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.from()
    }
    pub fn set_from(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.set_from(arg0)
    }
    pub fn to(&self) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.to()
    }
    pub fn set_to(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.set_to(arg0)
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityTeleportEvent<'mc>> for EntityPortalEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityTeleportEvent<'mc> {
        crate::event::entity::EntityTeleportEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityPortalEvent into crate::event::entity::EntityTeleportEvent",
        )
    }
}
/// Called when a <a title="interface in org.bukkit.entity" href="../../entity/Strider.html"><code>Strider</code></a>'s temperature has changed as a result of entering or exiting blocks it considers warm.
#[repr(C)]
pub struct StriderTemperatureChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for StriderTemperatureChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for StriderTemperatureChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate StriderTemperatureChangeEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/StriderTemperatureChangeEvent",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StriderTemperatureChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> StriderTemperatureChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Strider<'mc>>,
        arg1: bool,
    ) -> Result<crate::event::entity::StriderTemperatureChangeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/Strider;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let cls = jni.find_class("org/bukkit/event/entity/StriderTemperatureChangeEvent");
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
        crate::event::entity::StriderTemperatureChangeEvent::from_raw(&jni, res)
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

    pub fn is_shivering(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isShivering", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn entity(&self) -> Result<crate::entity::Strider<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Strider;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Strider::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for StriderTemperatureChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting StriderTemperatureChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for StriderTemperatureChangeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting StriderTemperatureChangeEvent into crate::event::entity::EntityEvent",
        )
    }
}
/// Called when an entity is damaged by a block
#[repr(C)]
pub struct EntityDamageByBlockEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum EntityDamageEventDamageCause<'mc> {
    Kill {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    WorldBorder {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Contact {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    EntityAttack {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    EntitySweepAttack {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Projectile {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Suffocation {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Fall {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Fire {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    FireTick {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Melting {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Lava {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Drowning {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    BlockExplosion {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    EntityExplosion {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Void {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Lightning {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Suicide {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Starvation {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Poison {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Magic {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Wither {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    FallingBlock {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Thorns {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    DragonBreath {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Custom {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    FlyIntoWall {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    HotFloor {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Cramming {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Dryout {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Freeze {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    SonicBoom {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityDamageEventDamageCause<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityDamageEventDamageCause::Kill { .. } => f.write_str("KILL"),
            EntityDamageEventDamageCause::WorldBorder { .. } => f.write_str("WORLD_BORDER"),
            EntityDamageEventDamageCause::Contact { .. } => f.write_str("CONTACT"),
            EntityDamageEventDamageCause::EntityAttack { .. } => f.write_str("ENTITY_ATTACK"),
            EntityDamageEventDamageCause::EntitySweepAttack { .. } => {
                f.write_str("ENTITY_SWEEP_ATTACK")
            }
            EntityDamageEventDamageCause::Projectile { .. } => f.write_str("PROJECTILE"),
            EntityDamageEventDamageCause::Suffocation { .. } => f.write_str("SUFFOCATION"),
            EntityDamageEventDamageCause::Fall { .. } => f.write_str("FALL"),
            EntityDamageEventDamageCause::Fire { .. } => f.write_str("FIRE"),
            EntityDamageEventDamageCause::FireTick { .. } => f.write_str("FIRE_TICK"),
            EntityDamageEventDamageCause::Melting { .. } => f.write_str("MELTING"),
            EntityDamageEventDamageCause::Lava { .. } => f.write_str("LAVA"),
            EntityDamageEventDamageCause::Drowning { .. } => f.write_str("DROWNING"),
            EntityDamageEventDamageCause::BlockExplosion { .. } => f.write_str("BLOCK_EXPLOSION"),
            EntityDamageEventDamageCause::EntityExplosion { .. } => f.write_str("ENTITY_EXPLOSION"),
            EntityDamageEventDamageCause::Void { .. } => f.write_str("VOID"),
            EntityDamageEventDamageCause::Lightning { .. } => f.write_str("LIGHTNING"),
            EntityDamageEventDamageCause::Suicide { .. } => f.write_str("SUICIDE"),
            EntityDamageEventDamageCause::Starvation { .. } => f.write_str("STARVATION"),
            EntityDamageEventDamageCause::Poison { .. } => f.write_str("POISON"),
            EntityDamageEventDamageCause::Magic { .. } => f.write_str("MAGIC"),
            EntityDamageEventDamageCause::Wither { .. } => f.write_str("WITHER"),
            EntityDamageEventDamageCause::FallingBlock { .. } => f.write_str("FALLING_BLOCK"),
            EntityDamageEventDamageCause::Thorns { .. } => f.write_str("THORNS"),
            EntityDamageEventDamageCause::DragonBreath { .. } => f.write_str("DRAGON_BREATH"),
            EntityDamageEventDamageCause::Custom { .. } => f.write_str("CUSTOM"),
            EntityDamageEventDamageCause::FlyIntoWall { .. } => f.write_str("FLY_INTO_WALL"),
            EntityDamageEventDamageCause::HotFloor { .. } => f.write_str("HOT_FLOOR"),
            EntityDamageEventDamageCause::Cramming { .. } => f.write_str("CRAMMING"),
            EntityDamageEventDamageCause::Dryout { .. } => f.write_str("DRYOUT"),
            EntityDamageEventDamageCause::Freeze { .. } => f.write_str("FREEZE"),
            EntityDamageEventDamageCause::SonicBoom { .. } => f.write_str("SONIC_BOOM"),
        }
    }
}

impl<'mc> EntityDamageEventDamageCause<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityDamageEventDamageCause<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityDamageEvent$DamageCause");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityDamageEvent$DamageCause;",
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
            "KILL" => Ok(EntityDamageEventDamageCause::Kill {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "WORLD_BORDER" => Ok(EntityDamageEventDamageCause::WorldBorder {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "CONTACT" => Ok(EntityDamageEventDamageCause::Contact {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "ENTITY_ATTACK" => Ok(EntityDamageEventDamageCause::EntityAttack {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "ENTITY_SWEEP_ATTACK" => Ok(EntityDamageEventDamageCause::EntitySweepAttack {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "PROJECTILE" => Ok(EntityDamageEventDamageCause::Projectile {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "SUFFOCATION" => Ok(EntityDamageEventDamageCause::Suffocation {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "FALL" => Ok(EntityDamageEventDamageCause::Fall {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "FIRE" => Ok(EntityDamageEventDamageCause::Fire {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "FIRE_TICK" => Ok(EntityDamageEventDamageCause::FireTick {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "MELTING" => Ok(EntityDamageEventDamageCause::Melting {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "LAVA" => Ok(EntityDamageEventDamageCause::Lava {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "DROWNING" => Ok(EntityDamageEventDamageCause::Drowning {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "BLOCK_EXPLOSION" => Ok(EntityDamageEventDamageCause::BlockExplosion {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "ENTITY_EXPLOSION" => Ok(EntityDamageEventDamageCause::EntityExplosion {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "VOID" => Ok(EntityDamageEventDamageCause::Void {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "LIGHTNING" => Ok(EntityDamageEventDamageCause::Lightning {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "SUICIDE" => Ok(EntityDamageEventDamageCause::Suicide {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "STARVATION" => Ok(EntityDamageEventDamageCause::Starvation {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "POISON" => Ok(EntityDamageEventDamageCause::Poison {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "MAGIC" => Ok(EntityDamageEventDamageCause::Magic {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "WITHER" => Ok(EntityDamageEventDamageCause::Wither {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "FALLING_BLOCK" => Ok(EntityDamageEventDamageCause::FallingBlock {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "THORNS" => Ok(EntityDamageEventDamageCause::Thorns {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "DRAGON_BREATH" => Ok(EntityDamageEventDamageCause::DragonBreath {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "CUSTOM" => Ok(EntityDamageEventDamageCause::Custom {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "FLY_INTO_WALL" => Ok(EntityDamageEventDamageCause::FlyIntoWall {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "HOT_FLOOR" => Ok(EntityDamageEventDamageCause::HotFloor {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "CRAMMING" => Ok(EntityDamageEventDamageCause::Cramming {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "DRYOUT" => Ok(EntityDamageEventDamageCause::Dryout {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "FREEZE" => Ok(EntityDamageEventDamageCause::Freeze {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "SONIC_BOOM" => Ok(EntityDamageEventDamageCause::SonicBoom {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityDamageEventDamageCauseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityDamageEventDamageCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Kill { inner } => inner.0.clone(),
            Self::WorldBorder { inner } => inner.0.clone(),
            Self::Contact { inner } => inner.0.clone(),
            Self::EntityAttack { inner } => inner.0.clone(),
            Self::EntitySweepAttack { inner } => inner.0.clone(),
            Self::Projectile { inner } => inner.0.clone(),
            Self::Suffocation { inner } => inner.0.clone(),
            Self::Fall { inner } => inner.0.clone(),
            Self::Fire { inner } => inner.0.clone(),
            Self::FireTick { inner } => inner.0.clone(),
            Self::Melting { inner } => inner.0.clone(),
            Self::Lava { inner } => inner.0.clone(),
            Self::Drowning { inner } => inner.0.clone(),
            Self::BlockExplosion { inner } => inner.0.clone(),
            Self::EntityExplosion { inner } => inner.0.clone(),
            Self::Void { inner } => inner.0.clone(),
            Self::Lightning { inner } => inner.0.clone(),
            Self::Suicide { inner } => inner.0.clone(),
            Self::Starvation { inner } => inner.0.clone(),
            Self::Poison { inner } => inner.0.clone(),
            Self::Magic { inner } => inner.0.clone(),
            Self::Wither { inner } => inner.0.clone(),
            Self::FallingBlock { inner } => inner.0.clone(),
            Self::Thorns { inner } => inner.0.clone(),
            Self::DragonBreath { inner } => inner.0.clone(),
            Self::Custom { inner } => inner.0.clone(),
            Self::FlyIntoWall { inner } => inner.0.clone(),
            Self::HotFloor { inner } => inner.0.clone(),
            Self::Cramming { inner } => inner.0.clone(),
            Self::Dryout { inner } => inner.0.clone(),
            Self::Freeze { inner } => inner.0.clone(),
            Self::SonicBoom { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Kill { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WorldBorder { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Contact { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::EntityAttack { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EntitySweepAttack { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Projectile { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Suffocation { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Fall { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Fire { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FireTick { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Melting { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Lava { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Drowning { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::BlockExplosion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EntityExplosion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Void { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Lightning { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Suicide { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Starvation { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Poison { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Magic { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Wither { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FallingBlock { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Thorns { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::DragonBreath { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FlyIntoWall { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HotFloor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cramming { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Dryout { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Freeze { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SonicBoom { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDamageEventDamageCause<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityDamageEventDamageCause from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityDamageEvent$DamageCause",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityDamageEventDamageCause object, got {}",
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
                "KILL" => Ok(EntityDamageEventDamageCause::Kill {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "WORLD_BORDER" => Ok(EntityDamageEventDamageCause::WorldBorder {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "CONTACT" => Ok(EntityDamageEventDamageCause::Contact {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "ENTITY_ATTACK" => Ok(EntityDamageEventDamageCause::EntityAttack {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "ENTITY_SWEEP_ATTACK" => Ok(EntityDamageEventDamageCause::EntitySweepAttack {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "PROJECTILE" => Ok(EntityDamageEventDamageCause::Projectile {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "SUFFOCATION" => Ok(EntityDamageEventDamageCause::Suffocation {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "FALL" => Ok(EntityDamageEventDamageCause::Fall {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "FIRE" => Ok(EntityDamageEventDamageCause::Fire {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "FIRE_TICK" => Ok(EntityDamageEventDamageCause::FireTick {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "MELTING" => Ok(EntityDamageEventDamageCause::Melting {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "LAVA" => Ok(EntityDamageEventDamageCause::Lava {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "DROWNING" => Ok(EntityDamageEventDamageCause::Drowning {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "BLOCK_EXPLOSION" => Ok(EntityDamageEventDamageCause::BlockExplosion {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "ENTITY_EXPLOSION" => Ok(EntityDamageEventDamageCause::EntityExplosion {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "VOID" => Ok(EntityDamageEventDamageCause::Void {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "LIGHTNING" => Ok(EntityDamageEventDamageCause::Lightning {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "SUICIDE" => Ok(EntityDamageEventDamageCause::Suicide {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "STARVATION" => Ok(EntityDamageEventDamageCause::Starvation {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "POISON" => Ok(EntityDamageEventDamageCause::Poison {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "MAGIC" => Ok(EntityDamageEventDamageCause::Magic {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "WITHER" => Ok(EntityDamageEventDamageCause::Wither {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "FALLING_BLOCK" => Ok(EntityDamageEventDamageCause::FallingBlock {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "THORNS" => Ok(EntityDamageEventDamageCause::Thorns {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "DRAGON_BREATH" => Ok(EntityDamageEventDamageCause::DragonBreath {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "CUSTOM" => Ok(EntityDamageEventDamageCause::Custom {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "FLY_INTO_WALL" => Ok(EntityDamageEventDamageCause::FlyIntoWall {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "HOT_FLOOR" => Ok(EntityDamageEventDamageCause::HotFloor {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "CRAMMING" => Ok(EntityDamageEventDamageCause::Cramming {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "DRYOUT" => Ok(EntityDamageEventDamageCause::Dryout {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "FREEZE" => Ok(EntityDamageEventDamageCause::Freeze {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "SONIC_BOOM" => Ok(EntityDamageEventDamageCause::SonicBoom {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityDamageEventDamageCauseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDamageEventDamageCauseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityDamageEventDamageCauseStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityDamageEvent$DamageCause",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityDamageEventDamageCauseStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityDamageEventDamageCauseStruct<'mc> {
    //Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum EntityDamageEventDamageModifier<'mc> {
    Base {
        inner: EntityDamageEventDamageModifierStruct<'mc>,
    },
    HardHat {
        inner: EntityDamageEventDamageModifierStruct<'mc>,
    },
    Blocking {
        inner: EntityDamageEventDamageModifierStruct<'mc>,
    },
    Armor {
        inner: EntityDamageEventDamageModifierStruct<'mc>,
    },
    Resistance {
        inner: EntityDamageEventDamageModifierStruct<'mc>,
    },
    Magic {
        inner: EntityDamageEventDamageModifierStruct<'mc>,
    },
    Absorption {
        inner: EntityDamageEventDamageModifierStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityDamageEventDamageModifier<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityDamageEventDamageModifier::Base { .. } => f.write_str("BASE"),
            EntityDamageEventDamageModifier::HardHat { .. } => f.write_str("HARD_HAT"),
            EntityDamageEventDamageModifier::Blocking { .. } => f.write_str("BLOCKING"),
            EntityDamageEventDamageModifier::Armor { .. } => f.write_str("ARMOR"),
            EntityDamageEventDamageModifier::Resistance { .. } => f.write_str("RESISTANCE"),
            EntityDamageEventDamageModifier::Magic { .. } => f.write_str("MAGIC"),
            EntityDamageEventDamageModifier::Absorption { .. } => f.write_str("ABSORPTION"),
        }
    }
}

impl<'mc> EntityDamageEventDamageModifier<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityDamageEventDamageModifier<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityDamageEvent$DamageModifier");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;",
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
            "BASE" => Ok(EntityDamageEventDamageModifier::Base {
                inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
            }),
            "HARD_HAT" => Ok(EntityDamageEventDamageModifier::HardHat {
                inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
            }),
            "BLOCKING" => Ok(EntityDamageEventDamageModifier::Blocking {
                inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
            }),
            "ARMOR" => Ok(EntityDamageEventDamageModifier::Armor {
                inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
            }),
            "RESISTANCE" => Ok(EntityDamageEventDamageModifier::Resistance {
                inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
            }),
            "MAGIC" => Ok(EntityDamageEventDamageModifier::Magic {
                inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
            }),
            "ABSORPTION" => Ok(EntityDamageEventDamageModifier::Absorption {
                inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityDamageEventDamageModifierStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityDamageEventDamageModifier<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Base { inner } => inner.0.clone(),
            Self::HardHat { inner } => inner.0.clone(),
            Self::Blocking { inner } => inner.0.clone(),
            Self::Armor { inner } => inner.0.clone(),
            Self::Resistance { inner } => inner.0.clone(),
            Self::Magic { inner } => inner.0.clone(),
            Self::Absorption { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Base { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::HardHat { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Blocking { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Armor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Resistance { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Magic { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Absorption { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDamageEventDamageModifier<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityDamageEventDamageModifier from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityDamageEvent$DamageModifier",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityDamageEventDamageModifier object, got {}",
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
                "BASE" => Ok(EntityDamageEventDamageModifier::Base {
                    inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
                }),
                "HARD_HAT" => Ok(EntityDamageEventDamageModifier::HardHat {
                    inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
                }),
                "BLOCKING" => Ok(EntityDamageEventDamageModifier::Blocking {
                    inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
                }),
                "ARMOR" => Ok(EntityDamageEventDamageModifier::Armor {
                    inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
                }),
                "RESISTANCE" => Ok(EntityDamageEventDamageModifier::Resistance {
                    inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
                }),
                "MAGIC" => Ok(EntityDamageEventDamageModifier::Magic {
                    inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
                }),
                "ABSORPTION" => Ok(EntityDamageEventDamageModifier::Absorption {
                    inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityDamageEventDamageModifierStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDamageEventDamageModifierStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityDamageEventDamageModifierStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityDamageEvent$DamageModifier",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityDamageEventDamageModifierStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityDamageEventDamageModifierStruct<'mc> {
    //Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for EntityDamageByBlockEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDamageByBlockEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityDamageByBlockEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityDamageByBlockEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityDamageByBlockEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityDamageByBlockEvent<'mc> {
    pub fn new_with_block(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::block::Block<'mc>>,
        arg1: impl Into<crate::entity::Entity<'mc>>,
        arg2: impl Into<crate::event::entity::EntityDamageEventDamageCause<'mc>>,
        arg3: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
        arg4: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::event::entity::EntityDamageByBlockEvent<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/Block;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Entity;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent$DamageCause;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        args.push(val_3);
        sig += "Ljava/util/Map;";
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        args.push(val_4);
        if let Some(a) = arg4 {
            sig += "Ljava/util/Map;";
            let val_5 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_5);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityDamageByBlockEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityDamageByBlockEvent::from_raw(&jni, res)
    }

    pub fn damager(&self) -> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDamager", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::Block::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    //EntityDamageEvent
    //crate::event::entity::EntityDamageEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
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
    pub fn set_damage_with_entity_damage_eventdamage_modifier(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
        arg1: std::option::Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a);
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setDamage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn get_damage(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.get_damage(arg0)
    }
    pub fn is_applicable(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.is_applicable(arg0)
    }
    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntityDamageEvent::handler_list(jni)
    }
    pub fn get_original_damage(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.get_original_damage(arg0)
    }
    pub fn final_damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.final_damage()
    }
    pub fn cause(
        &self,
    ) -> Result<crate::event::entity::EntityDamageEventDamageCause<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.cause()
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityDamageEvent<'mc>> for EntityDamageByBlockEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityDamageEvent<'mc> {
        crate::event::entity::EntityDamageEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EntityDamageByBlockEvent into crate::event::entity::EntityDamageEvent")
    }
}
pub enum SpawnReason<'mc> {
    Natural {
        inner: SpawnReasonStruct<'mc>,
    },
    Jockey {
        inner: SpawnReasonStruct<'mc>,
    },
    #[deprecated]
    ChunkGen {
        inner: SpawnReasonStruct<'mc>,
    },
    Spawner {
        inner: SpawnReasonStruct<'mc>,
    },
    Egg {
        inner: SpawnReasonStruct<'mc>,
    },
    SpawnerEgg {
        inner: SpawnReasonStruct<'mc>,
    },
    Lightning {
        inner: SpawnReasonStruct<'mc>,
    },
    BuildSnowman {
        inner: SpawnReasonStruct<'mc>,
    },
    BuildIrongolem {
        inner: SpawnReasonStruct<'mc>,
    },
    BuildWither {
        inner: SpawnReasonStruct<'mc>,
    },
    VillageDefense {
        inner: SpawnReasonStruct<'mc>,
    },
    VillageInvasion {
        inner: SpawnReasonStruct<'mc>,
    },
    Breeding {
        inner: SpawnReasonStruct<'mc>,
    },
    SlimeSplit {
        inner: SpawnReasonStruct<'mc>,
    },
    Reinforcements {
        inner: SpawnReasonStruct<'mc>,
    },
    NetherPortal {
        inner: SpawnReasonStruct<'mc>,
    },
    DispenseEgg {
        inner: SpawnReasonStruct<'mc>,
    },
    Infection {
        inner: SpawnReasonStruct<'mc>,
    },
    Cured {
        inner: SpawnReasonStruct<'mc>,
    },
    OcelotBaby {
        inner: SpawnReasonStruct<'mc>,
    },
    SilverfishBlock {
        inner: SpawnReasonStruct<'mc>,
    },
    Mount {
        inner: SpawnReasonStruct<'mc>,
    },
    Trap {
        inner: SpawnReasonStruct<'mc>,
    },
    EnderPearl {
        inner: SpawnReasonStruct<'mc>,
    },
    ShoulderEntity {
        inner: SpawnReasonStruct<'mc>,
    },
    Drowned {
        inner: SpawnReasonStruct<'mc>,
    },
    Sheared {
        inner: SpawnReasonStruct<'mc>,
    },
    Explosion {
        inner: SpawnReasonStruct<'mc>,
    },
    Raid {
        inner: SpawnReasonStruct<'mc>,
    },
    Patrol {
        inner: SpawnReasonStruct<'mc>,
    },
    Beehive {
        inner: SpawnReasonStruct<'mc>,
    },
    PiglinZombified {
        inner: SpawnReasonStruct<'mc>,
    },
    Spell {
        inner: SpawnReasonStruct<'mc>,
    },
    Frozen {
        inner: SpawnReasonStruct<'mc>,
    },
    Metamorphosis {
        inner: SpawnReasonStruct<'mc>,
    },
    Duplication {
        inner: SpawnReasonStruct<'mc>,
    },
    Command {
        inner: SpawnReasonStruct<'mc>,
    },
    Custom {
        inner: SpawnReasonStruct<'mc>,
    },
    Default {
        inner: SpawnReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for SpawnReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpawnReason::Natural { .. } => f.write_str("NATURAL"),
            SpawnReason::Jockey { .. } => f.write_str("JOCKEY"),
            SpawnReason::ChunkGen { .. } => f.write_str("CHUNK_GEN"),
            SpawnReason::Spawner { .. } => f.write_str("SPAWNER"),
            SpawnReason::Egg { .. } => f.write_str("EGG"),
            SpawnReason::SpawnerEgg { .. } => f.write_str("SPAWNER_EGG"),
            SpawnReason::Lightning { .. } => f.write_str("LIGHTNING"),
            SpawnReason::BuildSnowman { .. } => f.write_str("BUILD_SNOWMAN"),
            SpawnReason::BuildIrongolem { .. } => f.write_str("BUILD_IRONGOLEM"),
            SpawnReason::BuildWither { .. } => f.write_str("BUILD_WITHER"),
            SpawnReason::VillageDefense { .. } => f.write_str("VILLAGE_DEFENSE"),
            SpawnReason::VillageInvasion { .. } => f.write_str("VILLAGE_INVASION"),
            SpawnReason::Breeding { .. } => f.write_str("BREEDING"),
            SpawnReason::SlimeSplit { .. } => f.write_str("SLIME_SPLIT"),
            SpawnReason::Reinforcements { .. } => f.write_str("REINFORCEMENTS"),
            SpawnReason::NetherPortal { .. } => f.write_str("NETHER_PORTAL"),
            SpawnReason::DispenseEgg { .. } => f.write_str("DISPENSE_EGG"),
            SpawnReason::Infection { .. } => f.write_str("INFECTION"),
            SpawnReason::Cured { .. } => f.write_str("CURED"),
            SpawnReason::OcelotBaby { .. } => f.write_str("OCELOT_BABY"),
            SpawnReason::SilverfishBlock { .. } => f.write_str("SILVERFISH_BLOCK"),
            SpawnReason::Mount { .. } => f.write_str("MOUNT"),
            SpawnReason::Trap { .. } => f.write_str("TRAP"),
            SpawnReason::EnderPearl { .. } => f.write_str("ENDER_PEARL"),
            SpawnReason::ShoulderEntity { .. } => f.write_str("SHOULDER_ENTITY"),
            SpawnReason::Drowned { .. } => f.write_str("DROWNED"),
            SpawnReason::Sheared { .. } => f.write_str("SHEARED"),
            SpawnReason::Explosion { .. } => f.write_str("EXPLOSION"),
            SpawnReason::Raid { .. } => f.write_str("RAID"),
            SpawnReason::Patrol { .. } => f.write_str("PATROL"),
            SpawnReason::Beehive { .. } => f.write_str("BEEHIVE"),
            SpawnReason::PiglinZombified { .. } => f.write_str("PIGLIN_ZOMBIFIED"),
            SpawnReason::Spell { .. } => f.write_str("SPELL"),
            SpawnReason::Frozen { .. } => f.write_str("FROZEN"),
            SpawnReason::Metamorphosis { .. } => f.write_str("METAMORPHOSIS"),
            SpawnReason::Duplication { .. } => f.write_str("DUPLICATION"),
            SpawnReason::Command { .. } => f.write_str("COMMAND"),
            SpawnReason::Custom { .. } => f.write_str("CUSTOM"),
            SpawnReason::Default { .. } => f.write_str("DEFAULT"),
        }
    }
}

impl<'mc> SpawnReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<SpawnReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/SpawnReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/SpawnReason;",
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
            "NATURAL" => Ok(SpawnReason::Natural {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "JOCKEY" => Ok(SpawnReason::Jockey {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "CHUNK_GEN" => Ok(SpawnReason::ChunkGen {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SPAWNER" => Ok(SpawnReason::Spawner {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "EGG" => Ok(SpawnReason::Egg {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SPAWNER_EGG" => Ok(SpawnReason::SpawnerEgg {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "LIGHTNING" => Ok(SpawnReason::Lightning {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "BUILD_SNOWMAN" => Ok(SpawnReason::BuildSnowman {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "BUILD_IRONGOLEM" => Ok(SpawnReason::BuildIrongolem {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "BUILD_WITHER" => Ok(SpawnReason::BuildWither {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_DEFENSE" => Ok(SpawnReason::VillageDefense {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_INVASION" => Ok(SpawnReason::VillageInvasion {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "BREEDING" => Ok(SpawnReason::Breeding {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SLIME_SPLIT" => Ok(SpawnReason::SlimeSplit {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "REINFORCEMENTS" => Ok(SpawnReason::Reinforcements {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "NETHER_PORTAL" => Ok(SpawnReason::NetherPortal {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "DISPENSE_EGG" => Ok(SpawnReason::DispenseEgg {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "INFECTION" => Ok(SpawnReason::Infection {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "CURED" => Ok(SpawnReason::Cured {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "OCELOT_BABY" => Ok(SpawnReason::OcelotBaby {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SILVERFISH_BLOCK" => Ok(SpawnReason::SilverfishBlock {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "MOUNT" => Ok(SpawnReason::Mount {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "TRAP" => Ok(SpawnReason::Trap {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "ENDER_PEARL" => Ok(SpawnReason::EnderPearl {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SHOULDER_ENTITY" => Ok(SpawnReason::ShoulderEntity {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "DROWNED" => Ok(SpawnReason::Drowned {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SHEARED" => Ok(SpawnReason::Sheared {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "EXPLOSION" => Ok(SpawnReason::Explosion {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "RAID" => Ok(SpawnReason::Raid {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "PATROL" => Ok(SpawnReason::Patrol {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "BEEHIVE" => Ok(SpawnReason::Beehive {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "PIGLIN_ZOMBIFIED" => Ok(SpawnReason::PiglinZombified {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SPELL" => Ok(SpawnReason::Spell {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "FROZEN" => Ok(SpawnReason::Frozen {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "METAMORPHOSIS" => Ok(SpawnReason::Metamorphosis {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "DUPLICATION" => Ok(SpawnReason::Duplication {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "COMMAND" => Ok(SpawnReason::Command {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "CUSTOM" => Ok(SpawnReason::Custom {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),
            "DEFAULT" => Ok(SpawnReason::Default {
                inner: SpawnReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct SpawnReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SpawnReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Natural { inner } => inner.0.clone(),
            Self::Jockey { inner } => inner.0.clone(),
            Self::ChunkGen { inner } => inner.0.clone(),
            Self::Spawner { inner } => inner.0.clone(),
            Self::Egg { inner } => inner.0.clone(),
            Self::SpawnerEgg { inner } => inner.0.clone(),
            Self::Lightning { inner } => inner.0.clone(),
            Self::BuildSnowman { inner } => inner.0.clone(),
            Self::BuildIrongolem { inner } => inner.0.clone(),
            Self::BuildWither { inner } => inner.0.clone(),
            Self::VillageDefense { inner } => inner.0.clone(),
            Self::VillageInvasion { inner } => inner.0.clone(),
            Self::Breeding { inner } => inner.0.clone(),
            Self::SlimeSplit { inner } => inner.0.clone(),
            Self::Reinforcements { inner } => inner.0.clone(),
            Self::NetherPortal { inner } => inner.0.clone(),
            Self::DispenseEgg { inner } => inner.0.clone(),
            Self::Infection { inner } => inner.0.clone(),
            Self::Cured { inner } => inner.0.clone(),
            Self::OcelotBaby { inner } => inner.0.clone(),
            Self::SilverfishBlock { inner } => inner.0.clone(),
            Self::Mount { inner } => inner.0.clone(),
            Self::Trap { inner } => inner.0.clone(),
            Self::EnderPearl { inner } => inner.0.clone(),
            Self::ShoulderEntity { inner } => inner.0.clone(),
            Self::Drowned { inner } => inner.0.clone(),
            Self::Sheared { inner } => inner.0.clone(),
            Self::Explosion { inner } => inner.0.clone(),
            Self::Raid { inner } => inner.0.clone(),
            Self::Patrol { inner } => inner.0.clone(),
            Self::Beehive { inner } => inner.0.clone(),
            Self::PiglinZombified { inner } => inner.0.clone(),
            Self::Spell { inner } => inner.0.clone(),
            Self::Frozen { inner } => inner.0.clone(),
            Self::Metamorphosis { inner } => inner.0.clone(),
            Self::Duplication { inner } => inner.0.clone(),
            Self::Command { inner } => inner.0.clone(),
            Self::Custom { inner } => inner.0.clone(),
            Self::Default { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Natural { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Jockey { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ChunkGen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Spawner { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Egg { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SpawnerEgg { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Lightning { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BuildSnowman { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BuildIrongolem { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BuildWither { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageDefense { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageInvasion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Breeding { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SlimeSplit { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Reinforcements { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::NetherPortal { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DispenseEgg { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Infection { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Cured { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::OcelotBaby { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SilverfishBlock { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Mount { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Trap { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::EnderPearl { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShoulderEntity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Drowned { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Sheared { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Explosion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Raid { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Patrol { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Beehive { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PiglinZombified { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Spell { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Frozen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Metamorphosis { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Duplication { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Command { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Default { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SpawnReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SpawnReason from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/SpawnReason")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpawnReason object, got {}",
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
                "NATURAL" => Ok(SpawnReason::Natural {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "JOCKEY" => Ok(SpawnReason::Jockey {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "CHUNK_GEN" => Ok(SpawnReason::ChunkGen {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SPAWNER" => Ok(SpawnReason::Spawner {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "EGG" => Ok(SpawnReason::Egg {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SPAWNER_EGG" => Ok(SpawnReason::SpawnerEgg {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "LIGHTNING" => Ok(SpawnReason::Lightning {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "BUILD_SNOWMAN" => Ok(SpawnReason::BuildSnowman {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "BUILD_IRONGOLEM" => Ok(SpawnReason::BuildIrongolem {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "BUILD_WITHER" => Ok(SpawnReason::BuildWither {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_DEFENSE" => Ok(SpawnReason::VillageDefense {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_INVASION" => Ok(SpawnReason::VillageInvasion {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "BREEDING" => Ok(SpawnReason::Breeding {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SLIME_SPLIT" => Ok(SpawnReason::SlimeSplit {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "REINFORCEMENTS" => Ok(SpawnReason::Reinforcements {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "NETHER_PORTAL" => Ok(SpawnReason::NetherPortal {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "DISPENSE_EGG" => Ok(SpawnReason::DispenseEgg {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "INFECTION" => Ok(SpawnReason::Infection {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "CURED" => Ok(SpawnReason::Cured {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "OCELOT_BABY" => Ok(SpawnReason::OcelotBaby {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SILVERFISH_BLOCK" => Ok(SpawnReason::SilverfishBlock {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "MOUNT" => Ok(SpawnReason::Mount {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "TRAP" => Ok(SpawnReason::Trap {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "ENDER_PEARL" => Ok(SpawnReason::EnderPearl {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SHOULDER_ENTITY" => Ok(SpawnReason::ShoulderEntity {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "DROWNED" => Ok(SpawnReason::Drowned {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SHEARED" => Ok(SpawnReason::Sheared {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "EXPLOSION" => Ok(SpawnReason::Explosion {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "RAID" => Ok(SpawnReason::Raid {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "PATROL" => Ok(SpawnReason::Patrol {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "BEEHIVE" => Ok(SpawnReason::Beehive {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "PIGLIN_ZOMBIFIED" => Ok(SpawnReason::PiglinZombified {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SPELL" => Ok(SpawnReason::Spell {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "FROZEN" => Ok(SpawnReason::Frozen {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "METAMORPHOSIS" => Ok(SpawnReason::Metamorphosis {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "DUPLICATION" => Ok(SpawnReason::Duplication {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "COMMAND" => Ok(SpawnReason::Command {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "CUSTOM" => Ok(SpawnReason::Custom {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                "DEFAULT" => Ok(SpawnReason::Default {
                    inner: SpawnReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for SpawnReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SpawnReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SpawnReasonStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/SpawnReason")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpawnReasonStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SpawnReasonStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Called when a sheep regrows its wool
#[repr(C)]
pub struct SheepRegrowWoolEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SheepRegrowWoolEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SheepRegrowWoolEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SheepRegrowWoolEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/SheepRegrowWoolEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SheepRegrowWoolEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SheepRegrowWoolEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Sheep<'mc>>,
    ) -> Result<crate::event::entity::SheepRegrowWoolEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Sheep;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/SheepRegrowWoolEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::SheepRegrowWoolEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::Sheep<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Sheep;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Sheep::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for SheepRegrowWoolEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SheepRegrowWoolEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for SheepRegrowWoolEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SheepRegrowWoolEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when an <a href="../../entity/Entity.html" title="interface in org.bukkit.entity"><code>Entity</code></a> enters a block and is stored in that block.
/// <p>This event is called for bees entering a bee hive.
///
/// It is not called when a silverfish "enters" a stone block. For that listen to the <a href="EntityChangeBlockEvent.html" title="class in org.bukkit.event.entity"><code>EntityChangeBlockEvent</code></a>.</p>
#[repr(C)]
pub struct EntityEnterBlockEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityEnterBlockEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityEnterBlockEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityEnterBlockEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityEnterBlockEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityEnterBlockEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityEnterBlockEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::block::Block<'mc>>,
    ) -> Result<crate::event::entity::EntityEnterBlockEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/Block;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityEnterBlockEvent");
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
        crate::event::entity::EntityEnterBlockEvent::from_raw(&jni, res)
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

    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityEnterBlockEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityEnterBlockEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityEnterBlockEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityEnterBlockEvent into crate::event::entity::EntityEvent")
    }
}
/// Thrown when a entity picks an item up from the ground
#[repr(C)]
pub struct EntityPickupItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPickupItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPickupItemEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPickupItemEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityPickupItemEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPickupItemEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPickupItemEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
        arg1: impl Into<crate::entity::Item<'mc>>,
        arg2: i32,
    ) -> Result<crate::event::entity::EntityPickupItemEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/Item;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(arg2);
        let cls = jni.find_class("org/bukkit/event/entity/EntityPickupItemEvent");
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
        crate::event::entity::EntityPickupItemEvent::from_raw(&jni, res)
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

    pub fn remaining(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRemaining", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn item(&self) -> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Item;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getItem", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Item::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/LivingEntity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityPickupItemEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityPickupItemEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityPickupItemEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityPickupItemEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when a horse jumps.
#[repr(C)]
pub struct HorseJumpEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HorseJumpEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HorseJumpEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate HorseJumpEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/HorseJumpEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HorseJumpEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HorseJumpEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::AbstractHorse<'mc>>,
        arg1: f32,
    ) -> Result<crate::event::entity::HorseJumpEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/AbstractHorse;F)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Float(arg1);
        let cls = jni.find_class("org/bukkit/event/entity/HorseJumpEvent");
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
        crate::event::entity::HorseJumpEvent::from_raw(&jni, res)
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

    pub fn power(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    #[deprecated = "horse jumping was moved client side. "]
    /// Sets the power of the jump.<p>Jump power can be set to a value above 1.0 which will increase the strength of this jump above the horse's actual jump strength.</p> <p>Setting the jump power to 0 will result in the jump animation still playing, but the horse not leaving the ground. Only canceling this event will result in no jump animation at all.</p>
    pub fn set_power(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPower",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::AbstractHorse<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/AbstractHorse;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::AbstractHorse::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// horse jumping was moved client side.
    /// </div>
    /// horse jumping was moved client side.
    ///
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for HorseJumpEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HorseJumpEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for HorseJumpEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HorseJumpEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when an entity has made a decision to explode.
#[repr(C)]
pub struct ExplosionPrimeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ExplosionPrimeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ExplosionPrimeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ExplosionPrimeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/ExplosionPrimeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ExplosionPrimeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ExplosionPrimeEvent<'mc> {
    pub fn new_with_entity(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: std::option::Option<f32>,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::event::entity::ExplosionPrimeEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "F";
            let val_2 = jni::objects::JValueGen::Float(a);
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/ExplosionPrimeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::ExplosionPrimeEvent::from_raw(&jni, res)
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

    pub fn radius(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRadius", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the radius of the explosion
    pub fn set_radius(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRadius",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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

    pub fn fire(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFire", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether this explosion will create fire or not
    pub fn set_fire(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFire",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for ExplosionPrimeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ExplosionPrimeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for ExplosionPrimeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ExplosionPrimeEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when a creature is spawned into a world.
/// <p>If a Creature Spawn event is cancelled, the creature will not spawn.</p>
#[repr(C)]
pub struct CreatureSpawnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum CreatureSpawnEventSpawnReason<'mc> {
    Natural {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Jockey {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    ChunkGen {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Spawner {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Egg {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    SpawnerEgg {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Lightning {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    BuildSnowman {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    BuildIrongolem {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    BuildWither {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    VillageDefense {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    VillageInvasion {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Breeding {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    SlimeSplit {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Reinforcements {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    NetherPortal {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    DispenseEgg {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Infection {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Cured {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    OcelotBaby {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    SilverfishBlock {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Mount {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Trap {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    EnderPearl {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    ShoulderEntity {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Drowned {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Sheared {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Explosion {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Raid {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Patrol {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Beehive {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    PiglinZombified {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Spell {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Frozen {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Metamorphosis {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Duplication {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Command {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Custom {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Default {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for CreatureSpawnEventSpawnReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreatureSpawnEventSpawnReason::Natural { .. } => f.write_str("NATURAL"),
            CreatureSpawnEventSpawnReason::Jockey { .. } => f.write_str("JOCKEY"),
            CreatureSpawnEventSpawnReason::ChunkGen { .. } => f.write_str("CHUNK_GEN"),
            CreatureSpawnEventSpawnReason::Spawner { .. } => f.write_str("SPAWNER"),
            CreatureSpawnEventSpawnReason::Egg { .. } => f.write_str("EGG"),
            CreatureSpawnEventSpawnReason::SpawnerEgg { .. } => f.write_str("SPAWNER_EGG"),
            CreatureSpawnEventSpawnReason::Lightning { .. } => f.write_str("LIGHTNING"),
            CreatureSpawnEventSpawnReason::BuildSnowman { .. } => f.write_str("BUILD_SNOWMAN"),
            CreatureSpawnEventSpawnReason::BuildIrongolem { .. } => f.write_str("BUILD_IRONGOLEM"),
            CreatureSpawnEventSpawnReason::BuildWither { .. } => f.write_str("BUILD_WITHER"),
            CreatureSpawnEventSpawnReason::VillageDefense { .. } => f.write_str("VILLAGE_DEFENSE"),
            CreatureSpawnEventSpawnReason::VillageInvasion { .. } => {
                f.write_str("VILLAGE_INVASION")
            }
            CreatureSpawnEventSpawnReason::Breeding { .. } => f.write_str("BREEDING"),
            CreatureSpawnEventSpawnReason::SlimeSplit { .. } => f.write_str("SLIME_SPLIT"),
            CreatureSpawnEventSpawnReason::Reinforcements { .. } => f.write_str("REINFORCEMENTS"),
            CreatureSpawnEventSpawnReason::NetherPortal { .. } => f.write_str("NETHER_PORTAL"),
            CreatureSpawnEventSpawnReason::DispenseEgg { .. } => f.write_str("DISPENSE_EGG"),
            CreatureSpawnEventSpawnReason::Infection { .. } => f.write_str("INFECTION"),
            CreatureSpawnEventSpawnReason::Cured { .. } => f.write_str("CURED"),
            CreatureSpawnEventSpawnReason::OcelotBaby { .. } => f.write_str("OCELOT_BABY"),
            CreatureSpawnEventSpawnReason::SilverfishBlock { .. } => {
                f.write_str("SILVERFISH_BLOCK")
            }
            CreatureSpawnEventSpawnReason::Mount { .. } => f.write_str("MOUNT"),
            CreatureSpawnEventSpawnReason::Trap { .. } => f.write_str("TRAP"),
            CreatureSpawnEventSpawnReason::EnderPearl { .. } => f.write_str("ENDER_PEARL"),
            CreatureSpawnEventSpawnReason::ShoulderEntity { .. } => f.write_str("SHOULDER_ENTITY"),
            CreatureSpawnEventSpawnReason::Drowned { .. } => f.write_str("DROWNED"),
            CreatureSpawnEventSpawnReason::Sheared { .. } => f.write_str("SHEARED"),
            CreatureSpawnEventSpawnReason::Explosion { .. } => f.write_str("EXPLOSION"),
            CreatureSpawnEventSpawnReason::Raid { .. } => f.write_str("RAID"),
            CreatureSpawnEventSpawnReason::Patrol { .. } => f.write_str("PATROL"),
            CreatureSpawnEventSpawnReason::Beehive { .. } => f.write_str("BEEHIVE"),
            CreatureSpawnEventSpawnReason::PiglinZombified { .. } => {
                f.write_str("PIGLIN_ZOMBIFIED")
            }
            CreatureSpawnEventSpawnReason::Spell { .. } => f.write_str("SPELL"),
            CreatureSpawnEventSpawnReason::Frozen { .. } => f.write_str("FROZEN"),
            CreatureSpawnEventSpawnReason::Metamorphosis { .. } => f.write_str("METAMORPHOSIS"),
            CreatureSpawnEventSpawnReason::Duplication { .. } => f.write_str("DUPLICATION"),
            CreatureSpawnEventSpawnReason::Command { .. } => f.write_str("COMMAND"),
            CreatureSpawnEventSpawnReason::Custom { .. } => f.write_str("CUSTOM"),
            CreatureSpawnEventSpawnReason::Default { .. } => f.write_str("DEFAULT"),
        }
    }
}

impl<'mc> CreatureSpawnEventSpawnReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<CreatureSpawnEventSpawnReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/CreatureSpawnEvent$SpawnReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/CreatureSpawnEvent$SpawnReason;",
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
            "NATURAL" => Ok(CreatureSpawnEventSpawnReason::Natural {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "JOCKEY" => Ok(CreatureSpawnEventSpawnReason::Jockey {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "CHUNK_GEN" => Ok(CreatureSpawnEventSpawnReason::ChunkGen {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SPAWNER" => Ok(CreatureSpawnEventSpawnReason::Spawner {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "EGG" => Ok(CreatureSpawnEventSpawnReason::Egg {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SPAWNER_EGG" => Ok(CreatureSpawnEventSpawnReason::SpawnerEgg {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "LIGHTNING" => Ok(CreatureSpawnEventSpawnReason::Lightning {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "BUILD_SNOWMAN" => Ok(CreatureSpawnEventSpawnReason::BuildSnowman {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "BUILD_IRONGOLEM" => Ok(CreatureSpawnEventSpawnReason::BuildIrongolem {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "BUILD_WITHER" => Ok(CreatureSpawnEventSpawnReason::BuildWither {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_DEFENSE" => Ok(CreatureSpawnEventSpawnReason::VillageDefense {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_INVASION" => Ok(CreatureSpawnEventSpawnReason::VillageInvasion {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "BREEDING" => Ok(CreatureSpawnEventSpawnReason::Breeding {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SLIME_SPLIT" => Ok(CreatureSpawnEventSpawnReason::SlimeSplit {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "REINFORCEMENTS" => Ok(CreatureSpawnEventSpawnReason::Reinforcements {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "NETHER_PORTAL" => Ok(CreatureSpawnEventSpawnReason::NetherPortal {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "DISPENSE_EGG" => Ok(CreatureSpawnEventSpawnReason::DispenseEgg {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "INFECTION" => Ok(CreatureSpawnEventSpawnReason::Infection {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "CURED" => Ok(CreatureSpawnEventSpawnReason::Cured {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "OCELOT_BABY" => Ok(CreatureSpawnEventSpawnReason::OcelotBaby {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SILVERFISH_BLOCK" => Ok(CreatureSpawnEventSpawnReason::SilverfishBlock {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "MOUNT" => Ok(CreatureSpawnEventSpawnReason::Mount {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "TRAP" => Ok(CreatureSpawnEventSpawnReason::Trap {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "ENDER_PEARL" => Ok(CreatureSpawnEventSpawnReason::EnderPearl {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SHOULDER_ENTITY" => Ok(CreatureSpawnEventSpawnReason::ShoulderEntity {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "DROWNED" => Ok(CreatureSpawnEventSpawnReason::Drowned {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SHEARED" => Ok(CreatureSpawnEventSpawnReason::Sheared {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "EXPLOSION" => Ok(CreatureSpawnEventSpawnReason::Explosion {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "RAID" => Ok(CreatureSpawnEventSpawnReason::Raid {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "PATROL" => Ok(CreatureSpawnEventSpawnReason::Patrol {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "BEEHIVE" => Ok(CreatureSpawnEventSpawnReason::Beehive {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "PIGLIN_ZOMBIFIED" => Ok(CreatureSpawnEventSpawnReason::PiglinZombified {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SPELL" => Ok(CreatureSpawnEventSpawnReason::Spell {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "FROZEN" => Ok(CreatureSpawnEventSpawnReason::Frozen {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "METAMORPHOSIS" => Ok(CreatureSpawnEventSpawnReason::Metamorphosis {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "DUPLICATION" => Ok(CreatureSpawnEventSpawnReason::Duplication {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "COMMAND" => Ok(CreatureSpawnEventSpawnReason::Command {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "CUSTOM" => Ok(CreatureSpawnEventSpawnReason::Custom {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "DEFAULT" => Ok(CreatureSpawnEventSpawnReason::Default {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct CreatureSpawnEventSpawnReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CreatureSpawnEventSpawnReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Natural { inner } => inner.0.clone(),
            Self::Jockey { inner } => inner.0.clone(),
            Self::ChunkGen { inner } => inner.0.clone(),
            Self::Spawner { inner } => inner.0.clone(),
            Self::Egg { inner } => inner.0.clone(),
            Self::SpawnerEgg { inner } => inner.0.clone(),
            Self::Lightning { inner } => inner.0.clone(),
            Self::BuildSnowman { inner } => inner.0.clone(),
            Self::BuildIrongolem { inner } => inner.0.clone(),
            Self::BuildWither { inner } => inner.0.clone(),
            Self::VillageDefense { inner } => inner.0.clone(),
            Self::VillageInvasion { inner } => inner.0.clone(),
            Self::Breeding { inner } => inner.0.clone(),
            Self::SlimeSplit { inner } => inner.0.clone(),
            Self::Reinforcements { inner } => inner.0.clone(),
            Self::NetherPortal { inner } => inner.0.clone(),
            Self::DispenseEgg { inner } => inner.0.clone(),
            Self::Infection { inner } => inner.0.clone(),
            Self::Cured { inner } => inner.0.clone(),
            Self::OcelotBaby { inner } => inner.0.clone(),
            Self::SilverfishBlock { inner } => inner.0.clone(),
            Self::Mount { inner } => inner.0.clone(),
            Self::Trap { inner } => inner.0.clone(),
            Self::EnderPearl { inner } => inner.0.clone(),
            Self::ShoulderEntity { inner } => inner.0.clone(),
            Self::Drowned { inner } => inner.0.clone(),
            Self::Sheared { inner } => inner.0.clone(),
            Self::Explosion { inner } => inner.0.clone(),
            Self::Raid { inner } => inner.0.clone(),
            Self::Patrol { inner } => inner.0.clone(),
            Self::Beehive { inner } => inner.0.clone(),
            Self::PiglinZombified { inner } => inner.0.clone(),
            Self::Spell { inner } => inner.0.clone(),
            Self::Frozen { inner } => inner.0.clone(),
            Self::Metamorphosis { inner } => inner.0.clone(),
            Self::Duplication { inner } => inner.0.clone(),
            Self::Command { inner } => inner.0.clone(),
            Self::Custom { inner } => inner.0.clone(),
            Self::Default { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Natural { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Jockey { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ChunkGen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Spawner { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Egg { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SpawnerEgg { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Lightning { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BuildSnowman { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BuildIrongolem { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BuildWither { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageDefense { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageInvasion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Breeding { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SlimeSplit { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Reinforcements { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::NetherPortal { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DispenseEgg { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Infection { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Cured { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::OcelotBaby { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SilverfishBlock { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Mount { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Trap { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::EnderPearl { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShoulderEntity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Drowned { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Sheared { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Explosion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Raid { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Patrol { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Beehive { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PiglinZombified { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Spell { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Frozen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Metamorphosis { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Duplication { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Command { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Default { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CreatureSpawnEventSpawnReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CreatureSpawnEventSpawnReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/CreatureSpawnEvent$SpawnReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CreatureSpawnEventSpawnReason object, got {}",
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
                "NATURAL" => Ok(CreatureSpawnEventSpawnReason::Natural {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "JOCKEY" => Ok(CreatureSpawnEventSpawnReason::Jockey {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "CHUNK_GEN" => Ok(CreatureSpawnEventSpawnReason::ChunkGen {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SPAWNER" => Ok(CreatureSpawnEventSpawnReason::Spawner {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "EGG" => Ok(CreatureSpawnEventSpawnReason::Egg {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SPAWNER_EGG" => Ok(CreatureSpawnEventSpawnReason::SpawnerEgg {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "LIGHTNING" => Ok(CreatureSpawnEventSpawnReason::Lightning {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "BUILD_SNOWMAN" => Ok(CreatureSpawnEventSpawnReason::BuildSnowman {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "BUILD_IRONGOLEM" => Ok(CreatureSpawnEventSpawnReason::BuildIrongolem {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "BUILD_WITHER" => Ok(CreatureSpawnEventSpawnReason::BuildWither {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_DEFENSE" => Ok(CreatureSpawnEventSpawnReason::VillageDefense {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_INVASION" => Ok(CreatureSpawnEventSpawnReason::VillageInvasion {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "BREEDING" => Ok(CreatureSpawnEventSpawnReason::Breeding {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SLIME_SPLIT" => Ok(CreatureSpawnEventSpawnReason::SlimeSplit {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "REINFORCEMENTS" => Ok(CreatureSpawnEventSpawnReason::Reinforcements {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "NETHER_PORTAL" => Ok(CreatureSpawnEventSpawnReason::NetherPortal {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "DISPENSE_EGG" => Ok(CreatureSpawnEventSpawnReason::DispenseEgg {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "INFECTION" => Ok(CreatureSpawnEventSpawnReason::Infection {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "CURED" => Ok(CreatureSpawnEventSpawnReason::Cured {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "OCELOT_BABY" => Ok(CreatureSpawnEventSpawnReason::OcelotBaby {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SILVERFISH_BLOCK" => Ok(CreatureSpawnEventSpawnReason::SilverfishBlock {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "MOUNT" => Ok(CreatureSpawnEventSpawnReason::Mount {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "TRAP" => Ok(CreatureSpawnEventSpawnReason::Trap {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "ENDER_PEARL" => Ok(CreatureSpawnEventSpawnReason::EnderPearl {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SHOULDER_ENTITY" => Ok(CreatureSpawnEventSpawnReason::ShoulderEntity {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "DROWNED" => Ok(CreatureSpawnEventSpawnReason::Drowned {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SHEARED" => Ok(CreatureSpawnEventSpawnReason::Sheared {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "EXPLOSION" => Ok(CreatureSpawnEventSpawnReason::Explosion {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "RAID" => Ok(CreatureSpawnEventSpawnReason::Raid {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "PATROL" => Ok(CreatureSpawnEventSpawnReason::Patrol {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "BEEHIVE" => Ok(CreatureSpawnEventSpawnReason::Beehive {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "PIGLIN_ZOMBIFIED" => Ok(CreatureSpawnEventSpawnReason::PiglinZombified {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SPELL" => Ok(CreatureSpawnEventSpawnReason::Spell {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "FROZEN" => Ok(CreatureSpawnEventSpawnReason::Frozen {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "METAMORPHOSIS" => Ok(CreatureSpawnEventSpawnReason::Metamorphosis {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "DUPLICATION" => Ok(CreatureSpawnEventSpawnReason::Duplication {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "COMMAND" => Ok(CreatureSpawnEventSpawnReason::Command {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "CUSTOM" => Ok(CreatureSpawnEventSpawnReason::Custom {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "DEFAULT" => Ok(CreatureSpawnEventSpawnReason::Default {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for CreatureSpawnEventSpawnReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CreatureSpawnEventSpawnReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CreatureSpawnEventSpawnReasonStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/CreatureSpawnEvent$SpawnReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CreatureSpawnEventSpawnReasonStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CreatureSpawnEventSpawnReasonStruct<'mc> {
    //Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for CreatureSpawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CreatureSpawnEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CreatureSpawnEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/CreatureSpawnEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CreatureSpawnEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CreatureSpawnEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
        arg1: impl Into<crate::event::entity::CreatureSpawnEventSpawnReason<'mc>>,
    ) -> Result<crate::event::entity::CreatureSpawnEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/event/entity/CreatureSpawnEvent$SpawnReason;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/CreatureSpawnEvent");
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
        crate::event::entity::CreatureSpawnEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/LivingEntity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn spawn_reason(
        &self,
    ) -> Result<crate::event::entity::CreatureSpawnEventSpawnReason<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/CreatureSpawnEvent$SpawnReason;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnReason", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::CreatureSpawnEventSpawnReason::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntitySpawnEvent
    //crate::event::entity::EntitySpawnEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
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
        crate::event::entity::EntitySpawnEvent::handler_list(jni)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.location()
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntitySpawnEvent<'mc>> for CreatureSpawnEvent<'mc> {
    fn into(self) -> crate::event::entity::EntitySpawnEvent<'mc> {
        crate::event::entity::EntitySpawnEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting CreatureSpawnEvent into crate::event::entity::EntitySpawnEvent",
        )
    }
}
/// Called when an item is spawned into a world
#[repr(C)]
pub struct ItemSpawnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ItemSpawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ItemSpawnEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ItemSpawnEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/ItemSpawnEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemSpawnEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ItemSpawnEvent<'mc> {
    #[deprecated]

    pub fn new_with_item(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Item<'mc>>,
        arg1: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<crate::event::entity::ItemSpawnEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Item;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/Location;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/ItemSpawnEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::ItemSpawnEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Item;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Item::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntitySpawnEvent
    //crate::event::entity::EntitySpawnEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
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
        crate::event::entity::EntitySpawnEvent::handler_list(jni)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.location()
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntitySpawnEvent<'mc>> for ItemSpawnEvent<'mc> {
    fn into(self) -> crate::event::entity::EntitySpawnEvent<'mc> {
        crate::event::entity::EntitySpawnEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ItemSpawnEvent into crate::event::entity::EntitySpawnEvent")
    }
}
pub enum DamageModifier<'mc> {
    Base { inner: DamageModifierStruct<'mc> },
    HardHat { inner: DamageModifierStruct<'mc> },
    Blocking { inner: DamageModifierStruct<'mc> },
    Armor { inner: DamageModifierStruct<'mc> },
    Resistance { inner: DamageModifierStruct<'mc> },
    Magic { inner: DamageModifierStruct<'mc> },
    Absorption { inner: DamageModifierStruct<'mc> },
}
impl<'mc> std::fmt::Display for DamageModifier<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DamageModifier::Base { .. } => f.write_str("BASE"),
            DamageModifier::HardHat { .. } => f.write_str("HARD_HAT"),
            DamageModifier::Blocking { .. } => f.write_str("BLOCKING"),
            DamageModifier::Armor { .. } => f.write_str("ARMOR"),
            DamageModifier::Resistance { .. } => f.write_str("RESISTANCE"),
            DamageModifier::Magic { .. } => f.write_str("MAGIC"),
            DamageModifier::Absorption { .. } => f.write_str("ABSORPTION"),
        }
    }
}

impl<'mc> DamageModifier<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<DamageModifier<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/DamageModifier");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/DamageModifier;",
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
            "BASE" => Ok(DamageModifier::Base {
                inner: DamageModifierStruct::from_raw(env, obj)?,
            }),
            "HARD_HAT" => Ok(DamageModifier::HardHat {
                inner: DamageModifierStruct::from_raw(env, obj)?,
            }),
            "BLOCKING" => Ok(DamageModifier::Blocking {
                inner: DamageModifierStruct::from_raw(env, obj)?,
            }),
            "ARMOR" => Ok(DamageModifier::Armor {
                inner: DamageModifierStruct::from_raw(env, obj)?,
            }),
            "RESISTANCE" => Ok(DamageModifier::Resistance {
                inner: DamageModifierStruct::from_raw(env, obj)?,
            }),
            "MAGIC" => Ok(DamageModifier::Magic {
                inner: DamageModifierStruct::from_raw(env, obj)?,
            }),
            "ABSORPTION" => Ok(DamageModifier::Absorption {
                inner: DamageModifierStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct DamageModifierStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DamageModifier<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Base { inner } => inner.0.clone(),
            Self::HardHat { inner } => inner.0.clone(),
            Self::Blocking { inner } => inner.0.clone(),
            Self::Armor { inner } => inner.0.clone(),
            Self::Resistance { inner } => inner.0.clone(),
            Self::Magic { inner } => inner.0.clone(),
            Self::Absorption { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Base { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::HardHat { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Blocking { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Armor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Resistance { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Magic { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Absorption { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DamageModifier<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DamageModifier from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/DamageModifier")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DamageModifier object, got {}",
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
                "BASE" => Ok(DamageModifier::Base {
                    inner: DamageModifierStruct::from_raw(env, obj)?,
                }),
                "HARD_HAT" => Ok(DamageModifier::HardHat {
                    inner: DamageModifierStruct::from_raw(env, obj)?,
                }),
                "BLOCKING" => Ok(DamageModifier::Blocking {
                    inner: DamageModifierStruct::from_raw(env, obj)?,
                }),
                "ARMOR" => Ok(DamageModifier::Armor {
                    inner: DamageModifierStruct::from_raw(env, obj)?,
                }),
                "RESISTANCE" => Ok(DamageModifier::Resistance {
                    inner: DamageModifierStruct::from_raw(env, obj)?,
                }),
                "MAGIC" => Ok(DamageModifier::Magic {
                    inner: DamageModifierStruct::from_raw(env, obj)?,
                }),
                "ABSORPTION" => Ok(DamageModifier::Absorption {
                    inner: DamageModifierStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for DamageModifierStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DamageModifierStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DamageModifierStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/DamageModifier")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DamageModifierStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DamageModifierStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Thrown when an entity creates an item drop.
#[repr(C)]
pub struct EntityDropItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityDropItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDropItemEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityDropItemEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityDropItemEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityDropItemEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityDropItemEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::entity::Item<'mc>>,
    ) -> Result<crate::event::entity::EntityDropItemEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Item;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityDropItemEvent");
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
        crate::event::entity::EntityDropItemEvent::from_raw(&jni, res)
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

    pub fn item_drop(&self) -> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Item;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemDrop", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Item::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityDropItemEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityDropItemEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityDropItemEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityDropItemEvent into crate::event::entity::EntityEvent")
    }
}
pub enum Cause<'mc> {
    AreaEffectCloud { inner: CauseStruct<'mc> },
    Arrow { inner: CauseStruct<'mc> },
    Attack { inner: CauseStruct<'mc> },
    Axolotl { inner: CauseStruct<'mc> },
    Beacon { inner: CauseStruct<'mc> },
    Command { inner: CauseStruct<'mc> },
    Conduit { inner: CauseStruct<'mc> },
    Conversion { inner: CauseStruct<'mc> },
    Death { inner: CauseStruct<'mc> },
    Dolphin { inner: CauseStruct<'mc> },
    Expiration { inner: CauseStruct<'mc> },
    Food { inner: CauseStruct<'mc> },
    Illusion { inner: CauseStruct<'mc> },
    Milk { inner: CauseStruct<'mc> },
    PatrolCaptain { inner: CauseStruct<'mc> },
    Plugin { inner: CauseStruct<'mc> },
    PotionDrink { inner: CauseStruct<'mc> },
    PotionSplash { inner: CauseStruct<'mc> },
    SpiderSpawn { inner: CauseStruct<'mc> },
    Totem { inner: CauseStruct<'mc> },
    TurtleHelmet { inner: CauseStruct<'mc> },
    Unknown { inner: CauseStruct<'mc> },
    VillagerTrade { inner: CauseStruct<'mc> },
    Warden { inner: CauseStruct<'mc> },
    WitherRose { inner: CauseStruct<'mc> },
}
impl<'mc> std::fmt::Display for Cause<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cause::AreaEffectCloud { .. } => f.write_str("AREA_EFFECT_CLOUD"),
            Cause::Arrow { .. } => f.write_str("ARROW"),
            Cause::Attack { .. } => f.write_str("ATTACK"),
            Cause::Axolotl { .. } => f.write_str("AXOLOTL"),
            Cause::Beacon { .. } => f.write_str("BEACON"),
            Cause::Command { .. } => f.write_str("COMMAND"),
            Cause::Conduit { .. } => f.write_str("CONDUIT"),
            Cause::Conversion { .. } => f.write_str("CONVERSION"),
            Cause::Death { .. } => f.write_str("DEATH"),
            Cause::Dolphin { .. } => f.write_str("DOLPHIN"),
            Cause::Expiration { .. } => f.write_str("EXPIRATION"),
            Cause::Food { .. } => f.write_str("FOOD"),
            Cause::Illusion { .. } => f.write_str("ILLUSION"),
            Cause::Milk { .. } => f.write_str("MILK"),
            Cause::PatrolCaptain { .. } => f.write_str("PATROL_CAPTAIN"),
            Cause::Plugin { .. } => f.write_str("PLUGIN"),
            Cause::PotionDrink { .. } => f.write_str("POTION_DRINK"),
            Cause::PotionSplash { .. } => f.write_str("POTION_SPLASH"),
            Cause::SpiderSpawn { .. } => f.write_str("SPIDER_SPAWN"),
            Cause::Totem { .. } => f.write_str("TOTEM"),
            Cause::TurtleHelmet { .. } => f.write_str("TURTLE_HELMET"),
            Cause::Unknown { .. } => f.write_str("UNKNOWN"),
            Cause::VillagerTrade { .. } => f.write_str("VILLAGER_TRADE"),
            Cause::Warden { .. } => f.write_str("WARDEN"),
            Cause::WitherRose { .. } => f.write_str("WITHER_ROSE"),
        }
    }
}

impl<'mc> Cause<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Cause<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/Cause");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/Cause;",
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
            "AREA_EFFECT_CLOUD" => Ok(Cause::AreaEffectCloud {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "ARROW" => Ok(Cause::Arrow {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "ATTACK" => Ok(Cause::Attack {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "AXOLOTL" => Ok(Cause::Axolotl {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "BEACON" => Ok(Cause::Beacon {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "COMMAND" => Ok(Cause::Command {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "CONDUIT" => Ok(Cause::Conduit {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "CONVERSION" => Ok(Cause::Conversion {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "DEATH" => Ok(Cause::Death {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "DOLPHIN" => Ok(Cause::Dolphin {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "EXPIRATION" => Ok(Cause::Expiration {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "FOOD" => Ok(Cause::Food {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "ILLUSION" => Ok(Cause::Illusion {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "MILK" => Ok(Cause::Milk {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "PATROL_CAPTAIN" => Ok(Cause::PatrolCaptain {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "PLUGIN" => Ok(Cause::Plugin {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "POTION_DRINK" => Ok(Cause::PotionDrink {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "POTION_SPLASH" => Ok(Cause::PotionSplash {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "SPIDER_SPAWN" => Ok(Cause::SpiderSpawn {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "TOTEM" => Ok(Cause::Totem {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "TURTLE_HELMET" => Ok(Cause::TurtleHelmet {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(Cause::Unknown {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "VILLAGER_TRADE" => Ok(Cause::VillagerTrade {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "WARDEN" => Ok(Cause::Warden {
                inner: CauseStruct::from_raw(env, obj)?,
            }),
            "WITHER_ROSE" => Ok(Cause::WitherRose {
                inner: CauseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct CauseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Cause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::AreaEffectCloud { inner } => inner.0.clone(),
            Self::Arrow { inner } => inner.0.clone(),
            Self::Attack { inner } => inner.0.clone(),
            Self::Axolotl { inner } => inner.0.clone(),
            Self::Beacon { inner } => inner.0.clone(),
            Self::Command { inner } => inner.0.clone(),
            Self::Conduit { inner } => inner.0.clone(),
            Self::Conversion { inner } => inner.0.clone(),
            Self::Death { inner } => inner.0.clone(),
            Self::Dolphin { inner } => inner.0.clone(),
            Self::Expiration { inner } => inner.0.clone(),
            Self::Food { inner } => inner.0.clone(),
            Self::Illusion { inner } => inner.0.clone(),
            Self::Milk { inner } => inner.0.clone(),
            Self::PatrolCaptain { inner } => inner.0.clone(),
            Self::Plugin { inner } => inner.0.clone(),
            Self::PotionDrink { inner } => inner.0.clone(),
            Self::PotionSplash { inner } => inner.0.clone(),
            Self::SpiderSpawn { inner } => inner.0.clone(),
            Self::Totem { inner } => inner.0.clone(),
            Self::TurtleHelmet { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
            Self::VillagerTrade { inner } => inner.0.clone(),
            Self::Warden { inner } => inner.0.clone(),
            Self::WitherRose { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::AreaEffectCloud { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Arrow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Attack { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Axolotl { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Beacon { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Command { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Conduit { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Conversion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Death { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Dolphin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Expiration { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Food { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Illusion { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Milk { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PatrolCaptain { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Plugin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PotionDrink { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PotionSplash { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SpiderSpawn { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Totem { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::TurtleHelmet { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::VillagerTrade { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Warden { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WitherRose { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Cause<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Cause from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/Cause")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Cause object, got {}",
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
                "AREA_EFFECT_CLOUD" => Ok(Cause::AreaEffectCloud {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "ARROW" => Ok(Cause::Arrow {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "ATTACK" => Ok(Cause::Attack {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "AXOLOTL" => Ok(Cause::Axolotl {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "BEACON" => Ok(Cause::Beacon {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "COMMAND" => Ok(Cause::Command {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "CONDUIT" => Ok(Cause::Conduit {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "CONVERSION" => Ok(Cause::Conversion {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "DEATH" => Ok(Cause::Death {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "DOLPHIN" => Ok(Cause::Dolphin {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "EXPIRATION" => Ok(Cause::Expiration {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "FOOD" => Ok(Cause::Food {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "ILLUSION" => Ok(Cause::Illusion {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "MILK" => Ok(Cause::Milk {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "PATROL_CAPTAIN" => Ok(Cause::PatrolCaptain {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "PLUGIN" => Ok(Cause::Plugin {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "POTION_DRINK" => Ok(Cause::PotionDrink {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "POTION_SPLASH" => Ok(Cause::PotionSplash {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "SPIDER_SPAWN" => Ok(Cause::SpiderSpawn {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "TOTEM" => Ok(Cause::Totem {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "TURTLE_HELMET" => Ok(Cause::TurtleHelmet {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(Cause::Unknown {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "VILLAGER_TRADE" => Ok(Cause::VillagerTrade {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "WARDEN" => Ok(Cause::Warden {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                "WITHER_ROSE" => Ok(Cause::WitherRose {
                    inner: CauseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for CauseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CauseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CauseStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/Cause")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CauseStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CauseStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Called when a Slime splits into smaller Slimes upon death
#[repr(C)]
pub struct SlimeSplitEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SlimeSplitEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SlimeSplitEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SlimeSplitEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/SlimeSplitEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SlimeSplitEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SlimeSplitEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Slime<'mc>>,
        arg1: i32,
    ) -> Result<crate::event::entity::SlimeSplitEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Slime;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("org/bukkit/event/entity/SlimeSplitEvent");
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
        crate::event::entity::SlimeSplitEvent::from_raw(&jni, res)
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

    pub fn count(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn entity(&self) -> Result<crate::entity::Slime<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Slime;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Slime::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    /// Sets how many smaller slimes will spawn on the split
    pub fn set_count(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for SlimeSplitEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SlimeSplitEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for SlimeSplitEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SlimeSplitEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when a human entity's food level changes
#[repr(C)]
pub struct FoodLevelChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FoodLevelChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FoodLevelChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FoodLevelChangeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/FoodLevelChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FoodLevelChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FoodLevelChangeEvent<'mc> {
    pub fn new_with_human_entity(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::HumanEntity<'mc>>,
        arg1: i32,
        arg2: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::event::entity::FoodLevelChangeEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/HumanEntity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1);
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/inventory/ItemStack;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/FoodLevelChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::FoodLevelChangeEvent::from_raw(&jni, res)
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

    pub fn item(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getItem", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn food_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFoodLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the resultant food level that the entity involved in this event should be set to
    pub fn set_food_level(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFoodLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for FoodLevelChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FoodLevelChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for FoodLevelChangeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FoodLevelChangeEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when an Entity targets a <a title="interface in org.bukkit.entity" href="../../entity/LivingEntity.html"><code>LivingEntity</code></a> and can only target LivingEntity's.
#[repr(C)]
pub struct EntityTargetLivingEntityEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum EntityTargetEventTargetReason<'mc> {
    TargetDied {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    ClosestPlayer {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    TargetAttackedEntity {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    PigZombieTarget {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    ForgotTarget {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    TargetAttackedOwner {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    OwnerAttackedTarget {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    RandomTarget {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    DefendVillage {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    TargetAttackedNearbyEntity {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    ReinforcementTarget {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    Collision {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    Custom {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    ClosestEntity {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    FollowLeader {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    Tempt {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    Unknown {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityTargetEventTargetReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityTargetEventTargetReason::TargetDied { .. } => f.write_str("TARGET_DIED"),
            EntityTargetEventTargetReason::ClosestPlayer { .. } => f.write_str("CLOSEST_PLAYER"),
            EntityTargetEventTargetReason::TargetAttackedEntity { .. } => {
                f.write_str("TARGET_ATTACKED_ENTITY")
            }
            EntityTargetEventTargetReason::PigZombieTarget { .. } => {
                f.write_str("PIG_ZOMBIE_TARGET")
            }
            EntityTargetEventTargetReason::ForgotTarget { .. } => f.write_str("FORGOT_TARGET"),
            EntityTargetEventTargetReason::TargetAttackedOwner { .. } => {
                f.write_str("TARGET_ATTACKED_OWNER")
            }
            EntityTargetEventTargetReason::OwnerAttackedTarget { .. } => {
                f.write_str("OWNER_ATTACKED_TARGET")
            }
            EntityTargetEventTargetReason::RandomTarget { .. } => f.write_str("RANDOM_TARGET"),
            EntityTargetEventTargetReason::DefendVillage { .. } => f.write_str("DEFEND_VILLAGE"),
            EntityTargetEventTargetReason::TargetAttackedNearbyEntity { .. } => {
                f.write_str("TARGET_ATTACKED_NEARBY_ENTITY")
            }
            EntityTargetEventTargetReason::ReinforcementTarget { .. } => {
                f.write_str("REINFORCEMENT_TARGET")
            }
            EntityTargetEventTargetReason::Collision { .. } => f.write_str("COLLISION"),
            EntityTargetEventTargetReason::Custom { .. } => f.write_str("CUSTOM"),
            EntityTargetEventTargetReason::ClosestEntity { .. } => f.write_str("CLOSEST_ENTITY"),
            EntityTargetEventTargetReason::FollowLeader { .. } => f.write_str("FOLLOW_LEADER"),
            EntityTargetEventTargetReason::Tempt { .. } => f.write_str("TEMPT"),
            EntityTargetEventTargetReason::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}

impl<'mc> EntityTargetEventTargetReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityTargetEventTargetReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityTargetEvent$TargetReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityTargetEvent$TargetReason;",
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
            "TARGET_DIED" => Ok(EntityTargetEventTargetReason::TargetDied {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "CLOSEST_PLAYER" => Ok(EntityTargetEventTargetReason::ClosestPlayer {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "TARGET_ATTACKED_ENTITY" => Ok(EntityTargetEventTargetReason::TargetAttackedEntity {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "PIG_ZOMBIE_TARGET" => Ok(EntityTargetEventTargetReason::PigZombieTarget {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "FORGOT_TARGET" => Ok(EntityTargetEventTargetReason::ForgotTarget {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "TARGET_ATTACKED_OWNER" => Ok(EntityTargetEventTargetReason::TargetAttackedOwner {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "OWNER_ATTACKED_TARGET" => Ok(EntityTargetEventTargetReason::OwnerAttackedTarget {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "RANDOM_TARGET" => Ok(EntityTargetEventTargetReason::RandomTarget {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "DEFEND_VILLAGE" => Ok(EntityTargetEventTargetReason::DefendVillage {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "TARGET_ATTACKED_NEARBY_ENTITY" => {
                Ok(EntityTargetEventTargetReason::TargetAttackedNearbyEntity {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                })
            }
            "REINFORCEMENT_TARGET" => Ok(EntityTargetEventTargetReason::ReinforcementTarget {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "COLLISION" => Ok(EntityTargetEventTargetReason::Collision {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "CUSTOM" => Ok(EntityTargetEventTargetReason::Custom {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "CLOSEST_ENTITY" => Ok(EntityTargetEventTargetReason::ClosestEntity {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "FOLLOW_LEADER" => Ok(EntityTargetEventTargetReason::FollowLeader {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "TEMPT" => Ok(EntityTargetEventTargetReason::Tempt {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(EntityTargetEventTargetReason::Unknown {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityTargetEventTargetReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityTargetEventTargetReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::TargetDied { inner } => inner.0.clone(),
            Self::ClosestPlayer { inner } => inner.0.clone(),
            Self::TargetAttackedEntity { inner } => inner.0.clone(),
            Self::PigZombieTarget { inner } => inner.0.clone(),
            Self::ForgotTarget { inner } => inner.0.clone(),
            Self::TargetAttackedOwner { inner } => inner.0.clone(),
            Self::OwnerAttackedTarget { inner } => inner.0.clone(),
            Self::RandomTarget { inner } => inner.0.clone(),
            Self::DefendVillage { inner } => inner.0.clone(),
            Self::TargetAttackedNearbyEntity { inner } => inner.0.clone(),
            Self::ReinforcementTarget { inner } => inner.0.clone(),
            Self::Collision { inner } => inner.0.clone(),
            Self::Custom { inner } => inner.0.clone(),
            Self::ClosestEntity { inner } => inner.0.clone(),
            Self::FollowLeader { inner } => inner.0.clone(),
            Self::Tempt { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::TargetDied { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ClosestPlayer { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TargetAttackedEntity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PigZombieTarget { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ForgotTarget { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TargetAttackedOwner { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OwnerAttackedTarget { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RandomTarget { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DefendVillage { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TargetAttackedNearbyEntity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ReinforcementTarget { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Collision { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ClosestEntity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FollowLeader { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Tempt { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTargetEventTargetReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityTargetEventTargetReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityTargetEvent$TargetReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityTargetEventTargetReason object, got {}",
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
                "TARGET_DIED" => Ok(EntityTargetEventTargetReason::TargetDied {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "CLOSEST_PLAYER" => Ok(EntityTargetEventTargetReason::ClosestPlayer {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "TARGET_ATTACKED_ENTITY" => {
                    Ok(EntityTargetEventTargetReason::TargetAttackedEntity {
                        inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                    })
                }
                "PIG_ZOMBIE_TARGET" => Ok(EntityTargetEventTargetReason::PigZombieTarget {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "FORGOT_TARGET" => Ok(EntityTargetEventTargetReason::ForgotTarget {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "TARGET_ATTACKED_OWNER" => Ok(EntityTargetEventTargetReason::TargetAttackedOwner {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "OWNER_ATTACKED_TARGET" => Ok(EntityTargetEventTargetReason::OwnerAttackedTarget {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "RANDOM_TARGET" => Ok(EntityTargetEventTargetReason::RandomTarget {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "DEFEND_VILLAGE" => Ok(EntityTargetEventTargetReason::DefendVillage {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "TARGET_ATTACKED_NEARBY_ENTITY" => {
                    Ok(EntityTargetEventTargetReason::TargetAttackedNearbyEntity {
                        inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                    })
                }
                "REINFORCEMENT_TARGET" => Ok(EntityTargetEventTargetReason::ReinforcementTarget {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "COLLISION" => Ok(EntityTargetEventTargetReason::Collision {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "CUSTOM" => Ok(EntityTargetEventTargetReason::Custom {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "CLOSEST_ENTITY" => Ok(EntityTargetEventTargetReason::ClosestEntity {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "FOLLOW_LEADER" => Ok(EntityTargetEventTargetReason::FollowLeader {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "TEMPT" => Ok(EntityTargetEventTargetReason::Tempt {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(EntityTargetEventTargetReason::Unknown {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityTargetEventTargetReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTargetEventTargetReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityTargetEventTargetReasonStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityTargetEvent$TargetReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityTargetEventTargetReasonStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityTargetEventTargetReasonStruct<'mc> {
    //Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for EntityTargetLivingEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTargetLivingEntityEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityTargetLivingEntityEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityTargetLivingEntityEvent",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityTargetLivingEntityEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityTargetLivingEntityEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::entity::LivingEntity<'mc>>,
        arg2: impl Into<crate::event::entity::EntityTargetEventTargetReason<'mc>>,
    ) -> Result<crate::event::entity::EntityTargetLivingEntityEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/event/entity/EntityTargetEvent$TargetReason;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityTargetLivingEntityEvent");
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
        crate::event::entity::EntityTargetLivingEntityEvent::from_raw(&jni, res)
    }

    pub fn target(
        &self,
    ) -> Result<Option<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/LivingEntity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTarget", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::LivingEntity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_target(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTarget",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //EntityTargetEvent
    //crate::event::entity::EntityTargetEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTargetEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTargetEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
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
    pub fn reason(
        &self,
    ) -> Result<
        Option<crate::event::entity::EntityTargetEventTargetReason<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let temp_clone = crate::event::entity::EntityTargetEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTargetEvent = temp_clone.into();
        real.reason()
    }
    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntityTargetEvent::handler_list(jni)
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityTargetEvent<'mc>>
    for EntityTargetLivingEntityEvent<'mc>
{
    fn into(self) -> crate::event::entity::EntityTargetEvent<'mc> {
        crate::event::entity::EntityTargetEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EntityTargetLivingEntityEvent into crate::event::entity::EntityTargetEvent")
    }
}
/// Called when a LivingEntity shoots a bow firing an arrow
#[repr(C)]
pub struct EntityShootBowEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityShootBowEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityShootBowEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityShootBowEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityShootBowEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityShootBowEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityShootBowEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<crate::inventory::ItemStack<'mc>>,
        arg3: impl Into<crate::entity::Entity<'mc>>,
        arg4: impl Into<crate::inventory::EquipmentSlot<'mc>>,
        arg5: f32,
        arg6: bool,
    ) -> Result<crate::event::entity::EntityShootBowEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/entity/Entity;Lorg/bukkit/inventory/EquipmentSlot;FZ)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg4.into().jni_object().clone())
        });
        let val_6 = jni::objects::JValueGen::Float(arg5);
        let val_7 = jni::objects::JValueGen::Bool(arg6.into());
        let cls = jni.find_class("org/bukkit/event/entity/EntityShootBowEvent");
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
                jni::objects::JValueGen::from(val_6),
                jni::objects::JValueGen::from(val_7),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityShootBowEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/LivingEntity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn hand(&self) -> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn bow(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBow", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn consumable(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getConsumable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn projectile(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getProjectile", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_projectile(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProjectile",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn force(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getForce", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Set whether or not the consumable item should be consumed in this event. If set to false, it is recommended that a call to <a href="../../entity/Player.html#updateInventory()"><code>Player.updateInventory()</code></a> is made as the client may disagree with the server's decision to not consume a consumable item.
    /// <p>This value is ignored for entities where items are not required (skeletons, pillagers, etc.) or with crossbows (as no item is being consumed).</p>
    pub fn set_consume_item(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setConsumeItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn should_consume_item(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldConsumeItem",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityShootBowEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityShootBowEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityShootBowEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityShootBowEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when an entity dies and may have the opportunity to be resurrected. Will be called in a cancelled state if the entity does not have a totem equipped.
#[repr(C)]
pub struct EntityResurrectEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityResurrectEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityResurrectEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityResurrectEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityResurrectEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityResurrectEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityResurrectEvent<'mc> {
    pub fn new_with_living_entity(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
        arg1: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>,
    ) -> Result<crate::event::entity::EntityResurrectEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/LivingEntity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/inventory/EquipmentSlot;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityResurrectEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityResurrectEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn hand(&self) -> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityResurrectEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityResurrectEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityResurrectEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityResurrectEvent into crate::event::entity::EntityEvent")
    }
}
/// Stores all data related to the bartering interaction with a piglin. This event can be triggered by a piglin picking up an item that's on its bartering list.
#[repr(C)]
pub struct PiglinBarterEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PiglinBarterEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PiglinBarterEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PiglinBarterEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/PiglinBarterEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PiglinBarterEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PiglinBarterEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Piglin<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: Vec<impl Into<crate::event::entity::PiglinBarterEvent<'mc>>>,
    ) -> Result<crate::event::entity::PiglinBarterEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/entity/Piglin;Lorg/bukkit/inventory/ItemStack;Ljava/util/List;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg2 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            jni.call_method(
                &raw_val_3,
                "add",
                "(Lorg/bukkit/event/entity/crate::event::entity::PiglinBarterEvent)V",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_3 = jni::objects::JValueGen::Object(raw_val_3);
        let cls = jni.find_class("org/bukkit/event/entity/PiglinBarterEvent");
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
        crate::event::entity::PiglinBarterEvent::from_raw(&jni, res)
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

    pub fn input(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getInput", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn entity(&self) -> Result<crate::entity::Piglin<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Piglin;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Piglin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn outcome(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOutcome", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PiglinBarterEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PiglinBarterEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for PiglinBarterEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PiglinBarterEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when a Pig Zombie is angered by another entity.
/// <p>If the event is cancelled, the pig zombie will not be angered.</p>
#[repr(C)]
pub struct PigZombieAngerEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PigZombieAngerEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PigZombieAngerEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PigZombieAngerEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/PigZombieAngerEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PigZombieAngerEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PigZombieAngerEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::PigZombie<'mc>>,
        arg1: impl Into<crate::entity::Entity<'mc>>,
        arg2: i32,
    ) -> Result<crate::event::entity::PigZombieAngerEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/PigZombie;Lorg/bukkit/entity/Entity;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(arg2);
        let cls = jni.find_class("org/bukkit/event/entity/PigZombieAngerEvent");
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
        crate::event::entity::PigZombieAngerEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::PigZombie<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/PigZombie;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::PigZombie::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn new_anger(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewAnger", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the new anger resulting from this event.
    pub fn set_new_anger(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNewAnger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn target(&self) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTarget", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PigZombieAngerEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PigZombieAngerEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for PigZombieAngerEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PigZombieAngerEvent into crate::event::entity::EntityEvent")
    }
}
/// Thrown when a Living Entity creates a portal in a world.
#[repr(C)]
pub struct EntityCreatePortalEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityCreatePortalEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityCreatePortalEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityCreatePortalEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityCreatePortalEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityCreatePortalEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityCreatePortalEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
        arg1: Vec<impl Into<crate::event::entity::EntityCreatePortalEvent<'mc>>>,
        arg2: impl Into<crate::PortalType<'mc>>,
    ) -> Result<crate::event::entity::EntityCreatePortalEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Lorg/bukkit/entity/LivingEntity;Ljava/util/List;Lorg/bukkit/PortalType;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            jni.call_method(
                &raw_val_2,
                "add",
                "(Lorg/bukkit/event/entity/crate::event::entity::EntityCreatePortalEvent)V",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityCreatePortalEvent");
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
        crate::event::entity::EntityCreatePortalEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/LivingEntity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn blocks(&self) -> Result<Vec<crate::block::BlockState<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlocks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::block::BlockState::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    /// <span class="deprecated-label">Deprecated.</span>
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

    pub fn portal_type(&self) -> Result<crate::PortalType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/PortalType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPortalType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::PortalType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityCreatePortalEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityCreatePortalEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityCreatePortalEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityCreatePortalEvent into crate::event::entity::EntityEvent",
        )
    }
}
/// Sent when an entity's swimming status is toggled.
#[repr(C)]
pub struct EntityToggleSwimEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityToggleSwimEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityToggleSwimEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityToggleSwimEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityToggleSwimEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityToggleSwimEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityToggleSwimEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
        arg1: bool,
    ) -> Result<crate::event::entity::EntityToggleSwimEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let cls = jni.find_class("org/bukkit/event/entity/EntityToggleSwimEvent");
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
        crate::event::entity::EntityToggleSwimEvent::from_raw(&jni, res)
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

    pub fn is_swimming(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSwimming", sig.as_str(), vec![]);
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityToggleSwimEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityToggleSwimEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityToggleSwimEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityToggleSwimEvent into crate::event::entity::EntityEvent")
    }
}
/// Thrown when a non-player entity is teleported from one location to another.
///
/// This may be as a result of natural causes (Enderman, Shulker), pathfinding (Wolf), or commands (/teleport).
#[repr(C)]
pub struct EntityTeleportEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityTeleportEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTeleportEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityTeleportEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityTeleportEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityTeleportEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityTeleportEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::Location<'mc>>,
        arg2: impl Into<crate::Location<'mc>>,
    ) -> Result<crate::event::entity::EntityTeleportEvent<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/Location;Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityTeleportEvent");
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
        crate::event::entity::EntityTeleportEvent::from_raw(&jni, res)
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

    pub fn from(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFrom", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_from(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFrom",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn to(&self) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTo", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }

    pub fn set_to(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityTeleportEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTeleportEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityTeleportEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTeleportEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when an entity enters love mode.
///
/// This can be cancelled but the item will still be consumed that was used to make the entity enter into love mode.
#[repr(C)]
pub struct EntityEnterLoveModeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityEnterLoveModeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityEnterLoveModeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityEnterLoveModeEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityEnterLoveModeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityEnterLoveModeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityEnterLoveModeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Animals<'mc>>,
        arg1: impl Into<crate::entity::HumanEntity<'mc>>,
        arg2: i32,
    ) -> Result<crate::event::entity::EntityEnterLoveModeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/Animals;Lorg/bukkit/entity/HumanEntity;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(arg2);
        let cls = jni.find_class("org/bukkit/event/entity/EntityEnterLoveModeEvent");
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
        crate::event::entity::EntityEnterLoveModeEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn human_entity(
        &self,
    ) -> Result<Option<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHumanEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::HumanEntity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn ticks_in_love(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksInLove", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the amount of ticks that the animal will fall in love for.
    pub fn set_ticks_in_love(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksInLove",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityEnterLoveModeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityEnterLoveModeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityEnterLoveModeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityEnterLoveModeEvent into crate::event::entity::EntityEvent",
        )
    }
}
/// Called when a sheep's wool is dyed
#[repr(C)]
pub struct SheepDyeWoolEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SheepDyeWoolEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SheepDyeWoolEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SheepDyeWoolEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/SheepDyeWoolEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SheepDyeWoolEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SheepDyeWoolEvent<'mc> {
    pub fn new_with_sheep(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Sheep<'mc>>,
        arg1: impl Into<crate::DyeColor<'mc>>,
        arg2: std::option::Option<impl Into<crate::entity::Player<'mc>>>,
    ) -> Result<crate::event::entity::SheepDyeWoolEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Sheep;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/DyeColor;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/entity/Player;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/SheepDyeWoolEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::SheepDyeWoolEvent::from_raw(&jni, res)
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

    pub fn set_color(
        &self,
        arg0: impl Into<crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/DyeColor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
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

    pub fn color(&self) -> Result<Option<crate::DyeColor<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/DyeColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::DyeColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }

    pub fn player(&self) -> Result<Option<crate::entity::Player<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Player::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn entity(&self) -> Result<crate::entity::Sheep<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Sheep;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Sheep::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for SheepDyeWoolEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SheepDyeWoolEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for SheepDyeWoolEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SheepDyeWoolEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when an entity changes its pose.
#[repr(C)]
pub struct EntityPoseChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPoseChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPoseChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPoseChangeEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityPoseChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPoseChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPoseChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::entity::Pose<'mc>>,
    ) -> Result<crate::event::entity::EntityPoseChangeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Pose;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityPoseChangeEvent");
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
        crate::event::entity::EntityPoseChangeEvent::from_raw(&jni, res)
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

    pub fn pose(&self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Pose;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Pose::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityPoseChangeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityPoseChangeEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when an entity is damaged by an entity
#[repr(C)]
pub struct EntityDamageByEntityEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityDamageByEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDamageByEntityEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityDamageByEntityEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityDamageByEntityEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityDamageByEntityEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityDamageByEntityEvent<'mc> {
    pub fn new_with_entity(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::entity::Entity<'mc>>,
        arg2: impl Into<crate::event::entity::EntityDamageEventDamageCause<'mc>>,
        arg3: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
        arg4: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::event::entity::EntityDamageByEntityEvent<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Entity;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent$DamageCause;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        args.push(val_3);
        sig += "Ljava/util/Map;";
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        args.push(val_4);
        if let Some(a) = arg4 {
            sig += "Ljava/util/Map;";
            let val_5 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_5);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityDamageByEntityEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityDamageByEntityEvent::from_raw(&jni, res)
    }

    pub fn damager(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDamager", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    //EntityDamageEvent
    //crate::event::entity::EntityDamageEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
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
    pub fn set_damage_with_entity_damage_eventdamage_modifier(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
        arg1: std::option::Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a);
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setDamage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn get_damage(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.get_damage(arg0)
    }
    pub fn is_applicable(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.is_applicable(arg0)
    }
    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntityDamageEvent::handler_list(jni)
    }
    pub fn get_original_damage(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.get_original_damage(arg0)
    }
    pub fn final_damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.final_damage()
    }
    pub fn cause(
        &self,
    ) -> Result<crate::event::entity::EntityDamageEventDamageCause<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.cause()
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityDamageEvent<'mc>> for EntityDamageByEntityEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityDamageEvent<'mc> {
        crate::event::entity::EntityDamageEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EntityDamageByEntityEvent into crate::event::entity::EntityDamageEvent")
    }
}

#[repr(C)]
pub struct VillagerCareerChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum VillagerCareerChangeEventChangeReason<'mc> {
    LosingJob {
        inner: VillagerCareerChangeEventChangeReasonStruct<'mc>,
    },
    Employed {
        inner: VillagerCareerChangeEventChangeReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for VillagerCareerChangeEventChangeReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VillagerCareerChangeEventChangeReason::LosingJob { .. } => f.write_str("LOSING_JOB"),
            VillagerCareerChangeEventChangeReason::Employed { .. } => f.write_str("EMPLOYED"),
        }
    }
}

impl<'mc> VillagerCareerChangeEventChangeReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<VillagerCareerChangeEventChangeReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/VillagerCareerChangeEvent$ChangeReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/VillagerCareerChangeEvent$ChangeReason;",
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
            "LOSING_JOB" => Ok(VillagerCareerChangeEventChangeReason::LosingJob {
                inner: VillagerCareerChangeEventChangeReasonStruct::from_raw(env, obj)?,
            }),
            "EMPLOYED" => Ok(VillagerCareerChangeEventChangeReason::Employed {
                inner: VillagerCareerChangeEventChangeReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct VillagerCareerChangeEventChangeReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for VillagerCareerChangeEventChangeReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::LosingJob { inner } => inner.0.clone(),
            Self::Employed { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::LosingJob { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Employed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for VillagerCareerChangeEventChangeReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate VillagerCareerChangeEventChangeReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/VillagerCareerChangeEvent$ChangeReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a VillagerCareerChangeEventChangeReason object, got {}",
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
                "LOSING_JOB" => Ok(VillagerCareerChangeEventChangeReason::LosingJob {
                    inner: VillagerCareerChangeEventChangeReasonStruct::from_raw(env, obj)?,
                }),
                "EMPLOYED" => Ok(VillagerCareerChangeEventChangeReason::Employed {
                    inner: VillagerCareerChangeEventChangeReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for VillagerCareerChangeEventChangeReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for VillagerCareerChangeEventChangeReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                    "Tried to instantiate VillagerCareerChangeEventChangeReasonStruct from null object.")
                .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/VillagerCareerChangeEvent$ChangeReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a VillagerCareerChangeEventChangeReasonStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> VillagerCareerChangeEventChangeReasonStruct<'mc> {
    //Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for VillagerCareerChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for VillagerCareerChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate VillagerCareerChangeEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/VillagerCareerChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VillagerCareerChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> VillagerCareerChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Villager<'mc>>,
        arg1: impl Into<crate::entity::VillagerProfession<'mc>>,
        arg2: impl Into<crate::event::entity::VillagerCareerChangeEventChangeReason<'mc>>,
    ) -> Result<crate::event::entity::VillagerCareerChangeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/Villager;Lorg/bukkit/entity/Villager$Profession;Lorg/bukkit/event/entity/VillagerCareerChangeEvent$ChangeReason;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/VillagerCareerChangeEvent");
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
        crate::event::entity::VillagerCareerChangeEvent::from_raw(&jni, res)
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

    pub fn reason(
        &self,
    ) -> Result<
        Option<crate::event::entity::VillagerCareerChangeEventChangeReason<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig =
            String::from("()Lorg/bukkit/event/entity/VillagerCareerChangeEvent$ChangeReason;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getReason", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            crate::event::entity::VillagerCareerChangeEventChangeReason::from_raw(
                &self.jni_ref(),
                unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
            )?,
        ))
    }

    pub fn entity(&self) -> Result<crate::entity::Villager<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Villager;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Villager::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn profession(
        &self,
    ) -> Result<crate::entity::VillagerProfession<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Villager$Profession;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getProfession", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::VillagerProfession::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_profession(
        &self,
        arg0: impl Into<crate::entity::VillagerProfession<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Villager$Profession;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProfession",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for VillagerCareerChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting VillagerCareerChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for VillagerCareerChangeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting VillagerCareerChangeEvent into crate::event::entity::EntityEvent",
        )
    }
}
/// Represents an Entity-related event
#[repr(C)]
pub struct EntityEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EntityEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<crate::event::entity::EntityEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntityType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntityType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EntityType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Event<'mc>> for EntityEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityEvent into crate::event::Event")
    }
}
/// Called when a potion effect is modified on an entity.
/// <p>If the event is cancelled, no change will be made on the entity.</p>
#[repr(C)]
pub struct EntityPotionEffectEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum EntityPotionEffectEventAction<'mc> {
    Added {
        inner: EntityPotionEffectEventActionStruct<'mc>,
    },
    Changed {
        inner: EntityPotionEffectEventActionStruct<'mc>,
    },
    Cleared {
        inner: EntityPotionEffectEventActionStruct<'mc>,
    },
    Removed {
        inner: EntityPotionEffectEventActionStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityPotionEffectEventAction<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityPotionEffectEventAction::Added { .. } => f.write_str("ADDED"),
            EntityPotionEffectEventAction::Changed { .. } => f.write_str("CHANGED"),
            EntityPotionEffectEventAction::Cleared { .. } => f.write_str("CLEARED"),
            EntityPotionEffectEventAction::Removed { .. } => f.write_str("REMOVED"),
        }
    }
}

impl<'mc> EntityPotionEffectEventAction<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityPotionEffectEventAction<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityPotionEffectEvent$Action");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityPotionEffectEvent$Action;",
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
            "ADDED" => Ok(EntityPotionEffectEventAction::Added {
                inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
            }),
            "CHANGED" => Ok(EntityPotionEffectEventAction::Changed {
                inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
            }),
            "CLEARED" => Ok(EntityPotionEffectEventAction::Cleared {
                inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
            }),
            "REMOVED" => Ok(EntityPotionEffectEventAction::Removed {
                inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityPotionEffectEventActionStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPotionEffectEventAction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Added { inner } => inner.0.clone(),
            Self::Changed { inner } => inner.0.clone(),
            Self::Cleared { inner } => inner.0.clone(),
            Self::Removed { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Added { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Changed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cleared { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Removed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPotionEffectEventAction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPotionEffectEventAction from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityPotionEffectEvent$Action",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPotionEffectEventAction object, got {}",
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
                "ADDED" => Ok(EntityPotionEffectEventAction::Added {
                    inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
                }),
                "CHANGED" => Ok(EntityPotionEffectEventAction::Changed {
                    inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
                }),
                "CLEARED" => Ok(EntityPotionEffectEventAction::Cleared {
                    inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
                }),
                "REMOVED" => Ok(EntityPotionEffectEventAction::Removed {
                    inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityPotionEffectEventActionStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPotionEffectEventActionStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPotionEffectEventActionStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityPotionEffectEvent$Action",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityPotionEffectEventActionStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPotionEffectEventActionStruct<'mc> {
    //Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum EntityPotionEffectEventCause<'mc> {
    AreaEffectCloud {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Arrow {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Attack {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Axolotl {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Beacon {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Command {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Conduit {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Conversion {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Death {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Dolphin {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Expiration {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Food {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Illusion {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Milk {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    PatrolCaptain {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Plugin {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    PotionDrink {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    PotionSplash {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    SpiderSpawn {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Totem {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    TurtleHelmet {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Unknown {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    VillagerTrade {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Warden {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    WitherRose {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityPotionEffectEventCause<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityPotionEffectEventCause::AreaEffectCloud { .. } => {
                f.write_str("AREA_EFFECT_CLOUD")
            }
            EntityPotionEffectEventCause::Arrow { .. } => f.write_str("ARROW"),
            EntityPotionEffectEventCause::Attack { .. } => f.write_str("ATTACK"),
            EntityPotionEffectEventCause::Axolotl { .. } => f.write_str("AXOLOTL"),
            EntityPotionEffectEventCause::Beacon { .. } => f.write_str("BEACON"),
            EntityPotionEffectEventCause::Command { .. } => f.write_str("COMMAND"),
            EntityPotionEffectEventCause::Conduit { .. } => f.write_str("CONDUIT"),
            EntityPotionEffectEventCause::Conversion { .. } => f.write_str("CONVERSION"),
            EntityPotionEffectEventCause::Death { .. } => f.write_str("DEATH"),
            EntityPotionEffectEventCause::Dolphin { .. } => f.write_str("DOLPHIN"),
            EntityPotionEffectEventCause::Expiration { .. } => f.write_str("EXPIRATION"),
            EntityPotionEffectEventCause::Food { .. } => f.write_str("FOOD"),
            EntityPotionEffectEventCause::Illusion { .. } => f.write_str("ILLUSION"),
            EntityPotionEffectEventCause::Milk { .. } => f.write_str("MILK"),
            EntityPotionEffectEventCause::PatrolCaptain { .. } => f.write_str("PATROL_CAPTAIN"),
            EntityPotionEffectEventCause::Plugin { .. } => f.write_str("PLUGIN"),
            EntityPotionEffectEventCause::PotionDrink { .. } => f.write_str("POTION_DRINK"),
            EntityPotionEffectEventCause::PotionSplash { .. } => f.write_str("POTION_SPLASH"),
            EntityPotionEffectEventCause::SpiderSpawn { .. } => f.write_str("SPIDER_SPAWN"),
            EntityPotionEffectEventCause::Totem { .. } => f.write_str("TOTEM"),
            EntityPotionEffectEventCause::TurtleHelmet { .. } => f.write_str("TURTLE_HELMET"),
            EntityPotionEffectEventCause::Unknown { .. } => f.write_str("UNKNOWN"),
            EntityPotionEffectEventCause::VillagerTrade { .. } => f.write_str("VILLAGER_TRADE"),
            EntityPotionEffectEventCause::Warden { .. } => f.write_str("WARDEN"),
            EntityPotionEffectEventCause::WitherRose { .. } => f.write_str("WITHER_ROSE"),
        }
    }
}

impl<'mc> EntityPotionEffectEventCause<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityPotionEffectEventCause<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityPotionEffectEvent$Cause");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityPotionEffectEvent$Cause;",
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
            "AREA_EFFECT_CLOUD" => Ok(EntityPotionEffectEventCause::AreaEffectCloud {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "ARROW" => Ok(EntityPotionEffectEventCause::Arrow {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "ATTACK" => Ok(EntityPotionEffectEventCause::Attack {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "AXOLOTL" => Ok(EntityPotionEffectEventCause::Axolotl {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "BEACON" => Ok(EntityPotionEffectEventCause::Beacon {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "COMMAND" => Ok(EntityPotionEffectEventCause::Command {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "CONDUIT" => Ok(EntityPotionEffectEventCause::Conduit {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "CONVERSION" => Ok(EntityPotionEffectEventCause::Conversion {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "DEATH" => Ok(EntityPotionEffectEventCause::Death {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "DOLPHIN" => Ok(EntityPotionEffectEventCause::Dolphin {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "EXPIRATION" => Ok(EntityPotionEffectEventCause::Expiration {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "FOOD" => Ok(EntityPotionEffectEventCause::Food {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "ILLUSION" => Ok(EntityPotionEffectEventCause::Illusion {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "MILK" => Ok(EntityPotionEffectEventCause::Milk {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "PATROL_CAPTAIN" => Ok(EntityPotionEffectEventCause::PatrolCaptain {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "PLUGIN" => Ok(EntityPotionEffectEventCause::Plugin {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "POTION_DRINK" => Ok(EntityPotionEffectEventCause::PotionDrink {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "POTION_SPLASH" => Ok(EntityPotionEffectEventCause::PotionSplash {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "SPIDER_SPAWN" => Ok(EntityPotionEffectEventCause::SpiderSpawn {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "TOTEM" => Ok(EntityPotionEffectEventCause::Totem {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "TURTLE_HELMET" => Ok(EntityPotionEffectEventCause::TurtleHelmet {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(EntityPotionEffectEventCause::Unknown {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "VILLAGER_TRADE" => Ok(EntityPotionEffectEventCause::VillagerTrade {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "WARDEN" => Ok(EntityPotionEffectEventCause::Warden {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "WITHER_ROSE" => Ok(EntityPotionEffectEventCause::WitherRose {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityPotionEffectEventCauseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPotionEffectEventCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::AreaEffectCloud { inner } => inner.0.clone(),
            Self::Arrow { inner } => inner.0.clone(),
            Self::Attack { inner } => inner.0.clone(),
            Self::Axolotl { inner } => inner.0.clone(),
            Self::Beacon { inner } => inner.0.clone(),
            Self::Command { inner } => inner.0.clone(),
            Self::Conduit { inner } => inner.0.clone(),
            Self::Conversion { inner } => inner.0.clone(),
            Self::Death { inner } => inner.0.clone(),
            Self::Dolphin { inner } => inner.0.clone(),
            Self::Expiration { inner } => inner.0.clone(),
            Self::Food { inner } => inner.0.clone(),
            Self::Illusion { inner } => inner.0.clone(),
            Self::Milk { inner } => inner.0.clone(),
            Self::PatrolCaptain { inner } => inner.0.clone(),
            Self::Plugin { inner } => inner.0.clone(),
            Self::PotionDrink { inner } => inner.0.clone(),
            Self::PotionSplash { inner } => inner.0.clone(),
            Self::SpiderSpawn { inner } => inner.0.clone(),
            Self::Totem { inner } => inner.0.clone(),
            Self::TurtleHelmet { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
            Self::VillagerTrade { inner } => inner.0.clone(),
            Self::Warden { inner } => inner.0.clone(),
            Self::WitherRose { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::AreaEffectCloud { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Arrow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Attack { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Axolotl { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Beacon { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Command { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Conduit { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Conversion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Death { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Dolphin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Expiration { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Food { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Illusion { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Milk { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PatrolCaptain { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Plugin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PotionDrink { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PotionSplash { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SpiderSpawn { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Totem { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::TurtleHelmet { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::VillagerTrade { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Warden { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WitherRose { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPotionEffectEventCause<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPotionEffectEventCause from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityPotionEffectEvent$Cause",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPotionEffectEventCause object, got {}",
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
                "AREA_EFFECT_CLOUD" => Ok(EntityPotionEffectEventCause::AreaEffectCloud {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "ARROW" => Ok(EntityPotionEffectEventCause::Arrow {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "ATTACK" => Ok(EntityPotionEffectEventCause::Attack {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "AXOLOTL" => Ok(EntityPotionEffectEventCause::Axolotl {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "BEACON" => Ok(EntityPotionEffectEventCause::Beacon {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "COMMAND" => Ok(EntityPotionEffectEventCause::Command {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "CONDUIT" => Ok(EntityPotionEffectEventCause::Conduit {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "CONVERSION" => Ok(EntityPotionEffectEventCause::Conversion {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "DEATH" => Ok(EntityPotionEffectEventCause::Death {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "DOLPHIN" => Ok(EntityPotionEffectEventCause::Dolphin {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "EXPIRATION" => Ok(EntityPotionEffectEventCause::Expiration {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "FOOD" => Ok(EntityPotionEffectEventCause::Food {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "ILLUSION" => Ok(EntityPotionEffectEventCause::Illusion {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "MILK" => Ok(EntityPotionEffectEventCause::Milk {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "PATROL_CAPTAIN" => Ok(EntityPotionEffectEventCause::PatrolCaptain {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "PLUGIN" => Ok(EntityPotionEffectEventCause::Plugin {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "POTION_DRINK" => Ok(EntityPotionEffectEventCause::PotionDrink {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "POTION_SPLASH" => Ok(EntityPotionEffectEventCause::PotionSplash {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "SPIDER_SPAWN" => Ok(EntityPotionEffectEventCause::SpiderSpawn {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "TOTEM" => Ok(EntityPotionEffectEventCause::Totem {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "TURTLE_HELMET" => Ok(EntityPotionEffectEventCause::TurtleHelmet {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(EntityPotionEffectEventCause::Unknown {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "VILLAGER_TRADE" => Ok(EntityPotionEffectEventCause::VillagerTrade {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "WARDEN" => Ok(EntityPotionEffectEventCause::Warden {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "WITHER_ROSE" => Ok(EntityPotionEffectEventCause::WitherRose {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityPotionEffectEventCauseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPotionEffectEventCauseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPotionEffectEventCauseStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityPotionEffectEvent$Cause",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityPotionEffectEventCauseStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPotionEffectEventCauseStruct<'mc> {
    //Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for EntityPotionEffectEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPotionEffectEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPotionEffectEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityPotionEffectEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPotionEffectEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPotionEffectEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
        arg1: impl Into<crate::potion::PotionEffect<'mc>>,
        arg2: impl Into<crate::potion::PotionEffect<'mc>>,
        arg3: impl Into<crate::event::entity::EntityPotionEffectEventCause<'mc>>,
        arg4: impl Into<crate::event::entity::EntityPotionEffectEventAction<'mc>>,
        arg5: bool,
    ) -> Result<crate::event::entity::EntityPotionEffectEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/potion/PotionEffect;Lorg/bukkit/potion/PotionEffect;Lorg/bukkit/event/entity/EntityPotionEffectEvent$Cause;Lorg/bukkit/event/entity/EntityPotionEffectEvent$Action;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg4.into().jni_object().clone())
        });
        let val_6 = jni::objects::JValueGen::Bool(arg5.into());
        let cls = jni.find_class("org/bukkit/event/entity/EntityPotionEffectEvent");
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
                jni::objects::JValueGen::from(val_6),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityPotionEffectEvent::from_raw(&jni, res)
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

    pub fn action(
        &self,
    ) -> Result<crate::event::entity::EntityPotionEffectEventAction<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityPotionEffectEvent$Action;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAction", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityPotionEffectEventAction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn old_effect(
        &self,
    ) -> Result<Option<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffect;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOldEffect", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::potion::PotionEffect::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn new_effect(
        &self,
    ) -> Result<Option<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffect;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewEffect", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::potion::PotionEffect::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn modified_type(
        &self,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getModifiedType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffectType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_override(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOverride", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets if the new potion effect will override the old potion effect (Only applicable for the CHANGED action).
    pub fn set_override(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOverride",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cause(
        &self,
    ) -> Result<crate::event::entity::EntityPotionEffectEventCause<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityPotionEffectEvent$Cause;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCause", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityPotionEffectEventCause::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityPotionEffectEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityPotionEffectEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityPotionEffectEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityPotionEffectEvent into crate::event::entity::EntityEvent",
        )
    }
}
pub enum RegainReason<'mc> {
    Regen { inner: RegainReasonStruct<'mc> },
    Satiated { inner: RegainReasonStruct<'mc> },
    Eating { inner: RegainReasonStruct<'mc> },
    EnderCrystal { inner: RegainReasonStruct<'mc> },
    Magic { inner: RegainReasonStruct<'mc> },
    MagicRegen { inner: RegainReasonStruct<'mc> },
    WitherSpawn { inner: RegainReasonStruct<'mc> },
    Wither { inner: RegainReasonStruct<'mc> },
    Custom { inner: RegainReasonStruct<'mc> },
}
impl<'mc> std::fmt::Display for RegainReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegainReason::Regen { .. } => f.write_str("REGEN"),
            RegainReason::Satiated { .. } => f.write_str("SATIATED"),
            RegainReason::Eating { .. } => f.write_str("EATING"),
            RegainReason::EnderCrystal { .. } => f.write_str("ENDER_CRYSTAL"),
            RegainReason::Magic { .. } => f.write_str("MAGIC"),
            RegainReason::MagicRegen { .. } => f.write_str("MAGIC_REGEN"),
            RegainReason::WitherSpawn { .. } => f.write_str("WITHER_SPAWN"),
            RegainReason::Wither { .. } => f.write_str("WITHER"),
            RegainReason::Custom { .. } => f.write_str("CUSTOM"),
        }
    }
}

impl<'mc> RegainReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<RegainReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/RegainReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/RegainReason;",
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
            "REGEN" => Ok(RegainReason::Regen {
                inner: RegainReasonStruct::from_raw(env, obj)?,
            }),
            "SATIATED" => Ok(RegainReason::Satiated {
                inner: RegainReasonStruct::from_raw(env, obj)?,
            }),
            "EATING" => Ok(RegainReason::Eating {
                inner: RegainReasonStruct::from_raw(env, obj)?,
            }),
            "ENDER_CRYSTAL" => Ok(RegainReason::EnderCrystal {
                inner: RegainReasonStruct::from_raw(env, obj)?,
            }),
            "MAGIC" => Ok(RegainReason::Magic {
                inner: RegainReasonStruct::from_raw(env, obj)?,
            }),
            "MAGIC_REGEN" => Ok(RegainReason::MagicRegen {
                inner: RegainReasonStruct::from_raw(env, obj)?,
            }),
            "WITHER_SPAWN" => Ok(RegainReason::WitherSpawn {
                inner: RegainReasonStruct::from_raw(env, obj)?,
            }),
            "WITHER" => Ok(RegainReason::Wither {
                inner: RegainReasonStruct::from_raw(env, obj)?,
            }),
            "CUSTOM" => Ok(RegainReason::Custom {
                inner: RegainReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct RegainReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for RegainReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Regen { inner } => inner.0.clone(),
            Self::Satiated { inner } => inner.0.clone(),
            Self::Eating { inner } => inner.0.clone(),
            Self::EnderCrystal { inner } => inner.0.clone(),
            Self::Magic { inner } => inner.0.clone(),
            Self::MagicRegen { inner } => inner.0.clone(),
            Self::WitherSpawn { inner } => inner.0.clone(),
            Self::Wither { inner } => inner.0.clone(),
            Self::Custom { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Regen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Satiated { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Eating { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::EnderCrystal { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Magic { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::MagicRegen { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WitherSpawn { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Wither { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RegainReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate RegainReason from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/RegainReason")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RegainReason object, got {}",
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
                "REGEN" => Ok(RegainReason::Regen {
                    inner: RegainReasonStruct::from_raw(env, obj)?,
                }),
                "SATIATED" => Ok(RegainReason::Satiated {
                    inner: RegainReasonStruct::from_raw(env, obj)?,
                }),
                "EATING" => Ok(RegainReason::Eating {
                    inner: RegainReasonStruct::from_raw(env, obj)?,
                }),
                "ENDER_CRYSTAL" => Ok(RegainReason::EnderCrystal {
                    inner: RegainReasonStruct::from_raw(env, obj)?,
                }),
                "MAGIC" => Ok(RegainReason::Magic {
                    inner: RegainReasonStruct::from_raw(env, obj)?,
                }),
                "MAGIC_REGEN" => Ok(RegainReason::MagicRegen {
                    inner: RegainReasonStruct::from_raw(env, obj)?,
                }),
                "WITHER_SPAWN" => Ok(RegainReason::WitherSpawn {
                    inner: RegainReasonStruct::from_raw(env, obj)?,
                }),
                "WITHER" => Ok(RegainReason::Wither {
                    inner: RegainReasonStruct::from_raw(env, obj)?,
                }),
                "CUSTOM" => Ok(RegainReason::Custom {
                    inner: RegainReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for RegainReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RegainReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate RegainReasonStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/RegainReason")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RegainReasonStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> RegainReasonStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

#[repr(C)]
pub struct ItemMergeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ItemMergeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ItemMergeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ItemMergeEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/ItemMergeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemMergeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ItemMergeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Item<'mc>>,
        arg1: impl Into<crate::entity::Item<'mc>>,
    ) -> Result<crate::event::entity::ItemMergeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Item;Lorg/bukkit/entity/Item;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/ItemMergeEvent");
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
        crate::event::entity::ItemMergeEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Item;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Item::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn target(&self) -> Result<Option<crate::entity::Item<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Item;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTarget", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Item::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for ItemMergeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ItemMergeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for ItemMergeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ItemMergeEvent into crate::event::entity::EntityEvent")
    }
}
/// Called whenever a villager acquires a new trade.
#[repr(C)]
pub struct VillagerAcquireTradeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for VillagerAcquireTradeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for VillagerAcquireTradeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate VillagerAcquireTradeEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/VillagerAcquireTradeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VillagerAcquireTradeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> VillagerAcquireTradeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::AbstractVillager<'mc>>,
        arg1: impl Into<crate::inventory::MerchantRecipe<'mc>>,
    ) -> Result<crate::event::entity::VillagerAcquireTradeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Lorg/bukkit/entity/AbstractVillager;Lorg/bukkit/inventory/MerchantRecipe;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/VillagerAcquireTradeEvent");
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
        crate::event::entity::VillagerAcquireTradeEvent::from_raw(&jni, res)
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

    pub fn recipe(
        &self,
    ) -> Result<Option<crate::inventory::MerchantRecipe<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/MerchantRecipe;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRecipe", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::MerchantRecipe::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_recipe(
        &self,
        arg0: impl Into<crate::inventory::MerchantRecipe<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/MerchantRecipe;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRecipe",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for VillagerAcquireTradeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting VillagerAcquireTradeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for VillagerAcquireTradeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting VillagerAcquireTradeEvent into crate::event::entity::EntityEvent",
        )
    }
}
/// Called when a splash potion hits an area
#[repr(C)]
pub struct LingeringPotionSplashEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LingeringPotionSplashEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LingeringPotionSplashEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate LingeringPotionSplashEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/LingeringPotionSplashEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LingeringPotionSplashEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LingeringPotionSplashEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::ThrownPotion<'mc>>,
        arg1: impl Into<crate::entity::AreaEffectCloud<'mc>>,
    ) -> Result<crate::event::entity::LingeringPotionSplashEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig =
            String::from("(Lorg/bukkit/entity/ThrownPotion;Lorg/bukkit/entity/AreaEffectCloud;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/LingeringPotionSplashEvent");
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
        crate::event::entity::LingeringPotionSplashEvent::from_raw(&jni, res)
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

    pub fn area_effect_cloud(
        &self,
    ) -> Result<crate::entity::AreaEffectCloud<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/AreaEffectCloud;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAreaEffectCloud",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::AreaEffectCloud::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="ProjectileHitEvent.html#setCancelled(boolean)">ProjectileHitEvent</a></code></span>
    /// Whether to cancel the action that occurs when the projectile hits. In the case of an entity, it will not collide (unless it's a firework, then use <a href="FireworkExplodeEvent.html" title="class in org.bukkit.event.entity"><code>FireworkExplodeEvent</code></a>).
    ///
    /// In the case of a block, some blocks (eg target block, bell) will not perform the action associated.
    ///
    /// This does NOT prevent block collisions, and explosions will still occur unless their respective events are cancelled.
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
    //ProjectileHitEvent
    //crate::event::entity::ProjectileHitEvent
    pub fn hit_block(
        &self,
    ) -> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_block()
    }
    pub fn hit_block_face(
        &self,
    ) -> Result<Option<crate::block::BlockFace<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_block_face()
    }
    pub fn hit_entity(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_entity()
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for LingeringPotionSplashEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LingeringPotionSplashEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::ProjectileHitEvent<'mc>> for LingeringPotionSplashEvent<'mc> {
    fn into(self) -> crate::event::entity::ProjectileHitEvent<'mc> {
        crate::event::entity::ProjectileHitEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting LingeringPotionSplashEvent into crate::event::entity::ProjectileHitEvent")
    }
}
pub enum UnleashReason<'mc> {
    HolderGone { inner: UnleashReasonStruct<'mc> },
    PlayerUnleash { inner: UnleashReasonStruct<'mc> },
    Distance { inner: UnleashReasonStruct<'mc> },
    Unknown { inner: UnleashReasonStruct<'mc> },
}
impl<'mc> std::fmt::Display for UnleashReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnleashReason::HolderGone { .. } => f.write_str("HOLDER_GONE"),
            UnleashReason::PlayerUnleash { .. } => f.write_str("PLAYER_UNLEASH"),
            UnleashReason::Distance { .. } => f.write_str("DISTANCE"),
            UnleashReason::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}

impl<'mc> UnleashReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<UnleashReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/UnleashReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/UnleashReason;",
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
            "HOLDER_GONE" => Ok(UnleashReason::HolderGone {
                inner: UnleashReasonStruct::from_raw(env, obj)?,
            }),
            "PLAYER_UNLEASH" => Ok(UnleashReason::PlayerUnleash {
                inner: UnleashReasonStruct::from_raw(env, obj)?,
            }),
            "DISTANCE" => Ok(UnleashReason::Distance {
                inner: UnleashReasonStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(UnleashReason::Unknown {
                inner: UnleashReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct UnleashReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for UnleashReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::HolderGone { inner } => inner.0.clone(),
            Self::PlayerUnleash { inner } => inner.0.clone(),
            Self::Distance { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::HolderGone { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PlayerUnleash { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Distance { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for UnleashReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate UnleashReason from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/UnleashReason")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a UnleashReason object, got {}",
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
                "HOLDER_GONE" => Ok(UnleashReason::HolderGone {
                    inner: UnleashReasonStruct::from_raw(env, obj)?,
                }),
                "PLAYER_UNLEASH" => Ok(UnleashReason::PlayerUnleash {
                    inner: UnleashReasonStruct::from_raw(env, obj)?,
                }),
                "DISTANCE" => Ok(UnleashReason::Distance {
                    inner: UnleashReasonStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(UnleashReason::Unknown {
                    inner: UnleashReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for UnleashReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for UnleashReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate UnleashReasonStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/UnleashReason")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a UnleashReasonStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> UnleashReasonStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Thrown when a LivingEntity is tamed
#[repr(C)]
pub struct EntityTameEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityTameEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTameEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityTameEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityTameEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityTameEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityTameEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
        arg1: impl Into<crate::entity::AnimalTamer<'mc>>,
    ) -> Result<crate::event::entity::EntityTameEvent<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/AnimalTamer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityTameEvent");
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
        crate::event::entity::EntityTameEvent::from_raw(&jni, res)
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

    pub fn owner(
        &self,
    ) -> Result<Option<crate::entity::AnimalTamer<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/AnimalTamer;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getOwner", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::AnimalTamer::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/LivingEntity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityTameEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTameEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityTameEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTameEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when an <a title="interface in org.bukkit.entity" href="../../entity/Entity.html"><code>Entity</code></a> breaks a door
/// <p>Cancelling the event will cause the event to be delayed</p>
#[repr(C)]
pub struct EntityBreakDoorEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityBreakDoorEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityBreakDoorEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityBreakDoorEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityBreakDoorEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityBreakDoorEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityBreakDoorEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
        arg1: impl Into<crate::block::Block<'mc>>,
    ) -> Result<crate::event::entity::EntityBreakDoorEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/block/Block;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityBreakDoorEvent");
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
        crate::event::entity::EntityBreakDoorEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntityChangeBlockEvent
    //crate::event::entity::EntityChangeBlockEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityChangeBlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityChangeBlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
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
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityChangeBlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityChangeBlockEvent = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityChangeBlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityChangeBlockEvent = temp_clone.into();
        real.block_data()
    }
    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntityChangeBlockEvent::handler_list(jni)
    }
    pub fn to(&self) -> Result<Option<crate::Material<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityChangeBlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityChangeBlockEvent = temp_clone.into();
        real.to()
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityChangeBlockEvent<'mc>> for EntityBreakDoorEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityChangeBlockEvent<'mc> {
        crate::event::entity::EntityChangeBlockEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EntityBreakDoorEvent into crate::event::entity::EntityChangeBlockEvent")
    }
}
/// Called when one Entity breeds with another Entity.
#[repr(C)]
pub struct EntityBreedEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityBreedEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityBreedEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityBreedEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityBreedEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityBreedEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityBreedEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
        arg1: impl Into<crate::entity::LivingEntity<'mc>>,
        arg2: impl Into<crate::entity::LivingEntity<'mc>>,
        arg3: impl Into<crate::entity::LivingEntity<'mc>>,
        arg4: impl Into<crate::inventory::ItemStack<'mc>>,
        arg5: i32,
    ) -> Result<crate::event::entity::EntityBreedEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/inventory/ItemStack;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg4.into().jni_object().clone())
        });
        let val_6 = jni::objects::JValueGen::Int(arg5);
        let cls = jni.find_class("org/bukkit/event/entity/EntityBreedEvent");
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
                jni::objects::JValueGen::from(val_6),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityBreedEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn experience(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getExperience", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the amount of experience granted by breeding.
    pub fn set_experience(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExperience",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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

    pub fn mother(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMother", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn father(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFather", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn breeder(
        &self,
    ) -> Result<Option<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBreeder", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::LivingEntity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn bred_with(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBredWith", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityBreedEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityBreedEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityBreedEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityBreedEvent into crate::event::entity::EntityEvent")
    }
}
/// Triggered when a entity is created in the world by a player "placing" an item on a block.
///
/// Note that this event is currently only fired for four specific placements: armor stands, boats, minecarts, and end crystals.
#[repr(C)]
pub struct EntityPlaceEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPlaceEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPlaceEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityPlaceEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityPlaceEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPlaceEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPlaceEvent<'mc> {
    pub fn new_with_entity(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::entity::Player<'mc>>,
        arg2: impl Into<crate::block::Block<'mc>>,
        arg3: impl Into<crate::block::BlockFace<'mc>>,
        arg4: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>,
    ) -> Result<crate::event::entity::EntityPlaceEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Player;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/block/Block;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        args.push(val_3);
        sig += "Lorg/bukkit/block/BlockFace;";
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        args.push(val_4);
        if let Some(a) = arg4 {
            sig += "Lorg/bukkit/inventory/EquipmentSlot;";
            let val_5 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_5);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityPlaceEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityPlaceEvent::from_raw(&jni, res)
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

    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn player(&self) -> Result<Option<crate::entity::Player<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Player::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
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

    pub fn hand(&self) -> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn block_face(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlockFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityPlaceEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityPlaceEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityPlaceEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityPlaceEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when a block causes an entity to combust.
#[repr(C)]
pub struct EntityCombustByBlockEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityCombustByBlockEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityCombustByBlockEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityCombustByBlockEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityCombustByBlockEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityCombustByBlockEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityCombustByBlockEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::block::Block<'mc>>,
        arg1: impl Into<crate::entity::Entity<'mc>>,
        arg2: i32,
    ) -> Result<crate::event::entity::EntityCombustByBlockEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/entity/Entity;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(arg2);
        let cls = jni.find_class("org/bukkit/event/entity/EntityCombustByBlockEvent");
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
        crate::event::entity::EntityCombustByBlockEvent::from_raw(&jni, res)
    }

    pub fn combuster(
        &self,
    ) -> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCombuster", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::Block::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    //EntityCombustEvent
    //crate::event::entity::EntityCombustEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
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
    pub fn duration(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityCombustEvent = temp_clone.into();
        real.duration()
    }
    pub fn set_duration(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityCombustEvent = temp_clone.into();
        real.set_duration(arg0)
    }
    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntityCombustEvent::handler_list(jni)
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityCombustEvent<'mc>> for EntityCombustByBlockEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityCombustEvent<'mc> {
        crate::event::entity::EntityCombustEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EntityCombustByBlockEvent into crate::event::entity::EntityCombustEvent")
    }
}
/// This event is called when a <a title="interface in org.bukkit.entity" href="../../entity/Item.html"><code>Item</code></a> is removed from the world because it has existed for 5 minutes.
/// <p>Cancelling the event results in the item being allowed to exist for 5 more minutes. This behavior is not guaranteed and may change in future versions.</p>
#[repr(C)]
pub struct ItemDespawnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ItemDespawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ItemDespawnEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ItemDespawnEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/ItemDespawnEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemDespawnEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ItemDespawnEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Item<'mc>>,
        arg1: impl Into<crate::Location<'mc>>,
    ) -> Result<crate::event::entity::ItemDespawnEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Item;Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/ItemDespawnEvent");
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
        crate::event::entity::ItemDespawnEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Item;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Item::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for ItemDespawnEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ItemDespawnEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for ItemDespawnEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ItemDespawnEvent into crate::event::entity::EntityEvent")
    }
}
/// Stores data for health-regain events
#[repr(C)]
pub struct EntityRegainHealthEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum EntityRegainHealthEventRegainReason<'mc> {
    Regen {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    Satiated {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    Eating {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    EnderCrystal {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    Magic {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    MagicRegen {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    WitherSpawn {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    Wither {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    Custom {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityRegainHealthEventRegainReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityRegainHealthEventRegainReason::Regen { .. } => f.write_str("REGEN"),
            EntityRegainHealthEventRegainReason::Satiated { .. } => f.write_str("SATIATED"),
            EntityRegainHealthEventRegainReason::Eating { .. } => f.write_str("EATING"),
            EntityRegainHealthEventRegainReason::EnderCrystal { .. } => {
                f.write_str("ENDER_CRYSTAL")
            }
            EntityRegainHealthEventRegainReason::Magic { .. } => f.write_str("MAGIC"),
            EntityRegainHealthEventRegainReason::MagicRegen { .. } => f.write_str("MAGIC_REGEN"),
            EntityRegainHealthEventRegainReason::WitherSpawn { .. } => f.write_str("WITHER_SPAWN"),
            EntityRegainHealthEventRegainReason::Wither { .. } => f.write_str("WITHER"),
            EntityRegainHealthEventRegainReason::Custom { .. } => f.write_str("CUSTOM"),
        }
    }
}

impl<'mc> EntityRegainHealthEventRegainReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityRegainHealthEventRegainReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityRegainHealthEvent$RegainReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityRegainHealthEvent$RegainReason;",
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
            "REGEN" => Ok(EntityRegainHealthEventRegainReason::Regen {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "SATIATED" => Ok(EntityRegainHealthEventRegainReason::Satiated {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "EATING" => Ok(EntityRegainHealthEventRegainReason::Eating {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "ENDER_CRYSTAL" => Ok(EntityRegainHealthEventRegainReason::EnderCrystal {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "MAGIC" => Ok(EntityRegainHealthEventRegainReason::Magic {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "MAGIC_REGEN" => Ok(EntityRegainHealthEventRegainReason::MagicRegen {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "WITHER_SPAWN" => Ok(EntityRegainHealthEventRegainReason::WitherSpawn {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "WITHER" => Ok(EntityRegainHealthEventRegainReason::Wither {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "CUSTOM" => Ok(EntityRegainHealthEventRegainReason::Custom {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityRegainHealthEventRegainReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityRegainHealthEventRegainReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Regen { inner } => inner.0.clone(),
            Self::Satiated { inner } => inner.0.clone(),
            Self::Eating { inner } => inner.0.clone(),
            Self::EnderCrystal { inner } => inner.0.clone(),
            Self::Magic { inner } => inner.0.clone(),
            Self::MagicRegen { inner } => inner.0.clone(),
            Self::WitherSpawn { inner } => inner.0.clone(),
            Self::Wither { inner } => inner.0.clone(),
            Self::Custom { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Regen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Satiated { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Eating { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::EnderCrystal { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Magic { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::MagicRegen { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WitherSpawn { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Wither { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityRegainHealthEventRegainReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityRegainHealthEventRegainReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityRegainHealthEvent$RegainReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityRegainHealthEventRegainReason object, got {}",
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
                "REGEN" => Ok(EntityRegainHealthEventRegainReason::Regen {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "SATIATED" => Ok(EntityRegainHealthEventRegainReason::Satiated {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "EATING" => Ok(EntityRegainHealthEventRegainReason::Eating {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "ENDER_CRYSTAL" => Ok(EntityRegainHealthEventRegainReason::EnderCrystal {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "MAGIC" => Ok(EntityRegainHealthEventRegainReason::Magic {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "MAGIC_REGEN" => Ok(EntityRegainHealthEventRegainReason::MagicRegen {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "WITHER_SPAWN" => Ok(EntityRegainHealthEventRegainReason::WitherSpawn {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "WITHER" => Ok(EntityRegainHealthEventRegainReason::Wither {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "CUSTOM" => Ok(EntityRegainHealthEventRegainReason::Custom {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityRegainHealthEventRegainReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityRegainHealthEventRegainReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityRegainHealthEventRegainReasonStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityRegainHealthEvent$RegainReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityRegainHealthEventRegainReasonStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityRegainHealthEventRegainReasonStruct<'mc> {
    //Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for EntityRegainHealthEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityRegainHealthEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityRegainHealthEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityRegainHealthEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityRegainHealthEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityRegainHealthEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: f64,
        arg2: impl Into<crate::event::entity::EntityRegainHealthEventRegainReason<'mc>>,
    ) -> Result<crate::event::entity::EntityRegainHealthEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/Entity;DLorg/bukkit/event/entity/EntityRegainHealthEvent$RegainReason;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Double(arg1);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityRegainHealthEvent");
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
        crate::event::entity::EntityRegainHealthEvent::from_raw(&jni, res)
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

    pub fn amount(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Sets the amount of regained health
    pub fn set_amount(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAmount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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

    pub fn regain_reason(
        &self,
    ) -> Result<
        crate::event::entity::EntityRegainHealthEventRegainReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityRegainHealthEvent$RegainReason;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRegainReason", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityRegainHealthEventRegainReason::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityRegainHealthEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityRegainHealthEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityRegainHealthEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityRegainHealthEvent into crate::event::entity::EntityEvent",
        )
    }
}
/// Called when a bat attempts to sleep or wake up from its slumber.
/// <p>If a Bat Toggle Sleep event is cancelled, the Bat will not toggle its sleep state.</p>
#[repr(C)]
pub struct BatToggleSleepEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BatToggleSleepEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BatToggleSleepEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BatToggleSleepEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/BatToggleSleepEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BatToggleSleepEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BatToggleSleepEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Bat<'mc>>,
        arg1: bool,
    ) -> Result<crate::event::entity::BatToggleSleepEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Bat;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let cls = jni.find_class("org/bukkit/event/entity/BatToggleSleepEvent");
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
        crate::event::entity::BatToggleSleepEvent::from_raw(&jni, res)
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

    pub fn is_awake(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isAwake", sig.as_str(), vec![]);
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BatToggleSleepEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BatToggleSleepEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for BatToggleSleepEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BatToggleSleepEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when a projectile is launched.
#[repr(C)]
pub struct ProjectileLaunchEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ProjectileLaunchEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ProjectileLaunchEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ProjectileLaunchEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/ProjectileLaunchEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ProjectileLaunchEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ProjectileLaunchEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<crate::event::entity::ProjectileLaunchEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/ProjectileLaunchEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::ProjectileLaunchEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn entity(&self) -> Result<crate::entity::Projectile<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Projectile;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Projectile::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    //EntitySpawnEvent
    //crate::event::entity::EntitySpawnEvent
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
        crate::event::entity::EntitySpawnEvent::handler_list(jni)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.location()
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for ProjectileLaunchEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ProjectileLaunchEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntitySpawnEvent<'mc>> for ProjectileLaunchEvent<'mc> {
    fn into(self) -> crate::event::entity::EntitySpawnEvent<'mc> {
        crate::event::entity::EntitySpawnEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting ProjectileLaunchEvent into crate::event::entity::EntitySpawnEvent",
        )
    }
}
pub enum ExhaustionReason<'mc> {
    BlockMined { inner: ExhaustionReasonStruct<'mc> },
    HungerEffect { inner: ExhaustionReasonStruct<'mc> },
    Damaged { inner: ExhaustionReasonStruct<'mc> },
    Attack { inner: ExhaustionReasonStruct<'mc> },
    JumpSprint { inner: ExhaustionReasonStruct<'mc> },
    Jump { inner: ExhaustionReasonStruct<'mc> },
    Swim { inner: ExhaustionReasonStruct<'mc> },
    WalkUnderwater { inner: ExhaustionReasonStruct<'mc> },
    WalkOnWater { inner: ExhaustionReasonStruct<'mc> },
    Sprint { inner: ExhaustionReasonStruct<'mc> },
    Crouch { inner: ExhaustionReasonStruct<'mc> },
    Walk { inner: ExhaustionReasonStruct<'mc> },
    Regen { inner: ExhaustionReasonStruct<'mc> },
    Unknown { inner: ExhaustionReasonStruct<'mc> },
}
impl<'mc> std::fmt::Display for ExhaustionReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExhaustionReason::BlockMined { .. } => f.write_str("BLOCK_MINED"),
            ExhaustionReason::HungerEffect { .. } => f.write_str("HUNGER_EFFECT"),
            ExhaustionReason::Damaged { .. } => f.write_str("DAMAGED"),
            ExhaustionReason::Attack { .. } => f.write_str("ATTACK"),
            ExhaustionReason::JumpSprint { .. } => f.write_str("JUMP_SPRINT"),
            ExhaustionReason::Jump { .. } => f.write_str("JUMP"),
            ExhaustionReason::Swim { .. } => f.write_str("SWIM"),
            ExhaustionReason::WalkUnderwater { .. } => f.write_str("WALK_UNDERWATER"),
            ExhaustionReason::WalkOnWater { .. } => f.write_str("WALK_ON_WATER"),
            ExhaustionReason::Sprint { .. } => f.write_str("SPRINT"),
            ExhaustionReason::Crouch { .. } => f.write_str("CROUCH"),
            ExhaustionReason::Walk { .. } => f.write_str("WALK"),
            ExhaustionReason::Regen { .. } => f.write_str("REGEN"),
            ExhaustionReason::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}

impl<'mc> ExhaustionReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ExhaustionReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/ExhaustionReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/ExhaustionReason;",
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
            "BLOCK_MINED" => Ok(ExhaustionReason::BlockMined {
                inner: ExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "HUNGER_EFFECT" => Ok(ExhaustionReason::HungerEffect {
                inner: ExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "DAMAGED" => Ok(ExhaustionReason::Damaged {
                inner: ExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "ATTACK" => Ok(ExhaustionReason::Attack {
                inner: ExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "JUMP_SPRINT" => Ok(ExhaustionReason::JumpSprint {
                inner: ExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "JUMP" => Ok(ExhaustionReason::Jump {
                inner: ExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "SWIM" => Ok(ExhaustionReason::Swim {
                inner: ExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "WALK_UNDERWATER" => Ok(ExhaustionReason::WalkUnderwater {
                inner: ExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "WALK_ON_WATER" => Ok(ExhaustionReason::WalkOnWater {
                inner: ExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "SPRINT" => Ok(ExhaustionReason::Sprint {
                inner: ExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "CROUCH" => Ok(ExhaustionReason::Crouch {
                inner: ExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "WALK" => Ok(ExhaustionReason::Walk {
                inner: ExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "REGEN" => Ok(ExhaustionReason::Regen {
                inner: ExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(ExhaustionReason::Unknown {
                inner: ExhaustionReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ExhaustionReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ExhaustionReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::BlockMined { inner } => inner.0.clone(),
            Self::HungerEffect { inner } => inner.0.clone(),
            Self::Damaged { inner } => inner.0.clone(),
            Self::Attack { inner } => inner.0.clone(),
            Self::JumpSprint { inner } => inner.0.clone(),
            Self::Jump { inner } => inner.0.clone(),
            Self::Swim { inner } => inner.0.clone(),
            Self::WalkUnderwater { inner } => inner.0.clone(),
            Self::WalkOnWater { inner } => inner.0.clone(),
            Self::Sprint { inner } => inner.0.clone(),
            Self::Crouch { inner } => inner.0.clone(),
            Self::Walk { inner } => inner.0.clone(),
            Self::Regen { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::BlockMined { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HungerEffect { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Damaged { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Attack { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::JumpSprint { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Jump { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Swim { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WalkUnderwater { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WalkOnWater { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Sprint { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Crouch { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Walk { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Regen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ExhaustionReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ExhaustionReason from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/ExhaustionReason")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ExhaustionReason object, got {}",
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
                "BLOCK_MINED" => Ok(ExhaustionReason::BlockMined {
                    inner: ExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "HUNGER_EFFECT" => Ok(ExhaustionReason::HungerEffect {
                    inner: ExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "DAMAGED" => Ok(ExhaustionReason::Damaged {
                    inner: ExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "ATTACK" => Ok(ExhaustionReason::Attack {
                    inner: ExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "JUMP_SPRINT" => Ok(ExhaustionReason::JumpSprint {
                    inner: ExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "JUMP" => Ok(ExhaustionReason::Jump {
                    inner: ExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "SWIM" => Ok(ExhaustionReason::Swim {
                    inner: ExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "WALK_UNDERWATER" => Ok(ExhaustionReason::WalkUnderwater {
                    inner: ExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "WALK_ON_WATER" => Ok(ExhaustionReason::WalkOnWater {
                    inner: ExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "SPRINT" => Ok(ExhaustionReason::Sprint {
                    inner: ExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "CROUCH" => Ok(ExhaustionReason::Crouch {
                    inner: ExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "WALK" => Ok(ExhaustionReason::Walk {
                    inner: ExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "REGEN" => Ok(ExhaustionReason::Regen {
                    inner: ExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(ExhaustionReason::Unknown {
                    inner: ExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ExhaustionReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ExhaustionReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ExhaustionReasonStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/ExhaustionReason")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ExhaustionReasonStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ExhaustionReasonStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Thrown whenever a <a title="interface in org.bukkit.entity" href="../../entity/Player.html"><code>Player</code></a> dies
#[repr(C)]
pub struct PlayerDeathEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PlayerDeathEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PlayerDeathEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PlayerDeathEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/PlayerDeathEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PlayerDeathEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PlayerDeathEvent<'mc> {
    pub fn new_with_player(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Player<'mc>>,
        arg1: Vec<impl Into<crate::event::entity::PlayerDeathEvent<'mc>>>,
        arg2: i32,
        arg3: i32,
        arg4: std::option::Option<i32>,
        arg5: std::option::Option<i32>,
        arg6: std::option::Option<impl Into<String>>,
    ) -> Result<crate::event::entity::PlayerDeathEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Player;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/util/List;";
        let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            sig += "Lorg/bukkit/event/entity/crate::event::entity::PlayerDeathEvent;";
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            jni.call_method(
                &raw_val_2,
                "add",
                "(Lorg/bukkit/event/entity/crate::event::entity::PlayerDeathEvent)V",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        args.push(val_2);
        sig += "I";
        let val_3 = jni::objects::JValueGen::Int(arg2);
        args.push(val_3);
        sig += "I";
        let val_4 = jni::objects::JValueGen::Int(arg3);
        args.push(val_4);
        if let Some(a) = arg4 {
            sig += "I";
            let val_5 = jni::objects::JValueGen::Int(a);
            args.push(val_5);
        }
        if let Some(a) = arg5 {
            sig += "I";
            let val_6 = jni::objects::JValueGen::Int(a);
            args.push(val_6);
        }
        if let Some(a) = arg6 {
            sig += "Ljava/lang/String;";
            let val_7 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(a.into())?,
            ));
            args.push(val_7);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/PlayerDeathEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::PlayerDeathEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Player;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Player::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn new_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    #[deprecated]
    /// Sets the Level the Player should have at respawn.
    pub fn set_new_level(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNewLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_death_message(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDeathMessage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn death_message(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDeathMessage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            self.jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
                .to_string_lossy()
                .to_string(),
        ))
    }

    pub fn new_exp(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getNewExp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets how much EXP the Player should have at respawn.
    /// <p>This does not indicate how much EXP should be dropped, please see <a href="EntityDeathEvent.html#setDroppedExp(int)"><code>EntityDeathEvent.setDroppedExp(int)</code></a> for that.</p>
    pub fn set_new_exp(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNewExp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn new_total_exp(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewTotalExp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the Total EXP the Player should have at respawn.
    pub fn set_new_total_exp(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNewTotalExp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn keep_level(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getKeepLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets if the Player should keep all EXP at respawn.
    /// <p>This overrides all other EXP settings</p>
    /// <p><b>This doesn't prevent the EXP from dropping. <a href="EntityDeathEvent.html#setDroppedExp(int)"><code>EntityDeathEvent.setDroppedExp(int)</code></a> should be used stop the EXP from dropping.</b></p>
    pub fn set_keep_level(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setKeepLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets if the Player keeps inventory on death.
    /// <p><b>This doesn't prevent the items from dropping. <code>getDrops().clear()</code> should be used stop the items from dropping.</b></p>
    pub fn set_keep_inventory(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setKeepInventory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn keep_inventory(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKeepInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //EntityDeathEvent
    //crate::event::entity::EntityDeathEvent
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
    pub fn drops(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDrops", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntityDeathEvent::handler_list(jni)
    }
    pub fn dropped_exp(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDeathEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDeathEvent = temp_clone.into();
        real.dropped_exp()
    }
    pub fn set_dropped_exp(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDeathEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDeathEvent = temp_clone.into();
        real.set_dropped_exp(arg0)
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityDeathEvent<'mc>> for PlayerDeathEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityDeathEvent<'mc> {
        crate::event::entity::EntityDeathEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PlayerDeathEvent into crate::event::entity::EntityDeathEvent")
    }
}
/// Called when a lingering potion applies it's effects. Happens once every 5 ticks
#[repr(C)]
pub struct AreaEffectCloudApplyEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for AreaEffectCloudApplyEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AreaEffectCloudApplyEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate AreaEffectCloudApplyEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/AreaEffectCloudApplyEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AreaEffectCloudApplyEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AreaEffectCloudApplyEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::AreaEffectCloud<'mc>>,
        arg1: Vec<impl Into<crate::event::entity::AreaEffectCloudApplyEvent<'mc>>>,
    ) -> Result<crate::event::entity::AreaEffectCloudApplyEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/AreaEffectCloud;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            jni.call_method(
                &raw_val_2,
                "add",
                "(Lorg/bukkit/event/entity/crate::event::entity::AreaEffectCloudApplyEvent)V",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let cls = jni.find_class("org/bukkit/event/entity/AreaEffectCloudApplyEvent");
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
        crate::event::entity::AreaEffectCloudApplyEvent::from_raw(&jni, res)
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

    pub fn entity(
        &self,
    ) -> Result<crate::entity::AreaEffectCloud<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/AreaEffectCloud;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::AreaEffectCloud::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn affected_entities(
        &self,
    ) -> Result<Vec<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAffectedEntities",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::LivingEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for AreaEffectCloudApplyEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting AreaEffectCloudApplyEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for AreaEffectCloudApplyEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting AreaEffectCloudApplyEvent into crate::event::entity::EntityEvent",
        )
    }
}
/// Called when any Entity changes a block and a more specific event is not available.
#[repr(C)]
pub struct EntityChangeBlockEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityChangeBlockEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityChangeBlockEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityChangeBlockEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityChangeBlockEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityChangeBlockEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityChangeBlockEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::block::Block<'mc>>,
        arg2: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::event::entity::EntityChangeBlockEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/Block;Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityChangeBlockEvent");
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
        crate::event::entity::EntityChangeBlockEvent::from_raw(&jni, res)
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

    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlockData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn to(&self) -> Result<Option<crate::Material<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTo", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityChangeBlockEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityChangeBlockEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityChangeBlockEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityChangeBlockEvent into crate::event::entity::EntityEvent",
        )
    }
}
/// Called when an entity causes another entity to combust.
#[repr(C)]
pub struct EntityCombustByEntityEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityCombustByEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityCombustByEntityEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityCombustByEntityEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityCombustByEntityEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityCombustByEntityEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityCombustByEntityEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::entity::Entity<'mc>>,
        arg2: i32,
    ) -> Result<crate::event::entity::EntityCombustByEntityEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Entity;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(arg2);
        let cls = jni.find_class("org/bukkit/event/entity/EntityCombustByEntityEvent");
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
        crate::event::entity::EntityCombustByEntityEvent::from_raw(&jni, res)
    }

    pub fn combuster(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCombuster", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    //EntityCombustEvent
    //crate::event::entity::EntityCombustEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
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
    pub fn duration(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityCombustEvent = temp_clone.into();
        real.duration()
    }
    pub fn set_duration(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityCombustEvent = temp_clone.into();
        real.set_duration(arg0)
    }
    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntityCombustEvent::handler_list(jni)
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityCombustEvent<'mc>> for EntityCombustByEntityEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityCombustEvent<'mc> {
        crate::event::entity::EntityCombustEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EntityCombustByEntityEvent into crate::event::entity::EntityCombustEvent")
    }
}
pub enum TransformReason<'mc> {
    Cured { inner: TransformReasonStruct<'mc> },
    Frozen { inner: TransformReasonStruct<'mc> },
    Infection { inner: TransformReasonStruct<'mc> },
    Drowned { inner: TransformReasonStruct<'mc> },
    Sheared { inner: TransformReasonStruct<'mc> },
    Lightning { inner: TransformReasonStruct<'mc> },
    Split { inner: TransformReasonStruct<'mc> },
    PiglinZombified { inner: TransformReasonStruct<'mc> },
    Metamorphosis { inner: TransformReasonStruct<'mc> },
    Unknown { inner: TransformReasonStruct<'mc> },
}
impl<'mc> std::fmt::Display for TransformReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransformReason::Cured { .. } => f.write_str("CURED"),
            TransformReason::Frozen { .. } => f.write_str("FROZEN"),
            TransformReason::Infection { .. } => f.write_str("INFECTION"),
            TransformReason::Drowned { .. } => f.write_str("DROWNED"),
            TransformReason::Sheared { .. } => f.write_str("SHEARED"),
            TransformReason::Lightning { .. } => f.write_str("LIGHTNING"),
            TransformReason::Split { .. } => f.write_str("SPLIT"),
            TransformReason::PiglinZombified { .. } => f.write_str("PIGLIN_ZOMBIFIED"),
            TransformReason::Metamorphosis { .. } => f.write_str("METAMORPHOSIS"),
            TransformReason::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}

impl<'mc> TransformReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<TransformReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/TransformReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/TransformReason;",
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
            "CURED" => Ok(TransformReason::Cured {
                inner: TransformReasonStruct::from_raw(env, obj)?,
            }),
            "FROZEN" => Ok(TransformReason::Frozen {
                inner: TransformReasonStruct::from_raw(env, obj)?,
            }),
            "INFECTION" => Ok(TransformReason::Infection {
                inner: TransformReasonStruct::from_raw(env, obj)?,
            }),
            "DROWNED" => Ok(TransformReason::Drowned {
                inner: TransformReasonStruct::from_raw(env, obj)?,
            }),
            "SHEARED" => Ok(TransformReason::Sheared {
                inner: TransformReasonStruct::from_raw(env, obj)?,
            }),
            "LIGHTNING" => Ok(TransformReason::Lightning {
                inner: TransformReasonStruct::from_raw(env, obj)?,
            }),
            "SPLIT" => Ok(TransformReason::Split {
                inner: TransformReasonStruct::from_raw(env, obj)?,
            }),
            "PIGLIN_ZOMBIFIED" => Ok(TransformReason::PiglinZombified {
                inner: TransformReasonStruct::from_raw(env, obj)?,
            }),
            "METAMORPHOSIS" => Ok(TransformReason::Metamorphosis {
                inner: TransformReasonStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(TransformReason::Unknown {
                inner: TransformReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct TransformReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TransformReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Cured { inner } => inner.0.clone(),
            Self::Frozen { inner } => inner.0.clone(),
            Self::Infection { inner } => inner.0.clone(),
            Self::Drowned { inner } => inner.0.clone(),
            Self::Sheared { inner } => inner.0.clone(),
            Self::Lightning { inner } => inner.0.clone(),
            Self::Split { inner } => inner.0.clone(),
            Self::PiglinZombified { inner } => inner.0.clone(),
            Self::Metamorphosis { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Cured { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Frozen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Infection { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Drowned { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Sheared { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Lightning { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Split { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PiglinZombified { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Metamorphosis { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TransformReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate TransformReason from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/TransformReason")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TransformReason object, got {}",
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
                "CURED" => Ok(TransformReason::Cured {
                    inner: TransformReasonStruct::from_raw(env, obj)?,
                }),
                "FROZEN" => Ok(TransformReason::Frozen {
                    inner: TransformReasonStruct::from_raw(env, obj)?,
                }),
                "INFECTION" => Ok(TransformReason::Infection {
                    inner: TransformReasonStruct::from_raw(env, obj)?,
                }),
                "DROWNED" => Ok(TransformReason::Drowned {
                    inner: TransformReasonStruct::from_raw(env, obj)?,
                }),
                "SHEARED" => Ok(TransformReason::Sheared {
                    inner: TransformReasonStruct::from_raw(env, obj)?,
                }),
                "LIGHTNING" => Ok(TransformReason::Lightning {
                    inner: TransformReasonStruct::from_raw(env, obj)?,
                }),
                "SPLIT" => Ok(TransformReason::Split {
                    inner: TransformReasonStruct::from_raw(env, obj)?,
                }),
                "PIGLIN_ZOMBIFIED" => Ok(TransformReason::PiglinZombified {
                    inner: TransformReasonStruct::from_raw(env, obj)?,
                }),
                "METAMORPHOSIS" => Ok(TransformReason::Metamorphosis {
                    inner: TransformReasonStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(TransformReason::Unknown {
                    inner: TransformReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for TransformReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TransformReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TransformReasonStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/TransformReason")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TransformReasonStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TransformReasonStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Called when a human entity experiences exhaustion. An exhaustion level greater than 4.0 causes a decrease in saturation by 1.
#[repr(C)]
pub struct EntityExhaustionEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum EntityExhaustionEventExhaustionReason<'mc> {
    BlockMined {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    HungerEffect {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Damaged {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Attack {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    JumpSprint {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Jump {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Swim {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    WalkUnderwater {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    WalkOnWater {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Sprint {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Crouch {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Walk {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Regen {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Unknown {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityExhaustionEventExhaustionReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityExhaustionEventExhaustionReason::BlockMined { .. } => f.write_str("BLOCK_MINED"),
            EntityExhaustionEventExhaustionReason::HungerEffect { .. } => {
                f.write_str("HUNGER_EFFECT")
            }
            EntityExhaustionEventExhaustionReason::Damaged { .. } => f.write_str("DAMAGED"),
            EntityExhaustionEventExhaustionReason::Attack { .. } => f.write_str("ATTACK"),
            EntityExhaustionEventExhaustionReason::JumpSprint { .. } => f.write_str("JUMP_SPRINT"),
            EntityExhaustionEventExhaustionReason::Jump { .. } => f.write_str("JUMP"),
            EntityExhaustionEventExhaustionReason::Swim { .. } => f.write_str("SWIM"),
            EntityExhaustionEventExhaustionReason::WalkUnderwater { .. } => {
                f.write_str("WALK_UNDERWATER")
            }
            EntityExhaustionEventExhaustionReason::WalkOnWater { .. } => {
                f.write_str("WALK_ON_WATER")
            }
            EntityExhaustionEventExhaustionReason::Sprint { .. } => f.write_str("SPRINT"),
            EntityExhaustionEventExhaustionReason::Crouch { .. } => f.write_str("CROUCH"),
            EntityExhaustionEventExhaustionReason::Walk { .. } => f.write_str("WALK"),
            EntityExhaustionEventExhaustionReason::Regen { .. } => f.write_str("REGEN"),
            EntityExhaustionEventExhaustionReason::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}

impl<'mc> EntityExhaustionEventExhaustionReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityExhaustionEventExhaustionReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityExhaustionEvent$ExhaustionReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityExhaustionEvent$ExhaustionReason;",
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
            "BLOCK_MINED" => Ok(EntityExhaustionEventExhaustionReason::BlockMined {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "HUNGER_EFFECT" => Ok(EntityExhaustionEventExhaustionReason::HungerEffect {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "DAMAGED" => Ok(EntityExhaustionEventExhaustionReason::Damaged {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "ATTACK" => Ok(EntityExhaustionEventExhaustionReason::Attack {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "JUMP_SPRINT" => Ok(EntityExhaustionEventExhaustionReason::JumpSprint {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "JUMP" => Ok(EntityExhaustionEventExhaustionReason::Jump {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "SWIM" => Ok(EntityExhaustionEventExhaustionReason::Swim {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "WALK_UNDERWATER" => Ok(EntityExhaustionEventExhaustionReason::WalkUnderwater {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "WALK_ON_WATER" => Ok(EntityExhaustionEventExhaustionReason::WalkOnWater {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "SPRINT" => Ok(EntityExhaustionEventExhaustionReason::Sprint {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "CROUCH" => Ok(EntityExhaustionEventExhaustionReason::Crouch {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "WALK" => Ok(EntityExhaustionEventExhaustionReason::Walk {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "REGEN" => Ok(EntityExhaustionEventExhaustionReason::Regen {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(EntityExhaustionEventExhaustionReason::Unknown {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityExhaustionEventExhaustionReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityExhaustionEventExhaustionReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::BlockMined { inner } => inner.0.clone(),
            Self::HungerEffect { inner } => inner.0.clone(),
            Self::Damaged { inner } => inner.0.clone(),
            Self::Attack { inner } => inner.0.clone(),
            Self::JumpSprint { inner } => inner.0.clone(),
            Self::Jump { inner } => inner.0.clone(),
            Self::Swim { inner } => inner.0.clone(),
            Self::WalkUnderwater { inner } => inner.0.clone(),
            Self::WalkOnWater { inner } => inner.0.clone(),
            Self::Sprint { inner } => inner.0.clone(),
            Self::Crouch { inner } => inner.0.clone(),
            Self::Walk { inner } => inner.0.clone(),
            Self::Regen { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::BlockMined { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HungerEffect { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Damaged { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Attack { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::JumpSprint { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Jump { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Swim { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WalkUnderwater { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WalkOnWater { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Sprint { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Crouch { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Walk { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Regen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityExhaustionEventExhaustionReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityExhaustionEventExhaustionReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityExhaustionEvent$ExhaustionReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityExhaustionEventExhaustionReason object, got {}",
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
                "BLOCK_MINED" => Ok(EntityExhaustionEventExhaustionReason::BlockMined {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "HUNGER_EFFECT" => Ok(EntityExhaustionEventExhaustionReason::HungerEffect {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "DAMAGED" => Ok(EntityExhaustionEventExhaustionReason::Damaged {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "ATTACK" => Ok(EntityExhaustionEventExhaustionReason::Attack {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "JUMP_SPRINT" => Ok(EntityExhaustionEventExhaustionReason::JumpSprint {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "JUMP" => Ok(EntityExhaustionEventExhaustionReason::Jump {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "SWIM" => Ok(EntityExhaustionEventExhaustionReason::Swim {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "WALK_UNDERWATER" => Ok(EntityExhaustionEventExhaustionReason::WalkUnderwater {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "WALK_ON_WATER" => Ok(EntityExhaustionEventExhaustionReason::WalkOnWater {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "SPRINT" => Ok(EntityExhaustionEventExhaustionReason::Sprint {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "CROUCH" => Ok(EntityExhaustionEventExhaustionReason::Crouch {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "WALK" => Ok(EntityExhaustionEventExhaustionReason::Walk {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "REGEN" => Ok(EntityExhaustionEventExhaustionReason::Regen {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(EntityExhaustionEventExhaustionReason::Unknown {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityExhaustionEventExhaustionReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityExhaustionEventExhaustionReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                    "Tried to instantiate EntityExhaustionEventExhaustionReasonStruct from null object.")
                .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityExhaustionEvent$ExhaustionReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityExhaustionEventExhaustionReasonStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityExhaustionEventExhaustionReasonStruct<'mc> {
    //Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for EntityExhaustionEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityExhaustionEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityExhaustionEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityExhaustionEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityExhaustionEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityExhaustionEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::HumanEntity<'mc>>,
        arg1: impl Into<crate::event::entity::EntityExhaustionEventExhaustionReason<'mc>>,
        arg2: f32,
    ) -> Result<crate::event::entity::EntityExhaustionEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/HumanEntity;Lorg/bukkit/event/entity/EntityExhaustionEvent$ExhaustionReason;F)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Float(arg2);
        let cls = jni.find_class("org/bukkit/event/entity/EntityExhaustionEvent");
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
        crate::event::entity::EntityExhaustionEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/HumanEntity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn exhaustion(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getExhaustion", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Set the exhaustion to apply to the player. The maximum exhaustion that a player can have is 40. No error will be thrown if this limit is hit. This value may be negative, but there is unknown behavior for when exhaustion is below 0.
    pub fn set_exhaustion(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExhaustion",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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

    pub fn exhaustion_reason(
        &self,
    ) -> Result<
        crate::event::entity::EntityExhaustionEventExhaustionReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig =
            String::from("()Lorg/bukkit/event/entity/EntityExhaustionEvent$ExhaustionReason;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getExhaustionReason",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityExhaustionEventExhaustionReason::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityExhaustionEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityExhaustionEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityExhaustionEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityExhaustionEvent into crate::event::entity::EntityEvent")
    }
}
/// Called immediately prior to a creature being leashed by a player.
#[repr(C)]
pub struct PlayerLeashEntityEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PlayerLeashEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PlayerLeashEntityEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PlayerLeashEntityEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/PlayerLeashEntityEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PlayerLeashEntityEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PlayerLeashEntityEvent<'mc> {
    pub fn new_with_entity(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::entity::Entity<'mc>>,
        arg2: impl Into<crate::entity::Player<'mc>>,
        arg3: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>,
    ) -> Result<crate::event::entity::PlayerLeashEntityEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Entity;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/entity/Player;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        args.push(val_3);
        if let Some(a) = arg3 {
            sig += "Lorg/bukkit/inventory/EquipmentSlot;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/PlayerLeashEntityEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::PlayerLeashEntityEvent::from_raw(&jni, res)
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

    pub fn leash_holder(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLeashHolder", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn player(&self) -> Result<Option<crate::entity::Player<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Player::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn hand(&self) -> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerLeashEntityEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PlayerLeashEntityEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::Event<'mc>> for PlayerLeashEntityEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PlayerLeashEntityEvent into crate::event::Event")
    }
}
pub enum TargetReason<'mc> {
    TargetDied {
        inner: TargetReasonStruct<'mc>,
    },
    ClosestPlayer {
        inner: TargetReasonStruct<'mc>,
    },
    TargetAttackedEntity {
        inner: TargetReasonStruct<'mc>,
    },
    #[deprecated]
    PigZombieTarget {
        inner: TargetReasonStruct<'mc>,
    },
    ForgotTarget {
        inner: TargetReasonStruct<'mc>,
    },
    TargetAttackedOwner {
        inner: TargetReasonStruct<'mc>,
    },
    OwnerAttackedTarget {
        inner: TargetReasonStruct<'mc>,
    },
    RandomTarget {
        inner: TargetReasonStruct<'mc>,
    },
    DefendVillage {
        inner: TargetReasonStruct<'mc>,
    },
    TargetAttackedNearbyEntity {
        inner: TargetReasonStruct<'mc>,
    },
    ReinforcementTarget {
        inner: TargetReasonStruct<'mc>,
    },
    Collision {
        inner: TargetReasonStruct<'mc>,
    },
    Custom {
        inner: TargetReasonStruct<'mc>,
    },
    ClosestEntity {
        inner: TargetReasonStruct<'mc>,
    },
    FollowLeader {
        inner: TargetReasonStruct<'mc>,
    },
    Tempt {
        inner: TargetReasonStruct<'mc>,
    },
    Unknown {
        inner: TargetReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for TargetReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetReason::TargetDied { .. } => f.write_str("TARGET_DIED"),
            TargetReason::ClosestPlayer { .. } => f.write_str("CLOSEST_PLAYER"),
            TargetReason::TargetAttackedEntity { .. } => f.write_str("TARGET_ATTACKED_ENTITY"),
            TargetReason::PigZombieTarget { .. } => f.write_str("PIG_ZOMBIE_TARGET"),
            TargetReason::ForgotTarget { .. } => f.write_str("FORGOT_TARGET"),
            TargetReason::TargetAttackedOwner { .. } => f.write_str("TARGET_ATTACKED_OWNER"),
            TargetReason::OwnerAttackedTarget { .. } => f.write_str("OWNER_ATTACKED_TARGET"),
            TargetReason::RandomTarget { .. } => f.write_str("RANDOM_TARGET"),
            TargetReason::DefendVillage { .. } => f.write_str("DEFEND_VILLAGE"),
            TargetReason::TargetAttackedNearbyEntity { .. } => {
                f.write_str("TARGET_ATTACKED_NEARBY_ENTITY")
            }
            TargetReason::ReinforcementTarget { .. } => f.write_str("REINFORCEMENT_TARGET"),
            TargetReason::Collision { .. } => f.write_str("COLLISION"),
            TargetReason::Custom { .. } => f.write_str("CUSTOM"),
            TargetReason::ClosestEntity { .. } => f.write_str("CLOSEST_ENTITY"),
            TargetReason::FollowLeader { .. } => f.write_str("FOLLOW_LEADER"),
            TargetReason::Tempt { .. } => f.write_str("TEMPT"),
            TargetReason::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}

impl<'mc> TargetReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<TargetReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/TargetReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/TargetReason;",
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
            "TARGET_DIED" => Ok(TargetReason::TargetDied {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "CLOSEST_PLAYER" => Ok(TargetReason::ClosestPlayer {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "TARGET_ATTACKED_ENTITY" => Ok(TargetReason::TargetAttackedEntity {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "PIG_ZOMBIE_TARGET" => Ok(TargetReason::PigZombieTarget {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "FORGOT_TARGET" => Ok(TargetReason::ForgotTarget {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "TARGET_ATTACKED_OWNER" => Ok(TargetReason::TargetAttackedOwner {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "OWNER_ATTACKED_TARGET" => Ok(TargetReason::OwnerAttackedTarget {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "RANDOM_TARGET" => Ok(TargetReason::RandomTarget {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "DEFEND_VILLAGE" => Ok(TargetReason::DefendVillage {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "TARGET_ATTACKED_NEARBY_ENTITY" => Ok(TargetReason::TargetAttackedNearbyEntity {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "REINFORCEMENT_TARGET" => Ok(TargetReason::ReinforcementTarget {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "COLLISION" => Ok(TargetReason::Collision {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "CUSTOM" => Ok(TargetReason::Custom {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "CLOSEST_ENTITY" => Ok(TargetReason::ClosestEntity {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "FOLLOW_LEADER" => Ok(TargetReason::FollowLeader {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "TEMPT" => Ok(TargetReason::Tempt {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(TargetReason::Unknown {
                inner: TargetReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct TargetReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TargetReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::TargetDied { inner } => inner.0.clone(),
            Self::ClosestPlayer { inner } => inner.0.clone(),
            Self::TargetAttackedEntity { inner } => inner.0.clone(),
            Self::PigZombieTarget { inner } => inner.0.clone(),
            Self::ForgotTarget { inner } => inner.0.clone(),
            Self::TargetAttackedOwner { inner } => inner.0.clone(),
            Self::OwnerAttackedTarget { inner } => inner.0.clone(),
            Self::RandomTarget { inner } => inner.0.clone(),
            Self::DefendVillage { inner } => inner.0.clone(),
            Self::TargetAttackedNearbyEntity { inner } => inner.0.clone(),
            Self::ReinforcementTarget { inner } => inner.0.clone(),
            Self::Collision { inner } => inner.0.clone(),
            Self::Custom { inner } => inner.0.clone(),
            Self::ClosestEntity { inner } => inner.0.clone(),
            Self::FollowLeader { inner } => inner.0.clone(),
            Self::Tempt { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::TargetDied { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ClosestPlayer { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TargetAttackedEntity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PigZombieTarget { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ForgotTarget { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TargetAttackedOwner { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OwnerAttackedTarget { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RandomTarget { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DefendVillage { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TargetAttackedNearbyEntity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ReinforcementTarget { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Collision { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ClosestEntity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FollowLeader { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Tempt { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TargetReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TargetReason from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/TargetReason")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TargetReason object, got {}",
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
                "TARGET_DIED" => Ok(TargetReason::TargetDied {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "CLOSEST_PLAYER" => Ok(TargetReason::ClosestPlayer {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "TARGET_ATTACKED_ENTITY" => Ok(TargetReason::TargetAttackedEntity {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "PIG_ZOMBIE_TARGET" => Ok(TargetReason::PigZombieTarget {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "FORGOT_TARGET" => Ok(TargetReason::ForgotTarget {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "TARGET_ATTACKED_OWNER" => Ok(TargetReason::TargetAttackedOwner {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "OWNER_ATTACKED_TARGET" => Ok(TargetReason::OwnerAttackedTarget {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "RANDOM_TARGET" => Ok(TargetReason::RandomTarget {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "DEFEND_VILLAGE" => Ok(TargetReason::DefendVillage {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "TARGET_ATTACKED_NEARBY_ENTITY" => Ok(TargetReason::TargetAttackedNearbyEntity {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "REINFORCEMENT_TARGET" => Ok(TargetReason::ReinforcementTarget {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "COLLISION" => Ok(TargetReason::Collision {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "CUSTOM" => Ok(TargetReason::Custom {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "CLOSEST_ENTITY" => Ok(TargetReason::ClosestEntity {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "FOLLOW_LEADER" => Ok(TargetReason::FollowLeader {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "TEMPT" => Ok(TargetReason::Tempt {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(TargetReason::Unknown {
                    inner: TargetReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for TargetReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TargetReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate TargetReasonStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/TargetReason")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TargetReasonStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TargetReasonStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Stores data for pigs being zapped
#[repr(C)]
pub struct PigZapEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum EntityTransformEventTransformReason<'mc> {
    Cured {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Frozen {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Infection {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Drowned {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Sheared {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Lightning {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Split {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    PiglinZombified {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Metamorphosis {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Unknown {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityTransformEventTransformReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityTransformEventTransformReason::Cured { .. } => f.write_str("CURED"),
            EntityTransformEventTransformReason::Frozen { .. } => f.write_str("FROZEN"),
            EntityTransformEventTransformReason::Infection { .. } => f.write_str("INFECTION"),
            EntityTransformEventTransformReason::Drowned { .. } => f.write_str("DROWNED"),
            EntityTransformEventTransformReason::Sheared { .. } => f.write_str("SHEARED"),
            EntityTransformEventTransformReason::Lightning { .. } => f.write_str("LIGHTNING"),
            EntityTransformEventTransformReason::Split { .. } => f.write_str("SPLIT"),
            EntityTransformEventTransformReason::PiglinZombified { .. } => {
                f.write_str("PIGLIN_ZOMBIFIED")
            }
            EntityTransformEventTransformReason::Metamorphosis { .. } => {
                f.write_str("METAMORPHOSIS")
            }
            EntityTransformEventTransformReason::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}

impl<'mc> EntityTransformEventTransformReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityTransformEventTransformReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityTransformEvent$TransformReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityTransformEvent$TransformReason;",
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
            "CURED" => Ok(EntityTransformEventTransformReason::Cured {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "FROZEN" => Ok(EntityTransformEventTransformReason::Frozen {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "INFECTION" => Ok(EntityTransformEventTransformReason::Infection {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "DROWNED" => Ok(EntityTransformEventTransformReason::Drowned {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "SHEARED" => Ok(EntityTransformEventTransformReason::Sheared {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "LIGHTNING" => Ok(EntityTransformEventTransformReason::Lightning {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "SPLIT" => Ok(EntityTransformEventTransformReason::Split {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "PIGLIN_ZOMBIFIED" => Ok(EntityTransformEventTransformReason::PiglinZombified {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "METAMORPHOSIS" => Ok(EntityTransformEventTransformReason::Metamorphosis {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(EntityTransformEventTransformReason::Unknown {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityTransformEventTransformReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityTransformEventTransformReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Cured { inner } => inner.0.clone(),
            Self::Frozen { inner } => inner.0.clone(),
            Self::Infection { inner } => inner.0.clone(),
            Self::Drowned { inner } => inner.0.clone(),
            Self::Sheared { inner } => inner.0.clone(),
            Self::Lightning { inner } => inner.0.clone(),
            Self::Split { inner } => inner.0.clone(),
            Self::PiglinZombified { inner } => inner.0.clone(),
            Self::Metamorphosis { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Cured { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Frozen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Infection { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Drowned { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Sheared { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Lightning { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Split { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PiglinZombified { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Metamorphosis { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTransformEventTransformReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityTransformEventTransformReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityTransformEvent$TransformReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityTransformEventTransformReason object, got {}",
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
                "CURED" => Ok(EntityTransformEventTransformReason::Cured {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "FROZEN" => Ok(EntityTransformEventTransformReason::Frozen {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "INFECTION" => Ok(EntityTransformEventTransformReason::Infection {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "DROWNED" => Ok(EntityTransformEventTransformReason::Drowned {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "SHEARED" => Ok(EntityTransformEventTransformReason::Sheared {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "LIGHTNING" => Ok(EntityTransformEventTransformReason::Lightning {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "SPLIT" => Ok(EntityTransformEventTransformReason::Split {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "PIGLIN_ZOMBIFIED" => Ok(EntityTransformEventTransformReason::PiglinZombified {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "METAMORPHOSIS" => Ok(EntityTransformEventTransformReason::Metamorphosis {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(EntityTransformEventTransformReason::Unknown {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityTransformEventTransformReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTransformEventTransformReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityTransformEventTransformReasonStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityTransformEvent$TransformReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityTransformEventTransformReasonStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityTransformEventTransformReasonStruct<'mc> {
    //Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for PigZapEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PigZapEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PigZapEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/PigZapEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PigZapEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PigZapEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Pig<'mc>>,
        arg1: impl Into<crate::entity::LightningStrike<'mc>>,
        arg2: impl Into<crate::entity::PigZombie<'mc>>,
    ) -> Result<crate::event::entity::PigZapEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Pig;Lorg/bukkit/entity/LightningStrike;Lorg/bukkit/entity/PigZombie;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/PigZapEvent");
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
        crate::event::entity::PigZapEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    #[deprecated]

    pub fn pig_zombie(&self) -> Result<crate::entity::PigZombie<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/PigZombie;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPigZombie", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::PigZombie::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntityTransformEvent
    //crate::event::entity::EntityTransformEvent
    pub fn transformed_entity(
        &self,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTransformEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTransformEvent = temp_clone.into();
        real.transformed_entity()
    }
    pub fn transformed_entities(
        &self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTransformedEntities",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn transform_reason(
        &self,
    ) -> Result<
        crate::event::entity::EntityTransformEventTransformReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let temp_clone = crate::event::entity::EntityTransformEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTransformEvent = temp_clone.into();
        real.transform_reason()
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PigZapEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PigZapEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityTransformEvent<'mc>> for PigZapEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityTransformEvent<'mc> {
        crate::event::entity::EntityTransformEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PigZapEvent into crate::event::entity::EntityTransformEvent")
    }
}
/// Called when a firework explodes.
#[repr(C)]
pub struct FireworkExplodeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FireworkExplodeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FireworkExplodeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FireworkExplodeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/FireworkExplodeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FireworkExplodeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FireworkExplodeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Firework<'mc>>,
    ) -> Result<crate::event::entity::FireworkExplodeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Firework;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/FireworkExplodeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::FireworkExplodeEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::Firework<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Firework;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Firework::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the cancelled state of this event. If the firework explosion is cancelled, the firework will still be removed, but no particles will be displayed.
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for FireworkExplodeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FireworkExplodeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for FireworkExplodeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FireworkExplodeEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when an arrow enters or exists an entity's body.
#[repr(C)]
pub struct ArrowBodyCountChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ArrowBodyCountChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ArrowBodyCountChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ArrowBodyCountChangeEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/ArrowBodyCountChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ArrowBodyCountChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ArrowBodyCountChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
        arg1: i32,
        arg2: i32,
        arg3: bool,
    ) -> Result<crate::event::entity::ArrowBodyCountChangeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;IIZ)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let val_3 = jni::objects::JValueGen::Int(arg2);
        let val_4 = jni::objects::JValueGen::Bool(arg3.into());
        let cls = jni.find_class("org/bukkit/event/entity/ArrowBodyCountChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::ArrowBodyCountChangeEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/LivingEntity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn is_reset(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isReset", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn old_amount(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOldAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn new_amount(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the final amount of arrows in the entity's body.
    pub fn set_new_amount(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNewAmount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for ArrowBodyCountChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ArrowBodyCountChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for ArrowBodyCountChangeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting ArrowBodyCountChangeEvent into crate::event::entity::EntityEvent",
        )
    }
}
/// Called before an entity exits a portal.
/// <p>This event allows you to modify the velocity of the entity after they have successfully exited the portal.</p>
#[repr(C)]
pub struct EntityPortalExitEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPortalExitEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPortalExitEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPortalExitEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityPortalExitEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPortalExitEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPortalExitEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::Location<'mc>>,
        arg2: impl Into<crate::Location<'mc>>,
        arg3: impl Into<crate::util::Vector<'mc>>,
        arg4: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<crate::event::entity::EntityPortalExitEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/Location;Lorg/bukkit/Location;Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg4.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityPortalExitEvent");
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
        crate::event::entity::EntityPortalExitEvent::from_raw(&jni, res)
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
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn before(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBefore", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn after(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAfter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_after(
        &self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAfter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //EntityTeleportEvent
    //crate::event::entity::EntityTeleportEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
    }
    pub fn from(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.from()
    }
    pub fn set_from(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.set_from(arg0)
    }
    pub fn to(&self) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.to()
    }
    pub fn set_to(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.set_to(arg0)
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityTeleportEvent<'mc>> for EntityPortalExitEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityTeleportEvent<'mc> {
        crate::event::entity::EntityTeleportEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityPortalExitEvent into crate::event::entity::EntityTeleportEvent",
        )
    }
}
/// Stores data for damage events
#[repr(C)]
pub struct EntityDamageEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityDamageEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDamageEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityDamageEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityDamageEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityDamageEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityDamageEvent<'mc> {
    pub fn new_with_entity(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::event::entity::EntityDamageEventDamageCause<'mc>>,
        arg2: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
        arg3: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent$DamageCause;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Ljava/util/Map;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        args.push(val_3);
        if let Some(a) = arg3 {
            sig += "Ljava/util/Map;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityDamageEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityDamageEvent::from_raw(&jni, res)
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

    pub fn set_damage_with_entity_damage_eventdamage_modifier(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
        arg1: std::option::Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a);
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setDamage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_damage(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;)D");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn is_applicable(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isApplicable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
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

    pub fn get_original_damage(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;)D");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOriginalDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn final_damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFinalDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn cause(
        &self,
    ) -> Result<crate::event::entity::EntityDamageEventDamageCause<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityDamageEvent$DamageCause;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCause", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityDamageEventDamageCause::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityDamageEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityDamageEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityDamageEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityDamageEvent into crate::event::entity::EntityEvent")
    }
}
/// Thrown whenever a LivingEntity dies
#[repr(C)]
pub struct EntityDeathEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityDeathEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDeathEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityDeathEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityDeathEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityDeathEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityDeathEvent<'mc> {
    pub fn new_with_living_entity(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
        arg1: Vec<impl Into<crate::event::entity::EntityDeathEvent<'mc>>>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::event::entity::EntityDeathEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/LivingEntity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/util/List;";
        let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            sig += "Lorg/bukkit/event/entity/crate::event::entity::EntityDeathEvent;";
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            jni.call_method(
                &raw_val_2,
                "add",
                "(Lorg/bukkit/event/entity/crate::event::entity::EntityDeathEvent)V",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityDeathEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityDeathEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn drops(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDrops", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
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

    pub fn dropped_exp(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDroppedExp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets how much EXP should be dropped from this death.
    /// <p>This does not indicate how much EXP should be taken from the entity in question, merely how much should be created after its death.</p>
    pub fn set_dropped_exp(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDroppedExp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityDeathEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityDeathEvent into crate::event::entity::EntityEvent")
    }
}
/// Sent when an entity's gliding status is toggled with an Elytra. Examples of when this event would be called:
/// <ul>
/// <li>Player presses the jump key while in midair and using an Elytra</li>
/// <li>Player lands on ground while they are gliding (with an Elytra)</li>
/// </ul> This can be visually estimated by the animation in which a player turns horizontal.
#[repr(C)]
pub struct EntityToggleGlideEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityToggleGlideEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityToggleGlideEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityToggleGlideEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityToggleGlideEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityToggleGlideEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityToggleGlideEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
        arg1: bool,
    ) -> Result<crate::event::entity::EntityToggleGlideEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let cls = jni.find_class("org/bukkit/event/entity/EntityToggleGlideEvent");
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
        crate::event::entity::EntityToggleGlideEvent::from_raw(&jni, res)
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

    pub fn is_gliding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGliding", sig.as_str(), vec![]);
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityToggleGlideEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityToggleGlideEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityToggleGlideEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityToggleGlideEvent into crate::event::entity::EntityEvent",
        )
    }
}
/// Called when a creature targets or untargets another entity
#[repr(C)]
pub struct EntityTargetEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityTargetEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTargetEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityTargetEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityTargetEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityTargetEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityTargetEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::entity::Entity<'mc>>,
        arg2: impl Into<crate::event::entity::EntityTargetEventTargetReason<'mc>>,
    ) -> Result<crate::event::entity::EntityTargetEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Entity;Lorg/bukkit/event/entity/EntityTargetEvent$TargetReason;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityTargetEvent");
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
        crate::event::entity::EntityTargetEvent::from_raw(&jni, res)
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

    pub fn reason(
        &self,
    ) -> Result<
        Option<crate::event::entity::EntityTargetEventTargetReason<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityTargetEvent$TargetReason;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getReason", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            crate::event::entity::EntityTargetEventTargetReason::from_raw(
                &self.jni_ref(),
                unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
            )?,
        ))
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

    pub fn target(&self) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTarget", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_target(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTarget",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityTargetEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTargetEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityTargetEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTargetEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when a Creeper is struck by lightning.
/// <p>If a Creeper Power event is cancelled, the Creeper will not be powered.</p>
#[repr(C)]
pub struct CreeperPowerEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum CreeperPowerEventPowerCause<'mc> {
    Lightning {
        inner: CreeperPowerEventPowerCauseStruct<'mc>,
    },
    SetOn {
        inner: CreeperPowerEventPowerCauseStruct<'mc>,
    },
    SetOff {
        inner: CreeperPowerEventPowerCauseStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for CreeperPowerEventPowerCause<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreeperPowerEventPowerCause::Lightning { .. } => f.write_str("LIGHTNING"),
            CreeperPowerEventPowerCause::SetOn { .. } => f.write_str("SET_ON"),
            CreeperPowerEventPowerCause::SetOff { .. } => f.write_str("SET_OFF"),
        }
    }
}

impl<'mc> CreeperPowerEventPowerCause<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<CreeperPowerEventPowerCause<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/CreeperPowerEvent$PowerCause");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/CreeperPowerEvent$PowerCause;",
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
            "LIGHTNING" => Ok(CreeperPowerEventPowerCause::Lightning {
                inner: CreeperPowerEventPowerCauseStruct::from_raw(env, obj)?,
            }),
            "SET_ON" => Ok(CreeperPowerEventPowerCause::SetOn {
                inner: CreeperPowerEventPowerCauseStruct::from_raw(env, obj)?,
            }),
            "SET_OFF" => Ok(CreeperPowerEventPowerCause::SetOff {
                inner: CreeperPowerEventPowerCauseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct CreeperPowerEventPowerCauseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CreeperPowerEventPowerCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Lightning { inner } => inner.0.clone(),
            Self::SetOn { inner } => inner.0.clone(),
            Self::SetOff { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Lightning { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SetOn { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SetOff { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CreeperPowerEventPowerCause<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CreeperPowerEventPowerCause from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/CreeperPowerEvent$PowerCause")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CreeperPowerEventPowerCause object, got {}",
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
                "LIGHTNING" => Ok(CreeperPowerEventPowerCause::Lightning {
                    inner: CreeperPowerEventPowerCauseStruct::from_raw(env, obj)?,
                }),
                "SET_ON" => Ok(CreeperPowerEventPowerCause::SetOn {
                    inner: CreeperPowerEventPowerCauseStruct::from_raw(env, obj)?,
                }),
                "SET_OFF" => Ok(CreeperPowerEventPowerCause::SetOff {
                    inner: CreeperPowerEventPowerCauseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for CreeperPowerEventPowerCauseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CreeperPowerEventPowerCauseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CreeperPowerEventPowerCauseStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/CreeperPowerEvent$PowerCause")?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CreeperPowerEventPowerCauseStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CreeperPowerEventPowerCauseStruct<'mc> {
    //Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for CreeperPowerEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CreeperPowerEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CreeperPowerEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/CreeperPowerEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CreeperPowerEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CreeperPowerEvent<'mc> {
    pub fn new_with_creeper(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Creeper<'mc>>,
        arg1: impl Into<crate::entity::LightningStrike<'mc>>,
        arg2: std::option::Option<
            impl Into<crate::event::entity::CreeperPowerEventPowerCause<'mc>>,
        >,
    ) -> Result<crate::event::entity::CreeperPowerEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Creeper;";
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
            sig += "Lorg/bukkit/event/entity/CreeperPowerEvent$PowerCause;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/CreeperPowerEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::CreeperPowerEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::Creeper<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Creeper;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Creeper::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    ) -> Result<crate::event::entity::CreeperPowerEventPowerCause<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/CreeperPowerEvent$PowerCause;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCause", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::CreeperPowerEventPowerCause::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for CreeperPowerEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CreeperPowerEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for CreeperPowerEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CreeperPowerEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when an entity is spawned into a world by a spawner.
/// <p>If a Spawner Spawn event is cancelled, the entity will not spawn.</p>
#[repr(C)]
pub struct SpawnerSpawnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SpawnerSpawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SpawnerSpawnEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SpawnerSpawnEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/SpawnerSpawnEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpawnerSpawnEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SpawnerSpawnEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::block::CreatureSpawner<'mc>>,
    ) -> Result<crate::event::entity::SpawnerSpawnEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/CreatureSpawner;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/SpawnerSpawnEvent");
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
        crate::event::entity::SpawnerSpawnEvent::from_raw(&jni, res)
    }

    pub fn spawner(
        &self,
    ) -> Result<crate::block::CreatureSpawner<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/CreatureSpawner;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawner", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::CreatureSpawner::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntitySpawnEvent
    //crate::event::entity::EntitySpawnEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
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
        crate::event::entity::EntitySpawnEvent::handler_list(jni)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.location()
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntitySpawnEvent<'mc>> for SpawnerSpawnEvent<'mc> {
    fn into(self) -> crate::event::entity::EntitySpawnEvent<'mc> {
        crate::event::entity::EntitySpawnEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting SpawnerSpawnEvent into crate::event::entity::EntitySpawnEvent",
        )
    }
}
/// Called when an EnderDragon switches controller phase.
#[repr(C)]
pub struct EnderDragonChangePhaseEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EnderDragonChangePhaseEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnderDragonChangePhaseEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EnderDragonChangePhaseEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EnderDragonChangePhaseEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnderDragonChangePhaseEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EnderDragonChangePhaseEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::EnderDragon<'mc>>,
        arg1: impl Into<crate::entity::EnderDragonPhase<'mc>>,
        arg2: impl Into<crate::entity::EnderDragonPhase<'mc>>,
    ) -> Result<crate::event::entity::EnderDragonChangePhaseEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/EnderDragon;Lorg/bukkit/entity/EnderDragon$Phase;Lorg/bukkit/entity/EnderDragon$Phase;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EnderDragonChangePhaseEvent");
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
        crate::event::entity::EnderDragonChangePhaseEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::EnderDragon<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/EnderDragon;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EnderDragon::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn set_new_phase(
        &self,
        arg0: impl Into<crate::entity::EnderDragonPhase<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/EnderDragon$Phase;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNewPhase",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn current_phase(
        &self,
    ) -> Result<Option<crate::entity::EnderDragonPhase<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EnderDragon$Phase;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCurrentPhase", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EnderDragonPhase::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn new_phase(
        &self,
    ) -> Result<crate::entity::EnderDragonPhase<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EnderDragon$Phase;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewPhase", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EnderDragonPhase::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EnderDragonChangePhaseEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EnderDragonChangePhaseEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EnderDragonChangePhaseEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EnderDragonChangePhaseEvent into crate::event::entity::EntityEvent",
        )
    }
}
/// Called when a projectile hits an object
#[repr(C)]
pub struct ProjectileHitEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ProjectileHitEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ProjectileHitEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ProjectileHitEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/ProjectileHitEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ProjectileHitEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ProjectileHitEvent<'mc> {
    pub fn new_with_projectile(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Projectile<'mc>>,
        arg1: std::option::Option<impl Into<crate::entity::Entity<'mc>>>,
        arg2: std::option::Option<impl Into<crate::block::Block<'mc>>>,
        arg3: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,
    ) -> Result<crate::event::entity::ProjectileHitEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Projectile;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/entity/Entity;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/block/Block;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "Lorg/bukkit/block/BlockFace;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/ProjectileHitEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::ProjectileHitEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Whether to cancel the action that occurs when the projectile hits. In the case of an entity, it will not collide (unless it's a firework, then use <a href="FireworkExplodeEvent.html" title="class in org.bukkit.event.entity"><code>FireworkExplodeEvent</code></a>).
    ///
    /// In the case of a block, some blocks (eg target block, bell) will not perform the action associated.
    ///
    /// This does NOT prevent block collisions, and explosions will still occur unless their respective events are cancelled.
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

    pub fn hit_block(
        &self,
    ) -> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHitBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::Block::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn hit_block_face(
        &self,
    ) -> Result<Option<crate::block::BlockFace<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHitBlockFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn hit_entity(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHitEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for ProjectileHitEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ProjectileHitEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for ProjectileHitEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ProjectileHitEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when an entity comes into contact with a portal
#[repr(C)]
pub struct EntityPortalEnterEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPortalEnterEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPortalEnterEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPortalEnterEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityPortalEnterEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPortalEnterEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPortalEnterEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::Location<'mc>>,
    ) -> Result<crate::event::entity::EntityPortalEnterEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityPortalEnterEvent");
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
        crate::event::entity::EntityPortalEnterEvent::from_raw(&jni, res)
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
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityPortalEnterEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityPortalEnterEvent into crate::event::entity::EntityEvent",
        )
    }
}
/// Called when the amount of air an entity has remaining changes.
#[repr(C)]
pub struct EntityAirChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityAirChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityAirChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityAirChangeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityAirChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityAirChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityAirChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: i32,
    ) -> Result<crate::event::entity::EntityAirChangeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("org/bukkit/event/entity/EntityAirChangeEvent");
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
        crate::event::entity::EntityAirChangeEvent::from_raw(&jni, res)
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

    pub fn amount(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the amount of air remaining for the entity (measured in ticks.
    pub fn set_amount(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAmount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityAirChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityAirChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityAirChangeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityAirChangeEvent into crate::event::entity::EntityEvent")
    }
}
/// Called immediately prior to an entity being unleashed.
#[repr(C)]
pub struct EntityUnleashEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityUnleashEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityUnleashEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityUnleashEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityUnleashEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityUnleashEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityUnleashEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::event::entity::EntityUnleashEventUnleashReason<'mc>>,
    ) -> Result<crate::event::entity::EntityUnleashEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/event/entity/EntityUnleashEvent$UnleashReason;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityUnleashEvent");
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
        crate::event::entity::EntityUnleashEvent::from_raw(&jni, res)
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

    pub fn reason(
        &self,
    ) -> Result<
        Option<crate::event::entity::EntityUnleashEventUnleashReason<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityUnleashEvent$UnleashReason;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getReason", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            crate::event::entity::EntityUnleashEventUnleashReason::from_raw(
                &self.jni_ref(),
                unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
            )?,
        ))
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityUnleashEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityUnleashEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when a ThrownExpBottle hits and releases experience.
#[repr(C)]
pub struct ExpBottleEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ExpBottleEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ExpBottleEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ExpBottleEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/ExpBottleEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ExpBottleEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ExpBottleEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::ThrownExpBottle<'mc>>,
        arg1: i32,
    ) -> Result<crate::event::entity::ExpBottleEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/ThrownExpBottle;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("org/bukkit/event/entity/ExpBottleEvent");
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
        crate::event::entity::ExpBottleEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn experience(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getExperience", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// This method sets the amount of experience to be created.
    /// <p>The number indicates a total amount to be divided into orbs.</p>
    pub fn set_experience(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExperience",
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

    pub fn show_effect(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getShowEffect", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// This method sets if the particle effect will be shown.
    /// <p>This does not change the experience created.</p>
    pub fn set_show_effect(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setShowEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //ProjectileHitEvent
    //crate::event::entity::ProjectileHitEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
    }
    pub fn hit_block(
        &self,
    ) -> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_block()
    }
    pub fn hit_block_face(
        &self,
    ) -> Result<Option<crate::block::BlockFace<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_block_face()
    }
    pub fn hit_entity(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_entity()
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::ProjectileHitEvent<'mc>> for ExpBottleEvent<'mc> {
    fn into(self) -> crate::event::entity::ProjectileHitEvent<'mc> {
        crate::event::entity::ProjectileHitEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ExpBottleEvent into crate::event::entity::ProjectileHitEvent")
    }
}
/// Called when an entity combusts.
/// <p>If an Entity Combust event is cancelled, the entity will not combust.</p>
#[repr(C)]
pub struct EntityCombustEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityCombustEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityCombustEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityCombustEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityCombustEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityCombustEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityCombustEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: i32,
    ) -> Result<crate::event::entity::EntityCombustEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("org/bukkit/event/entity/EntityCombustEvent");
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
        crate::event::entity::EntityCombustEvent::from_raw(&jni, res)
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

    pub fn duration(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDuration", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// The number of seconds the combustee should be alight for.
    /// <p>This value will only ever increase the combustion time, not decrease existing combustion times.</p>
    pub fn set_duration(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDuration",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityCombustEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityCombustEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityCombustEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityCombustEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when a <a href="../../entity/Spellcaster.html" title="interface in org.bukkit.entity"><code>Spellcaster</code></a> casts a spell.
#[repr(C)]
pub struct EntitySpellCastEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntitySpellCastEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntitySpellCastEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntitySpellCastEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntitySpellCastEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntitySpellCastEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntitySpellCastEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Spellcaster<'mc>>,
        arg1: impl Into<crate::entity::SpellcasterSpell<'mc>>,
    ) -> Result<crate::event::entity::EntitySpellCastEvent<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/entity/Spellcaster;Lorg/bukkit/entity/Spellcaster$Spell;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntitySpellCastEvent");
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
        crate::event::entity::EntitySpellCastEvent::from_raw(&jni, res)
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

    pub fn spell(
        &self,
    ) -> Result<crate::entity::SpellcasterSpell<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Spellcaster$Spell;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSpell", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::SpellcasterSpell::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn entity(&self) -> Result<crate::entity::Spellcaster<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Spellcaster;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Spellcaster::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntitySpellCastEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntitySpellCastEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntitySpellCastEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntitySpellCastEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when an entity is about to be replaced by another entity.
#[repr(C)]
pub struct EntityTransformEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityTransformEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTransformEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityTransformEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityTransformEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityTransformEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityTransformEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: Vec<impl Into<crate::event::entity::EntityTransformEvent<'mc>>>,
        arg2: impl Into<crate::event::entity::EntityTransformEventTransformReason<'mc>>,
    ) -> Result<crate::event::entity::EntityTransformEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Ljava/util/List;Lorg/bukkit/event/entity/EntityTransformEvent$TransformReason;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            jni.call_method(
                &raw_val_2,
                "add",
                "(Lorg/bukkit/event/entity/crate::event::entity::EntityTransformEvent)V",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityTransformEvent");
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
        crate::event::entity::EntityTransformEvent::from_raw(&jni, res)
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

    pub fn transformed_entity(
        &self,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTransformedEntity",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn transformed_entities(
        &self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTransformedEntities",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn transform_reason(
        &self,
    ) -> Result<
        crate::event::entity::EntityTransformEventTransformReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityTransformEvent$TransformReason;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTransformReason",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityTransformEventTransformReason::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityTransformEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTransformEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityTransformEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTransformEvent into crate::event::entity::EntityEvent")
    }
}
pub enum DamageCause<'mc> {
    Kill { inner: DamageCauseStruct<'mc> },
    WorldBorder { inner: DamageCauseStruct<'mc> },
    Contact { inner: DamageCauseStruct<'mc> },
    EntityAttack { inner: DamageCauseStruct<'mc> },
    EntitySweepAttack { inner: DamageCauseStruct<'mc> },
    Projectile { inner: DamageCauseStruct<'mc> },
    Suffocation { inner: DamageCauseStruct<'mc> },
    Fall { inner: DamageCauseStruct<'mc> },
    Fire { inner: DamageCauseStruct<'mc> },
    FireTick { inner: DamageCauseStruct<'mc> },
    Melting { inner: DamageCauseStruct<'mc> },
    Lava { inner: DamageCauseStruct<'mc> },
    Drowning { inner: DamageCauseStruct<'mc> },
    BlockExplosion { inner: DamageCauseStruct<'mc> },
    EntityExplosion { inner: DamageCauseStruct<'mc> },
    Void { inner: DamageCauseStruct<'mc> },
    Lightning { inner: DamageCauseStruct<'mc> },
    Suicide { inner: DamageCauseStruct<'mc> },
    Starvation { inner: DamageCauseStruct<'mc> },
    Poison { inner: DamageCauseStruct<'mc> },
    Magic { inner: DamageCauseStruct<'mc> },
    Wither { inner: DamageCauseStruct<'mc> },
    FallingBlock { inner: DamageCauseStruct<'mc> },
    Thorns { inner: DamageCauseStruct<'mc> },
    DragonBreath { inner: DamageCauseStruct<'mc> },
    Custom { inner: DamageCauseStruct<'mc> },
    FlyIntoWall { inner: DamageCauseStruct<'mc> },
    HotFloor { inner: DamageCauseStruct<'mc> },
    Cramming { inner: DamageCauseStruct<'mc> },
    Dryout { inner: DamageCauseStruct<'mc> },
    Freeze { inner: DamageCauseStruct<'mc> },
    SonicBoom { inner: DamageCauseStruct<'mc> },
}
impl<'mc> std::fmt::Display for DamageCause<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DamageCause::Kill { .. } => f.write_str("KILL"),
            DamageCause::WorldBorder { .. } => f.write_str("WORLD_BORDER"),
            DamageCause::Contact { .. } => f.write_str("CONTACT"),
            DamageCause::EntityAttack { .. } => f.write_str("ENTITY_ATTACK"),
            DamageCause::EntitySweepAttack { .. } => f.write_str("ENTITY_SWEEP_ATTACK"),
            DamageCause::Projectile { .. } => f.write_str("PROJECTILE"),
            DamageCause::Suffocation { .. } => f.write_str("SUFFOCATION"),
            DamageCause::Fall { .. } => f.write_str("FALL"),
            DamageCause::Fire { .. } => f.write_str("FIRE"),
            DamageCause::FireTick { .. } => f.write_str("FIRE_TICK"),
            DamageCause::Melting { .. } => f.write_str("MELTING"),
            DamageCause::Lava { .. } => f.write_str("LAVA"),
            DamageCause::Drowning { .. } => f.write_str("DROWNING"),
            DamageCause::BlockExplosion { .. } => f.write_str("BLOCK_EXPLOSION"),
            DamageCause::EntityExplosion { .. } => f.write_str("ENTITY_EXPLOSION"),
            DamageCause::Void { .. } => f.write_str("VOID"),
            DamageCause::Lightning { .. } => f.write_str("LIGHTNING"),
            DamageCause::Suicide { .. } => f.write_str("SUICIDE"),
            DamageCause::Starvation { .. } => f.write_str("STARVATION"),
            DamageCause::Poison { .. } => f.write_str("POISON"),
            DamageCause::Magic { .. } => f.write_str("MAGIC"),
            DamageCause::Wither { .. } => f.write_str("WITHER"),
            DamageCause::FallingBlock { .. } => f.write_str("FALLING_BLOCK"),
            DamageCause::Thorns { .. } => f.write_str("THORNS"),
            DamageCause::DragonBreath { .. } => f.write_str("DRAGON_BREATH"),
            DamageCause::Custom { .. } => f.write_str("CUSTOM"),
            DamageCause::FlyIntoWall { .. } => f.write_str("FLY_INTO_WALL"),
            DamageCause::HotFloor { .. } => f.write_str("HOT_FLOOR"),
            DamageCause::Cramming { .. } => f.write_str("CRAMMING"),
            DamageCause::Dryout { .. } => f.write_str("DRYOUT"),
            DamageCause::Freeze { .. } => f.write_str("FREEZE"),
            DamageCause::SonicBoom { .. } => f.write_str("SONIC_BOOM"),
        }
    }
}

impl<'mc> DamageCause<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<DamageCause<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/DamageCause");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/DamageCause;",
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
            "KILL" => Ok(DamageCause::Kill {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "WORLD_BORDER" => Ok(DamageCause::WorldBorder {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "CONTACT" => Ok(DamageCause::Contact {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "ENTITY_ATTACK" => Ok(DamageCause::EntityAttack {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "ENTITY_SWEEP_ATTACK" => Ok(DamageCause::EntitySweepAttack {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "PROJECTILE" => Ok(DamageCause::Projectile {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "SUFFOCATION" => Ok(DamageCause::Suffocation {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "FALL" => Ok(DamageCause::Fall {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "FIRE" => Ok(DamageCause::Fire {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "FIRE_TICK" => Ok(DamageCause::FireTick {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "MELTING" => Ok(DamageCause::Melting {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "LAVA" => Ok(DamageCause::Lava {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "DROWNING" => Ok(DamageCause::Drowning {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "BLOCK_EXPLOSION" => Ok(DamageCause::BlockExplosion {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "ENTITY_EXPLOSION" => Ok(DamageCause::EntityExplosion {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "VOID" => Ok(DamageCause::Void {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "LIGHTNING" => Ok(DamageCause::Lightning {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "SUICIDE" => Ok(DamageCause::Suicide {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "STARVATION" => Ok(DamageCause::Starvation {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "POISON" => Ok(DamageCause::Poison {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "MAGIC" => Ok(DamageCause::Magic {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "WITHER" => Ok(DamageCause::Wither {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "FALLING_BLOCK" => Ok(DamageCause::FallingBlock {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "THORNS" => Ok(DamageCause::Thorns {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "DRAGON_BREATH" => Ok(DamageCause::DragonBreath {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "CUSTOM" => Ok(DamageCause::Custom {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "FLY_INTO_WALL" => Ok(DamageCause::FlyIntoWall {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "HOT_FLOOR" => Ok(DamageCause::HotFloor {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "CRAMMING" => Ok(DamageCause::Cramming {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "DRYOUT" => Ok(DamageCause::Dryout {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "FREEZE" => Ok(DamageCause::Freeze {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),
            "SONIC_BOOM" => Ok(DamageCause::SonicBoom {
                inner: DamageCauseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct DamageCauseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DamageCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Kill { inner } => inner.0.clone(),
            Self::WorldBorder { inner } => inner.0.clone(),
            Self::Contact { inner } => inner.0.clone(),
            Self::EntityAttack { inner } => inner.0.clone(),
            Self::EntitySweepAttack { inner } => inner.0.clone(),
            Self::Projectile { inner } => inner.0.clone(),
            Self::Suffocation { inner } => inner.0.clone(),
            Self::Fall { inner } => inner.0.clone(),
            Self::Fire { inner } => inner.0.clone(),
            Self::FireTick { inner } => inner.0.clone(),
            Self::Melting { inner } => inner.0.clone(),
            Self::Lava { inner } => inner.0.clone(),
            Self::Drowning { inner } => inner.0.clone(),
            Self::BlockExplosion { inner } => inner.0.clone(),
            Self::EntityExplosion { inner } => inner.0.clone(),
            Self::Void { inner } => inner.0.clone(),
            Self::Lightning { inner } => inner.0.clone(),
            Self::Suicide { inner } => inner.0.clone(),
            Self::Starvation { inner } => inner.0.clone(),
            Self::Poison { inner } => inner.0.clone(),
            Self::Magic { inner } => inner.0.clone(),
            Self::Wither { inner } => inner.0.clone(),
            Self::FallingBlock { inner } => inner.0.clone(),
            Self::Thorns { inner } => inner.0.clone(),
            Self::DragonBreath { inner } => inner.0.clone(),
            Self::Custom { inner } => inner.0.clone(),
            Self::FlyIntoWall { inner } => inner.0.clone(),
            Self::HotFloor { inner } => inner.0.clone(),
            Self::Cramming { inner } => inner.0.clone(),
            Self::Dryout { inner } => inner.0.clone(),
            Self::Freeze { inner } => inner.0.clone(),
            Self::SonicBoom { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Kill { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WorldBorder { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Contact { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::EntityAttack { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EntitySweepAttack { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Projectile { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Suffocation { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Fall { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Fire { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FireTick { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Melting { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Lava { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Drowning { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::BlockExplosion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EntityExplosion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Void { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Lightning { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Suicide { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Starvation { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Poison { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Magic { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Wither { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FallingBlock { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Thorns { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::DragonBreath { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FlyIntoWall { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HotFloor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cramming { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Dryout { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Freeze { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SonicBoom { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DamageCause<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DamageCause from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/DamageCause")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DamageCause object, got {}",
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
                "KILL" => Ok(DamageCause::Kill {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "WORLD_BORDER" => Ok(DamageCause::WorldBorder {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "CONTACT" => Ok(DamageCause::Contact {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "ENTITY_ATTACK" => Ok(DamageCause::EntityAttack {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "ENTITY_SWEEP_ATTACK" => Ok(DamageCause::EntitySweepAttack {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "PROJECTILE" => Ok(DamageCause::Projectile {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "SUFFOCATION" => Ok(DamageCause::Suffocation {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "FALL" => Ok(DamageCause::Fall {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "FIRE" => Ok(DamageCause::Fire {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "FIRE_TICK" => Ok(DamageCause::FireTick {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "MELTING" => Ok(DamageCause::Melting {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "LAVA" => Ok(DamageCause::Lava {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "DROWNING" => Ok(DamageCause::Drowning {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "BLOCK_EXPLOSION" => Ok(DamageCause::BlockExplosion {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "ENTITY_EXPLOSION" => Ok(DamageCause::EntityExplosion {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "VOID" => Ok(DamageCause::Void {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "LIGHTNING" => Ok(DamageCause::Lightning {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "SUICIDE" => Ok(DamageCause::Suicide {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "STARVATION" => Ok(DamageCause::Starvation {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "POISON" => Ok(DamageCause::Poison {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "MAGIC" => Ok(DamageCause::Magic {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "WITHER" => Ok(DamageCause::Wither {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "FALLING_BLOCK" => Ok(DamageCause::FallingBlock {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "THORNS" => Ok(DamageCause::Thorns {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "DRAGON_BREATH" => Ok(DamageCause::DragonBreath {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "CUSTOM" => Ok(DamageCause::Custom {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "FLY_INTO_WALL" => Ok(DamageCause::FlyIntoWall {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "HOT_FLOOR" => Ok(DamageCause::HotFloor {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "CRAMMING" => Ok(DamageCause::Cramming {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "DRYOUT" => Ok(DamageCause::Dryout {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "FREEZE" => Ok(DamageCause::Freeze {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                "SONIC_BOOM" => Ok(DamageCause::SonicBoom {
                    inner: DamageCauseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for DamageCauseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DamageCauseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DamageCauseStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/DamageCause")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DamageCauseStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DamageCauseStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Called when a splash potion hits an area
#[repr(C)]
pub struct PotionSplashEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PotionSplashEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PotionSplashEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PotionSplashEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/PotionSplashEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionSplashEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PotionSplashEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::ThrownPotion<'mc>>,
        arg1: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<crate::event::entity::PotionSplashEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/ThrownPotion;Ljava/util/Map;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/PotionSplashEvent");
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
        crate::event::entity::PotionSplashEvent::from_raw(&jni, res)
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

    pub fn entity(&self) -> Result<crate::entity::ThrownPotion<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/ThrownPotion;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::ThrownPotion::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="ProjectileHitEvent.html#setCancelled(boolean)">ProjectileHitEvent</a></code></span>
    /// Whether to cancel the action that occurs when the projectile hits. In the case of an entity, it will not collide (unless it's a firework, then use <a title="class in org.bukkit.event.entity" href="FireworkExplodeEvent.html"><code>FireworkExplodeEvent</code></a>).
    ///
    /// In the case of a block, some blocks (eg target block, bell) will not perform the action associated.
    ///
    /// This does NOT prevent block collisions, and explosions will still occur unless their respective events are cancelled.
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

    pub fn affected_entities(
        &self,
    ) -> Result<Vec<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAffectedEntities",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::LivingEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn potion(&self) -> Result<crate::entity::ThrownPotion<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/ThrownPotion;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPotion", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::ThrownPotion::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_intensity(
        &self,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;)D");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getIntensity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn set_intensity(
        &self,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
        arg1: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;D)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Double(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setIntensity",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //ProjectileHitEvent
    //crate::event::entity::ProjectileHitEvent
    pub fn hit_block(
        &self,
    ) -> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_block()
    }
    pub fn hit_block_face(
        &self,
    ) -> Result<Option<crate::block::BlockFace<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_block_face()
    }
    pub fn hit_entity(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_entity()
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PotionSplashEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PotionSplashEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::ProjectileHitEvent<'mc>> for PotionSplashEvent<'mc> {
    fn into(self) -> crate::event::entity::ProjectileHitEvent<'mc> {
        crate::event::entity::ProjectileHitEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting PotionSplashEvent into crate::event::entity::ProjectileHitEvent",
        )
    }
}
/// Called when an entity interacts with an object
#[repr(C)]
pub struct EntityInteractEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityInteractEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityInteractEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityInteractEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityInteractEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityInteractEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityInteractEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: impl Into<crate::block::Block<'mc>>,
    ) -> Result<crate::event::entity::EntityInteractEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/Block;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityInteractEvent");
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
        crate::event::entity::EntityInteractEvent::from_raw(&jni, res)
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

    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityInteractEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityInteractEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityInteractEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityInteractEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when an entity is spawned into a world.
/// <p>If an Entity Spawn event is cancelled, the entity will not spawn.</p>
#[repr(C)]
pub struct EntitySpawnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntitySpawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntitySpawnEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntitySpawnEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntitySpawnEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntitySpawnEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntitySpawnEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<crate::event::entity::EntitySpawnEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntitySpawnEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntitySpawnEvent::from_raw(&jni, res)
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

    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntitySpawnEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntitySpawnEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntitySpawnEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntitySpawnEvent into crate::event::entity::EntityEvent")
    }
}
/// Called when a <a title="interface in org.bukkit.entity" href="../../entity/Villager.html"><code>Villager</code></a> is about to restock one of its trades.
/// <p>If this event passes, the villager will reset the <a href="../../inventory/MerchantRecipe.html#getUses()"><code>uses</code></a> of the affected <a href="#getRecipe()"><code>MerchantRecipe</code></a> to <code>0</code>.</p>
#[repr(C)]
pub struct VillagerReplenishTradeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for VillagerReplenishTradeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for VillagerReplenishTradeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate VillagerReplenishTradeEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/VillagerReplenishTradeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VillagerReplenishTradeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> VillagerReplenishTradeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::AbstractVillager<'mc>>,
        arg1: impl Into<crate::inventory::MerchantRecipe<'mc>>,
    ) -> Result<crate::event::entity::VillagerReplenishTradeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Lorg/bukkit/entity/AbstractVillager;Lorg/bukkit/inventory/MerchantRecipe;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/VillagerReplenishTradeEvent");
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
        crate::event::entity::VillagerReplenishTradeEvent::from_raw(&jni, res)
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

    pub fn recipe(
        &self,
    ) -> Result<Option<crate::inventory::MerchantRecipe<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/MerchantRecipe;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRecipe", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::MerchantRecipe::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_recipe(
        &self,
        arg0: impl Into<crate::inventory::MerchantRecipe<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/MerchantRecipe;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRecipe",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    #[deprecated]

    pub fn bonus(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBonus", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    #[deprecated = "MC 1.14 has changed how villagers restock their trades. This has no effect anymore. "]
    /// Set the bonus uses added.
    pub fn set_bonus(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBonus",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //EntityEvent
    //crate::event::entity::EntityEvent
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }
    //Event
    //crate::event::Event
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
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for VillagerReplenishTradeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting VillagerReplenishTradeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for VillagerReplenishTradeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting VillagerReplenishTradeEvent into crate::event::entity::EntityEvent",
        )
    }
}
