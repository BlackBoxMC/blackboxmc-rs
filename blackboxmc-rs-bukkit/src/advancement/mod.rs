#![allow(deprecated)]
#![feature(anonymous_lifetime_in_impl_trait)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// An instantiatable struct that implements AdvancementProgress. Needed for returning it from Java.
pub struct AdvancementProgress<'mc> {
    pub(crate) env: blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) obj: jni::objects::JObject<'mc>,
}
impl<'mc> AdvancementProgress<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AdvancementProgress from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "AdvancementProgress")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AdvancementProgress object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self {
                env: env.clone(),
                obj: obj,
            })
        }
    }
    pub fn is_done(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDone", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn advancement(
        &mut self,
    ) -> Result<crate::advancement::Advancement<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAdvancement",
            "()Lorg/bukkit/advancement/Advancement;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::advancement::Advancement::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn award_criteria(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "awardCriteria",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn revoke_criteria(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "revokeCriteria",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for AdvancementProgress<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.env.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.obj.clone()) }
    }
}
pub enum AdvancementDisplayTypeEnum {
    Task,
    Challenge,
    Goal,
}
impl std::fmt::Display for AdvancementDisplayTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AdvancementDisplayTypeEnum::Task => f.write_str("TASK"),
            AdvancementDisplayTypeEnum::Challenge => f.write_str("CHALLENGE"),
            AdvancementDisplayTypeEnum::Goal => f.write_str("GOAL"),
        }
    }
}
pub struct AdvancementDisplayType<'mc> {
    pub(crate) env: blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) obj: jni::objects::JObject<'mc>,
    pub enu: AdvancementDisplayTypeEnum,
}
impl<'mc> std::ops::Deref for AdvancementDisplayType<'mc> {
    type Target = AdvancementDisplayTypeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.enu;
    }
}
impl<'mc> JNIRaw<'mc> for AdvancementDisplayType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.env.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.obj.clone()) }
    }
}
impl<'mc> AdvancementDisplayType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: AdvancementDisplayTypeEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate AdvancementDisplayType from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "AdvancementDisplayType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AdvancementDisplayType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self {
                env: env.clone(),
                obj: obj,
                enu: e,
            })
        }
    }
    pub const TASK: AdvancementDisplayTypeEnum = AdvancementDisplayTypeEnum::Task;
    pub const CHALLENGE: AdvancementDisplayTypeEnum = AdvancementDisplayTypeEnum::Challenge;
    pub const GOAL: AdvancementDisplayTypeEnum = AdvancementDisplayTypeEnum::Goal;
    pub fn from_string(str: String) -> std::option::Option<AdvancementDisplayTypeEnum> {
        match str.as_str() {
            "TASK" => Some(AdvancementDisplayTypeEnum::Task),
            "CHALLENGE" => Some(AdvancementDisplayTypeEnum::Challenge),
            "GOAL" => Some(AdvancementDisplayTypeEnum::Goal),
            _ => None,
        }
    }
}
/// An instantiatable struct that implements AdvancementDisplay. Needed for returning it from Java.
pub struct AdvancementDisplay<'mc> {
    pub(crate) env: blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) obj: jni::objects::JObject<'mc>,
}
impl<'mc> AdvancementDisplay<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AdvancementDisplay from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "AdvancementDisplay")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AdvancementDisplay object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self {
                env: env.clone(),
                obj: obj,
            })
        }
    }
    pub fn is_hidden(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isHidden", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn get_type(
        &mut self,
    ) -> Result<crate::advancement::AdvancementDisplayType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/advancement/AdvancementDisplayType;",
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
        crate::advancement::AdvancementDisplayType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::advancement::AdvancementDisplayType::from_string(variant_str).unwrap(),
        )
    }
    pub fn x(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    pub fn y(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    pub fn description(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDescription",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn title(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTitle", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn icon(&mut self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getIcon",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn should_show_toast(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "shouldShowToast", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn should_announce_chat(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "shouldAnnounceChat", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for AdvancementDisplay<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.env.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.obj.clone()) }
    }
}
/// An instantiatable struct that implements Advancement. Needed for returning it from Java.
pub struct Advancement<'mc> {
    pub(crate) env: blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) obj: jni::objects::JObject<'mc>,
}
impl<'mc> Advancement<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Advancement from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Advancement")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Advancement object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self {
                env: env.clone(),
                obj: obj,
            })
        }
    }
    pub fn display(
        &mut self,
    ) -> Result<crate::advancement::AdvancementDisplay<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplay",
            "()Lorg/bukkit/advancement/AdvancementDisplay;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::advancement::AdvancementDisplay::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn key(&mut self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKey",
            "()Lorg/bukkit/NamespacedKey;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::NamespacedKey::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for Advancement<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.env.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.obj.clone()) }
    }
}
