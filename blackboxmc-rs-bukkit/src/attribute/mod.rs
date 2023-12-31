#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents an object which may contain attributes.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Attributable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Attributable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Attributable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Attributable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/attribute/Attributable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Attributable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Attributable<'mc> {
    pub fn get_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
    ) -> Result<crate::attribute::AttributeInstance<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/attribute/Attribute;)Lorg/bukkit/attribute/AttributeInstance;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttribute",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::attribute::AttributeInstance::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum Attribute<'mc> {
    GenericMaxHealth { inner: AttributeStruct<'mc> },
    GenericFollowRange { inner: AttributeStruct<'mc> },
    GenericKnockbackResistance { inner: AttributeStruct<'mc> },
    GenericMovementSpeed { inner: AttributeStruct<'mc> },
    GenericFlyingSpeed { inner: AttributeStruct<'mc> },
    GenericAttackDamage { inner: AttributeStruct<'mc> },
    GenericAttackKnockback { inner: AttributeStruct<'mc> },
    GenericAttackSpeed { inner: AttributeStruct<'mc> },
    GenericArmor { inner: AttributeStruct<'mc> },
    GenericArmorToughness { inner: AttributeStruct<'mc> },
    GenericLuck { inner: AttributeStruct<'mc> },
    HorseJumpStrength { inner: AttributeStruct<'mc> },
    ZombieSpawnReinforcements { inner: AttributeStruct<'mc> },
}
impl<'mc> std::fmt::Display for Attribute<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::GenericMaxHealth { .. } => f.write_str("GENERIC_MAX_HEALTH"),
            Attribute::GenericFollowRange { .. } => f.write_str("GENERIC_FOLLOW_RANGE"),
            Attribute::GenericKnockbackResistance { .. } => {
                f.write_str("GENERIC_KNOCKBACK_RESISTANCE")
            }
            Attribute::GenericMovementSpeed { .. } => f.write_str("GENERIC_MOVEMENT_SPEED"),
            Attribute::GenericFlyingSpeed { .. } => f.write_str("GENERIC_FLYING_SPEED"),
            Attribute::GenericAttackDamage { .. } => f.write_str("GENERIC_ATTACK_DAMAGE"),
            Attribute::GenericAttackKnockback { .. } => f.write_str("GENERIC_ATTACK_KNOCKBACK"),
            Attribute::GenericAttackSpeed { .. } => f.write_str("GENERIC_ATTACK_SPEED"),
            Attribute::GenericArmor { .. } => f.write_str("GENERIC_ARMOR"),
            Attribute::GenericArmorToughness { .. } => f.write_str("GENERIC_ARMOR_TOUGHNESS"),
            Attribute::GenericLuck { .. } => f.write_str("GENERIC_LUCK"),
            Attribute::HorseJumpStrength { .. } => f.write_str("HORSE_JUMP_STRENGTH"),
            Attribute::ZombieSpawnReinforcements { .. } => {
                f.write_str("ZOMBIE_SPAWN_REINFORCEMENTS")
            }
        }
    }
}

impl<'mc> Attribute<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Attribute<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/attribute/Attribute");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/attribute/Attribute;",
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
            "GENERIC_MAX_HEALTH" => Ok(Attribute::GenericMaxHealth {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_FOLLOW_RANGE" => Ok(Attribute::GenericFollowRange {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_KNOCKBACK_RESISTANCE" => Ok(Attribute::GenericKnockbackResistance {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_MOVEMENT_SPEED" => Ok(Attribute::GenericMovementSpeed {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_FLYING_SPEED" => Ok(Attribute::GenericFlyingSpeed {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_ATTACK_DAMAGE" => Ok(Attribute::GenericAttackDamage {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_ATTACK_KNOCKBACK" => Ok(Attribute::GenericAttackKnockback {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_ATTACK_SPEED" => Ok(Attribute::GenericAttackSpeed {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_ARMOR" => Ok(Attribute::GenericArmor {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_ARMOR_TOUGHNESS" => Ok(Attribute::GenericArmorToughness {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_LUCK" => Ok(Attribute::GenericLuck {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "HORSE_JUMP_STRENGTH" => Ok(Attribute::HorseJumpStrength {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "ZOMBIE_SPAWN_REINFORCEMENTS" => Ok(Attribute::ZombieSpawnReinforcements {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct AttributeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Attribute<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::GenericMaxHealth { inner } => inner.0.clone(),
            Self::GenericFollowRange { inner } => inner.0.clone(),
            Self::GenericKnockbackResistance { inner } => inner.0.clone(),
            Self::GenericMovementSpeed { inner } => inner.0.clone(),
            Self::GenericFlyingSpeed { inner } => inner.0.clone(),
            Self::GenericAttackDamage { inner } => inner.0.clone(),
            Self::GenericAttackKnockback { inner } => inner.0.clone(),
            Self::GenericAttackSpeed { inner } => inner.0.clone(),
            Self::GenericArmor { inner } => inner.0.clone(),
            Self::GenericArmorToughness { inner } => inner.0.clone(),
            Self::GenericLuck { inner } => inner.0.clone(),
            Self::HorseJumpStrength { inner } => inner.0.clone(),
            Self::ZombieSpawnReinforcements { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::GenericMaxHealth { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericFollowRange { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericKnockbackResistance { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericMovementSpeed { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericFlyingSpeed { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericAttackDamage { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericAttackKnockback { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericAttackSpeed { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericArmor { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericArmorToughness { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericLuck { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HorseJumpStrength { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ZombieSpawnReinforcements { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Attribute<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Attribute from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/attribute/Attribute")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Attribute object, got {}",
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
                "GENERIC_MAX_HEALTH" => Ok(Attribute::GenericMaxHealth {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_FOLLOW_RANGE" => Ok(Attribute::GenericFollowRange {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_KNOCKBACK_RESISTANCE" => Ok(Attribute::GenericKnockbackResistance {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_MOVEMENT_SPEED" => Ok(Attribute::GenericMovementSpeed {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_FLYING_SPEED" => Ok(Attribute::GenericFlyingSpeed {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_ATTACK_DAMAGE" => Ok(Attribute::GenericAttackDamage {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_ATTACK_KNOCKBACK" => Ok(Attribute::GenericAttackKnockback {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_ATTACK_SPEED" => Ok(Attribute::GenericAttackSpeed {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_ARMOR" => Ok(Attribute::GenericArmor {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_ARMOR_TOUGHNESS" => Ok(Attribute::GenericArmorToughness {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_LUCK" => Ok(Attribute::GenericLuck {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "HORSE_JUMP_STRENGTH" => Ok(Attribute::HorseJumpStrength {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "ZOMBIE_SPAWN_REINFORCEMENTS" => Ok(Attribute::ZombieSpawnReinforcements {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for AttributeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AttributeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AttributeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/attribute/Attribute")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AttributeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AttributeStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a mutable instance of an attribute and its associated modifiers and values.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct AttributeInstance<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for AttributeInstance<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AttributeInstance<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AttributeInstance from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/attribute/AttributeInstance")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AttributeInstance object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AttributeInstance<'mc> {
    pub fn attribute(
        &self,
    ) -> Result<crate::attribute::Attribute<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/attribute/Attribute;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttribute", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::attribute::Attribute::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn base_value(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBaseValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Set the base value of this instance.
    pub fn set_base_value(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBaseValue",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_modifier(
        &self,
        arg0: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/attribute/AttributeModifier;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addModifier",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_modifier(
        &self,
        arg0: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/attribute/AttributeModifier;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeModifier",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn modifiers(
        &self,
    ) -> Result<Vec<crate::attribute::AttributeModifier<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getModifiers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::attribute::AttributeModifier::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    #[deprecated]

    pub fn value(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn default_value(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDefaultValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum Operation<'mc> {
    AddNumber { inner: OperationStruct<'mc> },
    AddScalar { inner: OperationStruct<'mc> },
    MultiplyScalar1 { inner: OperationStruct<'mc> },
}
impl<'mc> std::fmt::Display for Operation<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::AddNumber { .. } => f.write_str("ADD_NUMBER"),
            Operation::AddScalar { .. } => f.write_str("ADD_SCALAR"),
            Operation::MultiplyScalar1 { .. } => f.write_str("MULTIPLY_SCALAR_1"),
        }
    }
}

impl<'mc> Operation<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Operation<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/attribute/Operation");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/attribute/Operation;",
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
            "ADD_NUMBER" => Ok(Operation::AddNumber {
                inner: OperationStruct::from_raw(env, obj)?,
            }),
            "ADD_SCALAR" => Ok(Operation::AddScalar {
                inner: OperationStruct::from_raw(env, obj)?,
            }),
            "MULTIPLY_SCALAR_1" => Ok(Operation::MultiplyScalar1 {
                inner: OperationStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct OperationStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Operation<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::AddNumber { inner } => inner.0.clone(),
            Self::AddScalar { inner } => inner.0.clone(),
            Self::MultiplyScalar1 { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::AddNumber { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::AddScalar { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::MultiplyScalar1 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Operation<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Operation from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/attribute/Operation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Operation object, got {}",
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
                "ADD_NUMBER" => Ok(Operation::AddNumber {
                    inner: OperationStruct::from_raw(env, obj)?,
                }),
                "ADD_SCALAR" => Ok(Operation::AddScalar {
                    inner: OperationStruct::from_raw(env, obj)?,
                }),
                "MULTIPLY_SCALAR_1" => Ok(Operation::MultiplyScalar1 {
                    inner: OperationStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for OperationStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for OperationStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate OperationStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/attribute/Operation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a OperationStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> OperationStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Concrete implementation of an attribute modifier.
#[repr(C)]
pub struct AttributeModifier<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum AttributeModifierOperation<'mc> {
    AddNumber {
        inner: AttributeModifierOperationStruct<'mc>,
    },
    AddScalar {
        inner: AttributeModifierOperationStruct<'mc>,
    },
    MultiplyScalar1 {
        inner: AttributeModifierOperationStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for AttributeModifierOperation<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeModifierOperation::AddNumber { .. } => f.write_str("ADD_NUMBER"),
            AttributeModifierOperation::AddScalar { .. } => f.write_str("ADD_SCALAR"),
            AttributeModifierOperation::MultiplyScalar1 { .. } => f.write_str("MULTIPLY_SCALAR_1"),
        }
    }
}

impl<'mc> AttributeModifierOperation<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<AttributeModifierOperation<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/attribute/AttributeModifier$Operation");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/attribute/AttributeModifier$Operation;",
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
            "ADD_NUMBER" => Ok(AttributeModifierOperation::AddNumber {
                inner: AttributeModifierOperationStruct::from_raw(env, obj)?,
            }),
            "ADD_SCALAR" => Ok(AttributeModifierOperation::AddScalar {
                inner: AttributeModifierOperationStruct::from_raw(env, obj)?,
            }),
            "MULTIPLY_SCALAR_1" => Ok(AttributeModifierOperation::MultiplyScalar1 {
                inner: AttributeModifierOperationStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct AttributeModifierOperationStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for AttributeModifierOperation<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::AddNumber { inner } => inner.0.clone(),
            Self::AddScalar { inner } => inner.0.clone(),
            Self::MultiplyScalar1 { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::AddNumber { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::AddScalar { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::MultiplyScalar1 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AttributeModifierOperation<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate AttributeModifierOperation from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/attribute/AttributeModifier$Operation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AttributeModifierOperation object, got {}",
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
                "ADD_NUMBER" => Ok(AttributeModifierOperation::AddNumber {
                    inner: AttributeModifierOperationStruct::from_raw(env, obj)?,
                }),
                "ADD_SCALAR" => Ok(AttributeModifierOperation::AddScalar {
                    inner: AttributeModifierOperationStruct::from_raw(env, obj)?,
                }),
                "MULTIPLY_SCALAR_1" => Ok(AttributeModifierOperation::MultiplyScalar1 {
                    inner: AttributeModifierOperationStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for AttributeModifierOperationStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AttributeModifierOperationStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate AttributeModifierOperationStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/attribute/AttributeModifier$Operation")?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a AttributeModifierOperationStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AttributeModifierOperationStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<Vec<crate::attribute::AttributeModifierOperation<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()[Lorg/bukkit/attribute/AttributeModifier$Operation;");
        let cls = jni.find_class("org/bukkit/attribute/AttributeModifier$Operation");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::attribute::AttributeModifierOperation::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for AttributeModifier<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AttributeModifier<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AttributeModifier from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/attribute/AttributeModifier")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AttributeModifier object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AttributeModifier<'mc> {
    pub fn new_with_uuid(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
        arg1: impl Into<String>,
        arg2: f64,
        arg3: std::option::Option<impl Into<crate::attribute::AttributeModifierOperation<'mc>>>,
        arg4: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>,
    ) -> Result<crate::attribute::AttributeModifier<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg1.into())?,
        ));
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(arg2);
        args.push(val_3);
        if let Some(a) = arg3 {
            sig += "Lorg/bukkit/attribute/AttributeModifier$Operation;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        if let Some(a) = arg4 {
            sig += "Lorg/bukkit/inventory/EquipmentSlot;";
            let val_5 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_5);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/attribute/AttributeModifier");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::attribute::AttributeModifier::from_raw(&jni, res)
    }

    pub fn unique_id(
        &self,
    ) -> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/UUID;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "serialize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn deserialize(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<crate::attribute::AttributeModifier<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)Lorg/bukkit/attribute/AttributeModifier;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/attribute/AttributeModifier");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "deserialize",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::attribute::AttributeModifier::from_raw(&jni, obj)
    }

    pub fn amount(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn operation(
        &self,
    ) -> Result<crate::attribute::AttributeModifierOperation<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/attribute/AttributeModifier$Operation;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOperation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::attribute::AttributeModifierOperation::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn slot(
        &self,
    ) -> Result<Option<crate::inventory::EquipmentSlot<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSlot", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::EquipmentSlot::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> std::string::ToString for AttributeModifier<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling AttributeModifier.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for AttributeModifier<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting AttributeModifier into crate::configuration::serialization::ConfigurationSerializable")
    }
}
pub enum AttributeAttribute<'mc> {
    GenericMaxHealth {
        inner: AttributeAttributeStruct<'mc>,
    },
    GenericFollowRange {
        inner: AttributeAttributeStruct<'mc>,
    },
    GenericKnockbackResistance {
        inner: AttributeAttributeStruct<'mc>,
    },
    GenericMovementSpeed {
        inner: AttributeAttributeStruct<'mc>,
    },
    GenericFlyingSpeed {
        inner: AttributeAttributeStruct<'mc>,
    },
    GenericAttackDamage {
        inner: AttributeAttributeStruct<'mc>,
    },
    GenericAttackKnockback {
        inner: AttributeAttributeStruct<'mc>,
    },
    GenericAttackSpeed {
        inner: AttributeAttributeStruct<'mc>,
    },
    GenericArmor {
        inner: AttributeAttributeStruct<'mc>,
    },
    GenericArmorToughness {
        inner: AttributeAttributeStruct<'mc>,
    },
    GenericLuck {
        inner: AttributeAttributeStruct<'mc>,
    },
    HorseJumpStrength {
        inner: AttributeAttributeStruct<'mc>,
    },
    ZombieSpawnReinforcements {
        inner: AttributeAttributeStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for AttributeAttribute<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeAttribute::GenericMaxHealth { .. } => f.write_str("GENERIC_MAX_HEALTH"),
            AttributeAttribute::GenericFollowRange { .. } => f.write_str("GENERIC_FOLLOW_RANGE"),
            AttributeAttribute::GenericKnockbackResistance { .. } => {
                f.write_str("GENERIC_KNOCKBACK_RESISTANCE")
            }
            AttributeAttribute::GenericMovementSpeed { .. } => {
                f.write_str("GENERIC_MOVEMENT_SPEED")
            }
            AttributeAttribute::GenericFlyingSpeed { .. } => f.write_str("GENERIC_FLYING_SPEED"),
            AttributeAttribute::GenericAttackDamage { .. } => f.write_str("GENERIC_ATTACK_DAMAGE"),
            AttributeAttribute::GenericAttackKnockback { .. } => {
                f.write_str("GENERIC_ATTACK_KNOCKBACK")
            }
            AttributeAttribute::GenericAttackSpeed { .. } => f.write_str("GENERIC_ATTACK_SPEED"),
            AttributeAttribute::GenericArmor { .. } => f.write_str("GENERIC_ARMOR"),
            AttributeAttribute::GenericArmorToughness { .. } => {
                f.write_str("GENERIC_ARMOR_TOUGHNESS")
            }
            AttributeAttribute::GenericLuck { .. } => f.write_str("GENERIC_LUCK"),
            AttributeAttribute::HorseJumpStrength { .. } => f.write_str("HORSE_JUMP_STRENGTH"),
            AttributeAttribute::ZombieSpawnReinforcements { .. } => {
                f.write_str("ZOMBIE_SPAWN_REINFORCEMENTS")
            }
        }
    }
}

impl<'mc> AttributeAttribute<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<AttributeAttribute<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/attribute/Attribute$Attribute");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/attribute/Attribute$Attribute;",
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
            "GENERIC_MAX_HEALTH" => Ok(AttributeAttribute::GenericMaxHealth {
                inner: AttributeAttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_FOLLOW_RANGE" => Ok(AttributeAttribute::GenericFollowRange {
                inner: AttributeAttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_KNOCKBACK_RESISTANCE" => Ok(AttributeAttribute::GenericKnockbackResistance {
                inner: AttributeAttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_MOVEMENT_SPEED" => Ok(AttributeAttribute::GenericMovementSpeed {
                inner: AttributeAttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_FLYING_SPEED" => Ok(AttributeAttribute::GenericFlyingSpeed {
                inner: AttributeAttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_ATTACK_DAMAGE" => Ok(AttributeAttribute::GenericAttackDamage {
                inner: AttributeAttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_ATTACK_KNOCKBACK" => Ok(AttributeAttribute::GenericAttackKnockback {
                inner: AttributeAttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_ATTACK_SPEED" => Ok(AttributeAttribute::GenericAttackSpeed {
                inner: AttributeAttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_ARMOR" => Ok(AttributeAttribute::GenericArmor {
                inner: AttributeAttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_ARMOR_TOUGHNESS" => Ok(AttributeAttribute::GenericArmorToughness {
                inner: AttributeAttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_LUCK" => Ok(AttributeAttribute::GenericLuck {
                inner: AttributeAttributeStruct::from_raw(env, obj)?,
            }),
            "HORSE_JUMP_STRENGTH" => Ok(AttributeAttribute::HorseJumpStrength {
                inner: AttributeAttributeStruct::from_raw(env, obj)?,
            }),
            "ZOMBIE_SPAWN_REINFORCEMENTS" => Ok(AttributeAttribute::ZombieSpawnReinforcements {
                inner: AttributeAttributeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct AttributeAttributeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for AttributeAttribute<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::GenericMaxHealth { inner } => inner.0.clone(),
            Self::GenericFollowRange { inner } => inner.0.clone(),
            Self::GenericKnockbackResistance { inner } => inner.0.clone(),
            Self::GenericMovementSpeed { inner } => inner.0.clone(),
            Self::GenericFlyingSpeed { inner } => inner.0.clone(),
            Self::GenericAttackDamage { inner } => inner.0.clone(),
            Self::GenericAttackKnockback { inner } => inner.0.clone(),
            Self::GenericAttackSpeed { inner } => inner.0.clone(),
            Self::GenericArmor { inner } => inner.0.clone(),
            Self::GenericArmorToughness { inner } => inner.0.clone(),
            Self::GenericLuck { inner } => inner.0.clone(),
            Self::HorseJumpStrength { inner } => inner.0.clone(),
            Self::ZombieSpawnReinforcements { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::GenericMaxHealth { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericFollowRange { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericKnockbackResistance { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericMovementSpeed { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericFlyingSpeed { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericAttackDamage { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericAttackKnockback { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericAttackSpeed { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericArmor { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericArmorToughness { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericLuck { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HorseJumpStrength { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ZombieSpawnReinforcements { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AttributeAttribute<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AttributeAttribute from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/attribute/Attribute$Attribute")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AttributeAttribute object, got {}",
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
                "GENERIC_MAX_HEALTH" => Ok(AttributeAttribute::GenericMaxHealth {
                    inner: AttributeAttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_FOLLOW_RANGE" => Ok(AttributeAttribute::GenericFollowRange {
                    inner: AttributeAttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_KNOCKBACK_RESISTANCE" => {
                    Ok(AttributeAttribute::GenericKnockbackResistance {
                        inner: AttributeAttributeStruct::from_raw(env, obj)?,
                    })
                }
                "GENERIC_MOVEMENT_SPEED" => Ok(AttributeAttribute::GenericMovementSpeed {
                    inner: AttributeAttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_FLYING_SPEED" => Ok(AttributeAttribute::GenericFlyingSpeed {
                    inner: AttributeAttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_ATTACK_DAMAGE" => Ok(AttributeAttribute::GenericAttackDamage {
                    inner: AttributeAttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_ATTACK_KNOCKBACK" => Ok(AttributeAttribute::GenericAttackKnockback {
                    inner: AttributeAttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_ATTACK_SPEED" => Ok(AttributeAttribute::GenericAttackSpeed {
                    inner: AttributeAttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_ARMOR" => Ok(AttributeAttribute::GenericArmor {
                    inner: AttributeAttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_ARMOR_TOUGHNESS" => Ok(AttributeAttribute::GenericArmorToughness {
                    inner: AttributeAttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_LUCK" => Ok(AttributeAttribute::GenericLuck {
                    inner: AttributeAttributeStruct::from_raw(env, obj)?,
                }),
                "HORSE_JUMP_STRENGTH" => Ok(AttributeAttribute::HorseJumpStrength {
                    inner: AttributeAttributeStruct::from_raw(env, obj)?,
                }),
                "ZOMBIE_SPAWN_REINFORCEMENTS" => {
                    Ok(AttributeAttribute::ZombieSpawnReinforcements {
                        inner: AttributeAttributeStruct::from_raw(env, obj)?,
                    })
                }
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for AttributeAttributeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AttributeAttributeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate AttributeAttributeStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/attribute/Attribute$Attribute")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AttributeAttributeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AttributeAttributeStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
