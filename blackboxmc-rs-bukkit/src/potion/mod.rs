#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;

pub struct PotionEffectTypeWrapper<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PotionEffectTypeWrapper<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PotionEffectTypeWrapper<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PotionEffectTypeWrapper from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionEffectTypeWrapper")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionEffectTypeWrapper object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn color(&mut self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getColor", "()Lorg/bukkit/Color;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_instant(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInstant", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn duration_modifier(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDurationModifier", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn get_type(
        &mut self,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/potion/PotionEffectType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffectType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_by_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByKey",
            "(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/potion/PotionEffectType;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::PotionEffectType::from_raw(&jni, obj)
    }
    //@NotNull

    pub fn create_effect(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createEffect",
            "(II)Lorg/bukkit/potion/PotionEffect;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffect::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //@Deprecated

    #[deprecated]
    //@Nullable

    pub fn get_by_id(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<Option<crate::potion::PotionEffectType<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getById",
            "(I)Lorg/bukkit/potion/PotionEffectType;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        let obj = res.l()?;
        Ok(Some(crate::potion::PotionEffectType::from_raw(&jni, obj)?))
    }
    //

    pub fn register_potion_effect_type(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::potion::PotionEffectType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "registerPotionEffectType",
            "(Lorg/bukkit/potion/PotionEffectType;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(())
    }
    //

    pub fn stop_accepting_registrations(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "stopAcceptingRegistrations", "()V", &[]);
        let res = jni.translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

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
    //

    pub fn id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getId", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn get_by_name(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByName",
            "(Ljava/lang/String;)Lorg/bukkit/potion/PotionEffectType;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::PotionEffectType::from_raw(&jni, obj)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::potion::PotionEffectType<'mc>> for PotionEffectTypeWrapper<'mc> {
    fn into(self) -> crate::potion::PotionEffectType<'mc> {
        crate::potion::PotionEffectType::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PotionEffectTypeWrapper into crate::potion::PotionEffectType")
    }
}

