#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIInstantiatableEnum;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;

pub struct PotionEffectTypeWrapper<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PotionEffectTypeWrapper<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for PotionEffectTypeWrapper<'mc> {
    fn from_raw(
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
}

impl<'mc> PotionEffectTypeWrapper<'mc> {
    //

    pub fn color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_instant(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInstant", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn duration_modifier(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDurationModifier",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn get_type(
        &self,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
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
        let sig = String::from("(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/potion/PotionEffectType;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::PotionEffectType::from_raw(&jni, obj)
    }
    //@NotNull

    pub fn create_effect(
        &self,
        arg0: i32,
        arg1: i32,
    ) -> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(II)Lorg/bukkit/potion/PotionEffect;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createEffect",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
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
        let sig = String::from("(I)Lorg/bukkit/potion/PotionEffectType;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getById",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
        let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "registerPotionEffectType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(())
    }
    //

    pub fn stop_accepting_registrations(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "stopAcceptingRegistrations", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/NamespacedKey;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKey", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::NamespacedKey::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn get_by_name(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/potion/PotionEffectType;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::PotionEffectType::from_raw(&jni, obj)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for PotionEffectTypeWrapper<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling PotionEffectTypeWrapper.toString: {}", err),
        }
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

impl<'mc> JNIRaw<'mc> for PotionData<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for PotionData<'mc> {
    fn from_raw(
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
}

impl<'mc> PotionData<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::potion::PotionType<'mc>>,
        arg1: std::option::Option<bool>,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::potion::PotionData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/potion/PotionType;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Z";
            // 2
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            // 2
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/potion/PotionData");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::potion::PotionData::from_raw(&jni, res)
    }
    //

    pub fn is_upgraded(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUpgraded", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_extended(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isExtended", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn get_type(&self) -> Result<crate::potion::PotionType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
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
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for PotionData<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling PotionData.toString: {}", err),
        }
    }
}

/// Potion Adapter for pre-1.9 data values see @PotionMeta for 1.9+
pub struct Potion<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Potion<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for Potion<'mc> {
    fn from_raw(
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
}

