#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct AttributeModifier<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

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
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        key: impl Into<crate::NamespacedKey<'mc>>,
        amount: f64,
        operation: impl Into<crate::attribute::AttributeModifierOperation<'mc>>,
        slot: std::option::Option<impl Into<crate::inventory::EquipmentSlotGroup<'mc>>>,
    ) -> Result<crate::attribute::AttributeModifier<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/NamespacedKey;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(key.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(amount);
        args.push(val_2);
        sig += "Lorg/bukkit/attribute/AttributeModifier/Operation;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(operation.into().jni_object().clone())
        });
        args.push(val_3);
        if let Some(a) = slot {
            sig += "Lorg/bukkit/inventory/EquipmentSlotGroup;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/attribute/AttributeModifier");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::attribute::AttributeModifier::from_raw(&jni, res)
    }
    #[deprecated]
    /// Get the unique ID for this modifier.
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
    /// Get the name of this modifier.
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
    /// Get the amount by which this modifier will apply its {@link Operation}.
    pub fn amount(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Get the operation this modifier will apply.
    pub fn operation(
        &self,
    ) -> Result<crate::attribute::AttributeModifierOperation<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/attribute/AttributeModifier/Operation;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOperation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::attribute::AttributeModifierOperation::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// Get the {@link EquipmentSlot} this AttributeModifier is active on, or null if this modifier is applicable for any slot.
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
    /// Get the {@link EquipmentSlot} this AttributeModifier is active on,
    /// or null if this modifier is applicable for any slot.
    pub fn slot_group(
        &self,
    ) -> Result<crate::inventory::EquipmentSlotGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlotGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSlotGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::EquipmentSlotGroup::from_raw(&self.jni_ref(), unsafe {
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

    pub fn equals(
        &self,
        other: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(other);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
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

    pub fn deserialize(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        val_args: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<crate::attribute::AttributeModifier<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)Lorg/bukkit/attribute/AttributeModifier;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_args.into().jni_object().clone())
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
impl<'mc> Into<crate::Keyed<'mc>> for AttributeModifier<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting AttributeModifier into crate::Keyed")
    }
}
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
impl<'mc> std::ops::Deref for AttributeModifierOperation<'mc> {
    type Target = AttributeModifierOperationStruct<'mc>;
    fn deref(&self) -> &<AttributeModifierOperation<'mc> as std::ops::Deref>::Target {
        match self {
            AttributeModifierOperation::AddNumber { inner } => inner,
            AttributeModifierOperation::AddScalar { inner } => inner,
            AttributeModifierOperation::MultiplyScalar1 { inner } => inner,
        }
    }
}

