#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct AdvancementDisplay<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for AdvancementDisplay<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AdvancementDisplay<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AdvancementDisplay from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/advancement/AdvancementDisplay")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AdvancementDisplay object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AdvancementDisplay<'mc> {
    pub fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()LString;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn description(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()LString;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDescription", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn icon(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::inventory::ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getIcon", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn should_show_toast(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Lbool;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "shouldShowToast", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn should_announce_chat(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Lbool;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldAnnounceChat",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn is_hidden(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Lbool;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isHidden", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn x(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()Lf32;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    pub fn y(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()Lf32;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::advancement::AdvancementDisplayType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::advancement::AdvancementDisplayType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::advancement::AdvancementDisplayType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct AdvancementProgress<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for AdvancementProgress<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AdvancementProgress<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AdvancementProgress from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/advancement/AdvancementProgress")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AdvancementProgress object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AdvancementProgress<'mc> {
    pub fn advancement(
        &self,
    ) -> Result<crate::advancement::Advancement<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::advancement::Advancement;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAdvancement", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::advancement::Advancement::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn is_done(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Lbool;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn award_criteria(
        &self,
        criteria: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lbool;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(criteria.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "awardCriteria",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn revoke_criteria(
        &self,
        criteria: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lbool;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(criteria.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "revokeCriteria",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn get_date_awarded(
        &self,
        criteria: impl Into<String>,
    ) -> Result<Option<blackboxmc_java::util::JavaDate<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lblackboxmc_java::util::Date;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(criteria.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDateAwarded",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(blackboxmc_java::util::JavaDate::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    pub fn remaining_criteria(
        &self,
    ) -> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()LVec;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRemainingCriteria",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(obj);
        }
        Ok(new_vec)
    }
    pub fn awarded_criteria(
        &self,
    ) -> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()LVec;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAwardedCriteria",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(obj);
        }
        Ok(new_vec)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct Advancement<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Advancement<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Advancement<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Advancement from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/advancement/Advancement")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Advancement object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Advancement<'mc> {
    pub fn criteria(&self) -> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()LVec;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCriteria", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(obj);
        }
        Ok(new_vec)
    }
    pub fn display(
        &self,
    ) -> Result<Option<crate::advancement::AdvancementDisplay<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lcrate::advancement::AdvancementDisplay;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplay", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::advancement::AdvancementDisplay::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/NamespacedKey;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKey", sig.as_str(), args);
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
impl<'mc> Into<crate::Keyed<'mc>> for Advancement<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Advancement into crate::Keyed")
    }
}
pub enum AdvancementDisplayType<'mc> {
    Task {
        inner: AdvancementDisplayTypeStruct<'mc>,
    },
    Challenge {
        inner: AdvancementDisplayTypeStruct<'mc>,
    },
    Goal {
        inner: AdvancementDisplayTypeStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for AdvancementDisplayType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdvancementDisplayType::Task { .. } => f.write_str("TASK"),
            AdvancementDisplayType::Challenge { .. } => f.write_str("CHALLENGE"),
            AdvancementDisplayType::Goal { .. } => f.write_str("GOAL"),
        }
    }
}

impl<'mc> AdvancementDisplayType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<AdvancementDisplayType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/advancement/AdvancementDisplayType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/advancement/AdvancementDisplayType;",
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
            "TASK" => Ok(AdvancementDisplayType::Task {
                inner: AdvancementDisplayTypeStruct::from_raw(env, obj)?,
            }),
            "CHALLENGE" => Ok(AdvancementDisplayType::Challenge {
                inner: AdvancementDisplayTypeStruct::from_raw(env, obj)?,
            }),
            "GOAL" => Ok(AdvancementDisplayType::Goal {
                inner: AdvancementDisplayTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct AdvancementDisplayTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for AdvancementDisplayType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Task { inner } => inner.0.clone(),
            Self::Challenge { inner } => inner.0.clone(),
            Self::Goal { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Task { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Challenge { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Goal { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AdvancementDisplayType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate AdvancementDisplayType from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/advancement/AdvancementDisplayType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AdvancementDisplayType object, got {}",
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
                "TASK" => Ok(AdvancementDisplayType::Task {
                    inner: AdvancementDisplayTypeStruct::from_raw(env, obj)?,
                }),
                "CHALLENGE" => Ok(AdvancementDisplayType::Challenge {
                    inner: AdvancementDisplayTypeStruct::from_raw(env, obj)?,
                }),
                "GOAL" => Ok(AdvancementDisplayType::Goal {
                    inner: AdvancementDisplayTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for AdvancementDisplayTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AdvancementDisplayTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate AdvancementDisplayTypeStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/advancement/AdvancementDisplayType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AdvancementDisplayTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AdvancementDisplayTypeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::advancement::AdvancementDisplayType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::advancement::AdvancementDisplayType;");
        let cls = jni.find_class("org/bukkit/advancement/AdvancementDisplayType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::advancement::AdvancementDisplayType::from_raw(&jni, obj)
    }
    pub fn color(&self) -> Result<crate::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::ChatColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::ChatColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
