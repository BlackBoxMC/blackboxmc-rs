#![allow(deprecated)]
use crate::JNIRaw;
/// An instantiatable struct that implements ItemTagType. Needed for returning it from Java.
pub struct ItemTagType<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ItemTagType<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ItemTagType from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ItemTagType") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemTagType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn primitive_type(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrimitiveType",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn complex_type(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComplexType",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn to_primitive(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::bukkit::inventory::meta::tags::ItemTagAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res =
self.jni_ref().call_method(&self.jni_object(),"toPrimitive","(Ljava/lang/Object;Lorg/bukkit/inventory/meta/tags/ItemTagAdapterContext;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
        Ok(res.l().unwrap())
    }
    pub fn from_primitive(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::bukkit::inventory::meta::tags::ItemTagAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res =
self.jni_ref().call_method(&self.jni_object(),"fromPrimitive","(Ljava/lang/Object;Lorg/bukkit/inventory/meta/tags/ItemTagAdapterContext;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> crate::JNIRaw<'mc> for ItemTagType<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements ItemTagAdapterContext. Needed for returning it from Java.
pub struct ItemTagAdapterContext<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ItemTagAdapterContext<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ItemTagAdapterContext from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ItemTagAdapterContext") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemTagAdapterContext object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "newTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for ItemTagAdapterContext<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct ItemTagTypePrimitiveTagType<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for ItemTagTypePrimitiveTagType<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ItemTagTypePrimitiveTagType<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ItemTagTypePrimitiveTagType from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ItemTagTypePrimitiveTagType") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemTagTypePrimitiveTagType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn primitive_type(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrimitiveType",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn complex_type(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComplexType",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn to_primitive(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::bukkit::inventory::meta::tags::ItemTagAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res =
self.jni_ref().call_method(&self.jni_object(),"toPrimitive","(Ljava/lang/Object;Lorg/bukkit/inventory/meta/tags/ItemTagAdapterContext;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
        Ok(res.l().unwrap())
    }
    pub fn from_primitive(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::bukkit::inventory::meta::tags::ItemTagAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res =
self.jni_ref().call_method(&self.jni_object(),"fromPrimitive","(Ljava/lang/Object;Lorg/bukkit/inventory/meta/tags/ItemTagAdapterContext;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
        Ok(res.l().unwrap())
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
/// An instantiatable struct that implements CustomItemTagContainer. Needed for returning it from Java.
pub struct CustomItemTagContainer<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CustomItemTagContainer<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CustomItemTagContainer from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("CustomItemTagContainer") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CustomItemTagContainer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn adapter_context(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::ItemTagAdapterContext<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAdapterContext",
            "()Lorg/bukkit/inventory/meta/tags/ItemTagAdapterContext;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::ItemTagAdapterContext(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_custom_tag(
        &mut self,
        arg0: impl Into<crate::bukkit::NamespacedKey<'mc>>,
        arg1: impl Into<crate::bukkit::inventory::meta::tags::ItemTagType<'mc>>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let val_2 = arg2;
        self.jni_ref().call_method(&self.jni_object(),"setCustomTag","(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/meta/tags/ItemTagType;Ljava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        Ok(())
    }
    pub fn has_custom_tag(
        &mut self,
        arg0: impl Into<crate::bukkit::NamespacedKey<'mc>>,
        arg1: impl Into<crate::bukkit::inventory::meta::tags::ItemTagType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasCustomTag",
            "(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/meta/tags/ItemTagType;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_custom_tag(
        &mut self,
        arg0: impl Into<crate::bukkit::NamespacedKey<'mc>>,
        arg1: impl Into<crate::bukkit::inventory::meta::tags::ItemTagType<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res =
self.jni_ref().call_method(&self.jni_object(),"getCustomTag","(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/meta/tags/ItemTagType;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
        Ok(res.l().unwrap())
    }
    pub fn remove_custom_tag(
        &mut self,
        arg0: impl Into<crate::bukkit::NamespacedKey<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeCustomTag",
            "(Lorg/bukkit/NamespacedKey;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for CustomItemTagContainer<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
