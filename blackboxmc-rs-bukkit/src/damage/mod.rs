#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct DamageSource<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DamageSource<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DamageSource<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DamageSource from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/damage/DamageSource")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DamageSource object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DamageSource<'mc> {
    /// Get the {@link DamageType}.
    pub fn damage_type(
        &self,
    ) -> Result<crate::damage::DamageType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/damage/DamageType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDamageType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::damage::DamageType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Entity} that caused the damage to occur.
    ///
    /// Not to be confused with {@link #getDirectEntity()}, the causing entity is
    /// the entity to which the damage is ultimately attributed if the receiver
    /// is killed. If, for example, the receiver was damaged by a projectile, the
    /// shooter/thrower would be returned.
    pub fn causing_entity(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCausingEntity",
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
    /// Get the {@link Entity} that directly caused the damage.
    ///
    /// Not to be confused with {@link #getCausingEntity()}, the direct entity is
    /// the entity that actually inflicted the damage. If, for example, the
    /// receiver was damaged by a projectile, the projectile would be returned.
    pub fn direct_entity(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDirectEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Get the {@link Location} from where the damage originated. This will only
    /// be present if an entity did not cause the damage.
    pub fn damage_location(
        &self,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDamageLocation",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Get the {@link Location} from where the damage originated.
    ///
    /// This is a convenience method to get the final location of the damage.
    /// This method will attempt to return
    /// {@link #getDamageLocation() the damage location}. If this is null, the
    /// {@link #getCausingEntity() causing entity location} will be returned.
    /// Finally if there is no damage location nor a causing entity, null will be
    /// returned.
    pub fn source_location(
        &self,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSourceLocation",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Get if this damage is indirect.
    ///
    /// Damage is considered indirect if {@link #getCausingEntity()} is not equal
    /// to {@link #getDirectEntity()}. This will be the case, for example, if a
    /// skeleton shot an arrow or a player threw a potion.
    pub fn is_indirect(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isIndirect", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the amount of hunger exhaustion caused by this damage.
    pub fn food_exhaustion(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFoodExhaustion",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Gets if this source of damage scales with difficulty.
    pub fn scales_with_difficulty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "scalesWithDifficulty",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Create a new {@link DamageSource.Builder}.
    pub fn builder(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        damage_type: impl Into<crate::damage::DamageType<'mc>>,
    ) -> Result<crate::damage::DamageSourceBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/damage/DamageType;)Lorg/bukkit/damage/DamageSource/Builder;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(damage_type.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/damage/DamageSource");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "builder",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::damage::DamageSourceBuilder::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum DamageScaling<'mc> {
    Never { inner: DamageScalingStruct<'mc> },
    WhenCausedByLivingNonPlayer { inner: DamageScalingStruct<'mc> },
    Always { inner: DamageScalingStruct<'mc> },
}
impl<'mc> std::fmt::Display for DamageScaling<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DamageScaling::Never { .. } => f.write_str("NEVER"),
            DamageScaling::WhenCausedByLivingNonPlayer { .. } => {
                f.write_str("WHEN_CAUSED_BY_LIVING_NON_PLAYER")
            }
            DamageScaling::Always { .. } => f.write_str("ALWAYS"),
        }
    }
}

impl<'mc> DamageScaling<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<DamageScaling<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/damage/DamageScaling");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/damage/DamageScaling;",
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
            "NEVER" => Ok(DamageScaling::Never {
                inner: DamageScalingStruct::from_raw(env, obj)?,
            }),
            "WHEN_CAUSED_BY_LIVING_NON_PLAYER" => Ok(DamageScaling::WhenCausedByLivingNonPlayer {
                inner: DamageScalingStruct::from_raw(env, obj)?,
            }),
            "ALWAYS" => Ok(DamageScaling::Always {
                inner: DamageScalingStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct DamageScalingStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DamageScaling<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Never { inner } => inner.0.clone(),
            Self::WhenCausedByLivingNonPlayer { inner } => inner.0.clone(),
            Self::Always { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Never { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WhenCausedByLivingNonPlayer { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Always { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DamageScaling<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DamageScaling from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/damage/DamageScaling")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DamageScaling object, got {}",
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
                "NEVER" => Ok(DamageScaling::Never {
                    inner: DamageScalingStruct::from_raw(env, obj)?,
                }),
                "WHEN_CAUSED_BY_LIVING_NON_PLAYER" => {
                    Ok(DamageScaling::WhenCausedByLivingNonPlayer {
                        inner: DamageScalingStruct::from_raw(env, obj)?,
                    })
                }
                "ALWAYS" => Ok(DamageScaling::Always {
                    inner: DamageScalingStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for DamageScalingStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DamageScalingStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DamageScalingStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/damage/DamageScaling")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DamageScalingStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DamageScalingStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::damage::DamageScaling<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/damage/DamageScaling;");
        let cls = jni.find_class("org/bukkit/damage/DamageScaling");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::damage::DamageScaling::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct DamageEffect<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DamageEffect<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DamageEffect<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DamageEffect from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/damage/DamageEffect")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DamageEffect object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DamageEffect<'mc> {
    pub fn get_damage_effect(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        key: impl Into<String>,
    ) -> Result<crate::damage::DamageEffect<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/damage/DamageEffect;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(key.into())?,
        ));
        let cls = jni.find_class("org/bukkit/damage/DamageEffect");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getDamageEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::damage::DamageEffect::from_raw(&jni, obj)
    }
    /// Get the {@link Sound} played for this {@link DamageEffect}.
    pub fn sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSound", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum DeathMessageType<'mc> {
    Default { inner: DeathMessageTypeStruct<'mc> },
    FallVariants { inner: DeathMessageTypeStruct<'mc> },
    IntentionalGameDesign { inner: DeathMessageTypeStruct<'mc> },
}
impl<'mc> std::fmt::Display for DeathMessageType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeathMessageType::Default { .. } => f.write_str("DEFAULT"),
            DeathMessageType::FallVariants { .. } => f.write_str("FALL_VARIANTS"),
            DeathMessageType::IntentionalGameDesign { .. } => {
                f.write_str("INTENTIONAL_GAME_DESIGN")
            }
        }
    }
}

impl<'mc> DeathMessageType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<DeathMessageType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/damage/DeathMessageType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/damage/DeathMessageType;",
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
            "DEFAULT" => Ok(DeathMessageType::Default {
                inner: DeathMessageTypeStruct::from_raw(env, obj)?,
            }),
            "FALL_VARIANTS" => Ok(DeathMessageType::FallVariants {
                inner: DeathMessageTypeStruct::from_raw(env, obj)?,
            }),
            "INTENTIONAL_GAME_DESIGN" => Ok(DeathMessageType::IntentionalGameDesign {
                inner: DeathMessageTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct DeathMessageTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DeathMessageType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Default { inner } => inner.0.clone(),
            Self::FallVariants { inner } => inner.0.clone(),
            Self::IntentionalGameDesign { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Default { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FallVariants { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::IntentionalGameDesign { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DeathMessageType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DeathMessageType from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/damage/DeathMessageType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DeathMessageType object, got {}",
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
                "DEFAULT" => Ok(DeathMessageType::Default {
                    inner: DeathMessageTypeStruct::from_raw(env, obj)?,
                }),
                "FALL_VARIANTS" => Ok(DeathMessageType::FallVariants {
                    inner: DeathMessageTypeStruct::from_raw(env, obj)?,
                }),
                "INTENTIONAL_GAME_DESIGN" => Ok(DeathMessageType::IntentionalGameDesign {
                    inner: DeathMessageTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for DeathMessageTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DeathMessageTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate DeathMessageTypeStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/damage/DeathMessageType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DeathMessageTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DeathMessageTypeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::damage::DeathMessageType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/damage/DeathMessageType;");
        let cls = jni.find_class("org/bukkit/damage/DeathMessageType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::damage::DeathMessageType::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct DamageType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DamageType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DamageType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DamageType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/damage/DamageType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DamageType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DamageType<'mc> {
    pub fn get_damage_type(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        key: impl Into<String>,
    ) -> Result<crate::damage::DamageType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/damage/DamageType;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(key.into())?,
        ));
        let cls = jni.find_class("org/bukkit/damage/DamageType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getDamageType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::damage::DamageType::from_raw(&jni, obj)
    }
    /// {@inheritDoc}
    ///
    /// The returned key is that of the death message sent when this damage type
    /// is responsible for the death of an entity.
    ///
    /// <strong>Note</strong> This translation key is only used if
    /// {@link #getDeathMessageType()} is {@link DeathMessageType#DEFAULT}
    pub fn translation_key(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTranslationKey",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Get the {@link DamageScaling} for this damage type.
    pub fn damage_scaling(
        &self,
    ) -> Result<crate::damage::DamageScaling<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/damage/DamageScaling;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDamageScaling",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::damage::DamageScaling::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link DamageEffect} for this damage type.
    pub fn damage_effect(
        &self,
    ) -> Result<crate::damage::DamageEffect<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/damage/DamageEffect;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDamageEffect", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::damage::DamageEffect::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link DeathMessageType} for this damage type.
    pub fn death_message_type(
        &self,
    ) -> Result<crate::damage::DeathMessageType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/damage/DeathMessageType;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDeathMessageType",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::damage::DeathMessageType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of hunger exhaustion caused by this damage type.
    pub fn exhaustion(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getExhaustion", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Return the namespaced identifier for this object.
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for DamageType<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting DamageType into crate::Keyed")
    }
}
impl<'mc> Into<crate::Translatable<'mc>> for DamageType<'mc> {
    fn into(self) -> crate::Translatable<'mc> {
        crate::Translatable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting DamageType into crate::Translatable")
    }
}
#[repr(C)]
pub struct DamageSourceBuilder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DamageSourceBuilder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DamageSourceBuilder<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DamageSourceBuilder from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/damage/DamageSource/Builder")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DamageSourceBuilder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DamageSourceBuilder<'mc> {
    /// Set the {@link Entity} that caused the damage.
    pub fn with_causing_entity(
        &self,
        entity: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<crate::damage::DamageSourceBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/entity/Entity;)Lorg/bukkit/damage/DamageSource/Builder;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "withCausingEntity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::damage::DamageSourceBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the {@link Entity} that directly inflicted the damage.
    pub fn with_direct_entity(
        &self,
        entity: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<crate::damage::DamageSourceBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/entity/Entity;)Lorg/bukkit/damage/DamageSource/Builder;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "withDirectEntity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::damage::DamageSourceBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the {@link Location} of the source of damage.
    pub fn with_damage_location(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<crate::damage::DamageSourceBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Location;)Lorg/bukkit/damage/DamageSource/Builder;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "withDamageLocation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::damage::DamageSourceBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Create a new {@link DamageSource} instance using the supplied
    /// parameters.
    pub fn build(&self) -> Result<crate::damage::DamageSource<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/damage/DamageSource;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "build", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::damage::DamageSource::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
