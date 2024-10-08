#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct NotePlayEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for NotePlayEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for NotePlayEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate NotePlayEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/NotePlayEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a NotePlayEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> NotePlayEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        instrument: impl Into<crate::Instrument<'mc>>,
        note: impl Into<crate::Note<'mc>>,
    ) -> Result<crate::event::block::NotePlayEvent<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/Instrument;Lorg/bukkit/Note;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(instrument.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(note.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/NotePlayEvent");
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
        crate::event::block::NotePlayEvent::from_raw(&jni, res)
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
    /// Gets the {@link Instrument} to be used.
    pub fn instrument(&self) -> Result<crate::Instrument<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Instrument;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInstrument", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Instrument::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the {@link Note} to be played.
    pub fn note(&self) -> Result<crate::Note<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Note;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getNote", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Note::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// Overrides the {@link Instrument} to be used.
    pub fn set_instrument(
        &self,
        instrument: impl Into<crate::Instrument<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Instrument;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(instrument.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInstrument",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Overrides the {@link Note} to be played.
    pub fn set_note(
        &self,
        note: impl Into<crate::Note<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Note;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(note.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNote",
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
        let cls = jni.find_class("org/bukkit/event/block/NotePlayEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['isCancelled', 'setCancelled', 'getInstrument', 'getNote', 'setInstrument', 'setNote', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for NotePlayEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting NotePlayEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for NotePlayEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting NotePlayEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct CauldronLevelChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CauldronLevelChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CauldronLevelChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CauldronLevelChangeEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/CauldronLevelChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CauldronLevelChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CauldronLevelChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        entity: impl Into<crate::entity::Entity<'mc>>,
        reason: impl Into<crate::event::block::CauldronLevelChangeEventChangeReason<'mc>>,
        new_block: impl Into<crate::block::BlockState<'mc>>,
    ) -> Result<crate::event::block::CauldronLevelChangeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/entity/Entity;Lorg/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason;Lorg/bukkit/block/BlockState;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(reason.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_block.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/CauldronLevelChangeEvent");
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
        crate::event::block::CauldronLevelChangeEvent::from_raw(&jni, res)
    }
    /// Get entity which did this. May be null.
    pub fn entity(&self) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn reason(
        &self,
    ) -> Result<
        crate::event::block::CauldronLevelChangeEventChangeReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getReason", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::block::CauldronLevelChangeEventChangeReason::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }
    /// Gets the new state of the cauldron.
    pub fn new_state(&self) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewState", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// Gets the old level of the cauldron.
    pub fn old_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOldLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    #[deprecated]
    /// Gets the new level of the cauldron.
    pub fn new_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    #[deprecated]
    /// Sets the new level of the cauldron.
    pub fn set_new_level(&self, new_level: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(new_level);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNewLevel",
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
        let cls = jni.find_class("org/bukkit/event/block/CauldronLevelChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getEntity', 'getReason', 'getNewState', 'getOldLevel', 'getNewLevel', 'setNewLevel', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for CauldronLevelChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CauldronLevelChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for CauldronLevelChangeEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting CauldronLevelChangeEvent into crate::event::block::BlockEvent",
        )
    }
}
pub enum CauldronLevelChangeEventChangeReason<'mc> {
    BucketFill {
        inner: CauldronLevelChangeEventChangeReasonStruct<'mc>,
    },
    BucketEmpty {
        inner: CauldronLevelChangeEventChangeReasonStruct<'mc>,
    },
    BottleFill {
        inner: CauldronLevelChangeEventChangeReasonStruct<'mc>,
    },
    BottleEmpty {
        inner: CauldronLevelChangeEventChangeReasonStruct<'mc>,
    },
    BannerWash {
        inner: CauldronLevelChangeEventChangeReasonStruct<'mc>,
    },
    ArmorWash {
        inner: CauldronLevelChangeEventChangeReasonStruct<'mc>,
    },
    ShulkerWash {
        inner: CauldronLevelChangeEventChangeReasonStruct<'mc>,
    },
    Extinguish {
        inner: CauldronLevelChangeEventChangeReasonStruct<'mc>,
    },
    Evaporate {
        inner: CauldronLevelChangeEventChangeReasonStruct<'mc>,
    },
    NaturalFill {
        inner: CauldronLevelChangeEventChangeReasonStruct<'mc>,
    },
    Unknown {
        inner: CauldronLevelChangeEventChangeReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for CauldronLevelChangeEventChangeReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CauldronLevelChangeEventChangeReason::BucketFill { .. } => f.write_str("BUCKET_FILL"),
            CauldronLevelChangeEventChangeReason::BucketEmpty { .. } => f.write_str("BUCKET_EMPTY"),
            CauldronLevelChangeEventChangeReason::BottleFill { .. } => f.write_str("BOTTLE_FILL"),
            CauldronLevelChangeEventChangeReason::BottleEmpty { .. } => f.write_str("BOTTLE_EMPTY"),
            CauldronLevelChangeEventChangeReason::BannerWash { .. } => f.write_str("BANNER_WASH"),
            CauldronLevelChangeEventChangeReason::ArmorWash { .. } => f.write_str("ARMOR_WASH"),
            CauldronLevelChangeEventChangeReason::ShulkerWash { .. } => f.write_str("SHULKER_WASH"),
            CauldronLevelChangeEventChangeReason::Extinguish { .. } => f.write_str("EXTINGUISH"),
            CauldronLevelChangeEventChangeReason::Evaporate { .. } => f.write_str("EVAPORATE"),
            CauldronLevelChangeEventChangeReason::NaturalFill { .. } => f.write_str("NATURAL_FILL"),
            CauldronLevelChangeEventChangeReason::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}
impl<'mc> std::ops::Deref for CauldronLevelChangeEventChangeReason<'mc> {
    type Target = CauldronLevelChangeEventChangeReasonStruct<'mc>;
    fn deref(&self) -> &<CauldronLevelChangeEventChangeReason<'mc> as std::ops::Deref>::Target {
        match self {
            CauldronLevelChangeEventChangeReason::BucketFill { inner } => inner,
            CauldronLevelChangeEventChangeReason::BucketEmpty { inner } => inner,
            CauldronLevelChangeEventChangeReason::BottleFill { inner } => inner,
            CauldronLevelChangeEventChangeReason::BottleEmpty { inner } => inner,
            CauldronLevelChangeEventChangeReason::BannerWash { inner } => inner,
            CauldronLevelChangeEventChangeReason::ArmorWash { inner } => inner,
            CauldronLevelChangeEventChangeReason::ShulkerWash { inner } => inner,
            CauldronLevelChangeEventChangeReason::Extinguish { inner } => inner,
            CauldronLevelChangeEventChangeReason::Evaporate { inner } => inner,
            CauldronLevelChangeEventChangeReason::NaturalFill { inner } => inner,
            CauldronLevelChangeEventChangeReason::Unknown { inner } => inner,
        }
    }
}

impl<'mc> CauldronLevelChangeEventChangeReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<CauldronLevelChangeEventChangeReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason;",
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
            "BUCKET_FILL" => Ok(CauldronLevelChangeEventChangeReason::BucketFill {
                inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
            }),
            "BUCKET_EMPTY" => Ok(CauldronLevelChangeEventChangeReason::BucketEmpty {
                inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
            }),
            "BOTTLE_FILL" => Ok(CauldronLevelChangeEventChangeReason::BottleFill {
                inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
            }),
            "BOTTLE_EMPTY" => Ok(CauldronLevelChangeEventChangeReason::BottleEmpty {
                inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
            }),
            "BANNER_WASH" => Ok(CauldronLevelChangeEventChangeReason::BannerWash {
                inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
            }),
            "ARMOR_WASH" => Ok(CauldronLevelChangeEventChangeReason::ArmorWash {
                inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
            }),
            "SHULKER_WASH" => Ok(CauldronLevelChangeEventChangeReason::ShulkerWash {
                inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
            }),
            "EXTINGUISH" => Ok(CauldronLevelChangeEventChangeReason::Extinguish {
                inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
            }),
            "EVAPORATE" => Ok(CauldronLevelChangeEventChangeReason::Evaporate {
                inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
            }),
            "NATURAL_FILL" => Ok(CauldronLevelChangeEventChangeReason::NaturalFill {
                inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(CauldronLevelChangeEventChangeReason::Unknown {
                inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct CauldronLevelChangeEventChangeReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CauldronLevelChangeEventChangeReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::BucketFill { inner } => inner.0.clone(),
            Self::BucketEmpty { inner } => inner.0.clone(),
            Self::BottleFill { inner } => inner.0.clone(),
            Self::BottleEmpty { inner } => inner.0.clone(),
            Self::BannerWash { inner } => inner.0.clone(),
            Self::ArmorWash { inner } => inner.0.clone(),
            Self::ShulkerWash { inner } => inner.0.clone(),
            Self::Extinguish { inner } => inner.0.clone(),
            Self::Evaporate { inner } => inner.0.clone(),
            Self::NaturalFill { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::BucketFill { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BucketEmpty { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BottleFill { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BottleEmpty { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerWash { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ArmorWash { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShulkerWash { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Extinguish { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Evaporate { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::NaturalFill { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CauldronLevelChangeEventChangeReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CauldronLevelChangeEventChangeReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CauldronLevelChangeEventChangeReason object, got {}",
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
                "BUCKET_FILL" => Ok(CauldronLevelChangeEventChangeReason::BucketFill {
                    inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
                }),
                "BUCKET_EMPTY" => Ok(CauldronLevelChangeEventChangeReason::BucketEmpty {
                    inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
                }),
                "BOTTLE_FILL" => Ok(CauldronLevelChangeEventChangeReason::BottleFill {
                    inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
                }),
                "BOTTLE_EMPTY" => Ok(CauldronLevelChangeEventChangeReason::BottleEmpty {
                    inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
                }),
                "BANNER_WASH" => Ok(CauldronLevelChangeEventChangeReason::BannerWash {
                    inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
                }),
                "ARMOR_WASH" => Ok(CauldronLevelChangeEventChangeReason::ArmorWash {
                    inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
                }),
                "SHULKER_WASH" => Ok(CauldronLevelChangeEventChangeReason::ShulkerWash {
                    inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
                }),
                "EXTINGUISH" => Ok(CauldronLevelChangeEventChangeReason::Extinguish {
                    inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
                }),
                "EVAPORATE" => Ok(CauldronLevelChangeEventChangeReason::Evaporate {
                    inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
                }),
                "NATURAL_FILL" => Ok(CauldronLevelChangeEventChangeReason::NaturalFill {
                    inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(CauldronLevelChangeEventChangeReason::Unknown {
                    inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for CauldronLevelChangeEventChangeReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CauldronLevelChangeEventChangeReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CauldronLevelChangeEventChangeReasonStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CauldronLevelChangeEventChangeReasonStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CauldronLevelChangeEventChangeReasonStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        crate::event::block::CauldronLevelChangeEventChangeReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason;");
        let cls = jni.find_class("org/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::block::CauldronLevelChangeEventChangeReason::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct BlockPistonExtendEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockPistonExtendEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockPistonExtendEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BlockPistonExtendEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/BlockPistonExtendEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockPistonExtendEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockPistonExtendEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        blocks: Vec<jni::objects::JObject<'mc>>,
        direction: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<crate::event::block::BlockPistonExtendEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/Block;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/util/List;";
        let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in blocks {
            sig += "Ljava/lang/java/lang/Object;";
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_2,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        args.push(val_2);
        sig += "Lorg/bukkit/block/BlockFace;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(direction.into().jni_object().clone())
        });
        args.push(val_3);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/block/BlockPistonExtendEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::block::BlockPistonExtendEvent::from_raw(&jni, res)
    }
    #[deprecated]
    /// Get the amount of blocks which will be moved while extending.
    pub fn length(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLength", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get an immutable list of the blocks which will be moved by the
    /// extending.
    pub fn blocks(&self) -> Result<Vec<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
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
            new_vec.push(crate::block::Block::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
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
        let cls = jni.find_class("org/bukkit/event/block/BlockPistonExtendEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockPistonEvent ( ['getLength', 'getBlocks', 'getHandlers', 'getHandlerList'])

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPistonEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPistonEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancelled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPistonEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPistonEvent = temp_clone.into();
        real.set_cancelled(cancelled)
    }
    /// Returns true if the Piston in the event is sticky.
    pub fn is_sticky(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPistonEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPistonEvent = temp_clone.into();
        real.is_sticky()
    }
    /// Return the direction in which the piston will operate.
    pub fn direction(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPistonEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPistonEvent = temp_clone.into();
        real.direction()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::BlockPistonEvent<'mc>> for BlockPistonExtendEvent<'mc> {
    fn into(self) -> crate::event::block::BlockPistonEvent<'mc> {
        crate::event::block::BlockPistonEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting BlockPistonExtendEvent into crate::event::block::BlockPistonEvent",
        )
    }
}
#[repr(C)]
pub struct BlockIgniteEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockIgniteEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockIgniteEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockIgniteEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockIgniteEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockIgniteEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockIgniteEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        the_block: impl Into<crate::block::Block<'mc>>,
        cause: impl Into<crate::event::block::BlockIgniteEventIgniteCause<'mc>>,
        igniting_entity: impl Into<crate::entity::Entity<'mc>>,
        igniting_block: std::option::Option<impl Into<crate::block::Block<'mc>>>,
    ) -> Result<crate::event::block::BlockIgniteEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/Block;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(the_block.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/event/block/BlockIgniteEvent/IgniteCause;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(cause.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/entity/Entity;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(igniting_entity.into().jni_object().clone())
        });
        args.push(val_3);
        if let Some(a) = igniting_block {
            sig += "Lorg/bukkit/block/Block;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/block/BlockIgniteEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::block::BlockIgniteEvent::from_raw(&jni, res)
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
    /// Gets the cause of block ignite.
    pub fn cause(
        &self,
    ) -> Result<crate::event::block::BlockIgniteEventIgniteCause<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/block/BlockIgniteEvent/IgniteCause;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCause", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::block::BlockIgniteEventIgniteCause::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the player who ignited this block
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
    /// Gets the entity who ignited this block
    pub fn igniting_entity(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getIgnitingEntity",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the block which ignited this block
    pub fn igniting_block(
        &self,
    ) -> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getIgnitingBlock",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::Block::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
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
        let cls = jni.find_class("org/bukkit/event/block/BlockIgniteEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['isCancelled', 'setCancelled', 'getCause', 'getPlayer', 'getIgnitingEntity', 'getIgnitingBlock', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockIgniteEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockIgniteEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockIgniteEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockIgniteEvent into crate::event::block::BlockEvent")
    }
}
pub enum BlockIgniteEventIgniteCause<'mc> {
    Lava {
        inner: BlockIgniteEventIgniteCauseStruct<'mc>,
    },
    FlintAndSteel {
        inner: BlockIgniteEventIgniteCauseStruct<'mc>,
    },
    Spread {
        inner: BlockIgniteEventIgniteCauseStruct<'mc>,
    },
    Lightning {
        inner: BlockIgniteEventIgniteCauseStruct<'mc>,
    },
    Fireball {
        inner: BlockIgniteEventIgniteCauseStruct<'mc>,
    },
    EnderCrystal {
        inner: BlockIgniteEventIgniteCauseStruct<'mc>,
    },
    Explosion {
        inner: BlockIgniteEventIgniteCauseStruct<'mc>,
    },
    Arrow {
        inner: BlockIgniteEventIgniteCauseStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for BlockIgniteEventIgniteCause<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockIgniteEventIgniteCause::Lava { .. } => f.write_str("LAVA"),
            BlockIgniteEventIgniteCause::FlintAndSteel { .. } => f.write_str("FLINT_AND_STEEL"),
            BlockIgniteEventIgniteCause::Spread { .. } => f.write_str("SPREAD"),
            BlockIgniteEventIgniteCause::Lightning { .. } => f.write_str("LIGHTNING"),
            BlockIgniteEventIgniteCause::Fireball { .. } => f.write_str("FIREBALL"),
            BlockIgniteEventIgniteCause::EnderCrystal { .. } => f.write_str("ENDER_CRYSTAL"),
            BlockIgniteEventIgniteCause::Explosion { .. } => f.write_str("EXPLOSION"),
            BlockIgniteEventIgniteCause::Arrow { .. } => f.write_str("ARROW"),
        }
    }
}
impl<'mc> std::ops::Deref for BlockIgniteEventIgniteCause<'mc> {
    type Target = BlockIgniteEventIgniteCauseStruct<'mc>;
    fn deref(&self) -> &<BlockIgniteEventIgniteCause<'mc> as std::ops::Deref>::Target {
        match self {
            BlockIgniteEventIgniteCause::Lava { inner } => inner,
            BlockIgniteEventIgniteCause::FlintAndSteel { inner } => inner,
            BlockIgniteEventIgniteCause::Spread { inner } => inner,
            BlockIgniteEventIgniteCause::Lightning { inner } => inner,
            BlockIgniteEventIgniteCause::Fireball { inner } => inner,
            BlockIgniteEventIgniteCause::EnderCrystal { inner } => inner,
            BlockIgniteEventIgniteCause::Explosion { inner } => inner,
            BlockIgniteEventIgniteCause::Arrow { inner } => inner,
        }
    }
}

impl<'mc> BlockIgniteEventIgniteCause<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BlockIgniteEventIgniteCause<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/block/BlockIgniteEvent/IgniteCause");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/block/BlockIgniteEvent/IgniteCause;",
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
            "LAVA" => Ok(BlockIgniteEventIgniteCause::Lava {
                inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
            }),
            "FLINT_AND_STEEL" => Ok(BlockIgniteEventIgniteCause::FlintAndSteel {
                inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
            }),
            "SPREAD" => Ok(BlockIgniteEventIgniteCause::Spread {
                inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
            }),
            "LIGHTNING" => Ok(BlockIgniteEventIgniteCause::Lightning {
                inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
            }),
            "FIREBALL" => Ok(BlockIgniteEventIgniteCause::Fireball {
                inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
            }),
            "ENDER_CRYSTAL" => Ok(BlockIgniteEventIgniteCause::EnderCrystal {
                inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
            }),
            "EXPLOSION" => Ok(BlockIgniteEventIgniteCause::Explosion {
                inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
            }),
            "ARROW" => Ok(BlockIgniteEventIgniteCause::Arrow {
                inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct BlockIgniteEventIgniteCauseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockIgniteEventIgniteCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Lava { inner } => inner.0.clone(),
            Self::FlintAndSteel { inner } => inner.0.clone(),
            Self::Spread { inner } => inner.0.clone(),
            Self::Lightning { inner } => inner.0.clone(),
            Self::Fireball { inner } => inner.0.clone(),
            Self::EnderCrystal { inner } => inner.0.clone(),
            Self::Explosion { inner } => inner.0.clone(),
            Self::Arrow { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Lava { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FlintAndSteel { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Spread { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Lightning { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Fireball { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::EnderCrystal { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Explosion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Arrow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockIgniteEventIgniteCause<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BlockIgniteEventIgniteCause from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/BlockIgniteEvent/IgniteCause")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockIgniteEventIgniteCause object, got {}",
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
                "LAVA" => Ok(BlockIgniteEventIgniteCause::Lava {
                    inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
                }),
                "FLINT_AND_STEEL" => Ok(BlockIgniteEventIgniteCause::FlintAndSteel {
                    inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
                }),
                "SPREAD" => Ok(BlockIgniteEventIgniteCause::Spread {
                    inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
                }),
                "LIGHTNING" => Ok(BlockIgniteEventIgniteCause::Lightning {
                    inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
                }),
                "FIREBALL" => Ok(BlockIgniteEventIgniteCause::Fireball {
                    inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
                }),
                "ENDER_CRYSTAL" => Ok(BlockIgniteEventIgniteCause::EnderCrystal {
                    inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
                }),
                "EXPLOSION" => Ok(BlockIgniteEventIgniteCause::Explosion {
                    inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
                }),
                "ARROW" => Ok(BlockIgniteEventIgniteCause::Arrow {
                    inner: BlockIgniteEventIgniteCauseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for BlockIgniteEventIgniteCauseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockIgniteEventIgniteCauseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BlockIgniteEventIgniteCauseStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/BlockIgniteEvent/IgniteCause")?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockIgniteEventIgniteCauseStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockIgniteEventIgniteCauseStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::block::BlockIgniteEventIgniteCause<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/block/BlockIgniteEvent/IgniteCause;");
        let cls = jni.find_class("org/bukkit/event/block/BlockIgniteEvent/IgniteCause");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::block::BlockIgniteEventIgniteCause::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct BlockFromToEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockFromToEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockFromToEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockFromToEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockFromToEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockFromToEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockFromToEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        to_block: impl Into<crate::block::Block<'mc>>,
    ) -> Result<crate::event::block::BlockFromToEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/Block;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/block/Block;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(to_block.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/block/BlockFromToEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::block::BlockFromToEvent::from_raw(&jni, res)
    }
    /// Gets the BlockFace that the block is moving to.
    pub fn face(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Convenience method for getting the faced Block.
    pub fn to_block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getToBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
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
        let cls = jni.find_class("org/bukkit/event/block/BlockFromToEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getFace', 'getToBlock', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockFromToEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockFromToEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockFromToEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockFromToEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockFertilizeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockFertilizeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockFertilizeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockFertilizeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/BlockFertilizeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockFertilizeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockFertilizeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        the_block: impl Into<crate::block::Block<'mc>>,
        player: impl Into<crate::entity::Player<'mc>>,
        blocks: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<crate::event::block::BlockFertilizeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/entity/Player;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(the_block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(player.into().jni_object().clone())
        });
        let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in blocks {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_3,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_3 = jni::objects::JValueGen::Object(raw_val_3);
        let cls = jni.find_class("org/bukkit/event/block/BlockFertilizeEvent");
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
        crate::event::block::BlockFertilizeEvent::from_raw(&jni, res)
    }
    /// Gets the player that triggered the fertilization.
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
    /// Gets a list of all blocks changed by the fertilization.
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
        let cls = jni.find_class("org/bukkit/event/block/BlockFertilizeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getPlayer', 'getBlocks', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockFertilizeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockFertilizeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockFertilizeEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockFertilizeEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockShearEntityEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockShearEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockShearEntityEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BlockShearEntityEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/BlockShearEntityEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockShearEntityEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockShearEntityEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        dispenser: impl Into<crate::block::Block<'mc>>,
        sheared: impl Into<crate::entity::Entity<'mc>>,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::block::BlockShearEntityEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/block/Block;Lorg/bukkit/entity/Entity;Lorg/bukkit/inventory/ItemStack;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(dispenser.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sheared.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BlockShearEntityEvent");
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
        crate::event::block::BlockShearEntityEvent::from_raw(&jni, res)
    }
    /// Gets the entity that was sheared.
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
    /// Gets the item used to shear this sheep.
    pub fn tool(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTool", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
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
        let cls = jni.find_class("org/bukkit/event/block/BlockShearEntityEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getEntity', 'getTool', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockShearEntityEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockShearEntityEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockShearEntityEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockShearEntityEvent into crate::event::block::BlockEvent")
    }
}
pub enum Action<'mc> {
    LeftClickBlock { inner: ActionStruct<'mc> },
    RightClickBlock { inner: ActionStruct<'mc> },
    LeftClickAir { inner: ActionStruct<'mc> },
    RightClickAir { inner: ActionStruct<'mc> },
    Physical { inner: ActionStruct<'mc> },
}
impl<'mc> std::fmt::Display for Action<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::LeftClickBlock { .. } => f.write_str("LEFT_CLICK_BLOCK"),
            Action::RightClickBlock { .. } => f.write_str("RIGHT_CLICK_BLOCK"),
            Action::LeftClickAir { .. } => f.write_str("LEFT_CLICK_AIR"),
            Action::RightClickAir { .. } => f.write_str("RIGHT_CLICK_AIR"),
            Action::Physical { .. } => f.write_str("PHYSICAL"),
        }
    }
}
impl<'mc> std::ops::Deref for Action<'mc> {
    type Target = ActionStruct<'mc>;
    fn deref(&self) -> &<Action<'mc> as std::ops::Deref>::Target {
        match self {
            Action::LeftClickBlock { inner } => inner,
            Action::RightClickBlock { inner } => inner,
            Action::LeftClickAir { inner } => inner,
            Action::RightClickAir { inner } => inner,
            Action::Physical { inner } => inner,
        }
    }
}

impl<'mc> Action<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Action<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/block/Action");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/block/Action;",
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
            "LEFT_CLICK_BLOCK" => Ok(Action::LeftClickBlock {
                inner: ActionStruct::from_raw(env, obj)?,
            }),
            "RIGHT_CLICK_BLOCK" => Ok(Action::RightClickBlock {
                inner: ActionStruct::from_raw(env, obj)?,
            }),
            "LEFT_CLICK_AIR" => Ok(Action::LeftClickAir {
                inner: ActionStruct::from_raw(env, obj)?,
            }),
            "RIGHT_CLICK_AIR" => Ok(Action::RightClickAir {
                inner: ActionStruct::from_raw(env, obj)?,
            }),
            "PHYSICAL" => Ok(Action::Physical {
                inner: ActionStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ActionStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Action<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::LeftClickBlock { inner } => inner.0.clone(),
            Self::RightClickBlock { inner } => inner.0.clone(),
            Self::LeftClickAir { inner } => inner.0.clone(),
            Self::RightClickAir { inner } => inner.0.clone(),
            Self::Physical { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::LeftClickBlock { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RightClickBlock { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LeftClickAir { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RightClickAir { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Physical { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Action<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Action from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/Action")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Action object, got {}",
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
                "LEFT_CLICK_BLOCK" => Ok(Action::LeftClickBlock {
                    inner: ActionStruct::from_raw(env, obj)?,
                }),
                "RIGHT_CLICK_BLOCK" => Ok(Action::RightClickBlock {
                    inner: ActionStruct::from_raw(env, obj)?,
                }),
                "LEFT_CLICK_AIR" => Ok(Action::LeftClickAir {
                    inner: ActionStruct::from_raw(env, obj)?,
                }),
                "RIGHT_CLICK_AIR" => Ok(Action::RightClickAir {
                    inner: ActionStruct::from_raw(env, obj)?,
                }),
                "PHYSICAL" => Ok(Action::Physical {
                    inner: ActionStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ActionStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ActionStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ActionStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/Action")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ActionStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ActionStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::block::Action<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/block/Action;");
        let cls = jni.find_class("org/bukkit/event/block/Action");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::block::Action::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct BlockBreakEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockBreakEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockBreakEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockBreakEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockBreakEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockBreakEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockBreakEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        the_block: impl Into<crate::block::Block<'mc>>,
        player: impl Into<crate::entity::Player<'mc>>,
    ) -> Result<crate::event::block::BlockBreakEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/entity/Player;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(the_block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(player.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BlockBreakEvent");
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
        crate::event::block::BlockBreakEvent::from_raw(&jni, res)
    }
    /// Gets the Player that is breaking the block involved in this event.
    pub fn player(&self) -> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Player::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets whether or not the block will attempt to drop items as it normally
    /// would.
    /// If and only if this is false then {@link BlockDropItemEvent} will not be
    /// called after this event.
    pub fn set_drop_items(&self, drop_items: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(drop_items.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDropItems",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not the block will attempt to drop items.
    /// If and only if this is false then {@link BlockDropItemEvent} will not be
    /// called after this event.
    pub fn is_drop_items(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDropItems", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
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
    // SUPER CLASS: org.bukkit.event.block.BlockExpEvent ( ['getPlayer', 'setDropItems', 'isDropItems', 'isCancelled', 'setCancelled'])
    /// Get the experience dropped by the block after the event has processed
    pub fn exp_to_drop(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockExpEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockExpEvent = temp_clone.into();
        real.exp_to_drop()
    }
    /// Set the amount of experience dropped by the block after the event has
    /// processed
    pub fn set_exp_to_drop(&self, exp: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockExpEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockExpEvent = temp_clone.into();
        real.set_exp_to_drop(exp)
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
        crate::event::block::BlockExpEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockBreakEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockBreakEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockExpEvent<'mc>> for BlockBreakEvent<'mc> {
    fn into(self) -> crate::event::block::BlockExpEvent<'mc> {
        crate::event::block::BlockExpEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockBreakEvent into crate::event::block::BlockExpEvent")
    }
}
#[repr(C)]
pub struct SpongeAbsorbEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SpongeAbsorbEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SpongeAbsorbEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SpongeAbsorbEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/SpongeAbsorbEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpongeAbsorbEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SpongeAbsorbEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        waterblocks: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<crate::event::block::SpongeAbsorbEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in waterblocks {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_2,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let cls = jni.find_class("org/bukkit/event/block/SpongeAbsorbEvent");
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
        crate::event::block::SpongeAbsorbEvent::from_raw(&jni, res)
    }
    /// Get a list of all blocks to be removed by the sponge.
    ///
    /// This list is mutable and contains the blocks in their removed state, i.e.
    /// having a type of {@link Material#AIR}.
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
        let cls = jni.find_class("org/bukkit/event/block/SpongeAbsorbEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getBlocks', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for SpongeAbsorbEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SpongeAbsorbEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for SpongeAbsorbEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SpongeAbsorbEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockPistonEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockPistonEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockPistonEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockPistonEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockPistonEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockPistonEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockPistonEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        direction: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<crate::event::block::BlockPistonEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(direction.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BlockPistonEvent");
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
        crate::event::block::BlockPistonEvent::from_raw(&jni, res)
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
    /// Returns true if the Piston in the event is sticky.
    pub fn is_sticky(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSticky", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Return the direction in which the piston will operate.
    pub fn direction(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDirection", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['isCancelled', 'setCancelled', 'isSticky', 'getDirection'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockPistonEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockPistonEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockPistonEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockPistonEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockPistonRetractEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockPistonRetractEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockPistonRetractEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BlockPistonRetractEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/BlockPistonRetractEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockPistonRetractEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockPistonRetractEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        blocks: Vec<jni::objects::JObject<'mc>>,
        direction: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<crate::event::block::BlockPistonRetractEvent<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/Block;Ljava/util/List;Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in blocks {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_2,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(direction.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BlockPistonRetractEvent");
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
        crate::event::block::BlockPistonRetractEvent::from_raw(&jni, res)
    }
    #[deprecated]
    /// Gets the location where the possible moving block might be if the retracting piston is sticky.
    pub fn retract_location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRetractLocation",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get an immutable list of the blocks which will be moved by the
    /// extending.
    pub fn blocks(&self) -> Result<Vec<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
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
            new_vec.push(crate::block::Block::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
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
        let cls = jni.find_class("org/bukkit/event/block/BlockPistonRetractEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockPistonEvent ( ['getRetractLocation', 'getBlocks', 'getHandlers', 'getHandlerList'])

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPistonEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPistonEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancelled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPistonEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPistonEvent = temp_clone.into();
        real.set_cancelled(cancelled)
    }
    /// Returns true if the Piston in the event is sticky.
    pub fn is_sticky(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPistonEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPistonEvent = temp_clone.into();
        real.is_sticky()
    }
    /// Return the direction in which the piston will operate.
    pub fn direction(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPistonEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPistonEvent = temp_clone.into();
        real.direction()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::BlockPistonEvent<'mc>> for BlockPistonRetractEvent<'mc> {
    fn into(self) -> crate::event::block::BlockPistonEvent<'mc> {
        crate::event::block::BlockPistonEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting BlockPistonRetractEvent into crate::event::block::BlockPistonEvent",
        )
    }
}
#[repr(C)]
pub struct BlockDispenseArmorEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockDispenseArmorEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockDispenseArmorEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BlockDispenseArmorEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/BlockDispenseArmorEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockDispenseArmorEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockDispenseArmorEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        dispensed: impl Into<crate::inventory::ItemStack<'mc>>,
        target: impl Into<crate::entity::LivingEntity<'mc>>,
    ) -> Result<crate::event::block::BlockDispenseArmorEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/entity/LivingEntity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(dispensed.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(target.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BlockDispenseArmorEvent");
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
        crate::event::block::BlockDispenseArmorEvent::from_raw(&jni, res)
    }
    /// Get the living entity on which the armor was dispensed.
    pub fn tarentity(
        &self,
    ) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTargetEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.block.BlockDispenseEvent ( ['getTargetEntity'])
    /// Gets the item that is being dispensed. Modifying the returned item will
    /// have no effect, you must use {@link
    /// #setItem(org.bukkit.inventory.ItemStack)} instead.
    pub fn item(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockDispenseEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockDispenseEvent = temp_clone.into();
        real.item()
    }
    /// Sets the item being dispensed.
    pub fn set_item(
        &self,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockDispenseEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockDispenseEvent = temp_clone.into();
        real.set_item(item)
    }
    /// Gets the velocity in meters per tick.
    ///
    /// Note: Modifying the returned Vector will not change the velocity, you
    /// must use {@link #setVelocity(org.bukkit.util.Vector)} instead.
    pub fn velocity(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockDispenseEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockDispenseEvent = temp_clone.into();
        real.velocity()
    }
    /// Sets the velocity of the item being dispensed in meters per tick.
    pub fn set_velocity(
        &self,
        vel: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockDispenseEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockDispenseEvent = temp_clone.into();
        real.set_velocity(vel)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockDispenseEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockDispenseEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockDispenseEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockDispenseEvent = temp_clone.into();
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
        crate::event::block::BlockDispenseEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::BlockDispenseEvent<'mc>> for BlockDispenseArmorEvent<'mc> {
    fn into(self) -> crate::event::block::BlockDispenseEvent<'mc> {
        crate::event::block::BlockDispenseEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting BlockDispenseArmorEvent into crate::event::block::BlockDispenseEvent",
        )
    }
}
#[repr(C)]
pub struct EntityBlockFormEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityBlockFormEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityBlockFormEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityBlockFormEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/EntityBlockFormEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityBlockFormEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityBlockFormEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::Entity<'mc>>,
        block: impl Into<crate::block::Block<'mc>>,
        blockstate: impl Into<crate::block::BlockState<'mc>>,
    ) -> Result<crate::event::block::EntityBlockFormEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockState;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(blockstate.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/EntityBlockFormEvent");
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
        crate::event::block::EntityBlockFormEvent::from_raw(&jni, res)
    }
    /// Get the entity that formed the block.
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
    // SUPER CLASS: org.bukkit.event.block.BlockFormEvent ( ['getEntity'])

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
        crate::event::block::BlockFormEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::BlockFormEvent<'mc>> for EntityBlockFormEvent<'mc> {
    fn into(self) -> crate::event::block::BlockFormEvent<'mc> {
        crate::event::block::BlockFormEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityBlockFormEvent into crate::event::block::BlockFormEvent",
        )
    }
}
#[repr(C)]
pub struct SignChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SignChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SignChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SignChangeEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/SignChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SignChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SignChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        the_block: impl Into<crate::block::Block<'mc>>,
        the_player: impl Into<crate::entity::Player<'mc>>,
        the_lines: impl Into<String>,
        side: std::option::Option<impl Into<crate::block::sign::Side<'mc>>>,
    ) -> Result<crate::event::block::SignChangeEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/Block;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(the_block.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Player;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(the_player.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Ljava/lang/String;";
        let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(the_lines.into())?,
        ));
        args.push(val_3);
        if let Some(a) = side {
            sig += "Lorg/bukkit/block/sign/Side;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/block/SignChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::block::SignChangeEvent::from_raw(&jni, res)
    }
    /// Gets the player changing the sign involved in this event.
    pub fn player(&self) -> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Player::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets all of the lines of text from the sign involved in this event.
    pub fn lines(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLines", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets a single line of text from the sign involved in this event.
    pub fn get_line(&self, index: i32) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Int(index);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLine",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
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
    /// Sets a single line for the sign involved in this event
    pub fn set_line(
        &self,
        index: i32,
        line: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILjava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Int(index);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(line.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLine",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns which side is changed.
    pub fn side(&self) -> Result<crate::block::sign::Side<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/sign/Side;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSide", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::sign::Side::from_raw(&self.jni_ref(), unsafe {
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
        let cls = jni.find_class("org/bukkit/event/block/SignChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getPlayer', 'getLines', 'getLine', 'setLine', 'getSide', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for SignChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SignChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for SignChangeEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SignChangeEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockMultiPlaceEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockMultiPlaceEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockMultiPlaceEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockMultiPlaceEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/BlockMultiPlaceEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockMultiPlaceEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockMultiPlaceEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        states: Vec<jni::objects::JObject<'mc>>,
        clicked: impl Into<crate::block::Block<'mc>>,
        item_in_hand: impl Into<crate::inventory::ItemStack<'mc>>,
        the_player: impl Into<crate::entity::Player<'mc>>,
        can_build: bool,
    ) -> Result<crate::event::block::BlockMultiPlaceEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/entity/Player;Z)V");
        let raw_val_1 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in states {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(clicked.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item_in_hand.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(the_player.into().jni_object().clone())
        });
        let val_5 = jni::objects::JValueGen::Bool(can_build.into());
        let cls = jni.find_class("org/bukkit/event/block/BlockMultiPlaceEvent");
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
        crate::event::block::BlockMultiPlaceEvent::from_raw(&jni, res)
    }
    /// Gets a list of blockstates for all blocks which were replaced by the
    /// placement of the new blocks. Most of these blocks will just have a
    /// Material type of AIR.
    pub fn replaced_block_states(
        &self,
    ) -> Result<Vec<crate::block::BlockState<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getReplacedBlockStates",
            sig.as_str(),
            vec![],
        );
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
    // SUPER CLASS: org.bukkit.event.block.BlockPlaceEvent ( ['getReplacedBlockStates'])

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPlaceEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPlaceEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPlaceEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPlaceEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }
    /// Gets the player who placed the block involved in this event.
    pub fn player(&self) -> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPlaceEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPlaceEvent = temp_clone.into();
        real.player()
    }
    /// Clarity method for getting the placed block. Not really needed except
    /// for reasons of clarity.
    pub fn block_placed(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPlaceEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPlaceEvent = temp_clone.into();
        real.block_placed()
    }
    /// Gets the BlockState for the block which was replaced. Material type air
    /// mostly.
    pub fn block_replaced_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPlaceEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPlaceEvent = temp_clone.into();
        real.block_replaced_state()
    }
    /// Gets the block that this block was placed against
    pub fn block_against(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPlaceEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPlaceEvent = temp_clone.into();
        real.block_against()
    }
    /// Gets the item in the player's hand when they placed the block.
    pub fn item_in_hand(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPlaceEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPlaceEvent = temp_clone.into();
        real.item_in_hand()
    }
    /// Gets the hand which placed the block
    pub fn hand(&self) -> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPlaceEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPlaceEvent = temp_clone.into();
        real.hand()
    }
    /// Gets the value whether the player would be allowed to build here.
    /// Defaults to spawn if the server was going to stop them (such as, the
    /// player is in Spawn). Note that this is an entirely different check
    /// than BLOCK_CANBUILD, as this refers to a player, not universe-physics
    /// rule like cactus on dirt.
    pub fn can_build(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPlaceEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPlaceEvent = temp_clone.into();
        real.can_build()
    }
    /// Sets the canBuild state of this event. Set to true if you want the
    /// player to be able to build.
    pub fn set_build(&self, can_build: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockPlaceEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockPlaceEvent = temp_clone.into();
        real.set_build(can_build)
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
        crate::event::block::BlockPlaceEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::BlockPlaceEvent<'mc>> for BlockMultiPlaceEvent<'mc> {
    fn into(self) -> crate::event::block::BlockPlaceEvent<'mc> {
        crate::event::block::BlockPlaceEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting BlockMultiPlaceEvent into crate::event::block::BlockPlaceEvent",
        )
    }
}
#[repr(C)]
pub struct BlockBurnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockBurnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockBurnEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockBurnEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockBurnEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockBurnEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockBurnEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        igniting_block: std::option::Option<impl Into<crate::block::Block<'mc>>>,
    ) -> Result<crate::event::block::BlockBurnEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/Block;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = igniting_block {
            sig += "Lorg/bukkit/block/Block;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/block/BlockBurnEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::block::BlockBurnEvent::from_raw(&jni, res)
    }
    /// Gets the block which ignited this block.
    pub fn igniting_block(
        &self,
    ) -> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getIgnitingBlock",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::Block::from_raw(
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
        let cls = jni.find_class("org/bukkit/event/block/BlockBurnEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getIgnitingBlock', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockBurnEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockBurnEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockBurnEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockBurnEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockDropItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockDropItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockDropItemEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockDropItemEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockDropItemEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockDropItemEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockDropItemEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        block_state: impl Into<crate::block::BlockState<'mc>>,
        player: impl Into<crate::entity::Player<'mc>>,
        items: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<crate::event::block::BlockDropItemEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockState;Lorg/bukkit/entity/Player;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block_state.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(player.into().jni_object().clone())
        });
        let raw_val_4 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in items {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_4,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_4 = jni::objects::JValueGen::Object(raw_val_4);
        let cls = jni.find_class("org/bukkit/event/block/BlockDropItemEvent");
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
        crate::event::block::BlockDropItemEvent::from_raw(&jni, res)
    }
    /// Gets the Player that is breaking the block involved in this event.
    pub fn player(&self) -> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Player::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the BlockState of the block involved in this event before it was
    /// broken.
    pub fn block_state(&self) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlockState", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets list of the Item drops caused by the block break.
    /// This list is mutable - removing an item from it will cause it to not
    /// drop. It is not legal however to add new items to the list.
    pub fn items(&self) -> Result<Vec<crate::entity::Item<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getItems", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Item::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
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
        let cls = jni.find_class("org/bukkit/event/block/BlockDropItemEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getPlayer', 'getBlockState', 'getItems', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockDropItemEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockDropItemEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockDropItemEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockDropItemEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockSpreadEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockSpreadEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockSpreadEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockSpreadEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockSpreadEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockSpreadEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockSpreadEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        source: impl Into<crate::block::Block<'mc>>,
        new_state: impl Into<crate::block::BlockState<'mc>>,
    ) -> Result<crate::event::block::BlockSpreadEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/block/Block;Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockState;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(source.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_state.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BlockSpreadEvent");
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
        crate::event::block::BlockSpreadEvent::from_raw(&jni, res)
    }
    /// Gets the source block involved in this event.
    pub fn source(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSource", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
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
        let cls = jni.find_class("org/bukkit/event/block/BlockSpreadEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockFormEvent ( ['getSource', 'getHandlers', 'getHandlerList'])

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::BlockFormEvent<'mc>> for BlockSpreadEvent<'mc> {
    fn into(self) -> crate::event::block::BlockFormEvent<'mc> {
        crate::event::block::BlockFormEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockSpreadEvent into crate::event::block::BlockFormEvent")
    }
}
#[repr(C)]
pub struct BlockGrowEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockGrowEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockGrowEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockGrowEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockGrowEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockGrowEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockGrowEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        new_state: impl Into<crate::block::BlockState<'mc>>,
    ) -> Result<crate::event::block::BlockGrowEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockState;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_state.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BlockGrowEvent");
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
        crate::event::block::BlockGrowEvent::from_raw(&jni, res)
    }
    /// Gets the state of the block where it will form or spread to.
    pub fn new_state(&self) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewState", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
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
        let cls = jni.find_class("org/bukkit/event/block/BlockGrowEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getNewState', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockGrowEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockGrowEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockGrowEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockGrowEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockFormEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockFormEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockFormEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockFormEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockFormEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockFormEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockFormEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        new_state: impl Into<crate::block::BlockState<'mc>>,
    ) -> Result<crate::event::block::BlockFormEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockState;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_state.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BlockFormEvent");
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
        crate::event::block::BlockFormEvent::from_raw(&jni, res)
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
        let cls = jni.find_class("org/bukkit/event/block/BlockFormEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockGrowEvent ( ['getHandlers', 'getHandlerList'])
    /// Gets the state of the block where it will form or spread to.
    pub fn new_state(&self) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockGrowEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockGrowEvent = temp_clone.into();
        real.new_state()
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockGrowEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockGrowEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockGrowEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockGrowEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::BlockGrowEvent<'mc>> for BlockFormEvent<'mc> {
    fn into(self) -> crate::event::block::BlockGrowEvent<'mc> {
        crate::event::block::BlockGrowEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockFormEvent into crate::event::block::BlockGrowEvent")
    }
}
#[repr(C)]
pub struct LeavesDecayEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LeavesDecayEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LeavesDecayEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LeavesDecayEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/LeavesDecayEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LeavesDecayEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LeavesDecayEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
    ) -> Result<crate::event::block::LeavesDecayEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/LeavesDecayEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::block::LeavesDecayEvent::from_raw(&jni, res)
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
        let cls = jni.find_class("org/bukkit/event/block/LeavesDecayEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for LeavesDecayEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LeavesDecayEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for LeavesDecayEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LeavesDecayEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct CampfireStartEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CampfireStartEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CampfireStartEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CampfireStartEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/CampfireStartEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CampfireStartEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CampfireStartEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        furnace: impl Into<crate::block::Block<'mc>>,
        source: impl Into<crate::inventory::ItemStack<'mc>>,
        recipe: impl Into<crate::inventory::CampfireRecipe<'mc>>,
    ) -> Result<crate::event::block::CampfireStartEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/CampfireRecipe;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(furnace.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(source.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(recipe.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/CampfireStartEvent");
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
        crate::event::block::CampfireStartEvent::from_raw(&jni, res)
    }
    /// Gets the CampfireRecipe associated with this event.
    pub fn recipe(
        &self,
    ) -> Result<crate::inventory::CampfireRecipe<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/CampfireRecipe;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRecipe", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CampfireRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the total cook time associated with this event.
    pub fn total_cook_time(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTotalCookTime",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the total cook time for this event.
    pub fn set_total_cook_time(&self, cook_time: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(cook_time);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTotalCookTime",
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
        let cls = jni.find_class("org/bukkit/event/block/CampfireStartEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.InventoryBlockStartEvent ( ['getRecipe', 'getTotalCookTime', 'setTotalCookTime', 'getHandlers', 'getHandlerList'])
    /// Gets the source ItemStack for this event.
    pub fn source(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::block::InventoryBlockStartEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::block::InventoryBlockStartEvent = temp_clone.into();
        real.source()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::InventoryBlockStartEvent<'mc>> for CampfireStartEvent<'mc> {
    fn into(self) -> crate::event::block::InventoryBlockStartEvent<'mc> {
        crate::event::block::InventoryBlockStartEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting CampfireStartEvent into crate::event::block::InventoryBlockStartEvent")
    }
}
#[repr(C)]
pub struct BlockExplodeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockExplodeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockExplodeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockExplodeEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockExplodeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockExplodeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockExplodeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::block::Block<'mc>>,
        block_state: impl Into<crate::block::BlockState<'mc>>,
        blocks: Vec<jni::objects::JObject<'mc>>,
        val_yield: f32,
        result: impl Into<crate::ExplosionResult<'mc>>,
    ) -> Result<crate::event::block::BlockExplodeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockState;Ljava/util/List;FLorg/bukkit/ExplosionResult;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block_state.into().jni_object().clone())
        });
        let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in blocks {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_3,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_3 = jni::objects::JValueGen::Object(raw_val_3);
        let val_4 = jni::objects::JValueGen::Float(val_yield);
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(result.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BlockExplodeEvent");
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
        crate::event::block::BlockExplodeEvent::from_raw(&jni, res)
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
    /// Returns the result of the explosion if it is not cancelled.
    pub fn explosion_result(
        &self,
    ) -> Result<crate::ExplosionResult<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/ExplosionResult;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getExplosionResult",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::ExplosionResult::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the captured BlockState of the block that exploded.
    pub fn exploded_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getExplodedBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the list of blocks that would have been removed or were removed
    /// from the explosion event.
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
    /// Returns the percentage of blocks to drop from this explosion
    pub fn get_yield(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getYield", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the percentage of blocks to drop from this explosion
    pub fn set_yield(&self, val_yield: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(val_yield);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setYield",
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
        let cls = jni.find_class("org/bukkit/event/block/BlockExplodeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['isCancelled', 'setCancelled', 'getExplosionResult', 'getExplodedBlockState', 'blockList', 'getYield', 'setYield', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockExplodeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockExplodeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockExplodeEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockExplodeEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockCanBuildEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockCanBuildEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockCanBuildEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockCanBuildEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockCanBuildEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockCanBuildEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockCanBuildEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        player: impl Into<crate::entity::Player<'mc>>,
        val_type: impl Into<crate::block::data::BlockData<'mc>>,
        can_build: std::option::Option<bool>,
    ) -> Result<crate::event::block::BlockCanBuildEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/Block;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Player;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(player.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/block/data/BlockData;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_3);
        if let Some(a) = can_build {
            sig += "Z";
            let val_4 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/block/BlockCanBuildEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::block::BlockCanBuildEvent::from_raw(&jni, res)
    }
    /// Gets whether or not the block can be built here.
    ///
    /// By default, returns Minecraft's answer on whether the block can be
    /// built here or not.
    pub fn is_buildable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isBuildable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the block can be built here or not.
    pub fn set_buildable(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBuildable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the Material that we are trying to place.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the BlockData that we are trying to place.
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
    /// Gets the player who placed the block involved in this event.
    ///
    /// May be null for legacy calls of the event.
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
        let cls = jni.find_class("org/bukkit/event/block/BlockCanBuildEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['isBuildable', 'setBuildable', 'getMaterial', 'getBlockData', 'getPlayer', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockCanBuildEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockCanBuildEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct InventoryBlockStartEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InventoryBlockStartEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryBlockStartEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InventoryBlockStartEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/InventoryBlockStartEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryBlockStartEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InventoryBlockStartEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        source: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::block::InventoryBlockStartEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(source.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/InventoryBlockStartEvent");
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
        crate::event::block::InventoryBlockStartEvent::from_raw(&jni, res)
    }
    /// Gets the source ItemStack for this event.
    pub fn source(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSource", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
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
        let cls = jni.find_class("org/bukkit/event/block/InventoryBlockStartEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getSource', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for InventoryBlockStartEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting InventoryBlockStartEvent into crate::event::block::BlockEvent",
        )
    }
}
#[repr(C)]
pub struct MoistureChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MoistureChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MoistureChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MoistureChangeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/MoistureChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MoistureChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MoistureChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        new_state: impl Into<crate::block::BlockState<'mc>>,
    ) -> Result<crate::event::block::MoistureChangeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockState;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_state.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/MoistureChangeEvent");
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
        crate::event::block::MoistureChangeEvent::from_raw(&jni, res)
    }
    /// Gets the new state of the affected block.
    pub fn new_state(&self) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewState", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
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
        let cls = jni.find_class("org/bukkit/event/block/MoistureChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getNewState', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for MoistureChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MoistureChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for MoistureChangeEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MoistureChangeEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct TNTPrimeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TNTPrimeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TNTPrimeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TNTPrimeEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/TNTPrimeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TNTPrimeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TNTPrimeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        ignite_cause: impl Into<crate::event::block::TNTPrimeEventPrimeCause<'mc>>,
        priming_entity: impl Into<crate::entity::Entity<'mc>>,
        priming_block: impl Into<crate::block::Block<'mc>>,
    ) -> Result<crate::event::block::TNTPrimeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/event/block/TNTPrimeEvent/PrimeCause;Lorg/bukkit/entity/Entity;Lorg/bukkit/block/Block;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(ignite_cause.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(priming_entity.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(priming_block.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/TNTPrimeEvent");
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
        crate::event::block::TNTPrimeEvent::from_raw(&jni, res)
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
    /// Get the cause of the TNT becoming primed.
    pub fn cause(
        &self,
    ) -> Result<crate::event::block::TNTPrimeEventPrimeCause<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/block/TNTPrimeEvent/PrimeCause;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCause", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::block::TNTPrimeEventPrimeCause::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the entity that caused the TNT to be primed.
    pub fn priming_entity(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrimingEntity",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Get the block that caused the TNT to be primed.
    pub fn priming_block(
        &self,
    ) -> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPrimingBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::Block::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
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
        let cls = jni.find_class("org/bukkit/event/block/TNTPrimeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['isCancelled', 'setCancelled', 'getCause', 'getPrimingEntity', 'getPrimingBlock', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for TNTPrimeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TNTPrimeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for TNTPrimeEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TNTPrimeEvent into crate::event::block::BlockEvent")
    }
}
pub enum TNTPrimeEventPrimeCause<'mc> {
    Fire {
        inner: TNTPrimeEventPrimeCauseStruct<'mc>,
    },
    Redstone {
        inner: TNTPrimeEventPrimeCauseStruct<'mc>,
    },
    Player {
        inner: TNTPrimeEventPrimeCauseStruct<'mc>,
    },
    Explosion {
        inner: TNTPrimeEventPrimeCauseStruct<'mc>,
    },
    Projectile {
        inner: TNTPrimeEventPrimeCauseStruct<'mc>,
    },
    BlockBreak {
        inner: TNTPrimeEventPrimeCauseStruct<'mc>,
    },
    Dispenser {
        inner: TNTPrimeEventPrimeCauseStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for TNTPrimeEventPrimeCause<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TNTPrimeEventPrimeCause::Fire { .. } => f.write_str("FIRE"),
            TNTPrimeEventPrimeCause::Redstone { .. } => f.write_str("REDSTONE"),
            TNTPrimeEventPrimeCause::Player { .. } => f.write_str("PLAYER"),
            TNTPrimeEventPrimeCause::Explosion { .. } => f.write_str("EXPLOSION"),
            TNTPrimeEventPrimeCause::Projectile { .. } => f.write_str("PROJECTILE"),
            TNTPrimeEventPrimeCause::BlockBreak { .. } => f.write_str("BLOCK_BREAK"),
            TNTPrimeEventPrimeCause::Dispenser { .. } => f.write_str("DISPENSER"),
        }
    }
}
impl<'mc> std::ops::Deref for TNTPrimeEventPrimeCause<'mc> {
    type Target = TNTPrimeEventPrimeCauseStruct<'mc>;
    fn deref(&self) -> &<TNTPrimeEventPrimeCause<'mc> as std::ops::Deref>::Target {
        match self {
            TNTPrimeEventPrimeCause::Fire { inner } => inner,
            TNTPrimeEventPrimeCause::Redstone { inner } => inner,
            TNTPrimeEventPrimeCause::Player { inner } => inner,
            TNTPrimeEventPrimeCause::Explosion { inner } => inner,
            TNTPrimeEventPrimeCause::Projectile { inner } => inner,
            TNTPrimeEventPrimeCause::BlockBreak { inner } => inner,
            TNTPrimeEventPrimeCause::Dispenser { inner } => inner,
        }
    }
}

impl<'mc> TNTPrimeEventPrimeCause<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<TNTPrimeEventPrimeCause<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/block/TNTPrimeEvent/PrimeCause");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/block/TNTPrimeEvent/PrimeCause;",
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
            "FIRE" => Ok(TNTPrimeEventPrimeCause::Fire {
                inner: TNTPrimeEventPrimeCauseStruct::from_raw(env, obj)?,
            }),
            "REDSTONE" => Ok(TNTPrimeEventPrimeCause::Redstone {
                inner: TNTPrimeEventPrimeCauseStruct::from_raw(env, obj)?,
            }),
            "PLAYER" => Ok(TNTPrimeEventPrimeCause::Player {
                inner: TNTPrimeEventPrimeCauseStruct::from_raw(env, obj)?,
            }),
            "EXPLOSION" => Ok(TNTPrimeEventPrimeCause::Explosion {
                inner: TNTPrimeEventPrimeCauseStruct::from_raw(env, obj)?,
            }),
            "PROJECTILE" => Ok(TNTPrimeEventPrimeCause::Projectile {
                inner: TNTPrimeEventPrimeCauseStruct::from_raw(env, obj)?,
            }),
            "BLOCK_BREAK" => Ok(TNTPrimeEventPrimeCause::BlockBreak {
                inner: TNTPrimeEventPrimeCauseStruct::from_raw(env, obj)?,
            }),
            "DISPENSER" => Ok(TNTPrimeEventPrimeCause::Dispenser {
                inner: TNTPrimeEventPrimeCauseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct TNTPrimeEventPrimeCauseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TNTPrimeEventPrimeCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Fire { inner } => inner.0.clone(),
            Self::Redstone { inner } => inner.0.clone(),
            Self::Player { inner } => inner.0.clone(),
            Self::Explosion { inner } => inner.0.clone(),
            Self::Projectile { inner } => inner.0.clone(),
            Self::BlockBreak { inner } => inner.0.clone(),
            Self::Dispenser { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Fire { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Redstone { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Player { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Explosion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Projectile { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BlockBreak { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Dispenser { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TNTPrimeEventPrimeCause<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TNTPrimeEventPrimeCause from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/TNTPrimeEvent/PrimeCause")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TNTPrimeEventPrimeCause object, got {}",
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
                "FIRE" => Ok(TNTPrimeEventPrimeCause::Fire {
                    inner: TNTPrimeEventPrimeCauseStruct::from_raw(env, obj)?,
                }),
                "REDSTONE" => Ok(TNTPrimeEventPrimeCause::Redstone {
                    inner: TNTPrimeEventPrimeCauseStruct::from_raw(env, obj)?,
                }),
                "PLAYER" => Ok(TNTPrimeEventPrimeCause::Player {
                    inner: TNTPrimeEventPrimeCauseStruct::from_raw(env, obj)?,
                }),
                "EXPLOSION" => Ok(TNTPrimeEventPrimeCause::Explosion {
                    inner: TNTPrimeEventPrimeCauseStruct::from_raw(env, obj)?,
                }),
                "PROJECTILE" => Ok(TNTPrimeEventPrimeCause::Projectile {
                    inner: TNTPrimeEventPrimeCauseStruct::from_raw(env, obj)?,
                }),
                "BLOCK_BREAK" => Ok(TNTPrimeEventPrimeCause::BlockBreak {
                    inner: TNTPrimeEventPrimeCauseStruct::from_raw(env, obj)?,
                }),
                "DISPENSER" => Ok(TNTPrimeEventPrimeCause::Dispenser {
                    inner: TNTPrimeEventPrimeCauseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for TNTPrimeEventPrimeCauseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TNTPrimeEventPrimeCauseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TNTPrimeEventPrimeCauseStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/TNTPrimeEvent/PrimeCause")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TNTPrimeEventPrimeCauseStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TNTPrimeEventPrimeCauseStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::block::TNTPrimeEventPrimeCause<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/block/TNTPrimeEvent/PrimeCause;");
        let cls = jni.find_class("org/bukkit/event/block/TNTPrimeEvent/PrimeCause");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::block::TNTPrimeEventPrimeCause::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct BlockExpEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockExpEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockExpEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BlockExpEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockExpEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockExpEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockExpEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        exp: i32,
    ) -> Result<crate::event::block::BlockExpEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(exp);
        let cls = jni.find_class("org/bukkit/event/block/BlockExpEvent");
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
        crate::event::block::BlockExpEvent::from_raw(&jni, res)
    }
    /// Get the experience dropped by the block after the event has processed
    pub fn exp_to_drop(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getExpToDrop", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the amount of experience dropped by the block after the event has
    /// processed
    pub fn set_exp_to_drop(&self, exp: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(exp);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExpToDrop",
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
        let cls = jni.find_class("org/bukkit/event/block/BlockExpEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getExpToDrop', 'setExpToDrop', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockExpEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockExpEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockDamageAbortEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockDamageAbortEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockDamageAbortEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BlockDamageAbortEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/BlockDamageAbortEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockDamageAbortEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockDamageAbortEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        player: impl Into<crate::entity::Player<'mc>>,
        block: impl Into<crate::block::Block<'mc>>,
        item_in_hand: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::block::BlockDamageAbortEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(player.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item_in_hand.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BlockDamageAbortEvent");
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
        crate::event::block::BlockDamageAbortEvent::from_raw(&jni, res)
    }
    /// Gets the player that stopped damaging the block involved in this event.
    pub fn player(&self) -> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Player::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the ItemStack for the item currently in the player's hand.
    pub fn item_in_hand(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemInHand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
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
        let cls = jni.find_class("org/bukkit/event/block/BlockDamageAbortEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getPlayer', 'getItemInHand', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockDamageAbortEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockDamageAbortEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockFadeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockFadeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockFadeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockFadeEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockFadeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockFadeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockFadeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        new_state: impl Into<crate::block::BlockState<'mc>>,
    ) -> Result<crate::event::block::BlockFadeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockState;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_state.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BlockFadeEvent");
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
        crate::event::block::BlockFadeEvent::from_raw(&jni, res)
    }
    /// Gets the state of the block that will be fading, melting or
    /// disappearing.
    pub fn new_state(&self) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewState", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
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
        let cls = jni.find_class("org/bukkit/event/block/BlockFadeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getNewState', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockFadeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockFadeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockFadeEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockFadeEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BellResonateEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BellResonateEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BellResonateEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BellResonateEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BellResonateEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BellResonateEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BellResonateEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        the_block: impl Into<crate::block::Block<'mc>>,
        resonated_entities: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<crate::event::block::BellResonateEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(the_block.into().jni_object().clone())
        });
        let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in resonated_entities {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_2,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let cls = jni.find_class("org/bukkit/event/block/BellResonateEvent");
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
        crate::event::block::BellResonateEvent::from_raw(&jni, res)
    }
    /// Get a mutable list of all {@link LivingEntity LivingEntities} to be
    /// highlighted by the bell's resonating. This list can be added to or
    /// removed from to change which entities are highlighted, and may be empty
    /// if no entities were resonated as a result of this event.
    ///
    /// While the highlighted entities will change, the particles that display
    /// over a resonated entity and their colors will not. This is handled by the
    /// client and cannot be controlled by the server.
    pub fn resonated_entities(
        &self,
    ) -> Result<Vec<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResonatedEntities",
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
        let cls = jni.find_class("org/bukkit/event/block/BellResonateEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getResonatedEntities', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BellResonateEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BellResonateEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockDispenseEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockDispenseEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockDispenseEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockDispenseEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockDispenseEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockDispenseEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockDispenseEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        dispensed: impl Into<crate::inventory::ItemStack<'mc>>,
        velocity: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<crate::event::block::BlockDispenseEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/util/Vector;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(dispensed.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(velocity.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BlockDispenseEvent");
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
        crate::event::block::BlockDispenseEvent::from_raw(&jni, res)
    }
    /// Gets the item that is being dispensed. Modifying the returned item will
    /// have no effect, you must use {@link
    /// #setItem(org.bukkit.inventory.ItemStack)} instead.
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
    /// Sets the item being dispensed.
    pub fn set_item(
        &self,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the velocity in meters per tick.
    ///
    /// Note: Modifying the returned Vector will not change the velocity, you
    /// must use {@link #setVelocity(org.bukkit.util.Vector)} instead.
    pub fn velocity(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVelocity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the velocity of the item being dispensed in meters per tick.
    pub fn set_velocity(
        &self,
        vel: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(vel.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVelocity",
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
        let cls = jni.find_class("org/bukkit/event/block/BlockDispenseEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getItem', 'setItem', 'getVelocity', 'setVelocity', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockDispenseEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockDispenseEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockDispenseEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockDispenseEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockDamageEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockDamageEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockDamageEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockDamageEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockDamageEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockDamageEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockDamageEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        player: impl Into<crate::entity::Player<'mc>>,
        block: impl Into<crate::block::Block<'mc>>,
        item_in_hand: impl Into<crate::inventory::ItemStack<'mc>>,
        insta_break: bool,
    ) -> Result<crate::event::block::BlockDamageEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(player.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item_in_hand.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Bool(insta_break.into());
        let cls = jni.find_class("org/bukkit/event/block/BlockDamageEvent");
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
        crate::event::block::BlockDamageEvent::from_raw(&jni, res)
    }
    /// Gets the player damaging the block involved in this event.
    pub fn player(&self) -> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Player::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets if the block is set to instantly break when damaged by the player.
    pub fn insta_break(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInstaBreak", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets if the block should instantly break when damaged by the player.
    pub fn set_insta_break(&self, bool: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(bool.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInstaBreak",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the ItemStack for the item currently in the player's hand.
    pub fn item_in_hand(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemInHand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
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
        let cls = jni.find_class("org/bukkit/event/block/BlockDamageEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getPlayer', 'getInstaBreak', 'setInstaBreak', 'getItemInHand', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockDamageEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockDamageEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockDamageEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockDamageEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BlockEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        the_block: impl Into<crate::block::Block<'mc>>,
    ) -> Result<crate::event::block::BlockEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(the_block.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BlockEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::block::BlockEvent::from_raw(&jni, res)
    }
    /// Gets the block involved in this event.
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
    // SUPER CLASS: org.bukkit.event.Event ( ['getBlock'])
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
impl<'mc> Into<crate::event::Event<'mc>> for BlockEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockEvent into crate::event::Event")
    }
}
#[repr(C)]
pub struct BlockDispenseLootEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockDispenseLootEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockDispenseLootEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BlockDispenseLootEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/BlockDispenseLootEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockDispenseLootEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockDispenseLootEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        player: impl Into<crate::entity::Player<'mc>>,
        the_block: impl Into<crate::block::Block<'mc>>,
        dispensed_loot: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<crate::event::block::BlockDispenseLootEvent<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(player.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(the_block.into().jni_object().clone())
        });
        let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in dispensed_loot {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_3,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_3 = jni::objects::JValueGen::Object(raw_val_3);
        let cls = jni.find_class("org/bukkit/event/block/BlockDispenseLootEvent");
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
        crate::event::block::BlockDispenseLootEvent::from_raw(&jni, res)
    }
    /// Gets the loot that will be dispensed.
    pub fn dispensed_loot(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDispensedLoot",
            sig.as_str(),
            vec![],
        );
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
    /// Sets the loot that will be dispensed.
    pub fn set_dispensed_loot(
        &self,
        dispensed_loot: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)V");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in dispensed_loot {
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
            "setDispensedLoot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the player associated with this event.
    ///
    /// <b>Warning:</b> Some event instances like a
    /// {@link org.bukkit.block.TrialSpawner} dispensing its reward loot may not
    /// have a player associated with them and will return null.
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
        let cls = jni.find_class("org/bukkit/event/block/BlockDispenseLootEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getDispensedLoot', 'setDispensedLoot', 'getPlayer', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockDispenseLootEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockDispenseLootEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockDispenseLootEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockDispenseLootEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockPlaceEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockPlaceEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockPlaceEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockPlaceEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockPlaceEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockPlaceEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockPlaceEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        placed_block: impl Into<crate::block::Block<'mc>>,
        replaced_block_state: impl Into<crate::block::BlockState<'mc>>,
        placed_against: impl Into<crate::block::Block<'mc>>,
        item_in_hand: impl Into<crate::inventory::ItemStack<'mc>>,
        the_player: impl Into<crate::entity::Player<'mc>>,
        can_build: bool,
        hand: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>,
    ) -> Result<crate::event::block::BlockPlaceEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/Block;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(placed_block.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/block/BlockState;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(replaced_block_state.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/block/Block;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(placed_against.into().jni_object().clone())
        });
        args.push(val_3);
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item_in_hand.into().jni_object().clone())
        });
        args.push(val_4);
        sig += "Lorg/bukkit/entity/Player;";
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(the_player.into().jni_object().clone())
        });
        args.push(val_5);
        sig += "Z";
        let val_6 = jni::objects::JValueGen::Bool(can_build.into());
        args.push(val_6);
        if let Some(a) = hand {
            sig += "Lorg/bukkit/inventory/EquipmentSlot;";
            let val_7 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_7);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/block/BlockPlaceEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::block::BlockPlaceEvent::from_raw(&jni, res)
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
    /// Gets the player who placed the block involved in this event.
    pub fn player(&self) -> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Player::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Clarity method for getting the placed block. Not really needed except
    /// for reasons of clarity.
    pub fn block_placed(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlockPlaced", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the BlockState for the block which was replaced. Material type air
    /// mostly.
    pub fn block_replaced_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockReplacedState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block that this block was placed against
    pub fn block_against(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlockAgainst", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the item in the player's hand when they placed the block.
    pub fn item_in_hand(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemInHand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the hand which placed the block
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
    /// Gets the value whether the player would be allowed to build here.
    /// Defaults to spawn if the server was going to stop them (such as, the
    /// player is in Spawn). Note that this is an entirely different check
    /// than BLOCK_CANBUILD, as this refers to a player, not universe-physics
    /// rule like cactus on dirt.
    pub fn can_build(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "canBuild", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the canBuild state of this event. Set to true if you want the
    /// player to be able to build.
    pub fn set_build(&self, can_build: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(can_build.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBuild",
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
        let cls = jni.find_class("org/bukkit/event/block/BlockPlaceEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['isCancelled', 'setCancelled', 'getPlayer', 'getBlockPlaced', 'getBlockReplacedState', 'getBlockAgainst', 'getItemInHand', 'getHand', 'canBuild', 'setBuild', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockPlaceEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockPlaceEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockPlaceEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockPlaceEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BellRingEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BellRingEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BellRingEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BellRingEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BellRingEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BellRingEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BellRingEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        the_block: impl Into<crate::block::Block<'mc>>,
        direction: impl Into<crate::block::BlockFace<'mc>>,
        entity: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<crate::event::block::BellRingEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockFace;Lorg/bukkit/entity/Entity;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(the_block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(direction.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BellRingEvent");
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
        crate::event::block::BellRingEvent::from_raw(&jni, res)
    }
    /// Get the direction in which the bell was rung.
    pub fn direction(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDirection", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Entity} that rang the bell (if there was one).
    pub fn entity(&self) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
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

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
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
        let cls = jni.find_class("org/bukkit/event/block/BellRingEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getDirection', 'getEntity', 'setCancelled', 'isCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BellRingEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BellRingEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BellRingEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BellRingEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockRedstoneEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockRedstoneEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockRedstoneEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockRedstoneEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockRedstoneEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockRedstoneEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockRedstoneEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        old_current: i32,
        new_current: i32,
    ) -> Result<crate::event::block::BlockRedstoneEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;II)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(old_current);
        let val_3 = jni::objects::JValueGen::Int(new_current);
        let cls = jni.find_class("org/bukkit/event/block/BlockRedstoneEvent");
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
        crate::event::block::BlockRedstoneEvent::from_raw(&jni, res)
    }
    /// Gets the old current of this block
    pub fn old_current(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOldCurrent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the new current of this block
    pub fn new_current(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewCurrent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the new current of this block
    pub fn set_new_current(&self, new_current: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(new_current);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNewCurrent",
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
        let cls = jni.find_class("org/bukkit/event/block/BlockRedstoneEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getOldCurrent', 'getNewCurrent', 'setNewCurrent', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockRedstoneEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockRedstoneEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockReceiveGameEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockReceiveGameEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockReceiveGameEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BlockReceiveGameEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/BlockReceiveGameEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockReceiveGameEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockReceiveGameEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        event: impl Into<crate::GameEvent<'mc>>,
        block: impl Into<crate::block::Block<'mc>>,
        entity: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<crate::event::block::BlockReceiveGameEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/GameEvent;Lorg/bukkit/block/Block;Lorg/bukkit/entity/Entity;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(event.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BlockReceiveGameEvent");
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
        crate::event::block::BlockReceiveGameEvent::from_raw(&jni, res)
    }
    /// Get the underlying event.
    pub fn event(&self) -> Result<crate::GameEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/GameEvent;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEvent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::GameEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the entity which triggered this event, if present.
    pub fn entity(&self) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
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

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
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
        let cls = jni.find_class("org/bukkit/event/block/BlockReceiveGameEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getEvent', 'getEntity', 'setCancelled', 'isCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockReceiveGameEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockReceiveGameEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockReceiveGameEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockReceiveGameEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct CrafterCraftEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CrafterCraftEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CrafterCraftEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CrafterCraftEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/CrafterCraftEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CrafterCraftEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CrafterCraftEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        the_block: impl Into<crate::block::Block<'mc>>,
        recipe: impl Into<crate::inventory::CraftingRecipe<'mc>>,
        result: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::block::CrafterCraftEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/CraftingRecipe;Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(the_block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(recipe.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(result.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/CrafterCraftEvent");
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
        crate::event::block::CrafterCraftEvent::from_raw(&jni, res)
    }
    /// Gets the result for the craft.
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResult", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the result of the craft.
    pub fn set_result(
        &self,
        result: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(result.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the recipe that was used to craft this item.
    pub fn recipe(
        &self,
    ) -> Result<crate::inventory::CraftingRecipe<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/CraftingRecipe;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRecipe", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CraftingRecipe::from_raw(&self.jni_ref(), unsafe {
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
        let cls = jni.find_class("org/bukkit/event/block/CrafterCraftEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getResult', 'setResult', 'getRecipe', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for CrafterCraftEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CrafterCraftEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for CrafterCraftEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CrafterCraftEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BrewingStartEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BrewingStartEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BrewingStartEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BrewingStartEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BrewingStartEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BrewingStartEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BrewingStartEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        furnace: impl Into<crate::block::Block<'mc>>,
        source: impl Into<crate::inventory::ItemStack<'mc>>,
        brewing_time: i32,
    ) -> Result<crate::event::block::BrewingStartEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(furnace.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(source.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(brewing_time);
        let cls = jni.find_class("org/bukkit/event/block/BrewingStartEvent");
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
        crate::event::block::BrewingStartEvent::from_raw(&jni, res)
    }
    /// Gets the total brew time associated with this event.
    pub fn total_brew_time(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTotalBrewTime",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the total brew time for this event.
    pub fn set_total_brew_time(&self, brew_time: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(brew_time);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTotalBrewTime",
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
        let cls = jni.find_class("org/bukkit/event/block/BrewingStartEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.InventoryBlockStartEvent ( ['getTotalBrewTime', 'setTotalBrewTime', 'getHandlers', 'getHandlerList'])
    /// Gets the source ItemStack for this event.
    pub fn source(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::block::InventoryBlockStartEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::block::InventoryBlockStartEvent = temp_clone.into();
        real.source()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::InventoryBlockStartEvent<'mc>> for BrewingStartEvent<'mc> {
    fn into(self) -> crate::event::block::InventoryBlockStartEvent<'mc> {
        crate::event::block::InventoryBlockStartEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting BrewingStartEvent into crate::event::block::InventoryBlockStartEvent",
        )
    }
}
#[repr(C)]
pub struct BlockCookEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockCookEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockCookEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockCookEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockCookEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockCookEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockCookEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        source: impl Into<crate::inventory::ItemStack<'mc>>,
        result: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::block::BlockCookEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(source.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(result.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/BlockCookEvent");
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
        crate::event::block::BlockCookEvent::from_raw(&jni, res)
    }
    /// Gets the smelted ItemStack for this event
    pub fn source(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSource", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the resultant ItemStack for this event
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResult", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the resultant ItemStack for this event
    pub fn set_result(
        &self,
        result: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(result.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
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
        let cls = jni.find_class("org/bukkit/event/block/BlockCookEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getSource', 'getResult', 'setResult', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockCookEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockCookEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockCookEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockCookEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct SculkBloomEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SculkBloomEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SculkBloomEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SculkBloomEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/SculkBloomEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SculkBloomEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SculkBloomEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        the_block: impl Into<crate::block::Block<'mc>>,
        charge: i32,
    ) -> Result<crate::event::block::SculkBloomEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(the_block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(charge);
        let cls = jni.find_class("org/bukkit/event/block/SculkBloomEvent");
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
        crate::event::block::SculkBloomEvent::from_raw(&jni, res)
    }
    /// Returns the charge of the cursor, &lt; 1000 by default.
    pub fn charge(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCharge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the charge of the cursor.
    ///
    /// Increasing the charge of a cursor makes the cursor last longer, giving
    /// it more time to spread sculk blocks across a larger range.
    ///
    /// Typically, charges should be set to the exp reward of a mob
    /// ({@link EntityDeathEvent#getDroppedExp()}), which is usually
    /// 3-5 for animals, and 5-10 for the average mob (up to 50 for
    /// wither skeletons). Roughly speaking, for each charge, 1 more
    /// sculk block will be placed.
    pub fn set_charge(&self, charge: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(charge);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCharge",
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
        let cls = jni.find_class("org/bukkit/event/block/SculkBloomEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getCharge', 'setCharge', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for SculkBloomEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SculkBloomEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for SculkBloomEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SculkBloomEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct BlockPhysicsEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockPhysicsEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockPhysicsEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockPhysicsEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockPhysicsEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockPhysicsEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockPhysicsEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        block: impl Into<crate::block::Block<'mc>>,
        changed: impl Into<crate::block::data::BlockData<'mc>>,
        source_block: std::option::Option<impl Into<crate::block::Block<'mc>>>,
    ) -> Result<crate::event::block::BlockPhysicsEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/Block;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/block/data/BlockData;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(changed.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = source_block {
            sig += "Lorg/bukkit/block/Block;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/block/BlockPhysicsEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::block::BlockPhysicsEvent::from_raw(&jni, res)
    }
    /// Gets the source block that triggered this event.
    /// Note: This will default to block if not set.
    pub fn source_block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSourceBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the type of block that changed, causing this event
    pub fn changed_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChangedType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
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
        let cls = jni.find_class("org/bukkit/event/block/BlockPhysicsEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getSourceBlock', 'getChangedType', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockPhysicsEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockPhysicsEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockPhysicsEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockPhysicsEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct VaultDisplayItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for VaultDisplayItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for VaultDisplayItemEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate VaultDisplayItemEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/VaultDisplayItemEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VaultDisplayItemEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> VaultDisplayItemEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        the_block: impl Into<crate::block::Block<'mc>>,
        display_item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::block::VaultDisplayItemEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(the_block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(display_item.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/VaultDisplayItemEvent");
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
        crate::event::block::VaultDisplayItemEvent::from_raw(&jni, res)
    }
    /// Gets the item that will be displayed inside the vault.
    pub fn display_item(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayItem", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the item that will be displayed inside the vault.
    pub fn set_display_item(
        &self,
        display_item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(display_item.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayItem",
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
        let cls = jni.find_class("org/bukkit/event/block/VaultDisplayItemEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getDisplayItem', 'setDisplayItem', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for VaultDisplayItemEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting VaultDisplayItemEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for VaultDisplayItemEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting VaultDisplayItemEvent into crate::event::block::BlockEvent")
    }
}
#[repr(C)]
pub struct FluidLevelChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FluidLevelChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FluidLevelChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate FluidLevelChangeEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/block/FluidLevelChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FluidLevelChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FluidLevelChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        the_block: impl Into<crate::block::Block<'mc>>,
        new_data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::event::block::FluidLevelChangeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(the_block.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_data.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/block/FluidLevelChangeEvent");
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
        crate::event::block::FluidLevelChangeEvent::from_raw(&jni, res)
    }
    /// Gets the new data of the changed block.
    pub fn new_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the new data of the changed block. Must be of the same Material as
    /// the old one.
    pub fn set_new_data(
        &self,
        new_data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNewData",
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
        let cls = jni.find_class("org/bukkit/event/block/FluidLevelChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getNewData', 'setNewData', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for FluidLevelChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FluidLevelChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for FluidLevelChangeEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FluidLevelChangeEvent into crate::event::block::BlockEvent")
    }
}
