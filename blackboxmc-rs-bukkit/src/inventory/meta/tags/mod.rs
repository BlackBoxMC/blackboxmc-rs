#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// This class represents an enum with a generic content type. It defines the types a custom item tag can have.
/// <p>This interface can be used to create your own custom <a href="ItemTagType.html" title="interface in org.bukkit.inventory.meta.tags"><code>ItemTagType</code></a> with different complex types. This may be useful for the likes of a UUIDItemTagType:</p>
/// <pre> <code>
/// public class UUIDItemTagType implements ItemTagType&lt;byte[], UUID&gt; {
/// {@literal @Override}
/// public Class&lt;byte[]&gt; getPrimitiveType() {
/// return byte[].class;
/// }
/// {@literal @Override}
/// public Class&lt;UUID&gt; getComplexType() {
/// return UUID.class;
/// }
/// {@literal @Override}
/// public byte[] toPrimitive(UUID complex, ItemTagAdapterContext context) {
/// ByteBuffer bb = ByteBuffer.wrap(new byte[16]);
/// bb.putLong(complex.getMostSignificantBits());
/// bb.putLong(complex.getLeastSignificantBits());
/// return bb.array();
/// }
/// {@literal @Override}
/// public UUID fromPrimitive(byte[] primitive, ItemTagAdapterContext context) {
/// ByteBuffer bb = ByteBuffer.wrap(primitive);
/// long firstLong = bb.getLong();
/// long secondLong = bb.getLong();
/// return new UUID(firstLong, secondLong);
/// }
/// }</code></pre>
///
/// This is a representation of an abstract class.
pub struct ItemTagType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ItemTagType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ItemTagType from null object.").into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/tags/ItemTagType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemTagType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn primitive_type(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrimitiveType",
            "()Ljava/lang/Class;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn complex_type(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComplexType",
            "()Ljava/lang/Class;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn to_primitive(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::inventory::meta::tags::ItemTagAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"toPrimitive","(Ljava/lang/Object;Lorg/bukkit/inventory/meta/tags/ItemTagAdapterContext;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn from_primitive(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::inventory::meta::tags::ItemTagAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"fromPrimitive","(Ljava/lang/Object;Lorg/bukkit/inventory/meta/tags/ItemTagAdapterContext;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}
impl<'mc> JNIRaw<'mc> for ItemTagType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// This interface represents the context in which the <a title="interface in org.bukkit.inventory.meta.tags" href="ItemTagType.html"><code>ItemTagType</code></a> can serialize and deserialize the passed values.
///
/// This is a representation of an abstract class.
pub struct ItemTagAdapterContext<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ItemTagAdapterContext<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ItemTagAdapterContext from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/tags/ItemTagAdapterContext")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemTagAdapterContext object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn new_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "newTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for ItemTagAdapterContext<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// A default implementation that simply exists to pass on the retrieved or inserted value to the next layer. This implementation does not add any kind of logic, but is used to provide default implementations for the primitive types.
pub struct ItemTagTypePrimitiveTagType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ItemTagTypePrimitiveTagType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ItemTagTypePrimitiveTagType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ItemTagTypePrimitiveTagType from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/inventory/meta/tags/ItemTagType$PrimitiveTagType",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemTagTypePrimitiveTagType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn primitive_type(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrimitiveType",
            "()Ljava/lang/Class;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn complex_type(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComplexType",
            "()Ljava/lang/Class;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn to_primitive(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::inventory::meta::tags::ItemTagAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"toPrimitive","(Ljava/lang/Object;Lorg/bukkit/inventory/meta/tags/ItemTagAdapterContext;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn from_primitive(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::inventory::meta::tags::ItemTagAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"fromPrimitive","(Ljava/lang/Object;Lorg/bukkit/inventory/meta/tags/ItemTagAdapterContext;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
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
/// This interface represents a map like object, capable of storing custom tags in it.
///
/// This is a representation of an abstract class.
pub struct CustomItemTagContainer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CustomItemTagContainer<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CustomItemTagContainer from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/inventory/meta/tags/CustomItemTagContainer",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CustomItemTagContainer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn adapter_context(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::ItemTagAdapterContext<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAdapterContext",
            "()Lorg/bukkit/inventory/meta/tags/ItemTagAdapterContext;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::ItemTagAdapterContext::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_custom_tag(
        &mut self,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
        arg1: impl Into<crate::inventory::meta::tags::ItemTagType<'mc>>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = arg2;
        let res = self.jni_ref().call_method(&self.jni_object(),"setCustomTag","(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/meta/tags/ItemTagType;Ljava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn has_custom_tag(
        &mut self,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
        arg1: impl Into<crate::inventory::meta::tags::ItemTagType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasCustomTag",
            "(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/meta/tags/ItemTagType;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get_custom_tag(
        &mut self,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
        arg1: impl Into<crate::inventory::meta::tags::ItemTagType<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"getCustomTag","(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/meta/tags/ItemTagType;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn remove_custom_tag(
        &mut self,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeCustomTag",
            "(Lorg/bukkit/NamespacedKey;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
impl<'mc> JNIRaw<'mc> for CustomItemTagContainer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
