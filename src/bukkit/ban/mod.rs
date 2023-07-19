#![allow(deprecated)]
use crate::JNIRaw;
/// An instantiatable struct that implements ProfileBanList. Needed for returning it from Java.
pub struct ProfileBanList<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ProfileBanList<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ProfileBanList from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ProfileBanList") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ProfileBanList object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn add_ban_with_player_profile(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: String,
        arg2: jni::objects::JObject<'mc>,
        arg3: std::option::Option<String>,
    ) -> Result<crate::bukkit::BanEntry<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1).unwrap());
        let val_2 = arg2;
        let val_3 = jni::objects::JObject::from(self.jni_ref().new_string(arg3.unwrap()).unwrap());
        let res =
self.jni_ref().call_method(&self.jni_object(),"addBan","(Ljava/lang/Object;Ljava/lang/String;Ljava/util/Date;Ljava/lang/String;)Lorg/bukkit/BanEntry;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        let ret = {
            crate::bukkit::BanEntry(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn add_ban_with_object(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: String,
        arg2: jni::objects::JObject<'mc>,
        arg3: std::option::Option<String>,
    ) -> Result<crate::bukkit::BanEntry<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1).unwrap());
        let val_2 = arg2;
        let val_3 = jni::objects::JObject::from(self.jni_ref().new_string(arg3.unwrap()).unwrap());
        let res =
self.jni_ref().call_method(&self.jni_object(),"addBan","(Ljava/lang/Object;Ljava/lang/String;Ljava/time/Duration;Ljava/lang/String;)Lorg/bukkit/BanEntry;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        let ret = {
            crate::bukkit::BanEntry(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn add_ban_with_string(
        &mut self,
        arg0: String,
        arg1: String,
        arg2: jni::objects::JObject<'mc>,
        arg3: std::option::Option<String>,
    ) -> Result<crate::bukkit::BanEntry<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1).unwrap());
        let val_2 = arg2;
        let val_3 = jni::objects::JObject::from(self.jni_ref().new_string(arg3.unwrap()).unwrap());
        let res =
self.jni_ref().call_method(&self.jni_object(),"addBan","(Ljava/lang/String;Ljava/lang/String;Ljava/util/Date;Ljava/lang/String;)Lorg/bukkit/BanEntry;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        let ret = {
            crate::bukkit::BanEntry(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_ban_entry_with_string(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::bukkit::BanEntry<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBanEntry",
            "(Ljava/lang/Object;)Lorg/bukkit/BanEntry;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::BanEntry(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_banned_with_string(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBanned",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn pardon_with_object(
        &mut self,
        arg0: std::option::Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "pardon",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for ProfileBanList<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::BanList<'mc>> for ProfileBanList<'mc> {
    fn into(self) -> crate::bukkit::BanList<'mc> {
        crate::bukkit::BanList::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements IpBanList. Needed for returning it from Java.
pub struct IpBanList<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> IpBanList<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate IpBanList from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("IpBanList") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a IpBanList object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn get_ban_entry_with_string(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::bukkit::BanEntry<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBanEntry",
            "(Ljava/lang/Object;)Lorg/bukkit/BanEntry;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::BanEntry(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn add_ban_with_object(
        &mut self,
        arg0: String,
        arg1: String,
        arg2: jni::objects::JObject<'mc>,
        arg3: std::option::Option<String>,
    ) -> Result<crate::bukkit::BanEntry<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1).unwrap());
        let val_2 = arg2;
        let val_3 = jni::objects::JObject::from(self.jni_ref().new_string(arg3.unwrap()).unwrap());
        let res =
self.jni_ref().call_method(&self.jni_object(),"addBan","(Ljava/lang/String;Ljava/lang/String;Ljava/util/Date;Ljava/lang/String;)Lorg/bukkit/BanEntry;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        let ret = {
            crate::bukkit::BanEntry(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_banned_with_string(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBanned",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn pardon_with_object(
        &mut self,
        arg0: std::option::Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "pardon",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for IpBanList<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::BanList<'mc>> for IpBanList<'mc> {
    fn into(self) -> crate::bukkit::BanList<'mc> {
        crate::bukkit::BanList::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
