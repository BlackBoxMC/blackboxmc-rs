#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// An instantiatable struct that implements Attributable. Needed for returning it from Java.
pub struct Attributable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Attributable<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Attributable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Attributable")?;
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
    pub fn get_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
    ) -> Result<crate::attribute::AttributeInstance<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttribute",
            "(Lorg/bukkit/attribute/Attribute;)Lorg/bukkit/attribute/AttributeInstance;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::attribute::AttributeInstance::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for Attributable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub enum AttributeEnum {
    GenericMaxHealth,
    GenericFollowRange,
    GenericKnockbackResistance,
    GenericMovementSpeed,
    GenericFlyingSpeed,
    GenericAttackDamage,
    GenericAttackKnockback,
    GenericAttackSpeed,
    GenericArmor,
    GenericArmorToughness,
    GenericLuck,
    HorseJumpStrength,
    ZombieSpawnReinforcements,
}
impl std::fmt::Display for AttributeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AttributeEnum::GenericMaxHealth => f.write_str("GENERIC_MAX_HEALTH"),
            AttributeEnum::GenericFollowRange => f.write_str("GENERIC_FOLLOW_RANGE"),
            AttributeEnum::GenericKnockbackResistance => {
                f.write_str("GENERIC_KNOCKBACK_RESISTANCE")
            }
            AttributeEnum::GenericMovementSpeed => f.write_str("GENERIC_MOVEMENT_SPEED"),
            AttributeEnum::GenericFlyingSpeed => f.write_str("GENERIC_FLYING_SPEED"),
            AttributeEnum::GenericAttackDamage => f.write_str("GENERIC_ATTACK_DAMAGE"),
            AttributeEnum::GenericAttackKnockback => f.write_str("GENERIC_ATTACK_KNOCKBACK"),
            AttributeEnum::GenericAttackSpeed => f.write_str("GENERIC_ATTACK_SPEED"),
            AttributeEnum::GenericArmor => f.write_str("GENERIC_ARMOR"),
            AttributeEnum::GenericArmorToughness => f.write_str("GENERIC_ARMOR_TOUGHNESS"),
            AttributeEnum::GenericLuck => f.write_str("GENERIC_LUCK"),
            AttributeEnum::HorseJumpStrength => f.write_str("HORSE_JUMP_STRENGTH"),
            AttributeEnum::ZombieSpawnReinforcements => f.write_str("ZOMBIE_SPAWN_REINFORCEMENTS"),
        }
    }
}
pub struct Attribute<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub AttributeEnum,
);
impl<'mc> std::ops::Deref for Attribute<'mc> {
    type Target = AttributeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for Attribute<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Attribute<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: AttributeEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Attribute from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Attribute")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Attribute object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const GENERIC_MAX_HEALTH: AttributeEnum = AttributeEnum::GenericMaxHealth;
    pub const GENERIC_FOLLOW_RANGE: AttributeEnum = AttributeEnum::GenericFollowRange;
    pub const GENERIC_KNOCKBACK_RESISTANCE: AttributeEnum =
        AttributeEnum::GenericKnockbackResistance;
    pub const GENERIC_MOVEMENT_SPEED: AttributeEnum = AttributeEnum::GenericMovementSpeed;
    pub const GENERIC_FLYING_SPEED: AttributeEnum = AttributeEnum::GenericFlyingSpeed;
    pub const GENERIC_ATTACK_DAMAGE: AttributeEnum = AttributeEnum::GenericAttackDamage;
    pub const GENERIC_ATTACK_KNOCKBACK: AttributeEnum = AttributeEnum::GenericAttackKnockback;
    pub const GENERIC_ATTACK_SPEED: AttributeEnum = AttributeEnum::GenericAttackSpeed;
    pub const GENERIC_ARMOR: AttributeEnum = AttributeEnum::GenericArmor;
    pub const GENERIC_ARMOR_TOUGHNESS: AttributeEnum = AttributeEnum::GenericArmorToughness;
    pub const GENERIC_LUCK: AttributeEnum = AttributeEnum::GenericLuck;
    pub const HORSE_JUMP_STRENGTH: AttributeEnum = AttributeEnum::HorseJumpStrength;
    pub const ZOMBIE_SPAWN_REINFORCEMENTS: AttributeEnum = AttributeEnum::ZombieSpawnReinforcements;
    pub fn from_string(str: String) -> std::option::Option<AttributeEnum> {
        match str.as_str() {
            "GENERIC_MAX_HEALTH" => Some(AttributeEnum::GenericMaxHealth),
            "GENERIC_FOLLOW_RANGE" => Some(AttributeEnum::GenericFollowRange),
            "GENERIC_KNOCKBACK_RESISTANCE" => Some(AttributeEnum::GenericKnockbackResistance),
            "GENERIC_MOVEMENT_SPEED" => Some(AttributeEnum::GenericMovementSpeed),
            "GENERIC_FLYING_SPEED" => Some(AttributeEnum::GenericFlyingSpeed),
            "GENERIC_ATTACK_DAMAGE" => Some(AttributeEnum::GenericAttackDamage),
            "GENERIC_ATTACK_KNOCKBACK" => Some(AttributeEnum::GenericAttackKnockback),
            "GENERIC_ATTACK_SPEED" => Some(AttributeEnum::GenericAttackSpeed),
            "GENERIC_ARMOR" => Some(AttributeEnum::GenericArmor),
            "GENERIC_ARMOR_TOUGHNESS" => Some(AttributeEnum::GenericArmorToughness),
            "GENERIC_LUCK" => Some(AttributeEnum::GenericLuck),
            "HORSE_JUMP_STRENGTH" => Some(AttributeEnum::HorseJumpStrength),
            "ZOMBIE_SPAWN_REINFORCEMENTS" => Some(AttributeEnum::ZombieSpawnReinforcements),
            _ => None,
        }
    }
}
/// An instantiatable struct that implements AttributeInstance. Needed for returning it from Java.
pub struct AttributeInstance<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> AttributeInstance<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AttributeInstance from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "AttributeInstance")?;
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
    pub fn attribute(
        &mut self,
    ) -> Result<crate::attribute::Attribute<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttribute",
            "()Lorg/bukkit/attribute/Attribute;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::attribute::Attribute::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::attribute::Attribute::from_string(variant_str).unwrap(),
        )
    }
    pub fn base_value(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBaseValue", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn set_base_value(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBaseValue",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn add_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addModifier",
            "(Lorg/bukkit/attribute/AttributeModifier;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeModifier",
            "(Lorg/bukkit/attribute/AttributeModifier;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn modifiers(
        &mut self,
    ) -> Result<blackboxmc_java::JavaCollection<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getModifiers",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn value(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn default_value(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDefaultValue", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for AttributeInstance<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct AttributeModifier<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub struct AttributeModifierOperation<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for AttributeModifierOperation<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> AttributeModifierOperation<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate AttributeModifierOperation from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "AttributeModifierOperation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AttributeModifierOperation object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn describe_constable(
        &mut self,
    ) -> Result<blackboxmc_java::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            "()Ljava/util/Optional;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for AttributeModifier<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> AttributeModifier<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AttributeModifier from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "AttributeModifier")?;
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
    pub fn serialize(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "serialize", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn deserialize(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc blackboxmc_java::JavaMap<'mc>>,
    ) -> Result<crate::attribute::AttributeModifier<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/attribute/AttributeModifier")?;
        let res = jni.call_static_method(
            cls,
            "deserialize",
            "(Ljava/util/Map;)Lorg/bukkit/attribute/AttributeModifier;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        let mut obj = res.l()?;
        crate::attribute::AttributeModifier::from_raw(&jni, obj)
    }
    pub fn amount(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAmount", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn operation(
        &mut self,
    ) -> Result<crate::attribute::AttributeModifierOperation<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOperation",
            "()Lorg/bukkit/attribute/AttributeModifier$Operation;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::attribute::AttributeModifierOperation::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn slot(
        &mut self,
    ) -> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlot",
            "()Lorg/bukkit/inventory/EquipmentSlot;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::inventory::EquipmentSlot::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::inventory::EquipmentSlot::from_string(variant_str).unwrap(),
        )
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for AttributeModifier<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            self.1,
        )
        .unwrap()
    }
}
