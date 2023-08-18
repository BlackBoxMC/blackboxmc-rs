#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIInstantiatableEnum;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents an object which may contain attributes.
///
/// This is a representation of an abstract class.
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

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

pub struct AttributableClass;
impl blackboxmc_general::JNIProvidesClassName for AttributableClass {
    fn class_name(&self) -> &str {
        "org/bukkit/attribute/Attributable"
    }
}

#[derive(PartialEq, Eq)]
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
impl<'mc> std::fmt::Display for Attribute<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
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

impl<'mc> JNIInstantiatableEnum<'mc> for Attribute<'mc> {
    type Enum = AttributeEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
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
}

impl<'mc> Attribute<'mc> {
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
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
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

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

pub struct AttributeClass;
impl blackboxmc_general::JNIProvidesClassName for AttributeClass {
    fn class_name(&self) -> &str {
        "org/bukkit/attribute/Attribute"
    }
}

/// Represents a mutable instance of an attribute and its associated modifiers and values.
///
/// This is a representation of an abstract class.
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

    pub fn attribute(
        &self,
    ) -> Result<crate::attribute::Attribute<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/attribute/Attribute;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttribute", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
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
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
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

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

pub struct AttributeInstanceClass;
impl blackboxmc_general::JNIProvidesClassName for AttributeInstanceClass {
    fn class_name(&self) -> &str {
        "org/bukkit/attribute/AttributeInstance"
    }
}

#[derive(PartialEq, Eq)]
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
impl<'mc> std::fmt::Display for Operation<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
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

impl<'mc> JNIInstantiatableEnum<'mc> for Operation<'mc> {
    type Enum = OperationEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
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
}

impl<'mc> Operation<'mc> {
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
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
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

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

pub struct OperationClass;
impl blackboxmc_general::JNIProvidesClassName for OperationClass {
    fn class_name(&self) -> &str {
        "org/bukkit/attribute/Operation"
    }
}

/// Concrete implementation of an attribute modifier.
pub struct AttributeModifier<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
#[derive(PartialEq, Eq)]
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
impl<'mc> std::fmt::Display for AttributeModifierOperation<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
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

impl<'mc> JNIInstantiatableEnum<'mc> for AttributeModifierOperation<'mc> {
    type Enum = AttributeModifierOperationEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
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
}

impl<'mc> AttributeModifierOperation<'mc> {
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
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
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

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

pub struct AttributeModifierOperationClass;
impl blackboxmc_general::JNIProvidesClassName for AttributeModifierOperationClass {
    fn class_name(&self) -> &str {
        "org/bukkit/attribute/AttributeModifier$Operation"
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
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
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
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        Ok(Some(crate::inventory::EquipmentSlot::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::inventory::EquipmentSlot::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )?))
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
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
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

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
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

pub struct AttributeModifierClass;
impl blackboxmc_general::JNIProvidesClassName for AttributeModifierClass {
    fn class_name(&self) -> &str {
        "org/bukkit/attribute/AttributeModifier"
    }
}

#[derive(PartialEq, Eq)]
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
impl<'mc> std::fmt::Display for AttributeAttribute<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
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

impl<'mc> JNIInstantiatableEnum<'mc> for AttributeAttribute<'mc> {
    type Enum = AttributeAttributeEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
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
}

impl<'mc> AttributeAttribute<'mc> {
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
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
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

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

pub struct AttributeAttributeClass;
impl blackboxmc_general::JNIProvidesClassName for AttributeAttributeClass {
    fn class_name(&self) -> &str {
        "org/bukkit/attribute/Attribute$Attribute"
    }
}
