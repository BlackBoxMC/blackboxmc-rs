#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct HangingBreakEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HangingBreakEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HangingBreakEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate HangingBreakEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/hanging/HangingBreakEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HangingBreakEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HangingBreakEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        hanging: impl Into<crate::entity::Hanging<'mc>>,
        cause: impl Into<crate::event::hanging::HangingBreakEventRemoveCause<'mc>>,
    ) -> Result<crate::event::hanging::HangingBreakEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Hanging;Lorg/bukkit/event/hanging/HangingBreakEvent/RemoveCause;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(hanging.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(cause.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/hanging/HangingBreakEvent");
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
        crate::event::hanging::HangingBreakEvent::from_raw(&jni, res)
    }
    /// Gets the cause for the hanging entity's removal
    pub fn cause(
        &self,
    ) -> Result<crate::event::hanging::HangingBreakEventRemoveCause<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/hanging/HangingBreakEvent/RemoveCause;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCause", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::hanging::HangingBreakEventRemoveCause::from_raw(&self.jni_ref(), unsafe {
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
        let cls = jni.find_class("org/bukkit/event/hanging/HangingBreakEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.hanging.HangingEvent ( ['getCause', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the hanging entity involved in this event.
    pub fn entity(&self) -> Result<crate::entity::Hanging<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::hanging::HangingEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::hanging::HangingEvent = temp_clone.into();
        real.entity()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for HangingBreakEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HangingBreakEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::hanging::HangingEvent<'mc>> for HangingBreakEvent<'mc> {
    fn into(self) -> crate::event::hanging::HangingEvent<'mc> {
        crate::event::hanging::HangingEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HangingBreakEvent into crate::event::hanging::HangingEvent")
    }
}
pub enum HangingBreakEventRemoveCause<'mc> {
    Entity {
        inner: HangingBreakEventRemoveCauseStruct<'mc>,
    },
    Explosion {
        inner: HangingBreakEventRemoveCauseStruct<'mc>,
    },
    Obstruction {
        inner: HangingBreakEventRemoveCauseStruct<'mc>,
    },
    Physics {
        inner: HangingBreakEventRemoveCauseStruct<'mc>,
    },
    Default {
        inner: HangingBreakEventRemoveCauseStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for HangingBreakEventRemoveCause<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HangingBreakEventRemoveCause::Entity { .. } => f.write_str("ENTITY"),
            HangingBreakEventRemoveCause::Explosion { .. } => f.write_str("EXPLOSION"),
            HangingBreakEventRemoveCause::Obstruction { .. } => f.write_str("OBSTRUCTION"),
            HangingBreakEventRemoveCause::Physics { .. } => f.write_str("PHYSICS"),
            HangingBreakEventRemoveCause::Default { .. } => f.write_str("DEFAULT"),
        }
    }
}
impl<'mc> std::ops::Deref for HangingBreakEventRemoveCause<'mc> {
    type Target = HangingBreakEventRemoveCauseStruct<'mc>;
    fn deref(&self) -> &<HangingBreakEventRemoveCause<'mc> as std::ops::Deref>::Target {
        match self {
            HangingBreakEventRemoveCause::Entity { inner } => inner,
            HangingBreakEventRemoveCause::Explosion { inner } => inner,
            HangingBreakEventRemoveCause::Obstruction { inner } => inner,
            HangingBreakEventRemoveCause::Physics { inner } => inner,
            HangingBreakEventRemoveCause::Default { inner } => inner,
        }
    }
}