impl<'mc> Potion<'mc> {
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::potion::PotionType<'mc>>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<bool>,
        arg3: std::option::Option<bool>,
    ) -> Result<crate::potion::Potion<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/potion/PotionType;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            // 2
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "Z";
            // 2
            let val_4 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/potion/Potion");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::potion::Potion::from_raw(&jni, res)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Sets the level of this potion.
    pub fn set_level(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_type(
        &self,
        arg0: impl Into<crate::potion::PotionType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionType;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn effects(
        &self,
    ) -> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEffects", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
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
        &self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn splash(&self) -> Result<crate::potion::Potion<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/Potion;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "splash", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::Potion::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Sets whether this potion is a splash potion. Splash potions can be thrown for a radius effect.
    pub fn set_splash(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSplash",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn extend(&self) -> Result<crate::potion::Potion<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/Potion;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "extend", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::Potion::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set whether this potion has extended duration. This will cause the potion to have roughly 8/3 more duration than a regular potion.
    pub fn set_has_extended_duration(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHasExtendedDuration",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn brewer(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::potion::PotionBrewer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionBrewer;");
        let cls = jni.find_class("org/bukkit/potion/PotionBrewer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getBrewer", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::PotionBrewer::from_raw(&jni, obj)
    }
    //

    pub fn has_extended_duration(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasExtendedDuration",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_splash(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSplash", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_damage_value(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("()S");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toDamageValue", sig.as_str(), vec![]);
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
        let sig = String::from("(I)Lorg/bukkit/potion/Potion;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let cls = jni.find_class("org/bukkit/potion/Potion");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "fromDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Lorg/bukkit/potion/Potion;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/potion/Potion");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "fromItemStack",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
        let sig = String::from("(Lorg/bukkit/potion/PotionBrewer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "setPotionBrewer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(())
    }
    //

    pub fn name_id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getNameId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn apply(
        &self,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/LivingEntity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "apply", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get_type(&self) -> Result<crate::potion::PotionType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
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
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for Potion<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling Potion.toString: {}", err),
        }
    }
}

/// Represents a potion effect, that can be added to a <a href="../entity/LivingEntity.html" title="interface in org.bukkit.entity"><code>LivingEntity</code></a>. A potion effect has a duration that it will last for, an amplifier that will enhance its effects, and a <a href="PotionEffectType.html" title="class in org.bukkit.potion"><code>PotionEffectType</code></a>, that represents its effect on an entity.
pub struct PotionEffect<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PotionEffect<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for PotionEffect<'mc> {
    fn from_raw(
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
}

impl<'mc> PotionEffect<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::potion::PotionEffectType<'mc>>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<bool>,
        arg4: std::option::Option<bool>,
        arg5: std::option::Option<bool>,
    ) -> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/potion/PotionEffectType;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a.into());
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "Z";
            // 2
            let val_4 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_4);
        }
        if let Some(a) = arg4 {
            sig += "Z";
            // 2
            let val_5 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_5);
        }
        if let Some(a) = arg5 {
            sig += "Z";
            // 2
            let val_6 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_6);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/potion/PotionEffect");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::potion::PotionEffect::from_raw(&jni, res)
    }
    //

    pub fn serialize(&self) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "serialize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn duration(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDuration", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn amplifier(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAmplifier", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn is_shorter_than(
        &self,
        arg0: impl Into<crate::potion::PotionEffect<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionEffect;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isShorterThan",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_ambient(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isAmbient", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn has_particles(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasParticles", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn has_icon(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasIcon", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn apply(
        &self,
        arg0: impl Into<crate::entity::LivingEntity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "apply",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get_type(
        &self,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffectType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_infinite(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInfinite", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for PotionEffect<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling PotionEffect.toString: {}", err),
        }
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

impl<'mc> JNIRaw<'mc> for PotionEffectType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for PotionEffectType<'mc> {
    fn from_raw(
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
}

impl<'mc> PotionEffectType<'mc> {
    //

    pub fn color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
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
        let sig = String::from("(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/potion/PotionEffectType;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::PotionEffectType::from_raw(&jni, obj)
    }
    //@NotNull

    /// Creates a PotionEffect from this PotionEffectType, applying duration modifiers and checks.
    pub fn create_effect(
        &self,
        arg0: i32,
        arg1: i32,
    ) -> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(II)Lorg/bukkit/potion/PotionEffect;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createEffect",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffect::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_instant(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInstant", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn duration_modifier(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDurationModifier",
            sig.as_str(),
            vec![],
        );
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
        let sig = String::from("(I)Lorg/bukkit/potion/PotionEffectType;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getById",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
        let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "registerPotionEffectType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(())
    }
    //

    pub fn stop_accepting_registrations(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "stopAcceptingRegistrations", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        Ok(())
    }
    //

    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/NamespacedKey;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKey", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::NamespacedKey::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn get_by_name(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/potion/PotionEffectType;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("org/bukkit/potion/PotionEffectType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::potion::PotionEffectType::from_raw(&jni, obj)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for PotionEffectType<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling PotionEffectType.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::Keyed<'mc>> for PotionEffectType<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PotionEffectType into crate::Keyed")
    }
}
/// Represents a brewer that can create <a href="PotionEffect.html" title="class in org.bukkit.potion"><code>PotionEffect</code></a>s.
///
/// This is a representation of an abstract class.
pub struct PotionBrewer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PotionBrewer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for PotionBrewer<'mc> {
    fn from_raw(
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
}

impl<'mc> PotionBrewer<'mc> {
    //

    pub fn get_effects(
        &self,
        arg0: impl Into<crate::potion::PotionType<'mc>>,
        arg1: bool,
        arg2: bool,
    ) -> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionType;ZZ)Ljava/util/Collection;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        // -1
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffects",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn create_effect(
        &self,
        arg0: impl Into<crate::potion::PotionEffectType<'mc>>,
        arg1: i32,
        arg2: i32,
    ) -> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/potion/PotionEffectType;II)Lorg/bukkit/potion/PotionEffect;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createEffect",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
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
    /// Returns a collection of <a title="class in org.bukkit.potion" href="PotionEffect.html"><code>PotionEffect</code></a> that would be applied from a potion with the given data value.
    pub fn get_effects_from_damage(
        &self,
        arg0: i32,
    ) -> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/util/Collection;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectsFromDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
}
#[derive(PartialEq, Eq)]
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
impl<'mc> std::fmt::Display for PotionType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
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

impl<'mc> JNIInstantiatableEnum<'mc> for PotionType<'mc> {
    type Enum = PotionTypeEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
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
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
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
