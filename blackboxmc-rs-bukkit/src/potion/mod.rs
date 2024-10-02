#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub enum PotionType<'mc> {
    Water { inner: PotionTypeStruct<'mc> },
    Mundane { inner: PotionTypeStruct<'mc> },
    Thick { inner: PotionTypeStruct<'mc> },
    Awkward { inner: PotionTypeStruct<'mc> },
    NightVision { inner: PotionTypeStruct<'mc> },
    LongNightVision { inner: PotionTypeStruct<'mc> },
    Invisibility { inner: PotionTypeStruct<'mc> },
    LongInvisibility { inner: PotionTypeStruct<'mc> },
    Leaping { inner: PotionTypeStruct<'mc> },
    LongLeaping { inner: PotionTypeStruct<'mc> },
    StrongLeaping { inner: PotionTypeStruct<'mc> },
    FireResistance { inner: PotionTypeStruct<'mc> },
    LongFireResistance { inner: PotionTypeStruct<'mc> },
    Swiftness { inner: PotionTypeStruct<'mc> },
    LongSwiftness { inner: PotionTypeStruct<'mc> },
    StrongSwiftness { inner: PotionTypeStruct<'mc> },
    Slowness { inner: PotionTypeStruct<'mc> },
    LongSlowness { inner: PotionTypeStruct<'mc> },
    StrongSlowness { inner: PotionTypeStruct<'mc> },
    WaterBreathing { inner: PotionTypeStruct<'mc> },
    LongWaterBreathing { inner: PotionTypeStruct<'mc> },
    Healing { inner: PotionTypeStruct<'mc> },
    StrongHealing { inner: PotionTypeStruct<'mc> },
    Harming { inner: PotionTypeStruct<'mc> },
    StrongHarming { inner: PotionTypeStruct<'mc> },
    Poison { inner: PotionTypeStruct<'mc> },
    LongPoison { inner: PotionTypeStruct<'mc> },
    StrongPoison { inner: PotionTypeStruct<'mc> },
    Regeneration { inner: PotionTypeStruct<'mc> },
    LongRegeneration { inner: PotionTypeStruct<'mc> },
    StrongRegeneration { inner: PotionTypeStruct<'mc> },
    Strength { inner: PotionTypeStruct<'mc> },
    LongStrength { inner: PotionTypeStruct<'mc> },
    StrongStrength { inner: PotionTypeStruct<'mc> },
    Weakness { inner: PotionTypeStruct<'mc> },
    LongWeakness { inner: PotionTypeStruct<'mc> },
    Luck { inner: PotionTypeStruct<'mc> },
    TurtleMaster { inner: PotionTypeStruct<'mc> },
    LongTurtleMaster { inner: PotionTypeStruct<'mc> },
    StrongTurtleMaster { inner: PotionTypeStruct<'mc> },
    SlowFalling { inner: PotionTypeStruct<'mc> },
    LongSlowFalling { inner: PotionTypeStruct<'mc> },
    WindCharged { inner: PotionTypeStruct<'mc> },
    Weaving { inner: PotionTypeStruct<'mc> },
    Oozing { inner: PotionTypeStruct<'mc> },
    Infested { inner: PotionTypeStruct<'mc> },
}
impl<'mc> std::fmt::Display for PotionType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PotionType::Water { .. } => f.write_str("WATER"),
            PotionType::Mundane { .. } => f.write_str("MUNDANE"),
            PotionType::Thick { .. } => f.write_str("THICK"),
            PotionType::Awkward { .. } => f.write_str("AWKWARD"),
            PotionType::NightVision { .. } => f.write_str("NIGHT_VISION"),
            PotionType::LongNightVision { .. } => f.write_str("LONG_NIGHT_VISION"),
            PotionType::Invisibility { .. } => f.write_str("INVISIBILITY"),
            PotionType::LongInvisibility { .. } => f.write_str("LONG_INVISIBILITY"),
            PotionType::Leaping { .. } => f.write_str("LEAPING"),
            PotionType::LongLeaping { .. } => f.write_str("LONG_LEAPING"),
            PotionType::StrongLeaping { .. } => f.write_str("STRONG_LEAPING"),
            PotionType::FireResistance { .. } => f.write_str("FIRE_RESISTANCE"),
            PotionType::LongFireResistance { .. } => f.write_str("LONG_FIRE_RESISTANCE"),
            PotionType::Swiftness { .. } => f.write_str("SWIFTNESS"),
            PotionType::LongSwiftness { .. } => f.write_str("LONG_SWIFTNESS"),
            PotionType::StrongSwiftness { .. } => f.write_str("STRONG_SWIFTNESS"),
            PotionType::Slowness { .. } => f.write_str("SLOWNESS"),
            PotionType::LongSlowness { .. } => f.write_str("LONG_SLOWNESS"),
            PotionType::StrongSlowness { .. } => f.write_str("STRONG_SLOWNESS"),
            PotionType::WaterBreathing { .. } => f.write_str("WATER_BREATHING"),
            PotionType::LongWaterBreathing { .. } => f.write_str("LONG_WATER_BREATHING"),
            PotionType::Healing { .. } => f.write_str("HEALING"),
            PotionType::StrongHealing { .. } => f.write_str("STRONG_HEALING"),
            PotionType::Harming { .. } => f.write_str("HARMING"),
            PotionType::StrongHarming { .. } => f.write_str("STRONG_HARMING"),
            PotionType::Poison { .. } => f.write_str("POISON"),
            PotionType::LongPoison { .. } => f.write_str("LONG_POISON"),
            PotionType::StrongPoison { .. } => f.write_str("STRONG_POISON"),
            PotionType::Regeneration { .. } => f.write_str("REGENERATION"),
            PotionType::LongRegeneration { .. } => f.write_str("LONG_REGENERATION"),
            PotionType::StrongRegeneration { .. } => f.write_str("STRONG_REGENERATION"),
            PotionType::Strength { .. } => f.write_str("STRENGTH"),
            PotionType::LongStrength { .. } => f.write_str("LONG_STRENGTH"),
            PotionType::StrongStrength { .. } => f.write_str("STRONG_STRENGTH"),
            PotionType::Weakness { .. } => f.write_str("WEAKNESS"),
            PotionType::LongWeakness { .. } => f.write_str("LONG_WEAKNESS"),
            PotionType::Luck { .. } => f.write_str("LUCK"),
            PotionType::TurtleMaster { .. } => f.write_str("TURTLE_MASTER"),
            PotionType::LongTurtleMaster { .. } => f.write_str("LONG_TURTLE_MASTER"),
            PotionType::StrongTurtleMaster { .. } => f.write_str("STRONG_TURTLE_MASTER"),
            PotionType::SlowFalling { .. } => f.write_str("SLOW_FALLING"),
            PotionType::LongSlowFalling { .. } => f.write_str("LONG_SLOW_FALLING"),
            PotionType::WindCharged { .. } => f.write_str("WIND_CHARGED"),
            PotionType::Weaving { .. } => f.write_str("WEAVING"),
            PotionType::Oozing { .. } => f.write_str("OOZING"),
            PotionType::Infested { .. } => f.write_str("INFESTED"),
        }
    }
}
impl<'mc> std::ops::Deref for PotionType<'mc> {
    type Target = PotionTypeStruct<'mc>;
    fn deref(&self) -> &<PotionType<'mc> as std::ops::Deref>::Target {
        match self {
            PotionType::Water { inner } => inner,
            PotionType::Mundane { inner } => inner,
            PotionType::Thick { inner } => inner,
            PotionType::Awkward { inner } => inner,
            PotionType::NightVision { inner } => inner,
            PotionType::LongNightVision { inner } => inner,
            PotionType::Invisibility { inner } => inner,
            PotionType::LongInvisibility { inner } => inner,
            PotionType::Leaping { inner } => inner,
            PotionType::LongLeaping { inner } => inner,
            PotionType::StrongLeaping { inner } => inner,
            PotionType::FireResistance { inner } => inner,
            PotionType::LongFireResistance { inner } => inner,
            PotionType::Swiftness { inner } => inner,
            PotionType::LongSwiftness { inner } => inner,
            PotionType::StrongSwiftness { inner } => inner,
            PotionType::Slowness { inner } => inner,
            PotionType::LongSlowness { inner } => inner,
            PotionType::StrongSlowness { inner } => inner,
            PotionType::WaterBreathing { inner } => inner,
            PotionType::LongWaterBreathing { inner } => inner,
            PotionType::Healing { inner } => inner,
            PotionType::StrongHealing { inner } => inner,
            PotionType::Harming { inner } => inner,
            PotionType::StrongHarming { inner } => inner,
            PotionType::Poison { inner } => inner,
            PotionType::LongPoison { inner } => inner,
            PotionType::StrongPoison { inner } => inner,
            PotionType::Regeneration { inner } => inner,
            PotionType::LongRegeneration { inner } => inner,
            PotionType::StrongRegeneration { inner } => inner,
            PotionType::Strength { inner } => inner,
            PotionType::LongStrength { inner } => inner,
            PotionType::StrongStrength { inner } => inner,
            PotionType::Weakness { inner } => inner,
            PotionType::LongWeakness { inner } => inner,
            PotionType::Luck { inner } => inner,
            PotionType::TurtleMaster { inner } => inner,
            PotionType::LongTurtleMaster { inner } => inner,
            PotionType::StrongTurtleMaster { inner } => inner,
            PotionType::SlowFalling { inner } => inner,
            PotionType::LongSlowFalling { inner } => inner,
            PotionType::WindCharged { inner } => inner,
            PotionType::Weaving { inner } => inner,
            PotionType::Oozing { inner } => inner,
            PotionType::Infested { inner } => inner,
        }
    }
}

impl<'mc> PotionType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<PotionType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/potion/PotionType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/potion/PotionType;",
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
            "WATER" => Ok(PotionType::Water {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "MUNDANE" => Ok(PotionType::Mundane {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "THICK" => Ok(PotionType::Thick {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "AWKWARD" => Ok(PotionType::Awkward {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "NIGHT_VISION" => Ok(PotionType::NightVision {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "LONG_NIGHT_VISION" => Ok(PotionType::LongNightVision {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "INVISIBILITY" => Ok(PotionType::Invisibility {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "LONG_INVISIBILITY" => Ok(PotionType::LongInvisibility {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "LEAPING" => Ok(PotionType::Leaping {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "LONG_LEAPING" => Ok(PotionType::LongLeaping {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "STRONG_LEAPING" => Ok(PotionType::StrongLeaping {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "FIRE_RESISTANCE" => Ok(PotionType::FireResistance {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "LONG_FIRE_RESISTANCE" => Ok(PotionType::LongFireResistance {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "SWIFTNESS" => Ok(PotionType::Swiftness {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "LONG_SWIFTNESS" => Ok(PotionType::LongSwiftness {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "STRONG_SWIFTNESS" => Ok(PotionType::StrongSwiftness {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "SLOWNESS" => Ok(PotionType::Slowness {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "LONG_SLOWNESS" => Ok(PotionType::LongSlowness {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "STRONG_SLOWNESS" => Ok(PotionType::StrongSlowness {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "WATER_BREATHING" => Ok(PotionType::WaterBreathing {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "LONG_WATER_BREATHING" => Ok(PotionType::LongWaterBreathing {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "HEALING" => Ok(PotionType::Healing {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "STRONG_HEALING" => Ok(PotionType::StrongHealing {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "HARMING" => Ok(PotionType::Harming {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "STRONG_HARMING" => Ok(PotionType::StrongHarming {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "POISON" => Ok(PotionType::Poison {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "LONG_POISON" => Ok(PotionType::LongPoison {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "STRONG_POISON" => Ok(PotionType::StrongPoison {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "REGENERATION" => Ok(PotionType::Regeneration {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "LONG_REGENERATION" => Ok(PotionType::LongRegeneration {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "STRONG_REGENERATION" => Ok(PotionType::StrongRegeneration {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "STRENGTH" => Ok(PotionType::Strength {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "LONG_STRENGTH" => Ok(PotionType::LongStrength {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "STRONG_STRENGTH" => Ok(PotionType::StrongStrength {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "WEAKNESS" => Ok(PotionType::Weakness {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "LONG_WEAKNESS" => Ok(PotionType::LongWeakness {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "LUCK" => Ok(PotionType::Luck {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "TURTLE_MASTER" => Ok(PotionType::TurtleMaster {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "LONG_TURTLE_MASTER" => Ok(PotionType::LongTurtleMaster {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "STRONG_TURTLE_MASTER" => Ok(PotionType::StrongTurtleMaster {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "SLOW_FALLING" => Ok(PotionType::SlowFalling {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "LONG_SLOW_FALLING" => Ok(PotionType::LongSlowFalling {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "WIND_CHARGED" => Ok(PotionType::WindCharged {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "WEAVING" => Ok(PotionType::Weaving {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "OOZING" => Ok(PotionType::Oozing {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),
            "INFESTED" => Ok(PotionType::Infested {
                inner: PotionTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct PotionTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PotionType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Water { inner } => inner.0.clone(),
            Self::Mundane { inner } => inner.0.clone(),
            Self::Thick { inner } => inner.0.clone(),
            Self::Awkward { inner } => inner.0.clone(),
            Self::NightVision { inner } => inner.0.clone(),
            Self::LongNightVision { inner } => inner.0.clone(),
            Self::Invisibility { inner } => inner.0.clone(),
            Self::LongInvisibility { inner } => inner.0.clone(),
            Self::Leaping { inner } => inner.0.clone(),
            Self::LongLeaping { inner } => inner.0.clone(),
            Self::StrongLeaping { inner } => inner.0.clone(),
            Self::FireResistance { inner } => inner.0.clone(),
            Self::LongFireResistance { inner } => inner.0.clone(),
            Self::Swiftness { inner } => inner.0.clone(),
            Self::LongSwiftness { inner } => inner.0.clone(),
            Self::StrongSwiftness { inner } => inner.0.clone(),
            Self::Slowness { inner } => inner.0.clone(),
            Self::LongSlowness { inner } => inner.0.clone(),
            Self::StrongSlowness { inner } => inner.0.clone(),
            Self::WaterBreathing { inner } => inner.0.clone(),
            Self::LongWaterBreathing { inner } => inner.0.clone(),
            Self::Healing { inner } => inner.0.clone(),
            Self::StrongHealing { inner } => inner.0.clone(),
            Self::Harming { inner } => inner.0.clone(),
            Self::StrongHarming { inner } => inner.0.clone(),
            Self::Poison { inner } => inner.0.clone(),
            Self::LongPoison { inner } => inner.0.clone(),
            Self::StrongPoison { inner } => inner.0.clone(),
            Self::Regeneration { inner } => inner.0.clone(),
            Self::LongRegeneration { inner } => inner.0.clone(),
            Self::StrongRegeneration { inner } => inner.0.clone(),
            Self::Strength { inner } => inner.0.clone(),
            Self::LongStrength { inner } => inner.0.clone(),
            Self::StrongStrength { inner } => inner.0.clone(),
            Self::Weakness { inner } => inner.0.clone(),
            Self::LongWeakness { inner } => inner.0.clone(),
            Self::Luck { inner } => inner.0.clone(),
            Self::TurtleMaster { inner } => inner.0.clone(),
            Self::LongTurtleMaster { inner } => inner.0.clone(),
            Self::StrongTurtleMaster { inner } => inner.0.clone(),
            Self::SlowFalling { inner } => inner.0.clone(),
            Self::LongSlowFalling { inner } => inner.0.clone(),
            Self::WindCharged { inner } => inner.0.clone(),
            Self::Weaving { inner } => inner.0.clone(),
            Self::Oozing { inner } => inner.0.clone(),
            Self::Infested { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Water { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Mundane { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Thick { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Awkward { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::NightVision { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LongNightVision { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Invisibility { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LongInvisibility { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Leaping { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::LongLeaping { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongLeaping { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FireResistance { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LongFireResistance { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Swiftness { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LongSwiftness { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongSwiftness { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Slowness { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::LongSlowness { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongSlowness { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WaterBreathing { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LongWaterBreathing { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Healing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::StrongHealing { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Harming { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::StrongHarming { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Poison { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::LongPoison { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongPoison { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Regeneration { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LongRegeneration { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongRegeneration { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Strength { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::LongStrength { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongStrength { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Weakness { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::LongWeakness { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Luck { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::TurtleMaster { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LongTurtleMaster { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongTurtleMaster { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SlowFalling { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LongSlowFalling { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WindCharged { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Weaving { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Oozing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Infested { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PotionType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PotionType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionType object, got {}",
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
                "WATER" => Ok(PotionType::Water {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "MUNDANE" => Ok(PotionType::Mundane {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "THICK" => Ok(PotionType::Thick {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "AWKWARD" => Ok(PotionType::Awkward {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "NIGHT_VISION" => Ok(PotionType::NightVision {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "LONG_NIGHT_VISION" => Ok(PotionType::LongNightVision {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "INVISIBILITY" => Ok(PotionType::Invisibility {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "LONG_INVISIBILITY" => Ok(PotionType::LongInvisibility {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "LEAPING" => Ok(PotionType::Leaping {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "LONG_LEAPING" => Ok(PotionType::LongLeaping {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "STRONG_LEAPING" => Ok(PotionType::StrongLeaping {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "FIRE_RESISTANCE" => Ok(PotionType::FireResistance {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "LONG_FIRE_RESISTANCE" => Ok(PotionType::LongFireResistance {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "SWIFTNESS" => Ok(PotionType::Swiftness {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "LONG_SWIFTNESS" => Ok(PotionType::LongSwiftness {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "STRONG_SWIFTNESS" => Ok(PotionType::StrongSwiftness {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "SLOWNESS" => Ok(PotionType::Slowness {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "LONG_SLOWNESS" => Ok(PotionType::LongSlowness {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "STRONG_SLOWNESS" => Ok(PotionType::StrongSlowness {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "WATER_BREATHING" => Ok(PotionType::WaterBreathing {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "LONG_WATER_BREATHING" => Ok(PotionType::LongWaterBreathing {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "HEALING" => Ok(PotionType::Healing {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "STRONG_HEALING" => Ok(PotionType::StrongHealing {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "HARMING" => Ok(PotionType::Harming {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "STRONG_HARMING" => Ok(PotionType::StrongHarming {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "POISON" => Ok(PotionType::Poison {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "LONG_POISON" => Ok(PotionType::LongPoison {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "STRONG_POISON" => Ok(PotionType::StrongPoison {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "REGENERATION" => Ok(PotionType::Regeneration {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "LONG_REGENERATION" => Ok(PotionType::LongRegeneration {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "STRONG_REGENERATION" => Ok(PotionType::StrongRegeneration {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "STRENGTH" => Ok(PotionType::Strength {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "LONG_STRENGTH" => Ok(PotionType::LongStrength {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "STRONG_STRENGTH" => Ok(PotionType::StrongStrength {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "WEAKNESS" => Ok(PotionType::Weakness {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "LONG_WEAKNESS" => Ok(PotionType::LongWeakness {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "LUCK" => Ok(PotionType::Luck {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "TURTLE_MASTER" => Ok(PotionType::TurtleMaster {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "LONG_TURTLE_MASTER" => Ok(PotionType::LongTurtleMaster {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "STRONG_TURTLE_MASTER" => Ok(PotionType::StrongTurtleMaster {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "SLOW_FALLING" => Ok(PotionType::SlowFalling {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "LONG_SLOW_FALLING" => Ok(PotionType::LongSlowFalling {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "WIND_CHARGED" => Ok(PotionType::WindCharged {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "WEAVING" => Ok(PotionType::Weaving {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "OOZING" => Ok(PotionType::Oozing {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                "INFESTED" => Ok(PotionType::Infested {
                    inner: PotionTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for PotionTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PotionTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PotionTypeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PotionTypeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::potion::PotionType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionType;");
        let cls = jni.find_class("org/bukkit/potion/PotionType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::PotionType::from_raw(&jni, obj)
    }
    #[deprecated]

    pub fn effect_type(
        &self,
    ) -> Result<Option<crate::potion::PotionEffectType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEffectType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::potion::PotionEffectType::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn potion_effects(
        &self,
    ) -> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPotionEffects",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::potion::PotionEffect::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    #[deprecated]

    pub fn is_instant(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInstant", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if the potion type has an upgraded state.
    /// This refers to whether or not the potion type can be Tier 2,
    /// such as Potion of Fire Resistance II.
    pub fn is_upgradeable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUpgradeable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if the potion type has an extended state.
    /// This refers to the extended duration potions
    pub fn is_extendable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isExtendable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn max_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    #[deprecated]

    pub fn get_by_effect(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        effect_type: impl Into<crate::potion::PotionEffectType<'mc>>,
    ) -> Result<Option<crate::potion::PotionType<'mc>>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/potion/PotionEffectType;)Lorg/bukkit/potion/PotionType;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(effect_type.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/potion/PotionType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        let obj = res.l()?;
        Ok(Some(crate::potion::PotionType::from_raw(&jni, obj)?))
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct PotionTypeInternalPotionData<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PotionTypeInternalPotionData<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PotionTypeInternalPotionData<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PotionTypeInternalPotionData from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/potion/PotionType/InternalPotionData")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionTypeInternalPotionData object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PotionTypeInternalPotionData<'mc> {
    pub fn effect_type(
        &self,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEffectType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffectType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn potion_effects(
        &self,
    ) -> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPotionEffects",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn is_instant(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInstant", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_upgradeable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUpgradeable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_extendable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isExtendable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn max_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum PotionEffectType<'mc> {
    Speed { inner: PotionEffectTypeStruct<'mc> },
    Slowness { inner: PotionEffectTypeStruct<'mc> },
    Haste { inner: PotionEffectTypeStruct<'mc> },
    MiningFatigue { inner: PotionEffectTypeStruct<'mc> },
    Strength { inner: PotionEffectTypeStruct<'mc> },
    InstantHealth { inner: PotionEffectTypeStruct<'mc> },
    InstantDamage { inner: PotionEffectTypeStruct<'mc> },
    JumpBoost { inner: PotionEffectTypeStruct<'mc> },
    Nausea { inner: PotionEffectTypeStruct<'mc> },
    Regeneration { inner: PotionEffectTypeStruct<'mc> },
    Resistance { inner: PotionEffectTypeStruct<'mc> },
    FireResistance { inner: PotionEffectTypeStruct<'mc> },
    WaterBreathing { inner: PotionEffectTypeStruct<'mc> },
    Invisibility { inner: PotionEffectTypeStruct<'mc> },
    Blindness { inner: PotionEffectTypeStruct<'mc> },
    NightVision { inner: PotionEffectTypeStruct<'mc> },
    Hunger { inner: PotionEffectTypeStruct<'mc> },
    Weakness { inner: PotionEffectTypeStruct<'mc> },
    Poison { inner: PotionEffectTypeStruct<'mc> },
    Wither { inner: PotionEffectTypeStruct<'mc> },
    HealthBoost { inner: PotionEffectTypeStruct<'mc> },
    Absorption { inner: PotionEffectTypeStruct<'mc> },
    Saturation { inner: PotionEffectTypeStruct<'mc> },
    Glowing { inner: PotionEffectTypeStruct<'mc> },
    Levitation { inner: PotionEffectTypeStruct<'mc> },
    Luck { inner: PotionEffectTypeStruct<'mc> },
    Unluck { inner: PotionEffectTypeStruct<'mc> },
    SlowFalling { inner: PotionEffectTypeStruct<'mc> },
    ConduitPower { inner: PotionEffectTypeStruct<'mc> },
    DolphinsGrace { inner: PotionEffectTypeStruct<'mc> },
    BadOmen { inner: PotionEffectTypeStruct<'mc> },
    HeroOfTheVillage { inner: PotionEffectTypeStruct<'mc> },
    Darkness { inner: PotionEffectTypeStruct<'mc> },
    TrialOmen { inner: PotionEffectTypeStruct<'mc> },
    RaidOmen { inner: PotionEffectTypeStruct<'mc> },
    WindCharged { inner: PotionEffectTypeStruct<'mc> },
    Weaving { inner: PotionEffectTypeStruct<'mc> },
    Oozing { inner: PotionEffectTypeStruct<'mc> },
    Infested { inner: PotionEffectTypeStruct<'mc> },
}
impl<'mc> std::fmt::Display for PotionEffectType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PotionEffectType::Speed { .. } => f.write_str("SPEED"),
            PotionEffectType::Slowness { .. } => f.write_str("SLOWNESS"),
            PotionEffectType::Haste { .. } => f.write_str("HASTE"),
            PotionEffectType::MiningFatigue { .. } => f.write_str("MINING_FATIGUE"),
            PotionEffectType::Strength { .. } => f.write_str("STRENGTH"),
            PotionEffectType::InstantHealth { .. } => f.write_str("INSTANT_HEALTH"),
            PotionEffectType::InstantDamage { .. } => f.write_str("INSTANT_DAMAGE"),
            PotionEffectType::JumpBoost { .. } => f.write_str("JUMP_BOOST"),
            PotionEffectType::Nausea { .. } => f.write_str("NAUSEA"),
            PotionEffectType::Regeneration { .. } => f.write_str("REGENERATION"),
            PotionEffectType::Resistance { .. } => f.write_str("RESISTANCE"),
            PotionEffectType::FireResistance { .. } => f.write_str("FIRE_RESISTANCE"),
            PotionEffectType::WaterBreathing { .. } => f.write_str("WATER_BREATHING"),
            PotionEffectType::Invisibility { .. } => f.write_str("INVISIBILITY"),
            PotionEffectType::Blindness { .. } => f.write_str("BLINDNESS"),
            PotionEffectType::NightVision { .. } => f.write_str("NIGHT_VISION"),
            PotionEffectType::Hunger { .. } => f.write_str("HUNGER"),
            PotionEffectType::Weakness { .. } => f.write_str("WEAKNESS"),
            PotionEffectType::Poison { .. } => f.write_str("POISON"),
            PotionEffectType::Wither { .. } => f.write_str("WITHER"),
            PotionEffectType::HealthBoost { .. } => f.write_str("HEALTH_BOOST"),
            PotionEffectType::Absorption { .. } => f.write_str("ABSORPTION"),
            PotionEffectType::Saturation { .. } => f.write_str("SATURATION"),
            PotionEffectType::Glowing { .. } => f.write_str("GLOWING"),
            PotionEffectType::Levitation { .. } => f.write_str("LEVITATION"),
            PotionEffectType::Luck { .. } => f.write_str("LUCK"),
            PotionEffectType::Unluck { .. } => f.write_str("UNLUCK"),
            PotionEffectType::SlowFalling { .. } => f.write_str("SLOW_FALLING"),
            PotionEffectType::ConduitPower { .. } => f.write_str("CONDUIT_POWER"),
            PotionEffectType::DolphinsGrace { .. } => f.write_str("DOLPHINS_GRACE"),
            PotionEffectType::BadOmen { .. } => f.write_str("BAD_OMEN"),
            PotionEffectType::HeroOfTheVillage { .. } => f.write_str("HERO_OF_THE_VILLAGE"),
            PotionEffectType::Darkness { .. } => f.write_str("DARKNESS"),
            PotionEffectType::TrialOmen { .. } => f.write_str("TRIAL_OMEN"),
            PotionEffectType::RaidOmen { .. } => f.write_str("RAID_OMEN"),
            PotionEffectType::WindCharged { .. } => f.write_str("WIND_CHARGED"),
            PotionEffectType::Weaving { .. } => f.write_str("WEAVING"),
            PotionEffectType::Oozing { .. } => f.write_str("OOZING"),
            PotionEffectType::Infested { .. } => f.write_str("INFESTED"),
        }
    }
}
impl<'mc> std::ops::Deref for PotionEffectType<'mc> {
    type Target = PotionEffectTypeStruct<'mc>;
    fn deref(&self) -> &<PotionEffectType<'mc> as std::ops::Deref>::Target {
        match self {
            PotionEffectType::Speed { inner } => inner,
            PotionEffectType::Slowness { inner } => inner,
            PotionEffectType::Haste { inner } => inner,
            PotionEffectType::MiningFatigue { inner } => inner,
            PotionEffectType::Strength { inner } => inner,
            PotionEffectType::InstantHealth { inner } => inner,
            PotionEffectType::InstantDamage { inner } => inner,
            PotionEffectType::JumpBoost { inner } => inner,
            PotionEffectType::Nausea { inner } => inner,
            PotionEffectType::Regeneration { inner } => inner,
            PotionEffectType::Resistance { inner } => inner,
            PotionEffectType::FireResistance { inner } => inner,
            PotionEffectType::WaterBreathing { inner } => inner,
            PotionEffectType::Invisibility { inner } => inner,
            PotionEffectType::Blindness { inner } => inner,
            PotionEffectType::NightVision { inner } => inner,
            PotionEffectType::Hunger { inner } => inner,
            PotionEffectType::Weakness { inner } => inner,
            PotionEffectType::Poison { inner } => inner,
            PotionEffectType::Wither { inner } => inner,
            PotionEffectType::HealthBoost { inner } => inner,
            PotionEffectType::Absorption { inner } => inner,
            PotionEffectType::Saturation { inner } => inner,
            PotionEffectType::Glowing { inner } => inner,
            PotionEffectType::Levitation { inner } => inner,
            PotionEffectType::Luck { inner } => inner,
            PotionEffectType::Unluck { inner } => inner,
            PotionEffectType::SlowFalling { inner } => inner,
            PotionEffectType::ConduitPower { inner } => inner,
            PotionEffectType::DolphinsGrace { inner } => inner,
            PotionEffectType::BadOmen { inner } => inner,
            PotionEffectType::HeroOfTheVillage { inner } => inner,
            PotionEffectType::Darkness { inner } => inner,
            PotionEffectType::TrialOmen { inner } => inner,
            PotionEffectType::RaidOmen { inner } => inner,
            PotionEffectType::WindCharged { inner } => inner,
            PotionEffectType::Weaving { inner } => inner,
            PotionEffectType::Oozing { inner } => inner,
            PotionEffectType::Infested { inner } => inner,
        }
    }
}

impl<'mc> PotionEffectType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/potion/PotionEffectType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/potion/PotionEffectType;",
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
            "SPEED" => Ok(PotionEffectType::Speed {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "SLOWNESS" => Ok(PotionEffectType::Slowness {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "HASTE" => Ok(PotionEffectType::Haste {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "MINING_FATIGUE" => Ok(PotionEffectType::MiningFatigue {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "STRENGTH" => Ok(PotionEffectType::Strength {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "INSTANT_HEALTH" => Ok(PotionEffectType::InstantHealth {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "INSTANT_DAMAGE" => Ok(PotionEffectType::InstantDamage {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "JUMP_BOOST" => Ok(PotionEffectType::JumpBoost {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "NAUSEA" => Ok(PotionEffectType::Nausea {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "REGENERATION" => Ok(PotionEffectType::Regeneration {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "RESISTANCE" => Ok(PotionEffectType::Resistance {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "FIRE_RESISTANCE" => Ok(PotionEffectType::FireResistance {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "WATER_BREATHING" => Ok(PotionEffectType::WaterBreathing {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "INVISIBILITY" => Ok(PotionEffectType::Invisibility {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "BLINDNESS" => Ok(PotionEffectType::Blindness {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "NIGHT_VISION" => Ok(PotionEffectType::NightVision {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "HUNGER" => Ok(PotionEffectType::Hunger {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "WEAKNESS" => Ok(PotionEffectType::Weakness {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "POISON" => Ok(PotionEffectType::Poison {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "WITHER" => Ok(PotionEffectType::Wither {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "HEALTH_BOOST" => Ok(PotionEffectType::HealthBoost {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "ABSORPTION" => Ok(PotionEffectType::Absorption {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "SATURATION" => Ok(PotionEffectType::Saturation {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "GLOWING" => Ok(PotionEffectType::Glowing {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "LEVITATION" => Ok(PotionEffectType::Levitation {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "LUCK" => Ok(PotionEffectType::Luck {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "UNLUCK" => Ok(PotionEffectType::Unluck {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "SLOW_FALLING" => Ok(PotionEffectType::SlowFalling {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "CONDUIT_POWER" => Ok(PotionEffectType::ConduitPower {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "DOLPHINS_GRACE" => Ok(PotionEffectType::DolphinsGrace {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "BAD_OMEN" => Ok(PotionEffectType::BadOmen {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "HERO_OF_THE_VILLAGE" => Ok(PotionEffectType::HeroOfTheVillage {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "DARKNESS" => Ok(PotionEffectType::Darkness {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "TRIAL_OMEN" => Ok(PotionEffectType::TrialOmen {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "RAID_OMEN" => Ok(PotionEffectType::RaidOmen {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "WIND_CHARGED" => Ok(PotionEffectType::WindCharged {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "WEAVING" => Ok(PotionEffectType::Weaving {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "OOZING" => Ok(PotionEffectType::Oozing {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),
            "INFESTED" => Ok(PotionEffectType::Infested {
                inner: PotionEffectTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct PotionEffectTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PotionEffectType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Speed { inner } => inner.0.clone(),
            Self::Slowness { inner } => inner.0.clone(),
            Self::Haste { inner } => inner.0.clone(),
            Self::MiningFatigue { inner } => inner.0.clone(),
            Self::Strength { inner } => inner.0.clone(),
            Self::InstantHealth { inner } => inner.0.clone(),
            Self::InstantDamage { inner } => inner.0.clone(),
            Self::JumpBoost { inner } => inner.0.clone(),
            Self::Nausea { inner } => inner.0.clone(),
            Self::Regeneration { inner } => inner.0.clone(),
            Self::Resistance { inner } => inner.0.clone(),
            Self::FireResistance { inner } => inner.0.clone(),
            Self::WaterBreathing { inner } => inner.0.clone(),
            Self::Invisibility { inner } => inner.0.clone(),
            Self::Blindness { inner } => inner.0.clone(),
            Self::NightVision { inner } => inner.0.clone(),
            Self::Hunger { inner } => inner.0.clone(),
            Self::Weakness { inner } => inner.0.clone(),
            Self::Poison { inner } => inner.0.clone(),
            Self::Wither { inner } => inner.0.clone(),
            Self::HealthBoost { inner } => inner.0.clone(),
            Self::Absorption { inner } => inner.0.clone(),
            Self::Saturation { inner } => inner.0.clone(),
            Self::Glowing { inner } => inner.0.clone(),
            Self::Levitation { inner } => inner.0.clone(),
            Self::Luck { inner } => inner.0.clone(),
            Self::Unluck { inner } => inner.0.clone(),
            Self::SlowFalling { inner } => inner.0.clone(),
            Self::ConduitPower { inner } => inner.0.clone(),
            Self::DolphinsGrace { inner } => inner.0.clone(),
            Self::BadOmen { inner } => inner.0.clone(),
            Self::HeroOfTheVillage { inner } => inner.0.clone(),
            Self::Darkness { inner } => inner.0.clone(),
            Self::TrialOmen { inner } => inner.0.clone(),
            Self::RaidOmen { inner } => inner.0.clone(),
            Self::WindCharged { inner } => inner.0.clone(),
            Self::Weaving { inner } => inner.0.clone(),
            Self::Oozing { inner } => inner.0.clone(),
            Self::Infested { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Speed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Slowness { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Haste { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::MiningFatigue { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Strength { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::InstantHealth { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::InstantDamage { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::JumpBoost { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Nausea { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Regeneration { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Resistance { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FireResistance { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WaterBreathing { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Invisibility { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Blindness { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::NightVision { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Hunger { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Weakness { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Poison { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Wither { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::HealthBoost { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Absorption { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Saturation { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Glowing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Levitation { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Luck { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Unluck { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SlowFalling { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ConduitPower { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DolphinsGrace { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BadOmen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::HeroOfTheVillage { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Darkness { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::TrialOmen { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RaidOmen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WindCharged { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Weaving { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Oozing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Infested { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PotionEffectType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PotionEffectType from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionEffectType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionEffectType object, got {}",
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
                "SPEED" => Ok(PotionEffectType::Speed {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "SLOWNESS" => Ok(PotionEffectType::Slowness {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "HASTE" => Ok(PotionEffectType::Haste {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "MINING_FATIGUE" => Ok(PotionEffectType::MiningFatigue {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "STRENGTH" => Ok(PotionEffectType::Strength {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "INSTANT_HEALTH" => Ok(PotionEffectType::InstantHealth {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "INSTANT_DAMAGE" => Ok(PotionEffectType::InstantDamage {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "JUMP_BOOST" => Ok(PotionEffectType::JumpBoost {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "NAUSEA" => Ok(PotionEffectType::Nausea {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "REGENERATION" => Ok(PotionEffectType::Regeneration {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "RESISTANCE" => Ok(PotionEffectType::Resistance {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "FIRE_RESISTANCE" => Ok(PotionEffectType::FireResistance {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "WATER_BREATHING" => Ok(PotionEffectType::WaterBreathing {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "INVISIBILITY" => Ok(PotionEffectType::Invisibility {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "BLINDNESS" => Ok(PotionEffectType::Blindness {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "NIGHT_VISION" => Ok(PotionEffectType::NightVision {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "HUNGER" => Ok(PotionEffectType::Hunger {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "WEAKNESS" => Ok(PotionEffectType::Weakness {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "POISON" => Ok(PotionEffectType::Poison {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "WITHER" => Ok(PotionEffectType::Wither {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "HEALTH_BOOST" => Ok(PotionEffectType::HealthBoost {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "ABSORPTION" => Ok(PotionEffectType::Absorption {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "SATURATION" => Ok(PotionEffectType::Saturation {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "GLOWING" => Ok(PotionEffectType::Glowing {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "LEVITATION" => Ok(PotionEffectType::Levitation {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "LUCK" => Ok(PotionEffectType::Luck {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "UNLUCK" => Ok(PotionEffectType::Unluck {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "SLOW_FALLING" => Ok(PotionEffectType::SlowFalling {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "CONDUIT_POWER" => Ok(PotionEffectType::ConduitPower {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "DOLPHINS_GRACE" => Ok(PotionEffectType::DolphinsGrace {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "BAD_OMEN" => Ok(PotionEffectType::BadOmen {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "HERO_OF_THE_VILLAGE" => Ok(PotionEffectType::HeroOfTheVillage {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "DARKNESS" => Ok(PotionEffectType::Darkness {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "TRIAL_OMEN" => Ok(PotionEffectType::TrialOmen {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "RAID_OMEN" => Ok(PotionEffectType::RaidOmen {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "WIND_CHARGED" => Ok(PotionEffectType::WindCharged {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "WEAVING" => Ok(PotionEffectType::Weaving {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "OOZING" => Ok(PotionEffectType::Oozing {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                "INFESTED" => Ok(PotionEffectType::Infested {
                    inner: PotionEffectTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for PotionEffectTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PotionEffectTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PotionEffectTypeStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionEffectType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionEffectTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PotionEffectTypeStruct<'mc> {
    #[deprecated]
    /// Gets the PotionEffectType at the specified key
    pub fn get_by_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        key: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<Option<crate::potion::PotionEffectType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/potion/PotionEffectType;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(key.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
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
        Ok(Some(crate::potion::PotionEffectType::from_raw(&jni, obj)?))
    }
    #[deprecated]
    /// Gets the effect type specified by the unique id.
    pub fn get_by_id(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        id: i32,
    ) -> Result<Option<crate::potion::PotionEffectType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/potion/PotionEffectType;");
        let val_1 = jni::objects::JValueGen::Int(id);
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getById",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        let obj = res.l()?;
        Ok(Some(crate::potion::PotionEffectType::from_raw(&jni, obj)?))
    }
    #[deprecated]
    /// Gets the effect type specified by the given name.
    pub fn get_by_name(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        name: impl Into<String>,
    ) -> Result<Option<crate::potion::PotionEffectType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/potion/PotionEffectType;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(name.into())?,
        ));
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
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
        Ok(Some(crate::potion::PotionEffectType::from_raw(&jni, obj)?))
    }
    #[deprecated]

    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::PotionEffectType::from_raw(&jni, obj)
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
pub struct PotionEffect<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PotionEffect<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PotionEffect<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PotionEffect from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionEffect")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionEffect object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PotionEffect<'mc> {
    /// Creates a potion effect.
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        val_type: impl Into<crate::potion::PotionEffectType<'mc>>,
        duration: std::option::Option<i32>,
        amplifier: std::option::Option<i32>,
        ambient: std::option::Option<bool>,
        particles: std::option::Option<bool>,
        icon: std::option::Option<bool>,
    ) -> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/potion/PotionEffectType;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = duration {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = amplifier {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        if let Some(a) = ambient {
            sig += "Z";
            let val_4 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_4);
        }
        if let Some(a) = particles {
            sig += "Z";
            let val_5 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_5);
        }
        if let Some(a) = icon {
            sig += "Z";
            let val_6 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_6);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/potion/PotionEffect");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::potion::PotionEffect::from_raw(&jni, res)
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
    /// Attempts to add the effect represented by this object to the given
    /// {@link LivingEntity}.
    pub fn apply(
        &self,
        entity: impl Into<crate::entity::LivingEntity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "apply",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn equals(
        &self,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(obj);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the amplifier of this effect. A higher amplifier means the
    /// potion effect happens more often over its duration and in some cases
    /// has more effect on its target.
    pub fn amplifier(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAmplifier", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the duration (in ticks) that this effect will run for when
    /// applied to a {@link LivingEntity}.
    pub fn duration(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDuration", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns whether or not this potion effect has an infinite duration. Potion
    /// effects with infinite durations will display an infinite symbol and never
    /// expire unless manually removed.
    pub fn is_infinite(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInfinite", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns whether or not this potion effect has a shorter duration than the
    /// provided potion effect.
    ///
    /// An infinite duration is considered longer than non-infinite durations. If
    /// both potion effects have infinite durations, then neither is shorter than
    /// the other and this method will return false.
    pub fn is_shorter_than(
        &self,
        other: impl Into<crate::potion::PotionEffect<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionEffect;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isShorterThan",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the {@link PotionEffectType} of this effect.
    pub fn get_type(
        &self,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffectType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Makes potion effect produce more, translucent, particles.
    pub fn is_ambient(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isAmbient", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn has_particles(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasParticles", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    #[deprecated]

    pub fn color(&self) -> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }

    pub fn has_icon(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasIcon", sig.as_str(), vec![]);
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> std::string::ToString for PotionEffect<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling PotionEffect.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for PotionEffect<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting PotionEffect into crate::configuration::serialization::ConfigurationSerializable")
    }
}
#[repr(C)]
pub struct PotionData<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PotionData<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PotionData<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PotionData from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionData")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionData object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PotionData<'mc> {
    /// Instantiates a final PotionData object to contain information about a
    /// Potion
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        val_type: impl Into<crate::potion::PotionType<'mc>>,
        extended: std::option::Option<bool>,
        upgraded: std::option::Option<bool>,
    ) -> Result<crate::potion::PotionData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/potion/PotionType;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = extended {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        if let Some(a) = upgraded {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/potion/PotionData");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::potion::PotionData::from_raw(&jni, res)
    }
    /// Gets the type of the potion, Type matches up with each kind of craftable
    /// potion
    pub fn get_type(&self) -> Result<crate::potion::PotionType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the potion is in an upgraded state. This refers to whether or
    /// not the potion is Tier 2, such as Potion of Fire Resistance II.
    pub fn is_upgraded(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUpgraded", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if the potion is in an extended state. This refers to the extended
    /// duration potions
    pub fn is_extended(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isExtended", sig.as_str(), vec![]);
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

    pub fn equals(
        &self,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(obj);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
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
pub enum PotionEffectTypeCategory<'mc> {
    Beneficial {
        inner: PotionEffectTypeCategoryStruct<'mc>,
    },
    Harmful {
        inner: PotionEffectTypeCategoryStruct<'mc>,
    },
    Neutral {
        inner: PotionEffectTypeCategoryStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for PotionEffectTypeCategory<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PotionEffectTypeCategory::Beneficial { .. } => f.write_str("BENEFICIAL"),
            PotionEffectTypeCategory::Harmful { .. } => f.write_str("HARMFUL"),
            PotionEffectTypeCategory::Neutral { .. } => f.write_str("NEUTRAL"),
        }
    }
}
impl<'mc> std::ops::Deref for PotionEffectTypeCategory<'mc> {
    type Target = PotionEffectTypeCategoryStruct<'mc>;
    fn deref(&self) -> &<PotionEffectTypeCategory<'mc> as std::ops::Deref>::Target {
        match self {
            PotionEffectTypeCategory::Beneficial { inner } => inner,
            PotionEffectTypeCategory::Harmful { inner } => inner,
            PotionEffectTypeCategory::Neutral { inner } => inner,
        }
    }
}

impl<'mc> PotionEffectTypeCategory<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<PotionEffectTypeCategory<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/potion/PotionEffectTypeCategory");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/potion/PotionEffectTypeCategory;",
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
            "BENEFICIAL" => Ok(PotionEffectTypeCategory::Beneficial {
                inner: PotionEffectTypeCategoryStruct::from_raw(env, obj)?,
            }),
            "HARMFUL" => Ok(PotionEffectTypeCategory::Harmful {
                inner: PotionEffectTypeCategoryStruct::from_raw(env, obj)?,
            }),
            "NEUTRAL" => Ok(PotionEffectTypeCategory::Neutral {
                inner: PotionEffectTypeCategoryStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct PotionEffectTypeCategoryStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PotionEffectTypeCategory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Beneficial { inner } => inner.0.clone(),
            Self::Harmful { inner } => inner.0.clone(),
            Self::Neutral { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Beneficial { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Harmful { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Neutral { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PotionEffectTypeCategory<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PotionEffectTypeCategory from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/potion/PotionEffectTypeCategory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionEffectTypeCategory object, got {}",
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
                "BENEFICIAL" => Ok(PotionEffectTypeCategory::Beneficial {
                    inner: PotionEffectTypeCategoryStruct::from_raw(env, obj)?,
                }),
                "HARMFUL" => Ok(PotionEffectTypeCategory::Harmful {
                    inner: PotionEffectTypeCategoryStruct::from_raw(env, obj)?,
                }),
                "NEUTRAL" => Ok(PotionEffectTypeCategory::Neutral {
                    inner: PotionEffectTypeCategoryStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for PotionEffectTypeCategoryStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PotionEffectTypeCategoryStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PotionEffectTypeCategoryStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/potion/PotionEffectTypeCategory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionEffectTypeCategoryStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PotionEffectTypeCategoryStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::potion::PotionEffectTypeCategory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffectTypeCategory;");
        let cls = jni.find_class("org/bukkit/potion/PotionEffectTypeCategory");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::PotionEffectTypeCategory::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct PotionBrewer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PotionBrewer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PotionBrewer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PotionBrewer from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionBrewer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionBrewer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PotionBrewer<'mc> {
    /// Creates a {@link PotionEffect} from the given {@link PotionEffectType},
    /// applying duration modifiers and checks.
    pub fn create_effect(
        &self,
        potion: impl Into<crate::potion::PotionEffectType<'mc>>,
        duration: i32,
        amplifier: i32,
    ) -> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/potion/PotionEffectType;II)Lorg/bukkit/potion/PotionEffect;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(potion.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(duration);
        let val_3 = jni::objects::JValueGen::Int(amplifier);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createEffect",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffect::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// Returns a collection of {@link PotionEffect} that would be applied from a potion with the given data value.
    pub fn get_effects_from_damage(
        &self,
        damage: i32,
    ) -> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/util/Collection;");
        let val_1 = jni::objects::JValueGen::Int(damage);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectsFromDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    #[deprecated]
    /// Returns a collection of {@link PotionEffect} that would be applied from a potion with the given type.
    pub fn get_effects(
        &self,
        val_type: impl Into<crate::potion::PotionType<'mc>>,
        upgraded: bool,
        extended: bool,
    ) -> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionType;ZZ)Ljava/util/Collection;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(upgraded.into());
        let val_3 = jni::objects::JValueGen::Bool(extended.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffects",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct PotionEffectTypeWrapper<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PotionEffectTypeWrapper<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PotionEffectTypeWrapper<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PotionEffectTypeWrapper from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionEffectTypeWrapper")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionEffectTypeWrapper object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PotionEffectTypeWrapper<'mc> {
    /// Get the potion type bound to this wrapper.
    pub fn get_type(
        &self,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffectType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.potion.PotionEffectType ( ['getType'])
    /// Gets the PotionEffectType at the specified key
    pub fn get_by_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        key: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<Option<crate::potion::PotionEffectType<'mc>>, Box<dyn std::error::Error>> {
        crate::potion::PotionEffectTypeStruct::get_by_key(jni, key)
    }
    /// Gets the effect type specified by the unique id.
    pub fn get_by_id(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        id: i32,
    ) -> Result<Option<crate::potion::PotionEffectType<'mc>>, Box<dyn std::error::Error>> {
        crate::potion::PotionEffectTypeStruct::get_by_id(jni, id)
    }
    /// Gets the effect type specified by the given name.
    pub fn get_by_name(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        name: impl Into<String>,
    ) -> Result<Option<crate::potion::PotionEffectType<'mc>>, Box<dyn std::error::Error>> {
        crate::potion::PotionEffectTypeStruct::get_by_name(jni, name)
    }
    #[deprecated]

    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::PotionEffectType::from_raw(&jni, obj)
    }
    /// Return the namespaced identifier for this object.
    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::potion::PotionEffectType::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::potion::PotionEffectType = temp_clone.into();
        real.key()
    }
    /// Get the translation key, suitable for use in a translation component.
    pub fn translation_key(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::potion::PotionEffectType::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::potion::PotionEffectType = temp_clone.into();
        real.translation_key()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::potion::PotionEffectType<'mc>> for PotionEffectTypeWrapper<'mc> {
    fn into(self) -> crate::potion::PotionEffectType<'mc> {
        crate::potion::PotionEffectType::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PotionEffectTypeWrapper into crate::potion::PotionEffectType")
    }
}
