#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub enum Mirror<'mc> {
    None { inner: MirrorStruct<'mc> },
    LeftRight { inner: MirrorStruct<'mc> },
    FrontBack { inner: MirrorStruct<'mc> },
}
impl<'mc> std::fmt::Display for Mirror<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mirror::None { .. } => f.write_str("NONE"),
            Mirror::LeftRight { .. } => f.write_str("LEFT_RIGHT"),
            Mirror::FrontBack { .. } => f.write_str("FRONT_BACK"),
        }
    }
}

impl<'mc> Mirror<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Mirror<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/structure/Mirror");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/structure/Mirror;",
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
            "NONE" => Ok(Mirror::None {
                inner: MirrorStruct::from_raw(env, obj)?,
            }),
            "LEFT_RIGHT" => Ok(Mirror::LeftRight {
                inner: MirrorStruct::from_raw(env, obj)?,
            }),
            "FRONT_BACK" => Ok(Mirror::FrontBack {
                inner: MirrorStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct MirrorStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Mirror<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::None { inner } => inner.0.clone(),
            Self::LeftRight { inner } => inner.0.clone(),
            Self::FrontBack { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::None { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::LeftRight { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FrontBack { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Mirror<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Mirror from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/structure/Mirror")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Mirror object, got {}",
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
                "NONE" => Ok(Mirror::None {
                    inner: MirrorStruct::from_raw(env, obj)?,
                }),
                "LEFT_RIGHT" => Ok(Mirror::LeftRight {
                    inner: MirrorStruct::from_raw(env, obj)?,
                }),
                "FRONT_BACK" => Ok(Mirror::FrontBack {
                    inner: MirrorStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for MirrorStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MirrorStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MirrorStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/structure/Mirror")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MirrorStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MirrorStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::structure::Mirror<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::block::structure::Mirror;");
        let cls = jni.find_class("org/bukkit/block/structure/Mirror");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::structure::Mirror::from_raw(&jni, obj)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum StructureRotation<'mc> {
    None { inner: StructureRotationStruct<'mc> },
    Clockwise90 { inner: StructureRotationStruct<'mc> },
    Clockwise180 { inner: StructureRotationStruct<'mc> },
    Counterclockwise90 { inner: StructureRotationStruct<'mc> },
}
impl<'mc> std::fmt::Display for StructureRotation<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StructureRotation::None { .. } => f.write_str("NONE"),
            StructureRotation::Clockwise90 { .. } => f.write_str("CLOCKWISE_90"),
            StructureRotation::Clockwise180 { .. } => f.write_str("CLOCKWISE_180"),
            StructureRotation::Counterclockwise90 { .. } => f.write_str("COUNTERCLOCKWISE_90"),
        }
    }
}

impl<'mc> StructureRotation<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<StructureRotation<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/structure/StructureRotation");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/structure/StructureRotation;",
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
            "NONE" => Ok(StructureRotation::None {
                inner: StructureRotationStruct::from_raw(env, obj)?,
            }),
            "CLOCKWISE_90" => Ok(StructureRotation::Clockwise90 {
                inner: StructureRotationStruct::from_raw(env, obj)?,
            }),
            "CLOCKWISE_180" => Ok(StructureRotation::Clockwise180 {
                inner: StructureRotationStruct::from_raw(env, obj)?,
            }),
            "COUNTERCLOCKWISE_90" => Ok(StructureRotation::Counterclockwise90 {
                inner: StructureRotationStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct StructureRotationStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for StructureRotation<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::None { inner } => inner.0.clone(),
            Self::Clockwise90 { inner } => inner.0.clone(),
            Self::Clockwise180 { inner } => inner.0.clone(),
            Self::Counterclockwise90 { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::None { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Clockwise90 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Clockwise180 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Counterclockwise90 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for StructureRotation<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate StructureRotation from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/structure/StructureRotation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StructureRotation object, got {}",
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
                "NONE" => Ok(StructureRotation::None {
                    inner: StructureRotationStruct::from_raw(env, obj)?,
                }),
                "CLOCKWISE_90" => Ok(StructureRotation::Clockwise90 {
                    inner: StructureRotationStruct::from_raw(env, obj)?,
                }),
                "CLOCKWISE_180" => Ok(StructureRotation::Clockwise180 {
                    inner: StructureRotationStruct::from_raw(env, obj)?,
                }),
                "COUNTERCLOCKWISE_90" => Ok(StructureRotation::Counterclockwise90 {
                    inner: StructureRotationStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for StructureRotationStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for StructureRotationStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate StructureRotationStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/structure/StructureRotation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StructureRotationStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> StructureRotationStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::structure::StructureRotation<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::block::structure::StructureRotation;");
        let cls = jni.find_class("org/bukkit/block/structure/StructureRotation");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::structure::StructureRotation::from_raw(&jni, obj)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum UsageMode<'mc> {
    Save { inner: UsageModeStruct<'mc> },
    Load { inner: UsageModeStruct<'mc> },
    Corner { inner: UsageModeStruct<'mc> },
    Data { inner: UsageModeStruct<'mc> },
}
impl<'mc> std::fmt::Display for UsageMode<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UsageMode::Save { .. } => f.write_str("SAVE"),
            UsageMode::Load { .. } => f.write_str("LOAD"),
            UsageMode::Corner { .. } => f.write_str("CORNER"),
            UsageMode::Data { .. } => f.write_str("DATA"),
        }
    }
}

impl<'mc> UsageMode<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<UsageMode<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/structure/UsageMode");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/structure/UsageMode;",
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
            "SAVE" => Ok(UsageMode::Save {
                inner: UsageModeStruct::from_raw(env, obj)?,
            }),
            "LOAD" => Ok(UsageMode::Load {
                inner: UsageModeStruct::from_raw(env, obj)?,
            }),
            "CORNER" => Ok(UsageMode::Corner {
                inner: UsageModeStruct::from_raw(env, obj)?,
            }),
            "DATA" => Ok(UsageMode::Data {
                inner: UsageModeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct UsageModeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for UsageMode<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Save { inner } => inner.0.clone(),
            Self::Load { inner } => inner.0.clone(),
            Self::Corner { inner } => inner.0.clone(),
            Self::Data { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Save { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Load { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Corner { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Data { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for UsageMode<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate UsageMode from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/structure/UsageMode")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a UsageMode object, got {}",
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
                "SAVE" => Ok(UsageMode::Save {
                    inner: UsageModeStruct::from_raw(env, obj)?,
                }),
                "LOAD" => Ok(UsageMode::Load {
                    inner: UsageModeStruct::from_raw(env, obj)?,
                }),
                "CORNER" => Ok(UsageMode::Corner {
                    inner: UsageModeStruct::from_raw(env, obj)?,
                }),
                "DATA" => Ok(UsageMode::Data {
                    inner: UsageModeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for UsageModeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for UsageModeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate UsageModeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/structure/UsageMode")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a UsageModeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> UsageModeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::structure::UsageMode<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::block::structure::UsageMode;");
        let cls = jni.find_class("org/bukkit/block/structure/UsageMode");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::structure::UsageMode::from_raw(&jni, obj)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
