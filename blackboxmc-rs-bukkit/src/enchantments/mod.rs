#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub enum EnchantmentTarget<'mc> {
    All { inner: EnchantmentTargetStruct<'mc> },
    Armor { inner: EnchantmentTargetStruct<'mc> },
    ArmorFeet { inner: EnchantmentTargetStruct<'mc> },
    ArmorLegs { inner: EnchantmentTargetStruct<'mc> },
    ArmorTorso { inner: EnchantmentTargetStruct<'mc> },
    ArmorHead { inner: EnchantmentTargetStruct<'mc> },
    Weapon { inner: EnchantmentTargetStruct<'mc> },
    Tool { inner: EnchantmentTargetStruct<'mc> },
    Bow { inner: EnchantmentTargetStruct<'mc> },
    FishingRod { inner: EnchantmentTargetStruct<'mc> },
    Breakable { inner: EnchantmentTargetStruct<'mc> },
    Wearable { inner: EnchantmentTargetStruct<'mc> },
    Trident { inner: EnchantmentTargetStruct<'mc> },
    Crossbow { inner: EnchantmentTargetStruct<'mc> },
    Vanishable { inner: EnchantmentTargetStruct<'mc> },
}
impl<'mc> std::fmt::Display for EnchantmentTarget<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnchantmentTarget::All { .. } => f.write_str("ALL"),
            EnchantmentTarget::Armor { .. } => f.write_str("ARMOR"),
            EnchantmentTarget::ArmorFeet { .. } => f.write_str("ARMOR_FEET"),
            EnchantmentTarget::ArmorLegs { .. } => f.write_str("ARMOR_LEGS"),
            EnchantmentTarget::ArmorTorso { .. } => f.write_str("ARMOR_TORSO"),
            EnchantmentTarget::ArmorHead { .. } => f.write_str("ARMOR_HEAD"),
            EnchantmentTarget::Weapon { .. } => f.write_str("WEAPON"),
            EnchantmentTarget::Tool { .. } => f.write_str("TOOL"),
            EnchantmentTarget::Bow { .. } => f.write_str("BOW"),
            EnchantmentTarget::FishingRod { .. } => f.write_str("FISHING_ROD"),
            EnchantmentTarget::Breakable { .. } => f.write_str("BREAKABLE"),
            EnchantmentTarget::Wearable { .. } => f.write_str("WEARABLE"),
            EnchantmentTarget::Trident { .. } => f.write_str("TRIDENT"),
            EnchantmentTarget::Crossbow { .. } => f.write_str("CROSSBOW"),
            EnchantmentTarget::Vanishable { .. } => f.write_str("VANISHABLE"),
        }
    }
}
impl<'mc> std::ops::Deref for EnchantmentTarget<'mc> {
    type Target = EnchantmentTargetStruct<'mc>;
    fn deref(&self) -> &<EnchantmentTarget<'mc> as std::ops::Deref>::Target {
        match self {
            EnchantmentTarget::All { inner } => inner,
            EnchantmentTarget::Armor { inner } => inner,
            EnchantmentTarget::ArmorFeet { inner } => inner,
            EnchantmentTarget::ArmorLegs { inner } => inner,
            EnchantmentTarget::ArmorTorso { inner } => inner,
            EnchantmentTarget::ArmorHead { inner } => inner,
            EnchantmentTarget::Weapon { inner } => inner,
            EnchantmentTarget::Tool { inner } => inner,
            EnchantmentTarget::Bow { inner } => inner,
            EnchantmentTarget::FishingRod { inner } => inner,
            EnchantmentTarget::Breakable { inner } => inner,
            EnchantmentTarget::Wearable { inner } => inner,
            EnchantmentTarget::Trident { inner } => inner,
            EnchantmentTarget::Crossbow { inner } => inner,
            EnchantmentTarget::Vanishable { inner } => inner,
        }
    }
}

impl<'mc> EnchantmentTarget<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EnchantmentTarget<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/enchantments/EnchantmentTarget");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/enchantments/EnchantmentTarget;",
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
            "ALL" => Ok(EnchantmentTarget::All {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "ARMOR" => Ok(EnchantmentTarget::Armor {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "ARMOR_FEET" => Ok(EnchantmentTarget::ArmorFeet {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "ARMOR_LEGS" => Ok(EnchantmentTarget::ArmorLegs {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "ARMOR_TORSO" => Ok(EnchantmentTarget::ArmorTorso {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "ARMOR_HEAD" => Ok(EnchantmentTarget::ArmorHead {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "WEAPON" => Ok(EnchantmentTarget::Weapon {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "TOOL" => Ok(EnchantmentTarget::Tool {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "BOW" => Ok(EnchantmentTarget::Bow {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "FISHING_ROD" => Ok(EnchantmentTarget::FishingRod {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "BREAKABLE" => Ok(EnchantmentTarget::Breakable {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "WEARABLE" => Ok(EnchantmentTarget::Wearable {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "TRIDENT" => Ok(EnchantmentTarget::Trident {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "CROSSBOW" => Ok(EnchantmentTarget::Crossbow {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "VANISHABLE" => Ok(EnchantmentTarget::Vanishable {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EnchantmentTargetStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EnchantmentTarget<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::All { inner } => inner.0.clone(),
            Self::Armor { inner } => inner.0.clone(),
            Self::ArmorFeet { inner } => inner.0.clone(),
            Self::ArmorLegs { inner } => inner.0.clone(),
            Self::ArmorTorso { inner } => inner.0.clone(),
            Self::ArmorHead { inner } => inner.0.clone(),
            Self::Weapon { inner } => inner.0.clone(),
            Self::Tool { inner } => inner.0.clone(),
            Self::Bow { inner } => inner.0.clone(),
            Self::FishingRod { inner } => inner.0.clone(),
            Self::Breakable { inner } => inner.0.clone(),
            Self::Wearable { inner } => inner.0.clone(),
            Self::Trident { inner } => inner.0.clone(),
            Self::Crossbow { inner } => inner.0.clone(),
            Self::Vanishable { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::All { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Armor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ArmorFeet { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ArmorLegs { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ArmorTorso { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ArmorHead { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Weapon { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Tool { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Bow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FishingRod { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Breakable { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Wearable { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Trident { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Crossbow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Vanishable { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentTarget<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnchantmentTarget from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/enchantments/EnchantmentTarget")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantmentTarget object, got {}",
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
                "ALL" => Ok(EnchantmentTarget::All {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "ARMOR" => Ok(EnchantmentTarget::Armor {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "ARMOR_FEET" => Ok(EnchantmentTarget::ArmorFeet {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "ARMOR_LEGS" => Ok(EnchantmentTarget::ArmorLegs {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "ARMOR_TORSO" => Ok(EnchantmentTarget::ArmorTorso {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "ARMOR_HEAD" => Ok(EnchantmentTarget::ArmorHead {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "WEAPON" => Ok(EnchantmentTarget::Weapon {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "TOOL" => Ok(EnchantmentTarget::Tool {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "BOW" => Ok(EnchantmentTarget::Bow {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "FISHING_ROD" => Ok(EnchantmentTarget::FishingRod {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "BREAKABLE" => Ok(EnchantmentTarget::Breakable {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "WEARABLE" => Ok(EnchantmentTarget::Wearable {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "TRIDENT" => Ok(EnchantmentTarget::Trident {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "CROSSBOW" => Ok(EnchantmentTarget::Crossbow {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "VANISHABLE" => Ok(EnchantmentTarget::Vanishable {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EnchantmentTargetStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentTargetStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EnchantmentTargetStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/enchantments/EnchantmentTarget")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantmentTargetStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EnchantmentTargetStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::enchantments::EnchantmentTarget<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/enchantments/EnchantmentTarget;");
        let cls = jni.find_class("org/bukkit/enchantments/EnchantmentTarget");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::enchantments::EnchantmentTarget::from_raw(&jni, obj)
    }
    /// Check whether this target includes the specified item.
    pub fn includes(
        &self,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "includes",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum Enchantment<'mc> {
    Protection { inner: EnchantmentStruct<'mc> },
    FireProtection { inner: EnchantmentStruct<'mc> },
    FeatherFalling { inner: EnchantmentStruct<'mc> },
    BlastProtection { inner: EnchantmentStruct<'mc> },
    ProjectileProtection { inner: EnchantmentStruct<'mc> },
    Respiration { inner: EnchantmentStruct<'mc> },
    AquaAffinity { inner: EnchantmentStruct<'mc> },
    Thorns { inner: EnchantmentStruct<'mc> },
    DepthStrider { inner: EnchantmentStruct<'mc> },
    FrostWalker { inner: EnchantmentStruct<'mc> },
    BindingCurse { inner: EnchantmentStruct<'mc> },
    Sharpness { inner: EnchantmentStruct<'mc> },
    Smite { inner: EnchantmentStruct<'mc> },
    BaneOfArthropods { inner: EnchantmentStruct<'mc> },
    Knockback { inner: EnchantmentStruct<'mc> },
    FireAspect { inner: EnchantmentStruct<'mc> },
    Looting { inner: EnchantmentStruct<'mc> },
    SweepingEdge { inner: EnchantmentStruct<'mc> },
    Efficiency { inner: EnchantmentStruct<'mc> },
    SilkTouch { inner: EnchantmentStruct<'mc> },
    Unbreaking { inner: EnchantmentStruct<'mc> },
    Fortune { inner: EnchantmentStruct<'mc> },
    Power { inner: EnchantmentStruct<'mc> },
    Punch { inner: EnchantmentStruct<'mc> },
    Flame { inner: EnchantmentStruct<'mc> },
    Infinity { inner: EnchantmentStruct<'mc> },
    LuckOfTheSea { inner: EnchantmentStruct<'mc> },
    Lure { inner: EnchantmentStruct<'mc> },
    Loyalty { inner: EnchantmentStruct<'mc> },
    Impaling { inner: EnchantmentStruct<'mc> },
    Riptide { inner: EnchantmentStruct<'mc> },
    Channeling { inner: EnchantmentStruct<'mc> },
    Multishot { inner: EnchantmentStruct<'mc> },
    QuickCharge { inner: EnchantmentStruct<'mc> },
    Piercing { inner: EnchantmentStruct<'mc> },
    Density { inner: EnchantmentStruct<'mc> },
    Breach { inner: EnchantmentStruct<'mc> },
    WindBurst { inner: EnchantmentStruct<'mc> },
    Mending { inner: EnchantmentStruct<'mc> },
    VanishingCurse { inner: EnchantmentStruct<'mc> },
    SoulSpeed { inner: EnchantmentStruct<'mc> },
    SwiftSneak { inner: EnchantmentStruct<'mc> },
}
impl<'mc> std::fmt::Display for Enchantment<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Enchantment::Protection { .. } => f.write_str("PROTECTION"),
            Enchantment::FireProtection { .. } => f.write_str("FIRE_PROTECTION"),
            Enchantment::FeatherFalling { .. } => f.write_str("FEATHER_FALLING"),
            Enchantment::BlastProtection { .. } => f.write_str("BLAST_PROTECTION"),
            Enchantment::ProjectileProtection { .. } => f.write_str("PROJECTILE_PROTECTION"),
            Enchantment::Respiration { .. } => f.write_str("RESPIRATION"),
            Enchantment::AquaAffinity { .. } => f.write_str("AQUA_AFFINITY"),
            Enchantment::Thorns { .. } => f.write_str("THORNS"),
            Enchantment::DepthStrider { .. } => f.write_str("DEPTH_STRIDER"),
            Enchantment::FrostWalker { .. } => f.write_str("FROST_WALKER"),
            Enchantment::BindingCurse { .. } => f.write_str("BINDING_CURSE"),
            Enchantment::Sharpness { .. } => f.write_str("SHARPNESS"),
            Enchantment::Smite { .. } => f.write_str("SMITE"),
            Enchantment::BaneOfArthropods { .. } => f.write_str("BANE_OF_ARTHROPODS"),
            Enchantment::Knockback { .. } => f.write_str("KNOCKBACK"),
            Enchantment::FireAspect { .. } => f.write_str("FIRE_ASPECT"),
            Enchantment::Looting { .. } => f.write_str("LOOTING"),
            Enchantment::SweepingEdge { .. } => f.write_str("SWEEPING_EDGE"),
            Enchantment::Efficiency { .. } => f.write_str("EFFICIENCY"),
            Enchantment::SilkTouch { .. } => f.write_str("SILK_TOUCH"),
            Enchantment::Unbreaking { .. } => f.write_str("UNBREAKING"),
            Enchantment::Fortune { .. } => f.write_str("FORTUNE"),
            Enchantment::Power { .. } => f.write_str("POWER"),
            Enchantment::Punch { .. } => f.write_str("PUNCH"),
            Enchantment::Flame { .. } => f.write_str("FLAME"),
            Enchantment::Infinity { .. } => f.write_str("INFINITY"),
            Enchantment::LuckOfTheSea { .. } => f.write_str("LUCK_OF_THE_SEA"),
            Enchantment::Lure { .. } => f.write_str("LURE"),
            Enchantment::Loyalty { .. } => f.write_str("LOYALTY"),
            Enchantment::Impaling { .. } => f.write_str("IMPALING"),
            Enchantment::Riptide { .. } => f.write_str("RIPTIDE"),
            Enchantment::Channeling { .. } => f.write_str("CHANNELING"),
            Enchantment::Multishot { .. } => f.write_str("MULTISHOT"),
            Enchantment::QuickCharge { .. } => f.write_str("QUICK_CHARGE"),
            Enchantment::Piercing { .. } => f.write_str("PIERCING"),
            Enchantment::Density { .. } => f.write_str("DENSITY"),
            Enchantment::Breach { .. } => f.write_str("BREACH"),
            Enchantment::WindBurst { .. } => f.write_str("WIND_BURST"),
            Enchantment::Mending { .. } => f.write_str("MENDING"),
            Enchantment::VanishingCurse { .. } => f.write_str("VANISHING_CURSE"),
            Enchantment::SoulSpeed { .. } => f.write_str("SOUL_SPEED"),
            Enchantment::SwiftSneak { .. } => f.write_str("SWIFT_SNEAK"),
        }
    }
}
impl<'mc> std::ops::Deref for Enchantment<'mc> {
    type Target = EnchantmentStruct<'mc>;
    fn deref(&self) -> &<Enchantment<'mc> as std::ops::Deref>::Target {
        match self {
            Enchantment::Protection { inner } => inner,
            Enchantment::FireProtection { inner } => inner,
            Enchantment::FeatherFalling { inner } => inner,
            Enchantment::BlastProtection { inner } => inner,
            Enchantment::ProjectileProtection { inner } => inner,
            Enchantment::Respiration { inner } => inner,
            Enchantment::AquaAffinity { inner } => inner,
            Enchantment::Thorns { inner } => inner,
            Enchantment::DepthStrider { inner } => inner,
            Enchantment::FrostWalker { inner } => inner,
            Enchantment::BindingCurse { inner } => inner,
            Enchantment::Sharpness { inner } => inner,
            Enchantment::Smite { inner } => inner,
            Enchantment::BaneOfArthropods { inner } => inner,
            Enchantment::Knockback { inner } => inner,
            Enchantment::FireAspect { inner } => inner,
            Enchantment::Looting { inner } => inner,
            Enchantment::SweepingEdge { inner } => inner,
            Enchantment::Efficiency { inner } => inner,
            Enchantment::SilkTouch { inner } => inner,
            Enchantment::Unbreaking { inner } => inner,
            Enchantment::Fortune { inner } => inner,
            Enchantment::Power { inner } => inner,
            Enchantment::Punch { inner } => inner,
            Enchantment::Flame { inner } => inner,
            Enchantment::Infinity { inner } => inner,
            Enchantment::LuckOfTheSea { inner } => inner,
            Enchantment::Lure { inner } => inner,
            Enchantment::Loyalty { inner } => inner,
            Enchantment::Impaling { inner } => inner,
            Enchantment::Riptide { inner } => inner,
            Enchantment::Channeling { inner } => inner,
            Enchantment::Multishot { inner } => inner,
            Enchantment::QuickCharge { inner } => inner,
            Enchantment::Piercing { inner } => inner,
            Enchantment::Density { inner } => inner,
            Enchantment::Breach { inner } => inner,
            Enchantment::WindBurst { inner } => inner,
            Enchantment::Mending { inner } => inner,
            Enchantment::VanishingCurse { inner } => inner,
            Enchantment::SoulSpeed { inner } => inner,
            Enchantment::SwiftSneak { inner } => inner,
        }
    }
}

impl<'mc> Enchantment<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Enchantment<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/enchantments/Enchantment");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/enchantments/Enchantment;",
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
            "PROTECTION" => Ok(Enchantment::Protection {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "FIRE_PROTECTION" => Ok(Enchantment::FireProtection {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "FEATHER_FALLING" => Ok(Enchantment::FeatherFalling {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "BLAST_PROTECTION" => Ok(Enchantment::BlastProtection {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "PROJECTILE_PROTECTION" => Ok(Enchantment::ProjectileProtection {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "RESPIRATION" => Ok(Enchantment::Respiration {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "AQUA_AFFINITY" => Ok(Enchantment::AquaAffinity {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "THORNS" => Ok(Enchantment::Thorns {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "DEPTH_STRIDER" => Ok(Enchantment::DepthStrider {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "FROST_WALKER" => Ok(Enchantment::FrostWalker {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "BINDING_CURSE" => Ok(Enchantment::BindingCurse {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "SHARPNESS" => Ok(Enchantment::Sharpness {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "SMITE" => Ok(Enchantment::Smite {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "BANE_OF_ARTHROPODS" => Ok(Enchantment::BaneOfArthropods {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "KNOCKBACK" => Ok(Enchantment::Knockback {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "FIRE_ASPECT" => Ok(Enchantment::FireAspect {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "LOOTING" => Ok(Enchantment::Looting {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "SWEEPING_EDGE" => Ok(Enchantment::SweepingEdge {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "EFFICIENCY" => Ok(Enchantment::Efficiency {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "SILK_TOUCH" => Ok(Enchantment::SilkTouch {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "UNBREAKING" => Ok(Enchantment::Unbreaking {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "FORTUNE" => Ok(Enchantment::Fortune {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "POWER" => Ok(Enchantment::Power {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "PUNCH" => Ok(Enchantment::Punch {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "FLAME" => Ok(Enchantment::Flame {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "INFINITY" => Ok(Enchantment::Infinity {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "LUCK_OF_THE_SEA" => Ok(Enchantment::LuckOfTheSea {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "LURE" => Ok(Enchantment::Lure {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "LOYALTY" => Ok(Enchantment::Loyalty {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "IMPALING" => Ok(Enchantment::Impaling {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "RIPTIDE" => Ok(Enchantment::Riptide {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "CHANNELING" => Ok(Enchantment::Channeling {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "MULTISHOT" => Ok(Enchantment::Multishot {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "QUICK_CHARGE" => Ok(Enchantment::QuickCharge {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "PIERCING" => Ok(Enchantment::Piercing {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "DENSITY" => Ok(Enchantment::Density {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "BREACH" => Ok(Enchantment::Breach {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "WIND_BURST" => Ok(Enchantment::WindBurst {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "MENDING" => Ok(Enchantment::Mending {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "VANISHING_CURSE" => Ok(Enchantment::VanishingCurse {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "SOUL_SPEED" => Ok(Enchantment::SoulSpeed {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),
            "SWIFT_SNEAK" => Ok(Enchantment::SwiftSneak {
                inner: EnchantmentStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EnchantmentStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Enchantment<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Protection { inner } => inner.0.clone(),
            Self::FireProtection { inner } => inner.0.clone(),
            Self::FeatherFalling { inner } => inner.0.clone(),
            Self::BlastProtection { inner } => inner.0.clone(),
            Self::ProjectileProtection { inner } => inner.0.clone(),
            Self::Respiration { inner } => inner.0.clone(),
            Self::AquaAffinity { inner } => inner.0.clone(),
            Self::Thorns { inner } => inner.0.clone(),
            Self::DepthStrider { inner } => inner.0.clone(),
            Self::FrostWalker { inner } => inner.0.clone(),
            Self::BindingCurse { inner } => inner.0.clone(),
            Self::Sharpness { inner } => inner.0.clone(),
            Self::Smite { inner } => inner.0.clone(),
            Self::BaneOfArthropods { inner } => inner.0.clone(),
            Self::Knockback { inner } => inner.0.clone(),
            Self::FireAspect { inner } => inner.0.clone(),
            Self::Looting { inner } => inner.0.clone(),
            Self::SweepingEdge { inner } => inner.0.clone(),
            Self::Efficiency { inner } => inner.0.clone(),
            Self::SilkTouch { inner } => inner.0.clone(),
            Self::Unbreaking { inner } => inner.0.clone(),
            Self::Fortune { inner } => inner.0.clone(),
            Self::Power { inner } => inner.0.clone(),
            Self::Punch { inner } => inner.0.clone(),
            Self::Flame { inner } => inner.0.clone(),
            Self::Infinity { inner } => inner.0.clone(),
            Self::LuckOfTheSea { inner } => inner.0.clone(),
            Self::Lure { inner } => inner.0.clone(),
            Self::Loyalty { inner } => inner.0.clone(),
            Self::Impaling { inner } => inner.0.clone(),
            Self::Riptide { inner } => inner.0.clone(),
            Self::Channeling { inner } => inner.0.clone(),
            Self::Multishot { inner } => inner.0.clone(),
            Self::QuickCharge { inner } => inner.0.clone(),
            Self::Piercing { inner } => inner.0.clone(),
            Self::Density { inner } => inner.0.clone(),
            Self::Breach { inner } => inner.0.clone(),
            Self::WindBurst { inner } => inner.0.clone(),
            Self::Mending { inner } => inner.0.clone(),
            Self::VanishingCurse { inner } => inner.0.clone(),
            Self::SoulSpeed { inner } => inner.0.clone(),
            Self::SwiftSneak { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Protection { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FireProtection { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FeatherFalling { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BlastProtection { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ProjectileProtection { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Respiration { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::AquaAffinity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Thorns { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::DepthStrider { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FrostWalker { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BindingCurse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Sharpness { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Smite { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::BaneOfArthropods { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Knockback { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FireAspect { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Looting { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SweepingEdge { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Efficiency { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SilkTouch { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Unbreaking { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Fortune { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Power { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Punch { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Flame { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Infinity { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::LuckOfTheSea { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Lure { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Loyalty { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Impaling { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Riptide { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Channeling { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Multishot { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::QuickCharge { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Piercing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Density { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Breach { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WindBurst { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Mending { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::VanishingCurse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SoulSpeed { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SwiftSneak { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Enchantment<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Enchantment from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/enchantments/Enchantment")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Enchantment object, got {}",
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
                "PROTECTION" => Ok(Enchantment::Protection {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "FIRE_PROTECTION" => Ok(Enchantment::FireProtection {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "FEATHER_FALLING" => Ok(Enchantment::FeatherFalling {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "BLAST_PROTECTION" => Ok(Enchantment::BlastProtection {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "PROJECTILE_PROTECTION" => Ok(Enchantment::ProjectileProtection {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "RESPIRATION" => Ok(Enchantment::Respiration {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "AQUA_AFFINITY" => Ok(Enchantment::AquaAffinity {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "THORNS" => Ok(Enchantment::Thorns {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "DEPTH_STRIDER" => Ok(Enchantment::DepthStrider {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "FROST_WALKER" => Ok(Enchantment::FrostWalker {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "BINDING_CURSE" => Ok(Enchantment::BindingCurse {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "SHARPNESS" => Ok(Enchantment::Sharpness {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "SMITE" => Ok(Enchantment::Smite {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "BANE_OF_ARTHROPODS" => Ok(Enchantment::BaneOfArthropods {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "KNOCKBACK" => Ok(Enchantment::Knockback {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "FIRE_ASPECT" => Ok(Enchantment::FireAspect {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "LOOTING" => Ok(Enchantment::Looting {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "SWEEPING_EDGE" => Ok(Enchantment::SweepingEdge {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "EFFICIENCY" => Ok(Enchantment::Efficiency {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "SILK_TOUCH" => Ok(Enchantment::SilkTouch {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "UNBREAKING" => Ok(Enchantment::Unbreaking {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "FORTUNE" => Ok(Enchantment::Fortune {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "POWER" => Ok(Enchantment::Power {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "PUNCH" => Ok(Enchantment::Punch {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "FLAME" => Ok(Enchantment::Flame {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "INFINITY" => Ok(Enchantment::Infinity {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "LUCK_OF_THE_SEA" => Ok(Enchantment::LuckOfTheSea {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "LURE" => Ok(Enchantment::Lure {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "LOYALTY" => Ok(Enchantment::Loyalty {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "IMPALING" => Ok(Enchantment::Impaling {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "RIPTIDE" => Ok(Enchantment::Riptide {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "CHANNELING" => Ok(Enchantment::Channeling {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "MULTISHOT" => Ok(Enchantment::Multishot {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "QUICK_CHARGE" => Ok(Enchantment::QuickCharge {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "PIERCING" => Ok(Enchantment::Piercing {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "DENSITY" => Ok(Enchantment::Density {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "BREACH" => Ok(Enchantment::Breach {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "WIND_BURST" => Ok(Enchantment::WindBurst {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "MENDING" => Ok(Enchantment::Mending {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "VANISHING_CURSE" => Ok(Enchantment::VanishingCurse {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "SOUL_SPEED" => Ok(Enchantment::SoulSpeed {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                "SWIFT_SNEAK" => Ok(Enchantment::SwiftSneak {
                    inner: EnchantmentStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EnchantmentStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnchantmentStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/enchantments/Enchantment")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantmentStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EnchantmentStruct<'mc> {
    #[deprecated]
    /// Gets the Enchantment at the specified key
    pub fn get_by_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        key: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<Option<crate::enchantments::Enchantment<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/enchantments/Enchantment;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(key.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/enchantments/Enchantment");
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
        Ok(Some(crate::enchantments::Enchantment::from_raw(&jni, obj)?))
    }
    #[deprecated]
    /// Gets the Enchantment at the specified name
    pub fn get_by_name(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        name: impl Into<String>,
    ) -> Result<Option<crate::enchantments::Enchantment<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/enchantments/Enchantment;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(name.into())?,
        ));
        let cls = jni.find_class("org/bukkit/enchantments/Enchantment");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        let obj = res.l()?;
        Ok(Some(crate::enchantments::Enchantment::from_raw(&jni, obj)?))
    }
    #[deprecated]
    /// Gets an array of all the registered {@link Enchantment}s
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/enchantments/Enchantment;");
        let cls = jni.find_class("org/bukkit/enchantments/Enchantment");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::enchantments::Enchantment::from_raw(&jni, obj)
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
    /// Get the translation key, suitable for use in a translation component.
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
pub struct EnchantmentWrapper<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EnchantmentWrapper<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentWrapper<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnchantmentWrapper from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/enchantments/EnchantmentWrapper")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantmentWrapper object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EnchantmentWrapper<'mc> {
    /// Gets the enchantment bound to this wrapper
    pub fn enchantment(
        &self,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/enchantments/Enchantment;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchantment", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::enchantments::Enchantment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.enchantments.Enchantment ( ['getEnchantment'])
    /// Gets the Enchantment at the specified key
    pub fn get_by_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        key: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<Option<crate::enchantments::Enchantment<'mc>>, Box<dyn std::error::Error>> {
        crate::enchantments::EnchantmentStruct::get_by_key(jni, key)
    }
    /// Gets the Enchantment at the specified name
    pub fn get_by_name(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        name: impl Into<String>,
    ) -> Result<Option<crate::enchantments::Enchantment<'mc>>, Box<dyn std::error::Error>> {
        crate::enchantments::EnchantmentStruct::get_by_name(jni, name)
    }
    #[deprecated]
    /// Gets an array of all the registered {@link Enchantment}s
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/enchantments/Enchantment;");
        let cls = jni.find_class("org/bukkit/enchantments/Enchantment");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::enchantments::Enchantment::from_raw(&jni, obj)
    }
    /// Return the namespaced identifier for this object.
    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::enchantments::Enchantment::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::enchantments::Enchantment = temp_clone.into();
        real.key()
    }
    /// Get the translation key, suitable for use in a translation component.
    pub fn translation_key(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::enchantments::Enchantment::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::enchantments::Enchantment = temp_clone.into();
        real.translation_key()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::enchantments::Enchantment<'mc>> for EnchantmentWrapper<'mc> {
    fn into(self) -> crate::enchantments::Enchantment<'mc> {
        crate::enchantments::Enchantment::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EnchantmentWrapper into crate::enchantments::Enchantment")
    }
}
#[repr(C)]
pub struct EnchantmentOffer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EnchantmentOffer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentOffer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnchantmentOffer from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/enchantments/EnchantmentOffer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantmentOffer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EnchantmentOffer<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        enchantment: impl Into<crate::enchantments::Enchantment<'mc>>,
        enchantment_level: i32,
        cost: i32,
    ) -> Result<crate::enchantments::EnchantmentOffer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;II)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(enchantment.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(enchantment_level);
        let val_3 = jni::objects::JValueGen::Int(cost);
        let cls = jni.find_class("org/bukkit/enchantments/EnchantmentOffer");
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
        crate::enchantments::EnchantmentOffer::from_raw(&jni, res)
    }
    /// Get the type of the enchantment.
    pub fn enchantment(
        &self,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/enchantments/Enchantment;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchantment", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::enchantments::Enchantment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the type of the enchantment.
    pub fn set_enchantment(
        &self,
        enchantment: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(enchantment.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEnchantment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the level of the enchantment.
    pub fn enchantment_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantmentLevel",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the level of the enchantment.
    pub fn set_enchantment_level(
        &self,
        enchantment_level: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(enchantment_level);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEnchantmentLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the cost (minimum level) which is displayed as a number on the right
    /// hand side of the enchantment offer.
    pub fn cost(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCost", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the cost (minimum level) which is displayed as a number on the right
    /// hand side of the enchantment offer.
    pub fn set_cost(&self, cost: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(cost);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCost",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