pub struct PotionData<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PotionData<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PotionData<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PotionData from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionData")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionData object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_potion_type(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::potion::PotionType<'mc>>>,
        arg1: std::option::Option<bool>,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::potion::PotionData<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe {
            jni::objects::JObject::from_raw(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        // 0
        let val_3 = jni::objects::JValueGen::Bool(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let cls = jni.find_class("org/bukkit/potion/PotionData");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/potion/PotionType;ZZ)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::potion::PotionData::from_raw(&jni, res)
    }
    //

    pub fn is_upgraded(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUpgraded", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_extended(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isExtended", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn get_type(
        &mut self,
    ) -> Result<crate::potion::PotionType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/potion/PotionType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::potion::PotionType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::potion::PotionType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Potion Adapter for pre-1.9 data values see @PotionMeta for 1.9+
pub struct Potion<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Potion<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Potion<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Potion from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/Potion")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Potion object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_potion_type(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::potion::PotionType<'mc>>>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<bool>,
        arg3: std::option::Option<bool>,
    ) -> Result<crate::potion::Potion<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe {
            jni::objects::JObject::from_raw(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        // 0
        let val_3 = jni::objects::JValueGen::Bool(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        // 0
        let val_4 = jni::objects::JValueGen::Bool(
            arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let cls = jni.find_class("org/bukkit/potion/Potion");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/potion/PotionType;IZZ)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::potion::Potion::from_raw(&jni, res)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Sets the level of this potion.
    pub fn set_level(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLevel",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLevel", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_type(
        &mut self,
        arg0: impl Into<crate::potion::PotionType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/potion/PotionType;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn effects(
        &mut self,
    ) -> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffects",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.0, col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::potion::PotionEffect::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Converts this potion to an <a title="class in org.bukkit.inventory" href="../inventory/ItemStack.html"><code>ItemStack</code></a> with the specified amount and a correct damage value.
    pub fn to_item_stack(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn splash(&mut self) -> Result<crate::potion::Potion<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "splash",
            "()Lorg/bukkit/potion/Potion;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::Potion::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Sets whether this potion is a splash potion. Splash potions can be thrown for a radius effect.
    pub fn set_splash(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSplash",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn extend(&mut self) -> Result<crate::potion::Potion<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "extend",
            "()Lorg/bukkit/potion/Potion;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::Potion::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set whether this potion has extended duration. This will cause the potion to have roughly 8/3 more duration than a regular potion.
    pub fn set_has_extended_duration(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHasExtendedDuration",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn brewer(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::potion::PotionBrewer<'mc>, Box<dyn std::error::Error>> {
        let cls = jni.find_class("org/bukkit/potion/PotionBrewer");
        let cls = jni.translate_error_with_class(cls)?;
        let res =
            jni.call_static_method(cls, "getBrewer", "()Lorg/bukkit/potion/PotionBrewer;", &[]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::PotionBrewer::from_raw(&jni, obj)
    }
    //

    pub fn has_extended_duration(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasExtendedDuration", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_splash(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSplash", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_damage_value(&mut self) -> Result<i16, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toDamageValue", "()S", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s()?)
    }
    //@NotNull

    /// <span class="deprecated-label">Deprecated.</span>
    /// Gets the potion from its damage value.
    pub fn from_damage(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<crate::potion::Potion<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let cls = jni.find_class("org/bukkit/potion/Potion");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "fromDamage",
            "(I)Lorg/bukkit/potion/Potion;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::Potion::from_raw(&jni, obj)
    }
    //

    pub fn from_item_stack(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::potion::Potion<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = jni.find_class("org/bukkit/potion/Potion");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "fromItemStack",
            "(Lorg/bukkit/inventory/ItemStack;)Lorg/bukkit/potion/Potion;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::Potion::from_raw(&jni, obj)
    }
    //

    pub fn set_potion_brewer(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::potion::PotionBrewer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "setPotionBrewer",
            "(Lorg/bukkit/potion/PotionBrewer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(())
    }
    //

    pub fn name_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getNameId", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn apply_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<crate::entity::LivingEntity<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe {
            jni::objects::JObject::from_raw(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "apply",
            "(Lorg/bukkit/entity/LivingEntity;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get_type(
        &mut self,
    ) -> Result<crate::potion::PotionType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/potion/PotionType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::potion::PotionType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::potion::PotionType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents a potion effect, that can be added to a <a title="interface in org.bukkit.entity" href="../entity/LivingEntity.html"><code>LivingEntity</code></a>. A potion effect has a duration that it will last for, an amplifier that will enhance its effects, and a <a href="PotionEffectType.html" title="class in org.bukkit.potion"><code>PotionEffectType</code></a>, that represents its effect on an entity.
pub struct PotionEffect<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PotionEffect<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PotionEffect<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PotionEffect from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionEffect")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionEffect object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_map(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::potion::PotionEffectType<'mc>>>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe {
            jni::objects::JObject::from_raw(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let cls = jni.find_class("org/bukkit/potion/PotionEffect");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/potion/PotionEffectType;II)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::potion::PotionEffect::from_raw(&jni, res)
    }
    pub fn new_with_potion_effect_type(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::potion::PotionEffectType<'mc>>,
        arg1: i32,
        arg2: i32,
        arg3: bool,
        arg4: bool,
        arg5: std::option::Option<bool>,
    ) -> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        // 5
        let val_4 = jni::objects::JValueGen::Bool(arg3.into());
        // 5
        let val_5 = jni::objects::JValueGen::Bool(arg4.into());
        // 5
        let val_6 = jni::objects::JValueGen::Bool(
            arg5.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let cls = jni.find_class("org/bukkit/potion/PotionEffect");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/potion/PotionEffectType;IIZZZ)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
                jni::objects::JValueGen::from(&val_6),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::potion::PotionEffect::from_raw(&jni, res)
    }
    //

    pub fn serialize(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "serialize", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn color(&mut self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getColor", "()Lorg/bukkit/Color;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn duration(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDuration", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn amplifier(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAmplifier", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn is_shorter_than(
        &mut self,
        arg0: impl Into<crate::potion::PotionEffect<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isShorterThan",
            "(Lorg/bukkit/potion/PotionEffect;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_ambient(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isAmbient", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn has_particles(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasParticles", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn has_icon(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasIcon", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn apply(
        &mut self,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "apply",
            "(Lorg/bukkit/entity/LivingEntity;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get_type(
        &mut self,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/potion/PotionEffectType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffectType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_infinite(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInfinite", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for PotionEffect<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting PotionEffect into crate::configuration::serialization::ConfigurationSerializable")
    }
}
/// Represents a type of potion and its effect on an entity.
pub struct PotionEffectType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PotionEffectType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PotionEffectType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PotionEffectType from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionEffectType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionEffectType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn color(&mut self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getColor", "()Lorg/bukkit/Color;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_by_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByKey",
            "(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/potion/PotionEffectType;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::PotionEffectType::from_raw(&jni, obj)
    }
    //@NotNull

    /// Creates a PotionEffect from this PotionEffectType, applying duration modifiers and checks.
    pub fn create_effect(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createEffect",
            "(II)Lorg/bukkit/potion/PotionEffect;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffect::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_instant(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInstant", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn duration_modifier(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDurationModifier", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //@Deprecated

    #[deprecated]
    //@Nullable

    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Gets the effect type specified by the unique id.
    pub fn get_by_id(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<Option<crate::potion::PotionEffectType<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getById",
            "(I)Lorg/bukkit/potion/PotionEffectType;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        let obj = res.l()?;
        Ok(Some(crate::potion::PotionEffectType::from_raw(&jni, obj)?))
    }
    //

    pub fn register_potion_effect_type(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::potion::PotionEffectType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "registerPotionEffectType",
            "(Lorg/bukkit/potion/PotionEffectType;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(())
    }
    //

    pub fn stop_accepting_registrations(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "stopAcceptingRegistrations", "()V", &[]);
        let res = jni.translate_error(res)?;
        Ok(())
    }
    //

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

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
    //

    pub fn id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getId", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn get_by_name(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByName",
            "(Ljava/lang/String;)Lorg/bukkit/potion/PotionEffectType;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::PotionEffectType::from_raw(&jni, obj)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for PotionEffectType<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PotionEffectType into crate::Keyed")
    }
}
/// Represents a brewer that can create <a title="class in org.bukkit.potion" href="PotionEffect.html"><code>PotionEffect</code></a>s.
///
/// This is a representation of an abstract class.
pub struct PotionBrewer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PotionBrewer<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PotionBrewer from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionBrewer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionBrewer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn get_effects(
        &mut self,
        arg0: impl Into<crate::potion::PotionType<'mc>>,
        arg1: bool,
        arg2: bool,
    ) -> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffects",
            "(Lorg/bukkit/potion/PotionType;ZZ)Ljava/util/Collection;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn create_effect(
        &mut self,
        arg0: impl Into<crate::potion::PotionEffectType<'mc>>,
        arg1: i32,
        arg2: i32,
    ) -> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createEffect",
            "(Lorg/bukkit/potion/PotionEffectType;II)Lorg/bukkit/potion/PotionEffect;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffect::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //@Deprecated

    #[deprecated]
    //@NotNull

    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Non-Functional
    /// </div>
    /// Non-Functional
    ///
    /// Returns a collection of <a href="PotionEffect.html" title="class in org.bukkit.potion"><code>PotionEffect</code></a> that would be applied from a potion with the given data value.
    pub fn get_effects_from_damage(
        &mut self,
        arg0: i32,
    ) -> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectsFromDamage",
            "(I)Ljava/util/Collection;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
}
impl<'mc> JNIRaw<'mc> for PotionBrewer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
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
        match self {
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
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
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
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PotionType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: PotionTypeEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PotionType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
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

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<PotionType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/potion/PotionType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/potion/PotionType;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        PotionType::from_raw(
            &jni,
            raw_obj,
            PotionType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
