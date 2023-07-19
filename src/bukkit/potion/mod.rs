#![allow(deprecated)]
use crate::JNIRaw;
pub struct PotionEffectTypeWrapper<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for PotionEffectTypeWrapper<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PotionEffectTypeWrapper<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PotionEffectTypeWrapper from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PotionEffectTypeWrapper") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionEffectTypeWrapper object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn color(&mut self) -> Result<crate::bukkit::Color<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/Color;",
            &[],
        )?;
        let ret = {
            crate::bukkit::Color(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_instant(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInstant", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn duration_modifier(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDurationModifier", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn get_type(
        &mut self,
    ) -> Result<crate::bukkit::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/potion/PotionEffectType;",
            &[],
        )?;
        let ret = {
            crate::bukkit::potion::PotionEffectType(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_by_key(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::NamespacedKey<'mc>>,
    ) -> Result<crate::bukkit::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/potion/PotionEffectType")?;
        let res = jni.call_static_method(
            cls,
            "getByKey",
            "(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/potion/PotionEffectType;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::potion::PotionEffectType(jni, obj)
        };
        Ok(ret)
    }
    pub fn create_effect(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<crate::bukkit::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createEffect",
            "(II)Lorg/bukkit/potion/PotionEffect;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::potion::PotionEffect(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn get_by_id(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<crate::bukkit::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let cls = &jni.find_class("org/bukkit/potion/PotionEffectType")?;
        let res = jni.call_static_method(
            cls,
            "getById",
            "(I)Lorg/bukkit/potion/PotionEffectType;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::potion::PotionEffectType(jni, obj)
        };
        Ok(ret)
    }
    pub fn register_potion_effect_type(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::potion::PotionEffectType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let cls = &jni.find_class("void")?;
        let _res = jni.call_static_method(
            cls,
            "registerPotionEffectType",
            "(Lorg/bukkit/potion/PotionEffectType;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn stop_accepting_registrations(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cls = &jni.find_class("void")?;
        let _res = jni.call_static_method(cls, "stopAcceptingRegistrations", "()V", &[])?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn key(&mut self) -> Result<crate::bukkit::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKey",
            "()Lorg/bukkit/NamespacedKey;",
            &[],
        )?;
        let ret = {
            crate::bukkit::NamespacedKey(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getId", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn get_by_name(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/potion/PotionEffectType")?;
        let res = jni.call_static_method(
            cls,
            "getByName",
            "(Ljava/lang/String;)Lorg/bukkit/potion/PotionEffectType;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::potion::PotionEffectType(jni, obj)
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct PotionData<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for PotionData<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PotionData<'mc> {
    pub fn new_with_potion_type(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::bukkit::potion::PotionType<'mc>>>,
        arg1: std::option::Option<bool>,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::bukkit::potion::PotionData<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        // 0
        let val_1 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        let cls = &jni.find_class("org/bukkit/potion/PotionData")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/potion/PotionType;ZZ)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = { crate::bukkit::potion::PotionData(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PotionData from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PotionData") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionData object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn is_upgraded(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUpgraded", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_extended(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isExtended", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn get_type(
        &mut self,
    ) -> Result<crate::bukkit::potion::PotionType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/potion/PotionType;",
            &[],
        )?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::potion::PotionType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::potion::PotionType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct Potion<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Potion<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Potion<'mc> {
    #[deprecated]
    pub fn new_with_potion_type(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::bukkit::potion::PotionType<'mc>>>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<bool>,
        arg3: std::option::Option<bool>,
    ) -> Result<crate::bukkit::potion::Potion<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        // 0
        let val_3 = jni::objects::JValueGen::Bool(arg3.unwrap().into());
        let cls = &jni.find_class("org/bukkit/potion/Potion")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/potion/PotionType;IZZ)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        let ret = { crate::bukkit::potion::Potion(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Potion from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Potion") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Potion object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_level(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLevel",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLevel", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_type(
        &mut self,
        arg0: impl Into<crate::bukkit::potion::PotionType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/potion/PotionType;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn to_item_stack(
        &mut self,
        arg0: i32,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn splash(
        &mut self,
    ) -> Result<crate::bukkit::potion::Potion<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "splash",
            "()Lorg/bukkit/potion/Potion;",
            &[],
        )?;
        let ret = {
            crate::bukkit::potion::Potion(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_splash(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setSplash",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn extend(
        &mut self,
    ) -> Result<crate::bukkit::potion::Potion<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "extend",
            "()Lorg/bukkit/potion/Potion;",
            &[],
        )?;
        let ret = {
            crate::bukkit::potion::Potion(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_has_extended_duration(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setHasExtendedDuration",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn brewer(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<crate::bukkit::potion::PotionBrewer<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/potion/PotionBrewer")?;
        let res =
            jni.call_static_method(cls, "getBrewer", "()Lorg/bukkit/potion/PotionBrewer;", &[])?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::potion::PotionBrewer(jni, obj)
        };
        Ok(ret)
    }
    pub fn has_extended_duration(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasExtendedDuration", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_splash(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSplash", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn to_damage_value(&mut self) -> Result<i16, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toDamageValue", "()S", &[])?;
        Ok(res.s().unwrap())
    }
    pub fn from_damage(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<crate::bukkit::potion::Potion<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let cls = &jni.find_class("org/bukkit/potion/Potion")?;
        let res = jni.call_static_method(
            cls,
            "fromDamage",
            "(I)Lorg/bukkit/potion/Potion;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::potion::Potion(jni, obj)
        };
        Ok(ret)
    }
    pub fn from_item_stack(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::inventory::ItemStack<'mc>>,
    ) -> Result<crate::bukkit::potion::Potion<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/potion/Potion")?;
        let res = jni.call_static_method(
            cls,
            "fromItemStack",
            "(Lorg/bukkit/inventory/ItemStack;)Lorg/bukkit/potion/Potion;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::potion::Potion(jni, obj)
        };
        Ok(ret)
    }
    pub fn set_potion_brewer(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::potion::PotionBrewer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let cls = &jni.find_class("void")?;
        let _res = jni.call_static_method(
            cls,
            "setPotionBrewer",
            "(Lorg/bukkit/potion/PotionBrewer;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    #[deprecated]
    pub fn name_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getNameId", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn apply_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<crate::bukkit::entity::LivingEntity<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "apply",
            "(Lorg/bukkit/entity/LivingEntity;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn get_type(
        &mut self,
    ) -> Result<crate::bukkit::potion::PotionType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/potion/PotionType;",
            &[],
        )?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::potion::PotionType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::potion::PotionType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct PotionEffect<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for PotionEffect<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PotionEffect<'mc> {
    pub fn new_with_map(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::bukkit::potion::PotionEffectType<'mc>>>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::bukkit::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let cls = &jni.find_class("org/bukkit/potion/PotionEffect")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/potion/PotionEffectType;II)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = { crate::bukkit::potion::PotionEffect(jni, res) };
        Ok(ret)
    }
    pub fn new_with_potion_effect_type(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::potion::PotionEffectType<'mc>>,
        arg1: i32,
        arg2: i32,
        arg3: bool,
        arg4: bool,
        arg5: std::option::Option<bool>,
    ) -> Result<crate::bukkit::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        let val_2 = jni::objects::JValueGen::Int(arg2.into());
        // 5
        let val_3 = jni::objects::JValueGen::Bool(arg3.into());
        // 5
        let val_4 = jni::objects::JValueGen::Bool(arg4.into());
        // 5
        let val_5 = jni::objects::JValueGen::Bool(arg5.unwrap().into());
        let cls = &jni.find_class("org/bukkit/potion/PotionEffect")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/potion/PotionEffectType;IIZZZ)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
            ],
        )?;
        let ret = { crate::bukkit::potion::PotionEffect(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PotionEffect from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PotionEffect") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionEffect object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    #[deprecated]
    pub fn color(&mut self) -> Result<crate::bukkit::Color<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/Color;",
            &[],
        )?;
        let ret = {
            crate::bukkit::Color(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn duration(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDuration", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn amplifier(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAmplifier", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn is_shorter_than(
        &mut self,
        arg0: impl Into<crate::bukkit::potion::PotionEffect<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isShorterThan",
            "(Lorg/bukkit/potion/PotionEffect;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_ambient(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isAmbient", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_particles(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasParticles", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_icon(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasIcon", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn apply(
        &mut self,
        arg0: impl Into<crate::bukkit::entity::LivingEntity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "apply",
            "(Lorg/bukkit/entity/LivingEntity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_type(
        &mut self,
    ) -> Result<crate::bukkit::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/potion/PotionEffectType;",
            &[],
        )?;
        let ret = {
            crate::bukkit::potion::PotionEffectType(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_infinite(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInfinite", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::configuration::serialization::ConfigurationSerializable<'mc>>
    for PotionEffect<'mc>
{
    fn into(self) -> crate::bukkit::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::bukkit::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            self.1,
        )
        .unwrap()
    }
}
pub struct PotionEffectType<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for PotionEffectType<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PotionEffectType<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PotionEffectType from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PotionEffectType") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionEffectType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn color(&mut self) -> Result<crate::bukkit::Color<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/Color;",
            &[],
        )?;
        let ret = {
            crate::bukkit::Color(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_by_key(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::NamespacedKey<'mc>>,
    ) -> Result<crate::bukkit::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/potion/PotionEffectType")?;
        let res = jni.call_static_method(
            cls,
            "getByKey",
            "(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/potion/PotionEffectType;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::potion::PotionEffectType(jni, obj)
        };
        Ok(ret)
    }
    pub fn create_effect(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<crate::bukkit::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createEffect",
            "(II)Lorg/bukkit/potion/PotionEffect;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::potion::PotionEffect(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_instant(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInstant", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn duration_modifier(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDurationModifier", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    #[deprecated]
    pub fn get_by_id(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<crate::bukkit::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let cls = &jni.find_class("org/bukkit/potion/PotionEffectType")?;
        let res = jni.call_static_method(
            cls,
            "getById",
            "(I)Lorg/bukkit/potion/PotionEffectType;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::potion::PotionEffectType(jni, obj)
        };
        Ok(ret)
    }
    pub fn register_potion_effect_type(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::potion::PotionEffectType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let cls = &jni.find_class("void")?;
        let _res = jni.call_static_method(
            cls,
            "registerPotionEffectType",
            "(Lorg/bukkit/potion/PotionEffectType;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn stop_accepting_registrations(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cls = &jni.find_class("void")?;
        let _res = jni.call_static_method(cls, "stopAcceptingRegistrations", "()V", &[])?;
        Ok(())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn key(&mut self) -> Result<crate::bukkit::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKey",
            "()Lorg/bukkit/NamespacedKey;",
            &[],
        )?;
        let ret = {
            crate::bukkit::NamespacedKey(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getId", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn get_by_name(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/potion/PotionEffectType")?;
        let res = jni.call_static_method(
            cls,
            "getByName",
            "(Ljava/lang/String;)Lorg/bukkit/potion/PotionEffectType;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::potion::PotionEffectType(jni, obj)
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::Keyed<'mc>> for PotionEffectType<'mc> {
    fn into(self) -> crate::bukkit::Keyed<'mc> {
        crate::bukkit::Keyed::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements PotionBrewer. Needed for returning it from Java.
pub struct PotionBrewer<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PotionBrewer<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PotionBrewer from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PotionBrewer") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionBrewer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn create_effect(
        &mut self,
        arg0: impl Into<crate::bukkit::potion::PotionEffectType<'mc>>,
        arg1: i32,
        arg2: i32,
    ) -> Result<crate::bukkit::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        let val_2 = jni::objects::JValueGen::Int(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createEffect",
            "(Lorg/bukkit/potion/PotionEffectType;II)Lorg/bukkit/potion/PotionEffect;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = {
            crate::bukkit::potion::PotionEffect(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for PotionBrewer<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub enum PotionTypeEnum {
    Uncraftable,
    Water,
    Mundane,
    Thick,
    Awkward,
    NightVision,
    Invisibility,
    Jump,
    FireResistance,
    Speed,
    Slowness,
    WaterBreathing,
    InstantHeal,
    InstantDamage,
    Poison,
    Regen,
    Strength,
    Weakness,
    Luck,
    TurtleMaster,
    SlowFalling,
}
impl std::fmt::Display for PotionTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PotionTypeEnum::Uncraftable => f.write_str("UNCRAFTABLE"),
            PotionTypeEnum::Water => f.write_str("WATER"),
            PotionTypeEnum::Mundane => f.write_str("MUNDANE"),
            PotionTypeEnum::Thick => f.write_str("THICK"),
            PotionTypeEnum::Awkward => f.write_str("AWKWARD"),
            PotionTypeEnum::NightVision => f.write_str("NIGHT_VISION"),
            PotionTypeEnum::Invisibility => f.write_str("INVISIBILITY"),
            PotionTypeEnum::Jump => f.write_str("JUMP"),
            PotionTypeEnum::FireResistance => f.write_str("FIRE_RESISTANCE"),
            PotionTypeEnum::Speed => f.write_str("SPEED"),
            PotionTypeEnum::Slowness => f.write_str("SLOWNESS"),
            PotionTypeEnum::WaterBreathing => f.write_str("WATER_BREATHING"),
            PotionTypeEnum::InstantHeal => f.write_str("INSTANT_HEAL"),
            PotionTypeEnum::InstantDamage => f.write_str("INSTANT_DAMAGE"),
            PotionTypeEnum::Poison => f.write_str("POISON"),
            PotionTypeEnum::Regen => f.write_str("REGEN"),
            PotionTypeEnum::Strength => f.write_str("STRENGTH"),
            PotionTypeEnum::Weakness => f.write_str("WEAKNESS"),
            PotionTypeEnum::Luck => f.write_str("LUCK"),
            PotionTypeEnum::TurtleMaster => f.write_str("TURTLE_MASTER"),
            PotionTypeEnum::SlowFalling => f.write_str("SLOW_FALLING"),
        }
    }
}
pub struct PotionType<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub PotionTypeEnum,
);
impl<'mc> std::ops::Deref for PotionType<'mc> {
    type Target = PotionTypeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for PotionType<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PotionType<'mc> {
    pub const UNCRAFTABLE: PotionTypeEnum = PotionTypeEnum::Uncraftable;
    pub const WATER: PotionTypeEnum = PotionTypeEnum::Water;
    pub const MUNDANE: PotionTypeEnum = PotionTypeEnum::Mundane;
    pub const THICK: PotionTypeEnum = PotionTypeEnum::Thick;
    pub const AWKWARD: PotionTypeEnum = PotionTypeEnum::Awkward;
    pub const NIGHT_VISION: PotionTypeEnum = PotionTypeEnum::NightVision;
    pub const INVISIBILITY: PotionTypeEnum = PotionTypeEnum::Invisibility;
    pub const JUMP: PotionTypeEnum = PotionTypeEnum::Jump;
    pub const FIRE_RESISTANCE: PotionTypeEnum = PotionTypeEnum::FireResistance;
    pub const SPEED: PotionTypeEnum = PotionTypeEnum::Speed;
    pub const SLOWNESS: PotionTypeEnum = PotionTypeEnum::Slowness;
    pub const WATER_BREATHING: PotionTypeEnum = PotionTypeEnum::WaterBreathing;
    pub const INSTANT_HEAL: PotionTypeEnum = PotionTypeEnum::InstantHeal;
    pub const INSTANT_DAMAGE: PotionTypeEnum = PotionTypeEnum::InstantDamage;
    pub const POISON: PotionTypeEnum = PotionTypeEnum::Poison;
    pub const REGEN: PotionTypeEnum = PotionTypeEnum::Regen;
    pub const STRENGTH: PotionTypeEnum = PotionTypeEnum::Strength;
    pub const WEAKNESS: PotionTypeEnum = PotionTypeEnum::Weakness;
    pub const LUCK: PotionTypeEnum = PotionTypeEnum::Luck;
    pub const TURTLE_MASTER: PotionTypeEnum = PotionTypeEnum::TurtleMaster;
    pub const SLOW_FALLING: PotionTypeEnum = PotionTypeEnum::SlowFalling;
    pub fn from_string(str: String) -> std::option::Option<PotionTypeEnum> {
        match str.as_str() {
            "UNCRAFTABLE" => Some(PotionTypeEnum::Uncraftable),
            "WATER" => Some(PotionTypeEnum::Water),
            "MUNDANE" => Some(PotionTypeEnum::Mundane),
            "THICK" => Some(PotionTypeEnum::Thick),
            "AWKWARD" => Some(PotionTypeEnum::Awkward),
            "NIGHT_VISION" => Some(PotionTypeEnum::NightVision),
            "INVISIBILITY" => Some(PotionTypeEnum::Invisibility),
            "JUMP" => Some(PotionTypeEnum::Jump),
            "FIRE_RESISTANCE" => Some(PotionTypeEnum::FireResistance),
            "SPEED" => Some(PotionTypeEnum::Speed),
            "SLOWNESS" => Some(PotionTypeEnum::Slowness),
            "WATER_BREATHING" => Some(PotionTypeEnum::WaterBreathing),
            "INSTANT_HEAL" => Some(PotionTypeEnum::InstantHeal),
            "INSTANT_DAMAGE" => Some(PotionTypeEnum::InstantDamage),
            "POISON" => Some(PotionTypeEnum::Poison),
            "REGEN" => Some(PotionTypeEnum::Regen),
            "STRENGTH" => Some(PotionTypeEnum::Strength),
            "WEAKNESS" => Some(PotionTypeEnum::Weakness),
            "LUCK" => Some(PotionTypeEnum::Luck),
            "TURTLE_MASTER" => Some(PotionTypeEnum::TurtleMaster),
            "SLOW_FALLING" => Some(PotionTypeEnum::SlowFalling),
            _ => None,
        }
    }
    pub fn is_upgradeable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUpgradeable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_extendable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isExtendable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn max_level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxLevel", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn is_instant(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInstant", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn effect_type(
        &mut self,
    ) -> Result<crate::bukkit::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectType",
            "()Lorg/bukkit/potion/PotionEffectType;",
            &[],
        )?;
        let ret = {
            crate::bukkit::potion::PotionEffectType(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn get_by_effect(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::potion::PotionEffectType<'mc>>,
    ) -> Result<crate::bukkit::potion::PotionType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/potion/PotionType")?;
        let res = jni.call_static_method(
            cls,
            "getByEffect",
            "(Lorg/bukkit/potion/PotionEffectType;)Lorg/bukkit/potion/PotionType;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::potion::PotionType(
                jni,
                raw_obj,
                crate::bukkit::potion::PotionType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn value_of(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::potion::PotionType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/potion/PotionType")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/potion/PotionType;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::potion::PotionType(
                jni,
                raw_obj,
                crate::bukkit::potion::PotionType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
}
