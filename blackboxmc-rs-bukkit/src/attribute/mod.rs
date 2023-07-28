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