impl<'mc> HangingBreakEventRemoveCause<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<HangingBreakEventRemoveCause<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/hanging/HangingBreakEvent/RemoveCause");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/hanging/HangingBreakEvent/RemoveCause;",
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
            "ENTITY" => Ok(HangingBreakEventRemoveCause::Entity {
                inner: HangingBreakEventRemoveCauseStruct::from_raw(env, obj)?,
            }),
            "EXPLOSION" => Ok(HangingBreakEventRemoveCause::Explosion {
                inner: HangingBreakEventRemoveCauseStruct::from_raw(env, obj)?,
            }),
            "OBSTRUCTION" => Ok(HangingBreakEventRemoveCause::Obstruction {
                inner: HangingBreakEventRemoveCauseStruct::from_raw(env, obj)?,
            }),
            "PHYSICS" => Ok(HangingBreakEventRemoveCause::Physics {
                inner: HangingBreakEventRemoveCauseStruct::from_raw(env, obj)?,
            }),
            "DEFAULT" => Ok(HangingBreakEventRemoveCause::Default {
                inner: HangingBreakEventRemoveCauseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct HangingBreakEventRemoveCauseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HangingBreakEventRemoveCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Entity { inner } => inner.0.clone(),
            Self::Explosion { inner } => inner.0.clone(),
            Self::Obstruction { inner } => inner.0.clone(),
            Self::Physics { inner } => inner.0.clone(),
            Self::Default { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Entity { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Explosion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Obstruction { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Physics { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Default { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HangingBreakEventRemoveCause<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate HangingBreakEventRemoveCause from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/hanging/HangingBreakEvent/RemoveCause",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HangingBreakEventRemoveCause object, got {}",
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
                "ENTITY" => Ok(HangingBreakEventRemoveCause::Entity {
                    inner: HangingBreakEventRemoveCauseStruct::from_raw(env, obj)?,
                }),
                "EXPLOSION" => Ok(HangingBreakEventRemoveCause::Explosion {
                    inner: HangingBreakEventRemoveCauseStruct::from_raw(env, obj)?,
                }),
                "OBSTRUCTION" => Ok(HangingBreakEventRemoveCause::Obstruction {
                    inner: HangingBreakEventRemoveCauseStruct::from_raw(env, obj)?,
                }),
                "PHYSICS" => Ok(HangingBreakEventRemoveCause::Physics {
                    inner: HangingBreakEventRemoveCauseStruct::from_raw(env, obj)?,
                }),
                "DEFAULT" => Ok(HangingBreakEventRemoveCause::Default {
                    inner: HangingBreakEventRemoveCauseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for HangingBreakEventRemoveCauseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HangingBreakEventRemoveCauseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate HangingBreakEventRemoveCauseStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/hanging/HangingBreakEvent/RemoveCause",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a HangingBreakEventRemoveCauseStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HangingBreakEventRemoveCauseStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::hanging::HangingBreakEventRemoveCause<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/hanging/HangingBreakEvent/RemoveCause;");
        let cls = jni.find_class("org/bukkit/event/hanging/HangingBreakEvent/RemoveCause");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::hanging::HangingBreakEventRemoveCause::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct HangingEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HangingEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HangingEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HangingEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/hanging/HangingEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HangingEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HangingEvent<'mc> {
    /// Gets the hanging entity involved in this event.
    pub fn entity(&self) -> Result<crate::entity::Hanging<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Hanging;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Hanging::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.Event ( ['getEntity'])
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
impl<'mc> Into<crate::event::Event<'mc>> for HangingEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HangingEvent into crate::event::Event")
    }
}
#[repr(C)]
pub struct HangingBreakByEntityEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HangingBreakByEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HangingBreakByEntityEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate HangingBreakByEntityEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/hanging/HangingBreakByEntityEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HangingBreakByEntityEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HangingBreakByEntityEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        hanging: impl Into<crate::entity::Hanging<'mc>>,
        remover: impl Into<crate::entity::Entity<'mc>>,
        cause: std::option::Option<
            impl Into<crate::event::hanging::HangingBreakEventRemoveCause<'mc>>,
        >,
    ) -> Result<crate::event::hanging::HangingBreakByEntityEvent<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Hanging;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(hanging.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Entity;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(remover.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = cause {
            sig += "Lorg/bukkit/event/hanging/HangingBreakEvent/RemoveCause;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/hanging/HangingBreakByEntityEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::hanging::HangingBreakByEntityEvent::from_raw(&jni, res)
    }
    /// Gets the entity that removed the hanging entity.
    /// May be null, for example when broken by an explosion.
    pub fn remover(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRemover", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    // SUPER CLASS: org.bukkit.event.hanging.HangingBreakEvent ( ['getRemover'])
    /// Gets the cause for the hanging entity's removal
    pub fn cause(
        &self,
    ) -> Result<crate::event::hanging::HangingBreakEventRemoveCause<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::event::hanging::HangingBreakEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::hanging::HangingBreakEvent = temp_clone.into();
        real.cause()
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::hanging::HangingBreakEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::hanging::HangingBreakEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::hanging::HangingBreakEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::hanging::HangingBreakEvent = temp_clone.into();
        real.set_cancelled(cancel)
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
        crate::event::hanging::HangingBreakEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::hanging::HangingBreakEvent<'mc>> for HangingBreakByEntityEvent<'mc> {
    fn into(self) -> crate::event::hanging::HangingBreakEvent<'mc> {
        crate::event::hanging::HangingBreakEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting HangingBreakByEntityEvent into crate::event::hanging::HangingBreakEvent")
    }
}
#[repr(C)]
pub struct HangingPlaceEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HangingPlaceEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HangingPlaceEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate HangingPlaceEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/hanging/HangingPlaceEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HangingPlaceEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HangingPlaceEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        hanging: impl Into<crate::entity::Hanging<'mc>>,
        player: impl Into<crate::entity::Player<'mc>>,
        block: impl Into<crate::block::Block<'mc>>,
        block_face: impl Into<crate::block::BlockFace<'mc>>,
        hand: impl Into<crate::inventory::EquipmentSlot<'mc>>,
        item_stack: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::event::hanging::HangingPlaceEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Hanging;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(hanging.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Player;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(player.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/block/Block;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        args.push(val_3);
        sig += "Lorg/bukkit/block/BlockFace;";
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block_face.into().jni_object().clone())
        });
        args.push(val_4);
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(hand.into().jni_object().clone())
        });
        args.push(val_5);
        if let Some(a) = item_stack {
            sig += "Lorg/bukkit/inventory/ItemStack;";
            let val_6 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_6);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/hanging/HangingPlaceEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::hanging::HangingPlaceEvent::from_raw(&jni, res)
    }
    /// Returns the player placing the hanging entity
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
    /// Returns the block that the hanging entity was placed on
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
    /// Returns the face of the block that the hanging entity was placed on
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
    /// Returns the hand that was used to place the hanging entity, or null
    /// if a player did not place the hanging entity.
    pub fn hand(
        &self,
    ) -> Result<Option<crate::inventory::EquipmentSlot<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::EquipmentSlot::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the item from which the hanging entity originated
    pub fn item_stack(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemStack", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
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
        let cls = jni.find_class("org/bukkit/event/hanging/HangingPlaceEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.hanging.HangingEvent ( ['getPlayer', 'getBlock', 'getBlockFace', 'getHand', 'getItemStack', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the hanging entity involved in this event.
    pub fn entity(&self) -> Result<crate::entity::Hanging<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::hanging::HangingEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::hanging::HangingEvent = temp_clone.into();
        real.entity()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for HangingPlaceEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HangingPlaceEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::hanging::HangingEvent<'mc>> for HangingPlaceEvent<'mc> {
    fn into(self) -> crate::event::hanging::HangingEvent<'mc> {
        crate::event::hanging::HangingEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HangingPlaceEvent into crate::event::hanging::HangingEvent")
    }
}
