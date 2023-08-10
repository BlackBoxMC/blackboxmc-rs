#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents an object which may contain attributes.
///
/// This is a representation of an abstract class.
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
    //

    pub fn get_attribute(
        &mut self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
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
        match self {
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/attribute/Attribute")?;
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

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Attribute<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/attribute/Attribute");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/attribute/Attribute;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        Attribute::from_raw(
            &jni,
            raw_obj,
            Attribute::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
/// Represents a mutable instance of an attribute and its associated modifiers and values.
///
/// This is a representation of an abstract class.
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
    //

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
        let variant = self
            .jni_ref()
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::attribute::Attribute::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::attribute::Attribute::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn base_value(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBaseValue", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    /// Set the base value of this instance.
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
    //

    pub fn add_modifier(
        &mut self,
        arg0: impl Into<crate::attribute::AttributeModifier<'mc>>,
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
    //

    pub fn remove_modifier(
        &mut self,
        arg0: impl Into<crate::attribute::AttributeModifier<'mc>>,
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
    //

    pub fn modifiers(
        &mut self,
    ) -> Result<Vec<crate::attribute::AttributeModifier<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getModifiers",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::attribute::AttributeModifier::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    //

    pub fn value(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn default_value(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDefaultValue", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
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
pub enum OperationEnum {
    AddNumber,
    AddScalar,
    MultiplyScalar1,
}
impl std::fmt::Display for OperationEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationEnum::AddNumber => f.write_str("ADD_NUMBER"),
            OperationEnum::AddScalar => f.write_str("ADD_SCALAR"),
            OperationEnum::MultiplyScalar1 => f.write_str("MULTIPLY_SCALAR_1"),
        }
    }
}
pub struct Operation<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub OperationEnum,
);
impl<'mc> std::ops::Deref for Operation<'mc> {
    type Target = OperationEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for Operation<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Operation<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: OperationEnum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const ADD_NUMBER: OperationEnum = OperationEnum::AddNumber;
    pub const ADD_SCALAR: OperationEnum = OperationEnum::AddScalar;
    pub const MULTIPLY_SCALAR_1: OperationEnum = OperationEnum::MultiplyScalar1;
    pub fn from_string(str: String) -> std::option::Option<OperationEnum> {
        match str.as_str() {
            "ADD_NUMBER" => Some(OperationEnum::AddNumber),
            "ADD_SCALAR" => Some(OperationEnum::AddScalar),
            "MULTIPLY_SCALAR_1" => Some(OperationEnum::MultiplyScalar1),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Operation<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/attribute/Operation");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/attribute/Operation;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        Operation::from_raw(
            &jni,
            raw_obj,
            Operation::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
/// Concrete implementation of an attribute modifier.
pub struct AttributeModifier<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum AttributeModifierOperationEnum {
    AddNumber,
    AddScalar,
    MultiplyScalar1,
}
impl std::fmt::Display for AttributeModifierOperationEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeModifierOperationEnum::AddNumber => f.write_str("ADD_NUMBER"),
            AttributeModifierOperationEnum::AddScalar => f.write_str("ADD_SCALAR"),
            AttributeModifierOperationEnum::MultiplyScalar1 => f.write_str("MULTIPLY_SCALAR_1"),
        }
    }
}
pub struct AttributeModifierOperation<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub AttributeModifierOperationEnum,
);
impl<'mc> std::ops::Deref for AttributeModifierOperation<'mc> {
    type Target = AttributeModifierOperationEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for AttributeModifierOperation<'mc> {
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
        e: AttributeModifierOperationEnum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const ADD_NUMBER: AttributeModifierOperationEnum =
        AttributeModifierOperationEnum::AddNumber;
    pub const ADD_SCALAR: AttributeModifierOperationEnum =
        AttributeModifierOperationEnum::AddScalar;
    pub const MULTIPLY_SCALAR_1: AttributeModifierOperationEnum =
        AttributeModifierOperationEnum::MultiplyScalar1;
    pub fn from_string(str: String) -> std::option::Option<AttributeModifierOperationEnum> {
        match str.as_str() {
            "ADD_NUMBER" => Some(AttributeModifierOperationEnum::AddNumber),
            "ADD_SCALAR" => Some(AttributeModifierOperationEnum::AddScalar),
            "MULTIPLY_SCALAR_1" => Some(AttributeModifierOperationEnum::MultiplyScalar1),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<AttributeModifierOperation<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/attribute/AttributeModifier$Operation");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/attribute/AttributeModifier$Operation;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        AttributeModifierOperation::from_raw(
            &jni,
            raw_obj,
            AttributeModifierOperation::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    //
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
    pub fn new_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u128,
        arg1: impl Into<String>,
        arg2: std::option::Option<f64>,
        arg3: std::option::Option<impl Into<crate::attribute::AttributeModifierOperation<'mc>>>,
    ) -> Result<crate::attribute::AttributeModifier<'mc>, Box<dyn std::error::Error>> {
        let upper = (arg0 >> 64) as u64 as i64;
        let lower = arg0 as u64 as i64;
        let val_1 = jni::objects::JValueGen::Object(jni.new_object(
            "java/util/UUID",
            "(JJ)V",
            &[upper.into(), lower.into()],
        )?);
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.into())?);
        let val_3 = jni::objects::JValueGen::Double(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_4 = unsafe {
            jni::objects::JObject::from_raw(
                arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let cls = jni.find_class("org/bukkit/attribute/AttributeModifier");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls,
"(Ljava/util/UUID;Ljava/lang/String;DLorg/bukkit/attribute/AttributeModifier$Operation;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
        let res = jni.translate_error_no_gen(res)?;
        crate::attribute::AttributeModifier::from_raw(&jni, res)
    }
    pub fn new_with_uuid(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u128,
        arg1: impl Into<String>,
        arg2: f64,
        arg3: impl Into<crate::attribute::AttributeModifierOperation<'mc>>,
        arg4: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>,
    ) -> Result<crate::attribute::AttributeModifier<'mc>, Box<dyn std::error::Error>> {
        let upper = (arg0 >> 64) as u64 as i64;
        let lower = arg0 as u64 as i64;
        let val_1 = jni::objects::JValueGen::Object(jni.new_object(
            "java/util/UUID",
            "(JJ)V",
            &[upper.into(), lower.into()],
        )?);
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.into())?);
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        let val_5 = unsafe {
            jni::objects::JObject::from_raw(
                arg4.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let cls = jni.find_class("org/bukkit/attribute/AttributeModifier");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls,
"(Ljava/util/UUID;Ljava/lang/String;DLorg/bukkit/attribute/AttributeModifier$Operation;Lorg/bukkit/inventory/EquipmentSlot;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
        let res = jni.translate_error_no_gen(res)?;
        crate::attribute::AttributeModifier::from_raw(&jni, res)
    }
    //

    //

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
    //

    pub fn deserialize(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<blackboxmc_java::JavaMap<'mc>>,
    ) -> Result<crate::attribute::AttributeModifier<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = jni.find_class("org/bukkit/attribute/AttributeModifier");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "deserialize",
            "(Ljava/util/Map;)Lorg/bukkit/attribute/AttributeModifier;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::attribute::AttributeModifier::from_raw(&jni, obj)
    }
    //

    pub fn amount(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAmount", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

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
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::attribute::AttributeModifierOperation::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::attribute::AttributeModifierOperation::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

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
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::inventory::EquipmentSlot::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::inventory::EquipmentSlot::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
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
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

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
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting AttributeModifier into crate::configuration::serialization::ConfigurationSerializable")
    }
}
pub enum AttributeAttributeEnum {
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
impl std::fmt::Display for AttributeAttributeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeAttributeEnum::GenericMaxHealth => f.write_str("GENERIC_MAX_HEALTH"),
            AttributeAttributeEnum::GenericFollowRange => f.write_str("GENERIC_FOLLOW_RANGE"),
            AttributeAttributeEnum::GenericKnockbackResistance => {
                f.write_str("GENERIC_KNOCKBACK_RESISTANCE")
            }
            AttributeAttributeEnum::GenericMovementSpeed => f.write_str("GENERIC_MOVEMENT_SPEED"),
            AttributeAttributeEnum::GenericFlyingSpeed => f.write_str("GENERIC_FLYING_SPEED"),
            AttributeAttributeEnum::GenericAttackDamage => f.write_str("GENERIC_ATTACK_DAMAGE"),
            AttributeAttributeEnum::GenericAttackKnockback => {
                f.write_str("GENERIC_ATTACK_KNOCKBACK")
            }
            AttributeAttributeEnum::GenericAttackSpeed => f.write_str("GENERIC_ATTACK_SPEED"),
            AttributeAttributeEnum::GenericArmor => f.write_str("GENERIC_ARMOR"),
            AttributeAttributeEnum::GenericArmorToughness => f.write_str("GENERIC_ARMOR_TOUGHNESS"),
            AttributeAttributeEnum::GenericLuck => f.write_str("GENERIC_LUCK"),
            AttributeAttributeEnum::HorseJumpStrength => f.write_str("HORSE_JUMP_STRENGTH"),
            AttributeAttributeEnum::ZombieSpawnReinforcements => {
                f.write_str("ZOMBIE_SPAWN_REINFORCEMENTS")
            }
        }
    }
}
pub struct AttributeAttribute<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub AttributeAttributeEnum,
);
impl<'mc> std::ops::Deref for AttributeAttribute<'mc> {
    type Target = AttributeAttributeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for AttributeAttribute<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> AttributeAttribute<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: AttributeAttributeEnum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const GENERIC_MAX_HEALTH: AttributeAttributeEnum = AttributeAttributeEnum::GenericMaxHealth;
    pub const GENERIC_FOLLOW_RANGE: AttributeAttributeEnum =
        AttributeAttributeEnum::GenericFollowRange;
    pub const GENERIC_KNOCKBACK_RESISTANCE: AttributeAttributeEnum =
        AttributeAttributeEnum::GenericKnockbackResistance;
    pub const GENERIC_MOVEMENT_SPEED: AttributeAttributeEnum =
        AttributeAttributeEnum::GenericMovementSpeed;
    pub const GENERIC_FLYING_SPEED: AttributeAttributeEnum =
        AttributeAttributeEnum::GenericFlyingSpeed;
    pub const GENERIC_ATTACK_DAMAGE: AttributeAttributeEnum =
        AttributeAttributeEnum::GenericAttackDamage;
    pub const GENERIC_ATTACK_KNOCKBACK: AttributeAttributeEnum =
        AttributeAttributeEnum::GenericAttackKnockback;
    pub const GENERIC_ATTACK_SPEED: AttributeAttributeEnum =
        AttributeAttributeEnum::GenericAttackSpeed;
    pub const GENERIC_ARMOR: AttributeAttributeEnum = AttributeAttributeEnum::GenericArmor;
    pub const GENERIC_ARMOR_TOUGHNESS: AttributeAttributeEnum =
        AttributeAttributeEnum::GenericArmorToughness;
    pub const GENERIC_LUCK: AttributeAttributeEnum = AttributeAttributeEnum::GenericLuck;
    pub const HORSE_JUMP_STRENGTH: AttributeAttributeEnum =
        AttributeAttributeEnum::HorseJumpStrength;
    pub const ZOMBIE_SPAWN_REINFORCEMENTS: AttributeAttributeEnum =
        AttributeAttributeEnum::ZombieSpawnReinforcements;
    pub fn from_string(str: String) -> std::option::Option<AttributeAttributeEnum> {
        match str.as_str() {
            "GENERIC_MAX_HEALTH" => Some(AttributeAttributeEnum::GenericMaxHealth),
            "GENERIC_FOLLOW_RANGE" => Some(AttributeAttributeEnum::GenericFollowRange),
            "GENERIC_KNOCKBACK_RESISTANCE" => {
                Some(AttributeAttributeEnum::GenericKnockbackResistance)
            }
            "GENERIC_MOVEMENT_SPEED" => Some(AttributeAttributeEnum::GenericMovementSpeed),
            "GENERIC_FLYING_SPEED" => Some(AttributeAttributeEnum::GenericFlyingSpeed),
            "GENERIC_ATTACK_DAMAGE" => Some(AttributeAttributeEnum::GenericAttackDamage),
            "GENERIC_ATTACK_KNOCKBACK" => Some(AttributeAttributeEnum::GenericAttackKnockback),
            "GENERIC_ATTACK_SPEED" => Some(AttributeAttributeEnum::GenericAttackSpeed),
            "GENERIC_ARMOR" => Some(AttributeAttributeEnum::GenericArmor),
            "GENERIC_ARMOR_TOUGHNESS" => Some(AttributeAttributeEnum::GenericArmorToughness),
            "GENERIC_LUCK" => Some(AttributeAttributeEnum::GenericLuck),
            "HORSE_JUMP_STRENGTH" => Some(AttributeAttributeEnum::HorseJumpStrength),
            "ZOMBIE_SPAWN_REINFORCEMENTS" => {
                Some(AttributeAttributeEnum::ZombieSpawnReinforcements)
            }
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<AttributeAttribute<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/attribute/Attribute$Attribute");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/attribute/Attribute$Attribute;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        AttributeAttribute::from_raw(
            &jni,
            raw_obj,
            AttributeAttribute::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
