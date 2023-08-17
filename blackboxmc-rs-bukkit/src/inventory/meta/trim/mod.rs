#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIInstantiatableEnum;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents an armor trim that may be applied to an item.
pub struct ArmorTrim<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ArmorTrim<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for ArmorTrim<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ArmorTrim from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/trim/ArmorTrim")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ArmorTrim object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ArmorTrim<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::meta::trim::TrimMaterial<'mc>>,
        arg1: impl Into<crate::inventory::meta::trim::TrimPattern<'mc>>,
    ) -> Result<crate::inventory::meta::trim::ArmorTrim<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/meta/trim/TrimMaterial;Lorg/bukkit/inventory/meta/trim/TrimPattern;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/inventory/meta/trim/ArmorTrim");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::meta::trim::ArmorTrim::from_raw(&jni, res)
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
    //@NotNull

    pub fn material(
        &self,
    ) -> Result<crate::inventory::meta::trim::TrimMaterial<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/meta/trim/TrimMaterial;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::trim::TrimMaterial::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //@NotNull

    //@NotNull

    pub fn pattern(
        &self,
    ) -> Result<crate::inventory::meta::trim::TrimPattern<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/meta/trim/TrimPattern;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPattern", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::trim::TrimPattern::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn wait_with_long(
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

impl<'mc> std::string::ToString for ArmorTrim<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling ArmorTrim.toString: {}", err),
        }
    }
}

/// Represents a material that may be used in an <a title="class in org.bukkit.inventory.meta.trim" href="ArmorTrim.html"><code>ArmorTrim</code></a>.
///
/// This is a representation of an abstract class.
pub struct TrimMaterial<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TrimMaterial<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for TrimMaterial<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TrimMaterial from null object.").into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/trim/TrimMaterial")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TrimMaterial object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TrimMaterial<'mc> {
    //@NotNull

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
}
impl<'mc> Into<crate::Keyed<'mc>> for TrimMaterial<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TrimMaterial into crate::Keyed")
    }
}
/// Represents a pattern that may be used in an <a href="ArmorTrim.html" title="class in org.bukkit.inventory.meta.trim"><code>ArmorTrim</code></a>.
///
/// This is a representation of an abstract class.
pub struct TrimPattern<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TrimPattern<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for TrimPattern<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TrimPattern from null object.").into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/trim/TrimPattern")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TrimPattern object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TrimPattern<'mc> {
    //@NotNull

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
}
impl<'mc> Into<crate::Keyed<'mc>> for TrimPattern<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TrimPattern into crate::Keyed")
    }
}
