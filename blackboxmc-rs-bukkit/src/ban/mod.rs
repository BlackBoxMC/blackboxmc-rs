#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct ProfileBanList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ProfileBanList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ProfileBanList<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ProfileBanList from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/ban/ProfileBanList")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ProfileBanList object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ProfileBanList<'mc> {
    /// Adds a ban to this list. If a previous ban exists, this will
    /// update the previous entry.
    pub fn add_ban(
        &self,
        target: jni::objects::JObject<'mc>,
        reason: impl Into<String>,
        duration: jni::objects::JObject<'mc>,
        source: impl Into<String>,
    ) -> Result<Option<crate::BanEntry<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "LT;";
        let val_1 = jni::objects::JValueGen::Object(target);
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(reason.into())?,
        ));
        args.push(val_2);
        sig += "Ljava/time/Duration;";
        let val_3 = jni::objects::JValueGen::Object(duration);
        args.push(val_3);
        sig += "Ljava/lang/String;";
        let val_4 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(source.into())?,
        ));
        args.push(val_4);
        sig += ")Lorg/bukkit/BanEntry;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addBan", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::BanEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Gets a {@link BanEntry} by target.
    pub fn get_ban_entry(
        &self,
        target: jni::objects::JObject<'mc>,
    ) -> Result<Option<crate::BanEntry<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "LT;";
        let val_1 = jni::objects::JValueGen::Object(target);
        args.push(val_1);
        sig += ")Lorg/bukkit/BanEntry;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBanEntry", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::BanEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    #[deprecated]
    /// Gets a set containing every {@link BanEntry} in this list.
    pub fn ban_entries(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBanEntries", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a set containing every {@link BanEntry} in this list.
    pub fn entries(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntries", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// Gets if a {@link BanEntry} exists for the target, indicating an active ban status.
    pub fn is_banned(&self, target: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(target.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBanned", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    #[deprecated]
    /// Removes the specified target from this list, therefore indicating a "not banned" status.
    pub fn pardon(&self, target: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(target.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "pardon", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::BanList<'mc>> for ProfileBanList<'mc> {
    fn into(self) -> crate::BanList<'mc> {
        crate::BanList::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ProfileBanList into crate::BanList")
    }
}
#[repr(C)]
pub struct IpBanList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for IpBanList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for IpBanList<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate IpBanList from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/ban/IpBanList")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a IpBanList object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> IpBanList<'mc> {
    /// Gets a {@link BanEntry} by target.
    pub fn get_ban_entry(
        &self,
        target: jni::objects::JObject<'mc>,
    ) -> Result<Option<crate::BanEntry<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "LT;";
        let val_1 = jni::objects::JValueGen::Object(target);
        args.push(val_1);
        sig += ")Lorg/bukkit/BanEntry;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBanEntry", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::BanEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Adds a ban to this list. If a previous ban exists, this will
    /// update the previous entry.
    pub fn add_ban(
        &self,
        target: jni::objects::JObject<'mc>,
        reason: impl Into<String>,
        duration: jni::objects::JObject<'mc>,
        source: impl Into<String>,
    ) -> Result<Option<crate::BanEntry<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "LT;";
        let val_1 = jni::objects::JValueGen::Object(target);
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(reason.into())?,
        ));
        args.push(val_2);
        sig += "Ljava/time/Duration;";
        let val_3 = jni::objects::JValueGen::Object(duration);
        args.push(val_3);
        sig += "Ljava/lang/String;";
        let val_4 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(source.into())?,
        ));
        args.push(val_4);
        sig += ")Lorg/bukkit/BanEntry;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addBan", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::BanEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    #[deprecated]
    /// Gets a set containing every {@link BanEntry} in this list.
    pub fn ban_entries(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBanEntries", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a set containing every {@link BanEntry} in this list.
    pub fn entries(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntries", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// Gets if a {@link BanEntry} exists for the target, indicating an active ban status.
    pub fn is_banned(&self, target: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(target.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBanned", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    #[deprecated]
    /// Removes the specified target from this list, therefore indicating a "not banned" status.
    pub fn pardon(&self, target: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(target.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "pardon", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::BanList<'mc>> for IpBanList<'mc> {
    fn into(self) -> crate::BanList<'mc> {
        crate::BanList::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting IpBanList into crate::BanList")
    }
}
