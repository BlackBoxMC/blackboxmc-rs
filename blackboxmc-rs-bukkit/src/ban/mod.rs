#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// A <a href="../BanList.html" title="interface in org.bukkit"><code>BanList</code></a> targeting player profile bans.
///
/// This is a representation of an abstract class.
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
    pub fn add_ban_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<String>,
        arg2: impl Into<blackboxmc_java::util::JavaDate<'mc>>,
        arg3: impl Into<String>,
    ) -> Result<crate::BanEntry<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        sig += "Ljava/util/Date;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        args.push(val_3);
        sig += "Ljava/lang/String;";
        let val_4 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg3.into())?,
        ));
        args.push(val_4);
        sig += ")Lorg/bukkit/BanEntry;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addBan", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::BanEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_ban_entry_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<crate::BanEntry<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += ")Lorg/bukkit/BanEntry;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBanEntry", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::BanEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

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
    pub fn is_banned_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBanned", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn pardon_with_string(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "pardon", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
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
/// A <a href="../BanList.html" title="interface in org.bukkit"><code>BanList</code></a> targeting IP bans.
///
/// This is a representation of an abstract class.
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
