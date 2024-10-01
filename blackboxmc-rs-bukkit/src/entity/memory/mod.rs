#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub enum MemoryKey<'mc> {
    Home { inner: MemoryKeyStruct<'mc> },
    PotentialJobSite { inner: MemoryKeyStruct<'mc> },
    JobSite { inner: MemoryKeyStruct<'mc> },
    MeetingPoint { inner: MemoryKeyStruct<'mc> },
    GolemDetectedRecently { inner: MemoryKeyStruct<'mc> },
    LastSlept { inner: MemoryKeyStruct<'mc> },
    LastWoken { inner: MemoryKeyStruct<'mc> },
    LastWorkedAtPoi { inner: MemoryKeyStruct<'mc> },
    UniversalAnger { inner: MemoryKeyStruct<'mc> },
    AngryAt { inner: MemoryKeyStruct<'mc> },
    AdmiringItem { inner: MemoryKeyStruct<'mc> },
    AdmiringDisabled { inner: MemoryKeyStruct<'mc> },
    HuntedRecently { inner: MemoryKeyStruct<'mc> },
    PlayDeadTicks { inner: MemoryKeyStruct<'mc> },
    TemptationCooldownTicks { inner: MemoryKeyStruct<'mc> },
    IsTempted { inner: MemoryKeyStruct<'mc> },
    LongJumpCoolingDown { inner: MemoryKeyStruct<'mc> },
    HasHuntingCooldown { inner: MemoryKeyStruct<'mc> },
    RamCooldownTicks { inner: MemoryKeyStruct<'mc> },
    LikedPlayer { inner: MemoryKeyStruct<'mc> },
    LikedNoteblockPosition { inner: MemoryKeyStruct<'mc> },
    LikedNoteblockCooldownTicks { inner: MemoryKeyStruct<'mc> },
    ItemPickupCooldownTicks { inner: MemoryKeyStruct<'mc> },
    SnifferExploredPositions { inner: MemoryKeyStruct<'mc> },
}
impl<'mc> std::fmt::Display for MemoryKey<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryKey::Home { .. } => f.write_str("HOME"),
            MemoryKey::PotentialJobSite { .. } => f.write_str("POTENTIAL_JOB_SITE"),
            MemoryKey::JobSite { .. } => f.write_str("JOB_SITE"),
            MemoryKey::MeetingPoint { .. } => f.write_str("MEETING_POINT"),
            MemoryKey::GolemDetectedRecently { .. } => f.write_str("GOLEM_DETECTED_RECENTLY"),
            MemoryKey::LastSlept { .. } => f.write_str("LAST_SLEPT"),
            MemoryKey::LastWoken { .. } => f.write_str("LAST_WOKEN"),
            MemoryKey::LastWorkedAtPoi { .. } => f.write_str("LAST_WORKED_AT_POI"),
            MemoryKey::UniversalAnger { .. } => f.write_str("UNIVERSAL_ANGER"),
            MemoryKey::AngryAt { .. } => f.write_str("ANGRY_AT"),
            MemoryKey::AdmiringItem { .. } => f.write_str("ADMIRING_ITEM"),
            MemoryKey::AdmiringDisabled { .. } => f.write_str("ADMIRING_DISABLED"),
            MemoryKey::HuntedRecently { .. } => f.write_str("HUNTED_RECENTLY"),
            MemoryKey::PlayDeadTicks { .. } => f.write_str("PLAY_DEAD_TICKS"),
            MemoryKey::TemptationCooldownTicks { .. } => f.write_str("TEMPTATION_COOLDOWN_TICKS"),
            MemoryKey::IsTempted { .. } => f.write_str("IS_TEMPTED"),
            MemoryKey::LongJumpCoolingDown { .. } => f.write_str("LONG_JUMP_COOLING_DOWN"),
            MemoryKey::HasHuntingCooldown { .. } => f.write_str("HAS_HUNTING_COOLDOWN"),
            MemoryKey::RamCooldownTicks { .. } => f.write_str("RAM_COOLDOWN_TICKS"),
            MemoryKey::LikedPlayer { .. } => f.write_str("LIKED_PLAYER"),
            MemoryKey::LikedNoteblockPosition { .. } => f.write_str("LIKED_NOTEBLOCK_POSITION"),
            MemoryKey::LikedNoteblockCooldownTicks { .. } => {
                f.write_str("LIKED_NOTEBLOCK_COOLDOWN_TICKS")
            }
            MemoryKey::ItemPickupCooldownTicks { .. } => f.write_str("ITEM_PICKUP_COOLDOWN_TICKS"),
            MemoryKey::SnifferExploredPositions { .. } => f.write_str("SNIFFER_EXPLORED_POSITIONS"),
        }
    }
}

impl<'mc> MemoryKey<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<MemoryKey<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/entity/memory/MemoryKey");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/entity/memory/MemoryKey;",
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
            "HOME" => Ok(MemoryKey::Home {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "POTENTIAL_JOB_SITE" => Ok(MemoryKey::PotentialJobSite {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "JOB_SITE" => Ok(MemoryKey::JobSite {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "MEETING_POINT" => Ok(MemoryKey::MeetingPoint {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "GOLEM_DETECTED_RECENTLY" => Ok(MemoryKey::GolemDetectedRecently {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "LAST_SLEPT" => Ok(MemoryKey::LastSlept {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "LAST_WOKEN" => Ok(MemoryKey::LastWoken {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "LAST_WORKED_AT_POI" => Ok(MemoryKey::LastWorkedAtPoi {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "UNIVERSAL_ANGER" => Ok(MemoryKey::UniversalAnger {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "ANGRY_AT" => Ok(MemoryKey::AngryAt {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "ADMIRING_ITEM" => Ok(MemoryKey::AdmiringItem {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "ADMIRING_DISABLED" => Ok(MemoryKey::AdmiringDisabled {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "HUNTED_RECENTLY" => Ok(MemoryKey::HuntedRecently {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "PLAY_DEAD_TICKS" => Ok(MemoryKey::PlayDeadTicks {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "TEMPTATION_COOLDOWN_TICKS" => Ok(MemoryKey::TemptationCooldownTicks {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "IS_TEMPTED" => Ok(MemoryKey::IsTempted {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "LONG_JUMP_COOLING_DOWN" => Ok(MemoryKey::LongJumpCoolingDown {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "HAS_HUNTING_COOLDOWN" => Ok(MemoryKey::HasHuntingCooldown {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "RAM_COOLDOWN_TICKS" => Ok(MemoryKey::RamCooldownTicks {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "LIKED_PLAYER" => Ok(MemoryKey::LikedPlayer {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "LIKED_NOTEBLOCK_POSITION" => Ok(MemoryKey::LikedNoteblockPosition {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "LIKED_NOTEBLOCK_COOLDOWN_TICKS" => Ok(MemoryKey::LikedNoteblockCooldownTicks {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "ITEM_PICKUP_COOLDOWN_TICKS" => Ok(MemoryKey::ItemPickupCooldownTicks {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),
            "SNIFFER_EXPLORED_POSITIONS" => Ok(MemoryKey::SnifferExploredPositions {
                inner: MemoryKeyStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct MemoryKeyStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MemoryKey<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Home { inner } => inner.0.clone(),
            Self::PotentialJobSite { inner } => inner.0.clone(),
            Self::JobSite { inner } => inner.0.clone(),
            Self::MeetingPoint { inner } => inner.0.clone(),
            Self::GolemDetectedRecently { inner } => inner.0.clone(),
            Self::LastSlept { inner } => inner.0.clone(),
            Self::LastWoken { inner } => inner.0.clone(),
            Self::LastWorkedAtPoi { inner } => inner.0.clone(),
            Self::UniversalAnger { inner } => inner.0.clone(),
            Self::AngryAt { inner } => inner.0.clone(),
            Self::AdmiringItem { inner } => inner.0.clone(),
            Self::AdmiringDisabled { inner } => inner.0.clone(),
            Self::HuntedRecently { inner } => inner.0.clone(),
            Self::PlayDeadTicks { inner } => inner.0.clone(),
            Self::TemptationCooldownTicks { inner } => inner.0.clone(),
            Self::IsTempted { inner } => inner.0.clone(),
            Self::LongJumpCoolingDown { inner } => inner.0.clone(),
            Self::HasHuntingCooldown { inner } => inner.0.clone(),
            Self::RamCooldownTicks { inner } => inner.0.clone(),
            Self::LikedPlayer { inner } => inner.0.clone(),
            Self::LikedNoteblockPosition { inner } => inner.0.clone(),
            Self::LikedNoteblockCooldownTicks { inner } => inner.0.clone(),
            Self::ItemPickupCooldownTicks { inner } => inner.0.clone(),
            Self::SnifferExploredPositions { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Home { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PotentialJobSite { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::JobSite { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::MeetingPoint { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GolemDetectedRecently { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LastSlept { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LastWoken { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LastWorkedAtPoi { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::UniversalAnger { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::AngryAt { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::AdmiringItem { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::AdmiringDisabled { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HuntedRecently { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PlayDeadTicks { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TemptationCooldownTicks { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::IsTempted { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LongJumpCoolingDown { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HasHuntingCooldown { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RamCooldownTicks { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LikedPlayer { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LikedNoteblockPosition { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LikedNoteblockCooldownTicks { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ItemPickupCooldownTicks { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SnifferExploredPositions { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MemoryKey<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MemoryKey from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/entity/memory/MemoryKey")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MemoryKey object, got {}",
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
                "HOME" => Ok(MemoryKey::Home {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "POTENTIAL_JOB_SITE" => Ok(MemoryKey::PotentialJobSite {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "JOB_SITE" => Ok(MemoryKey::JobSite {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "MEETING_POINT" => Ok(MemoryKey::MeetingPoint {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "GOLEM_DETECTED_RECENTLY" => Ok(MemoryKey::GolemDetectedRecently {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "LAST_SLEPT" => Ok(MemoryKey::LastSlept {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "LAST_WOKEN" => Ok(MemoryKey::LastWoken {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "LAST_WORKED_AT_POI" => Ok(MemoryKey::LastWorkedAtPoi {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "UNIVERSAL_ANGER" => Ok(MemoryKey::UniversalAnger {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "ANGRY_AT" => Ok(MemoryKey::AngryAt {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "ADMIRING_ITEM" => Ok(MemoryKey::AdmiringItem {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "ADMIRING_DISABLED" => Ok(MemoryKey::AdmiringDisabled {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "HUNTED_RECENTLY" => Ok(MemoryKey::HuntedRecently {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "PLAY_DEAD_TICKS" => Ok(MemoryKey::PlayDeadTicks {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "TEMPTATION_COOLDOWN_TICKS" => Ok(MemoryKey::TemptationCooldownTicks {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "IS_TEMPTED" => Ok(MemoryKey::IsTempted {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "LONG_JUMP_COOLING_DOWN" => Ok(MemoryKey::LongJumpCoolingDown {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "HAS_HUNTING_COOLDOWN" => Ok(MemoryKey::HasHuntingCooldown {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "RAM_COOLDOWN_TICKS" => Ok(MemoryKey::RamCooldownTicks {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "LIKED_PLAYER" => Ok(MemoryKey::LikedPlayer {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "LIKED_NOTEBLOCK_POSITION" => Ok(MemoryKey::LikedNoteblockPosition {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "LIKED_NOTEBLOCK_COOLDOWN_TICKS" => Ok(MemoryKey::LikedNoteblockCooldownTicks {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "ITEM_PICKUP_COOLDOWN_TICKS" => Ok(MemoryKey::ItemPickupCooldownTicks {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                "SNIFFER_EXPLORED_POSITIONS" => Ok(MemoryKey::SnifferExploredPositions {
                    inner: MemoryKeyStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for MemoryKeyStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MemoryKeyStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MemoryKeyStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/entity/memory/MemoryKey")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MemoryKeyStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MemoryKeyStruct<'mc> {
    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/NamespacedKey;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKey", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::NamespacedKey::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the class of values associated with this memory.
    pub fn memory_class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMemoryClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    /// Returns a {@link MemoryKey} by a {@link NamespacedKey}.
    pub fn get_by_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        namespaced_key: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<Option<crate::entity::memory::MemoryKey<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/entity/memory/MemoryKey;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(namespaced_key.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/entity/memory/MemoryKey");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        let obj = res.l()?;
        Ok(Some(crate::entity::memory::MemoryKey::from_raw(&jni, obj)?))
    }
    /// Returns the set of all MemoryKeys.
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let cls = jni.find_class("org/bukkit/entity/memory/MemoryKey");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        blackboxmc_java::util::JavaSet::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
