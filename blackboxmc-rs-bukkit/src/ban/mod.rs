#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// A <a href="../BanList.html" title="interface in org.bukkit"><code>BanList</code></a> targeting player profile bans.
///
/// This is a representation of an abstract class.
pub struct ProfileBanList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ProfileBanList<'mc> {
    pub fn from_raw(
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
    //

    pub fn add_ban_with_player_profile(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<String>,
        arg2: impl Into<blackboxmc_java::JavaDate<'mc>>,
        arg3: std::option::Option<impl Into<String>>,
    ) -> Result<crate::BanEntry<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into())?);
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = jni::objects::JObject::from(
            self.jni_ref().new_string(
                arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into(),
            )?,
        );
        let res = self.jni_ref().call_method(&self.jni_object(),"addBan","(Ljava/lang/Object;Ljava/lang/String;Ljava/util/Date;Ljava/lang/String;)Lorg/bukkit/BanEntry;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::BanEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_ban_with_object(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<String>,
        arg2: jni::objects::JObject<'mc>,
        arg3: std::option::Option<impl Into<String>>,
    ) -> Result<crate::BanEntry<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into())?);
        let val_3 = arg2;
        let val_4 = jni::objects::JObject::from(
            self.jni_ref().new_string(
                arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into(),
            )?,
        );
        let res = self.jni_ref().call_method(&self.jni_object(),"addBan","(Ljava/lang/Object;Ljava/lang/String;Ljava/time/Duration;Ljava/lang/String;)Lorg/bukkit/BanEntry;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::BanEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_ban_with_string(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
        arg2: impl Into<blackboxmc_java::JavaDate<'mc>>,
        arg3: std::option::Option<impl Into<String>>,
    ) -> Result<crate::BanEntry<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into())?);
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = jni::objects::JObject::from(
            self.jni_ref().new_string(
                arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into(),
            )?,
        );
        let res = self.jni_ref().call_method(&self.jni_object(),"addBan","(Ljava/lang/String;Ljava/lang/String;Ljava/util/Date;Ljava/lang/String;)Lorg/bukkit/BanEntry;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::BanEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_ban_entry_with_string(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::BanEntry<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBanEntry",
            "(Ljava/lang/Object;)Lorg/bukkit/BanEntry;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::BanEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn ban_entries(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBanEntries",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_banned_with_string(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBanned",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn pardon_with_object(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(
            self.jni_ref().new_string(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into(),
            )?,
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pardon",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn entries(&mut self) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntries", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for ProfileBanList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
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
pub struct IpBanList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> IpBanList<'mc> {
    pub fn from_raw(
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
    //

    pub fn get_ban_entry_with_string(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::BanEntry<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBanEntry",
            "(Ljava/lang/Object;)Lorg/bukkit/BanEntry;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::BanEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_ban_with_object(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
        arg2: impl Into<blackboxmc_java::JavaDate<'mc>>,
        arg3: std::option::Option<impl Into<String>>,
    ) -> Result<crate::BanEntry<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into())?);
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = jni::objects::JObject::from(
            self.jni_ref().new_string(
                arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into(),
            )?,
        );
        let res = self.jni_ref().call_method(&self.jni_object(),"addBan","(Ljava/lang/String;Ljava/lang/String;Ljava/util/Date;Ljava/lang/String;)Lorg/bukkit/BanEntry;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::BanEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn ban_entries(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBanEntries",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_banned_with_string(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBanned",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn pardon_with_object(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(
            self.jni_ref().new_string(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into(),
            )?,
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pardon",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn entries(&mut self) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntries", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for IpBanList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::BanList<'mc>> for IpBanList<'mc> {
    fn into(self) -> crate::BanList<'mc> {
        crate::BanList::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting IpBanList into crate::BanList")
    }
}
