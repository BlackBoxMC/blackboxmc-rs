#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub enum CraftingBookCategory<'mc> {
    Building {
        inner: CraftingBookCategoryStruct<'mc>,
    },
    Redstone {
        inner: CraftingBookCategoryStruct<'mc>,
    },
    Equipment {
        inner: CraftingBookCategoryStruct<'mc>,
    },
    Misc {
        inner: CraftingBookCategoryStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for CraftingBookCategory<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CraftingBookCategory::Building { .. } => f.write_str("BUILDING"),
            CraftingBookCategory::Redstone { .. } => f.write_str("REDSTONE"),
            CraftingBookCategory::Equipment { .. } => f.write_str("EQUIPMENT"),
            CraftingBookCategory::Misc { .. } => f.write_str("MISC"),
        }
    }
}

impl<'mc> CraftingBookCategory<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<CraftingBookCategory<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/inventory/recipe/CraftingBookCategory");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/inventory/recipe/CraftingBookCategory;",
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
            "BUILDING" => Ok(CraftingBookCategory::Building {
                inner: CraftingBookCategoryStruct::from_raw(env, obj)?,
            }),
            "REDSTONE" => Ok(CraftingBookCategory::Redstone {
                inner: CraftingBookCategoryStruct::from_raw(env, obj)?,
            }),
            "EQUIPMENT" => Ok(CraftingBookCategory::Equipment {
                inner: CraftingBookCategoryStruct::from_raw(env, obj)?,
            }),
            "MISC" => Ok(CraftingBookCategory::Misc {
                inner: CraftingBookCategoryStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct CraftingBookCategoryStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CraftingBookCategory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Building { inner } => inner.0.clone(),
            Self::Redstone { inner } => inner.0.clone(),
            Self::Equipment { inner } => inner.0.clone(),
            Self::Misc { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Building { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Redstone { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Equipment { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Misc { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CraftingBookCategory<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CraftingBookCategory from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/recipe/CraftingBookCategory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CraftingBookCategory object, got {}",
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
                "BUILDING" => Ok(CraftingBookCategory::Building {
                    inner: CraftingBookCategoryStruct::from_raw(env, obj)?,
                }),
                "REDSTONE" => Ok(CraftingBookCategory::Redstone {
                    inner: CraftingBookCategoryStruct::from_raw(env, obj)?,
                }),
                "EQUIPMENT" => Ok(CraftingBookCategory::Equipment {
                    inner: CraftingBookCategoryStruct::from_raw(env, obj)?,
                }),
                "MISC" => Ok(CraftingBookCategory::Misc {
                    inner: CraftingBookCategoryStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for CraftingBookCategoryStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CraftingBookCategoryStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CraftingBookCategoryStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/recipe/CraftingBookCategory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CraftingBookCategoryStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CraftingBookCategoryStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::inventory::recipe::CraftingBookCategory<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/inventory/recipe/CraftingBookCategory;");
        let cls = jni.find_class("org/bukkit/inventory/recipe/CraftingBookCategory");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::inventory::recipe::CraftingBookCategory::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum CookingBookCategory<'mc> {
    Food {
        inner: CookingBookCategoryStruct<'mc>,
    },
    Blocks {
        inner: CookingBookCategoryStruct<'mc>,
    },
    Misc {
        inner: CookingBookCategoryStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for CookingBookCategory<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CookingBookCategory::Food { .. } => f.write_str("FOOD"),
            CookingBookCategory::Blocks { .. } => f.write_str("BLOCKS"),
            CookingBookCategory::Misc { .. } => f.write_str("MISC"),
        }
    }
}

impl<'mc> CookingBookCategory<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<CookingBookCategory<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/inventory/recipe/CookingBookCategory");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/inventory/recipe/CookingBookCategory;",
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
            "FOOD" => Ok(CookingBookCategory::Food {
                inner: CookingBookCategoryStruct::from_raw(env, obj)?,
            }),
            "BLOCKS" => Ok(CookingBookCategory::Blocks {
                inner: CookingBookCategoryStruct::from_raw(env, obj)?,
            }),
            "MISC" => Ok(CookingBookCategory::Misc {
                inner: CookingBookCategoryStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct CookingBookCategoryStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CookingBookCategory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Food { inner } => inner.0.clone(),
            Self::Blocks { inner } => inner.0.clone(),
            Self::Misc { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Food { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Blocks { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Misc { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CookingBookCategory<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CookingBookCategory from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/recipe/CookingBookCategory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CookingBookCategory object, got {}",
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
                "FOOD" => Ok(CookingBookCategory::Food {
                    inner: CookingBookCategoryStruct::from_raw(env, obj)?,
                }),
                "BLOCKS" => Ok(CookingBookCategory::Blocks {
                    inner: CookingBookCategoryStruct::from_raw(env, obj)?,
                }),
                "MISC" => Ok(CookingBookCategory::Misc {
                    inner: CookingBookCategoryStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for CookingBookCategoryStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CookingBookCategoryStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CookingBookCategoryStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/recipe/CookingBookCategory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CookingBookCategoryStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CookingBookCategoryStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::inventory::recipe::CookingBookCategory<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/inventory/recipe/CookingBookCategory;");
        let cls = jni.find_class("org/bukkit/inventory/recipe/CookingBookCategory");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::inventory::recipe::CookingBookCategory::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