impl<'mc> AttributeModifierOperation<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<AttributeModifierOperation<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/attribute/AttributeModifier/Operation");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/attribute/AttributeModifier/Operation;",
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
            env.validate_name(&obj, "org/bukkit/attribute/AttributeModifier/Operation")?;
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
            env.validate_name(&obj, "org/bukkit/attribute/AttributeModifier/Operation")?;
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
    ) -> Result<crate::attribute::AttributeModifierOperation<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/attribute/AttributeModifier/Operation;");
        let cls = jni.find_class("org/bukkit/attribute/AttributeModifier/Operation");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::attribute::AttributeModifierOperation::from_raw(&jni, obj)
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
    GenericFallDamageMultiplier { inner: AttributeStruct<'mc> },
    GenericLuck { inner: AttributeStruct<'mc> },
    GenericMaxAbsorption { inner: AttributeStruct<'mc> },
    GenericSafeFallDistance { inner: AttributeStruct<'mc> },
    GenericScale { inner: AttributeStruct<'mc> },
    GenericStepHeight { inner: AttributeStruct<'mc> },
    GenericGravity { inner: AttributeStruct<'mc> },
    GenericJumpStrength { inner: AttributeStruct<'mc> },
    GenericBurningTime { inner: AttributeStruct<'mc> },
    GenericExplosionKnockbackResistance { inner: AttributeStruct<'mc> },
    GenericMovementEfficiency { inner: AttributeStruct<'mc> },
    GenericOxygenBonus { inner: AttributeStruct<'mc> },
    GenericWaterMovementEfficiency { inner: AttributeStruct<'mc> },
    PlayerBlockInteractionRange { inner: AttributeStruct<'mc> },
    PlayerEntityInteractionRange { inner: AttributeStruct<'mc> },
    PlayerBlockBreakSpeed { inner: AttributeStruct<'mc> },
    PlayerMiningEfficiency { inner: AttributeStruct<'mc> },
    PlayerSneakingSpeed { inner: AttributeStruct<'mc> },
    PlayerSubmergedMiningSpeed { inner: AttributeStruct<'mc> },
    PlayerSweepingDamageRatio { inner: AttributeStruct<'mc> },
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
            Attribute::GenericFallDamageMultiplier { .. } => {
                f.write_str("GENERIC_FALL_DAMAGE_MULTIPLIER")
            }
            Attribute::GenericLuck { .. } => f.write_str("GENERIC_LUCK"),
            Attribute::GenericMaxAbsorption { .. } => f.write_str("GENERIC_MAX_ABSORPTION"),
            Attribute::GenericSafeFallDistance { .. } => f.write_str("GENERIC_SAFE_FALL_DISTANCE"),
            Attribute::GenericScale { .. } => f.write_str("GENERIC_SCALE"),
            Attribute::GenericStepHeight { .. } => f.write_str("GENERIC_STEP_HEIGHT"),
            Attribute::GenericGravity { .. } => f.write_str("GENERIC_GRAVITY"),
            Attribute::GenericJumpStrength { .. } => f.write_str("GENERIC_JUMP_STRENGTH"),
            Attribute::GenericBurningTime { .. } => f.write_str("GENERIC_BURNING_TIME"),
            Attribute::GenericExplosionKnockbackResistance { .. } => {
                f.write_str("GENERIC_EXPLOSION_KNOCKBACK_RESISTANCE")
            }
            Attribute::GenericMovementEfficiency { .. } => {
                f.write_str("GENERIC_MOVEMENT_EFFICIENCY")
            }
            Attribute::GenericOxygenBonus { .. } => f.write_str("GENERIC_OXYGEN_BONUS"),
            Attribute::GenericWaterMovementEfficiency { .. } => {
                f.write_str("GENERIC_WATER_MOVEMENT_EFFICIENCY")
            }
            Attribute::PlayerBlockInteractionRange { .. } => {
                f.write_str("PLAYER_BLOCK_INTERACTION_RANGE")
            }
            Attribute::PlayerEntityInteractionRange { .. } => {
                f.write_str("PLAYER_ENTITY_INTERACTION_RANGE")
            }
            Attribute::PlayerBlockBreakSpeed { .. } => f.write_str("PLAYER_BLOCK_BREAK_SPEED"),
            Attribute::PlayerMiningEfficiency { .. } => f.write_str("PLAYER_MINING_EFFICIENCY"),
            Attribute::PlayerSneakingSpeed { .. } => f.write_str("PLAYER_SNEAKING_SPEED"),
            Attribute::PlayerSubmergedMiningSpeed { .. } => {
                f.write_str("PLAYER_SUBMERGED_MINING_SPEED")
            }
            Attribute::PlayerSweepingDamageRatio { .. } => {
                f.write_str("PLAYER_SWEEPING_DAMAGE_RATIO")
            }
            Attribute::ZombieSpawnReinforcements { .. } => {
                f.write_str("ZOMBIE_SPAWN_REINFORCEMENTS")
            }
        }
    }
}
impl<'mc> std::ops::Deref for Attribute<'mc> {
    type Target = AttributeStruct<'mc>;
    fn deref(&self) -> &<Attribute<'mc> as std::ops::Deref>::Target {
        match self {
            Attribute::GenericMaxHealth { inner } => inner,
            Attribute::GenericFollowRange { inner } => inner,
            Attribute::GenericKnockbackResistance { inner } => inner,
            Attribute::GenericMovementSpeed { inner } => inner,
            Attribute::GenericFlyingSpeed { inner } => inner,
            Attribute::GenericAttackDamage { inner } => inner,
            Attribute::GenericAttackKnockback { inner } => inner,
            Attribute::GenericAttackSpeed { inner } => inner,
            Attribute::GenericArmor { inner } => inner,
            Attribute::GenericArmorToughness { inner } => inner,
            Attribute::GenericFallDamageMultiplier { inner } => inner,
            Attribute::GenericLuck { inner } => inner,
            Attribute::GenericMaxAbsorption { inner } => inner,
            Attribute::GenericSafeFallDistance { inner } => inner,
            Attribute::GenericScale { inner } => inner,
            Attribute::GenericStepHeight { inner } => inner,
            Attribute::GenericGravity { inner } => inner,
            Attribute::GenericJumpStrength { inner } => inner,
            Attribute::GenericBurningTime { inner } => inner,
            Attribute::GenericExplosionKnockbackResistance { inner } => inner,
            Attribute::GenericMovementEfficiency { inner } => inner,
            Attribute::GenericOxygenBonus { inner } => inner,
            Attribute::GenericWaterMovementEfficiency { inner } => inner,
            Attribute::PlayerBlockInteractionRange { inner } => inner,
            Attribute::PlayerEntityInteractionRange { inner } => inner,
            Attribute::PlayerBlockBreakSpeed { inner } => inner,
            Attribute::PlayerMiningEfficiency { inner } => inner,
            Attribute::PlayerSneakingSpeed { inner } => inner,
            Attribute::PlayerSubmergedMiningSpeed { inner } => inner,
            Attribute::PlayerSweepingDamageRatio { inner } => inner,
            Attribute::ZombieSpawnReinforcements { inner } => inner,
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
            "GENERIC_FALL_DAMAGE_MULTIPLIER" => Ok(Attribute::GenericFallDamageMultiplier {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_LUCK" => Ok(Attribute::GenericLuck {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_MAX_ABSORPTION" => Ok(Attribute::GenericMaxAbsorption {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_SAFE_FALL_DISTANCE" => Ok(Attribute::GenericSafeFallDistance {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_SCALE" => Ok(Attribute::GenericScale {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_STEP_HEIGHT" => Ok(Attribute::GenericStepHeight {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_GRAVITY" => Ok(Attribute::GenericGravity {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_JUMP_STRENGTH" => Ok(Attribute::GenericJumpStrength {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_BURNING_TIME" => Ok(Attribute::GenericBurningTime {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_EXPLOSION_KNOCKBACK_RESISTANCE" => {
                Ok(Attribute::GenericExplosionKnockbackResistance {
                    inner: AttributeStruct::from_raw(env, obj)?,
                })
            }
            "GENERIC_MOVEMENT_EFFICIENCY" => Ok(Attribute::GenericMovementEfficiency {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_OXYGEN_BONUS" => Ok(Attribute::GenericOxygenBonus {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "GENERIC_WATER_MOVEMENT_EFFICIENCY" => Ok(Attribute::GenericWaterMovementEfficiency {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "PLAYER_BLOCK_INTERACTION_RANGE" => Ok(Attribute::PlayerBlockInteractionRange {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "PLAYER_ENTITY_INTERACTION_RANGE" => Ok(Attribute::PlayerEntityInteractionRange {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "PLAYER_BLOCK_BREAK_SPEED" => Ok(Attribute::PlayerBlockBreakSpeed {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "PLAYER_MINING_EFFICIENCY" => Ok(Attribute::PlayerMiningEfficiency {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "PLAYER_SNEAKING_SPEED" => Ok(Attribute::PlayerSneakingSpeed {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "PLAYER_SUBMERGED_MINING_SPEED" => Ok(Attribute::PlayerSubmergedMiningSpeed {
                inner: AttributeStruct::from_raw(env, obj)?,
            }),
            "PLAYER_SWEEPING_DAMAGE_RATIO" => Ok(Attribute::PlayerSweepingDamageRatio {
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
            Self::GenericFallDamageMultiplier { inner } => inner.0.clone(),
            Self::GenericLuck { inner } => inner.0.clone(),
            Self::GenericMaxAbsorption { inner } => inner.0.clone(),
            Self::GenericSafeFallDistance { inner } => inner.0.clone(),
            Self::GenericScale { inner } => inner.0.clone(),
            Self::GenericStepHeight { inner } => inner.0.clone(),
            Self::GenericGravity { inner } => inner.0.clone(),
            Self::GenericJumpStrength { inner } => inner.0.clone(),
            Self::GenericBurningTime { inner } => inner.0.clone(),
            Self::GenericExplosionKnockbackResistance { inner } => inner.0.clone(),
            Self::GenericMovementEfficiency { inner } => inner.0.clone(),
            Self::GenericOxygenBonus { inner } => inner.0.clone(),
            Self::GenericWaterMovementEfficiency { inner } => inner.0.clone(),
            Self::PlayerBlockInteractionRange { inner } => inner.0.clone(),
            Self::PlayerEntityInteractionRange { inner } => inner.0.clone(),
            Self::PlayerBlockBreakSpeed { inner } => inner.0.clone(),
            Self::PlayerMiningEfficiency { inner } => inner.0.clone(),
            Self::PlayerSneakingSpeed { inner } => inner.0.clone(),
            Self::PlayerSubmergedMiningSpeed { inner } => inner.0.clone(),
            Self::PlayerSweepingDamageRatio { inner } => inner.0.clone(),
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
            Self::GenericFallDamageMultiplier { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericLuck { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericMaxAbsorption { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericSafeFallDistance { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericScale { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericStepHeight { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericGravity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericJumpStrength { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericBurningTime { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericExplosionKnockbackResistance { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericMovementEfficiency { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericOxygenBonus { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GenericWaterMovementEfficiency { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PlayerBlockInteractionRange { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PlayerEntityInteractionRange { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PlayerBlockBreakSpeed { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PlayerMiningEfficiency { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PlayerSneakingSpeed { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PlayerSubmergedMiningSpeed { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PlayerSweepingDamageRatio { inner } => unsafe {
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
                "GENERIC_FALL_DAMAGE_MULTIPLIER" => Ok(Attribute::GenericFallDamageMultiplier {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_LUCK" => Ok(Attribute::GenericLuck {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_MAX_ABSORPTION" => Ok(Attribute::GenericMaxAbsorption {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_SAFE_FALL_DISTANCE" => Ok(Attribute::GenericSafeFallDistance {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_SCALE" => Ok(Attribute::GenericScale {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_STEP_HEIGHT" => Ok(Attribute::GenericStepHeight {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_GRAVITY" => Ok(Attribute::GenericGravity {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_JUMP_STRENGTH" => Ok(Attribute::GenericJumpStrength {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_BURNING_TIME" => Ok(Attribute::GenericBurningTime {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_EXPLOSION_KNOCKBACK_RESISTANCE" => {
                    Ok(Attribute::GenericExplosionKnockbackResistance {
                        inner: AttributeStruct::from_raw(env, obj)?,
                    })
                }
                "GENERIC_MOVEMENT_EFFICIENCY" => Ok(Attribute::GenericMovementEfficiency {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_OXYGEN_BONUS" => Ok(Attribute::GenericOxygenBonus {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "GENERIC_WATER_MOVEMENT_EFFICIENCY" => {
                    Ok(Attribute::GenericWaterMovementEfficiency {
                        inner: AttributeStruct::from_raw(env, obj)?,
                    })
                }
                "PLAYER_BLOCK_INTERACTION_RANGE" => Ok(Attribute::PlayerBlockInteractionRange {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "PLAYER_ENTITY_INTERACTION_RANGE" => Ok(Attribute::PlayerEntityInteractionRange {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "PLAYER_BLOCK_BREAK_SPEED" => Ok(Attribute::PlayerBlockBreakSpeed {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "PLAYER_MINING_EFFICIENCY" => Ok(Attribute::PlayerMiningEfficiency {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "PLAYER_SNEAKING_SPEED" => Ok(Attribute::PlayerSneakingSpeed {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "PLAYER_SUBMERGED_MINING_SPEED" => Ok(Attribute::PlayerSubmergedMiningSpeed {
                    inner: AttributeStruct::from_raw(env, obj)?,
                }),
                "PLAYER_SWEEPING_DAMAGE_RATIO" => Ok(Attribute::PlayerSweepingDamageRatio {
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
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::attribute::Attribute<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/attribute/Attribute;");
        let cls = jni.find_class("org/bukkit/attribute/Attribute");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::attribute::Attribute::from_raw(&jni, obj)
    }

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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
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
    /// The attribute pertaining to this instance.
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
    /// Base value of this instance before modifiers are applied.
    pub fn base_value(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBaseValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Set the base value of this instance.
    pub fn set_base_value(&self, value: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBaseValue",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get all modifiers present on this instance.
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
    /// Add a modifier to this instance.
    pub fn add_modifier(
        &self,
        modifier: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/attribute/AttributeModifier;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(modifier.into().jni_object().clone())
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
    /// Remove a modifier from this instance.
    pub fn remove_modifier(
        &self,
        modifier: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/attribute/AttributeModifier;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(modifier.into().jni_object().clone())
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
    /// Get the value of this instance after all associated modifiers have been
    /// applied.
    pub fn value(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the default value of the Attribute attached to this instance.
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
    /// Gets the specified attribute instance from the object. This instance will
    /// be backed directly to the object and any changes will be visible at once.
    pub fn get_attribute(
        &self,
        attribute: impl Into<crate::attribute::Attribute<'mc>>,
    ) -> Result<Option<crate::attribute::AttributeInstance<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/attribute/Attribute;)Lorg/bukkit/attribute/AttributeInstance;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(attribute.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttribute",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::attribute::AttributeInstance::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
