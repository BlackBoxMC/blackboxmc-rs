#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// The individual status of an advancement for a player. This class is not reference safe as the underlying advancement may be reloaded.
///
/// This is a representation of an abstract class.
pub struct AdvancementProgress<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
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
    /// Check if all criteria for this advancement have been met.
    pub fn is_done(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDone", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// The advancement this progress is concerning.
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
    /// Mark the specified criteria as awarded at the current time.
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
    /// Mark the specified criteria as uncompleted.
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
    /// Get the date the specified criteria was awarded.
    pub fn get_date_awarded(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<blackboxmc_java::JavaDate<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDateAwarded",
            "(Ljava/lang/String;)Ljava/util/Date;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaDate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the criteria which have not been awarded.
    pub fn remaining_criteria(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRemainingCriteria",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Gets the criteria which have been awarded.
    pub fn awarded_criteria(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAwardedCriteria",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
}
impl<'mc> JNIRaw<'mc> for AdvancementProgress<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
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
pub struct AdvancementDisplayType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub AdvancementDisplayTypeEnum,
);
impl<'mc> std::ops::Deref for AdvancementDisplayType<'mc> {
    type Target = AdvancementDisplayTypeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for AdvancementDisplayType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
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
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/advancement/AdvancementDisplayType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AdvancementDisplayType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
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
/// Holds information about how the advancement is displayed by the game.
///
/// This is a representation of an abstract class.
pub struct AdvancementDisplay<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
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
    /// Gets the visible description of the advancement.
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
    /// The X position of the advancement in the advancement screen.
    pub fn x(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    /// The Y position of the advancement in the advancement screen.
    pub fn y(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    /// Gets the title of the advancement.
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
    /// The icon that is used for this advancement.
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
    /// Whether to show a toast to the player when this advancement has been completed.
    pub fn should_show_toast(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "shouldShowToast", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Whether to announce in the chat when this advancement has been completed.
    pub fn should_announce_chat(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "shouldAnnounceChat", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Whether to hide this advancement and all its children from the advancement screen until this advancement have been completed. Has no effect on root advancements themselves, but still affects all their children.
    pub fn is_hidden(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isHidden", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// The display type of this advancement.
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
}
impl<'mc> JNIRaw<'mc> for AdvancementDisplay<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents an advancement that may be awarded to a player. This class is not reference safe as the underlying advancement may be reloaded.
///
/// This is a representation of an abstract class.
pub struct Advancement<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Advancement<'mc> {
    pub fn from_raw(
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
    /// Get all the criteria present in this advancement.
    pub fn criteria(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCriteria",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Returns the display information for this advancement. This includes it's name, description and other visible tags.
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
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for Advancement<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
