#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// The <a href="PersistentDataHolder.html" title="interface in org.bukkit.persistence"><code>PersistentDataHolder</code></a> interface defines an object that can store custom persistent meta data on it.
///
/// This is a representation of an abstract class.
pub struct PersistentDataHolder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PersistentDataHolder<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PersistentDataHolder from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/persistence/PersistentDataHolder")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PersistentDataHolder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Returns a custom tag container capable of storing tags on the object. Note that the tags stored on this container are all stored under their own custom namespace therefore modifying default tags using this <a title="interface in org.bukkit.persistence" href="PersistentDataHolder.html"><code>PersistentDataHolder</code></a> is impossible.
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for PersistentDataHolder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// A default implementation that simply exists to pass on the retrieved or inserted value to the next layer.
/// <p>This implementation does not add any kind of logic, but is used to provide default implementations for the primitive types.</p>
pub struct PersistentDataTypePrimitivePersistentDataType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PersistentDataTypePrimitivePersistentDataType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PersistentDataTypePrimitivePersistentDataType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
        "Tried to instantiate PersistentDataTypePrimitivePersistentDataType from null object.")
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/persistence/PersistentDataTypePrimitivePersistentDataType",
        )?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a PersistentDataTypePrimitivePersistentDataType object, got {}",
        name
    )
    .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="PersistentDataType.html#getPrimitiveType()">PersistentDataType</a></code></span>
    /// Returns the primitive data type of this tag.
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
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="PersistentDataType.html#getComplexType()">PersistentDataType</a></code></span>
    /// Returns the complex object type the primitive value resembles.
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
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="PersistentDataType.html#toPrimitive(Z,org.bukkit.persistence.PersistentDataAdapterContext)">PersistentDataType</a></code></span>
    /// Returns the primitive data that resembles the complex object passed to this method.
    pub fn to_primitive(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<&'mc crate::persistence::PersistentDataAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"toPrimitive","(Ljava/lang/Object;Lorg/bukkit/persistence/PersistentDataAdapterContext;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="PersistentDataType.html#fromPrimitive(T,org.bukkit.persistence.PersistentDataAdapterContext)">PersistentDataType</a></code></span>
    /// Creates a complex object based of the passed primitive value
    pub fn from_primitive(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<&'mc crate::persistence::PersistentDataAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"fromPrimitive","(Ljava/lang/Object;Lorg/bukkit/persistence/PersistentDataAdapterContext;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
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
        Ok(res.z().unwrap())
    }

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

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// This interface represents the context in which the <a href="PersistentDataType.html" title="interface in org.bukkit.persistence"><code>PersistentDataType</code></a> can serialize and deserialize the passed values.
///
/// This is a representation of an abstract class.
pub struct PersistentDataAdapterContext<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PersistentDataAdapterContext<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PersistentDataAdapterContext from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/persistence/PersistentDataAdapterContext")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PersistentDataAdapterContext object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Creates a new and empty meta container instance.
    pub fn new_persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "newPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for PersistentDataAdapterContext<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// This class represents an enum with a generic content type. It defines the types a custom tag can have.
/// <p>This interface can be used to create your own custom <a href="PersistentDataType.html" title="interface in org.bukkit.persistence"><code>PersistentDataType</code></a> with different complex types. This may be useful for the likes of a UUIDTagType:</p>
/// <pre> <code>
/// public class UUIDTagType implements PersistentDataType&lt;byte[], UUID&gt; {
/// {@literal @Override}
/// public Class&lt;byte[]&gt; getPrimitiveType() {
/// return byte[].class;
/// }
/// {@literal @Override}
/// public Class&lt;UUID&gt; getComplexType() {
/// return UUID.class;
/// }
/// {@literal @Override}
/// public byte[] toPrimitive(UUID complex, PersistentDataAdapterContext context) {
/// ByteBuffer bb = ByteBuffer.wrap(new byte[16]);
/// bb.putLong(complex.getMostSignificantBits());
/// bb.putLong(complex.getLeastSignificantBits());
/// return bb.array();
/// }
/// {@literal @Override}
/// public UUID fromPrimitive(byte[] primitive, PersistentDataAdapterContext context) {
/// ByteBuffer bb = ByteBuffer.wrap(primitive);
/// long firstLong = bb.getLong();
/// long secondLong = bb.getLong();
/// return new UUID(firstLong, secondLong);
/// }
/// }</code></pre>
///
/// This is a representation of an abstract class.
pub struct PersistentDataType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PersistentDataType<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "PersistentDataType", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PersistentDataType from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/persistence/PersistentDataType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PersistentDataType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Returns the primitive data type of this tag.
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
    /// Returns the complex object type the primitive value resembles.
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
    /// Returns the primitive data that resembles the complex object passed to this method.
    pub fn to_primitive(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<&'mc crate::persistence::PersistentDataAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"toPrimitive","(Ljava/lang/Object;Lorg/bukkit/persistence/PersistentDataAdapterContext;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    /// Creates a complex object based of the passed primitive value
    pub fn from_primitive(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<&'mc crate::persistence::PersistentDataAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"fromPrimitive","(Ljava/lang/Object;Lorg/bukkit/persistence/PersistentDataAdapterContext;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for PersistentDataType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// A convenience implementation to convert between Byte and Boolean as there is no native implementation for booleans.
///
/// Any byte value not equal to 0 is considered to be true.
pub struct PersistentDataTypeBooleanPersistentDataType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PersistentDataTypeBooleanPersistentDataType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PersistentDataTypeBooleanPersistentDataType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
        "Tried to instantiate PersistentDataTypeBooleanPersistentDataType from null object.")
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/persistence/PersistentDataTypeBooleanPersistentDataType",
        )?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a PersistentDataTypeBooleanPersistentDataType object, got {}",
        name
    )
    .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        crate::persistence::PersistentDataTypeBooleanPersistentDataType<'mc>,
        Box<dyn std::error::Error>,
    > {
        let cls =
            &jni.find_class("org/bukkit/persistence/PersistentDataType$BooleanPersistentDataType")?;
        let res = jni.new_object(cls, "()V", &[])?;
        crate::persistence::PersistentDataTypeBooleanPersistentDataType::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="PersistentDataType.html#getPrimitiveType()">PersistentDataType</a></code></span>
    /// Returns the primitive data type of this tag.
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
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="PersistentDataType.html#getComplexType()">PersistentDataType</a></code></span>
    /// Returns the complex object type the primitive value resembles.
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
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="PersistentDataType.html#toPrimitive(Z,org.bukkit.persistence.PersistentDataAdapterContext)">PersistentDataType</a></code></span>
    /// Returns the primitive data that resembles the complex object passed to this method.
    pub fn to_primitive_with_object(
        &mut self,
        arg0: bool,
        arg1: std::option::Option<
            impl Into<&'mc crate::persistence::PersistentDataAdapterContext<'mc>>,
        >,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        // 1
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toPrimitive",
            "(ZLorg/bukkit/persistence/PersistentDataAdapterContext;)B",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="PersistentDataType.html#fromPrimitive(T,org.bukkit.persistence.PersistentDataAdapterContext)">PersistentDataType</a></code></span>
    /// Creates a complex object based of the passed primitive value
    pub fn from_primitive_with_byte(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<
            impl Into<&'mc crate::persistence::PersistentDataAdapterContext<'mc>>,
        >,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"fromPrimitive","(Ljava/lang/Object;Lorg/bukkit/persistence/PersistentDataAdapterContext;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
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
        Ok(res.z().unwrap())
    }

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

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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
pub struct PersistentDataContainer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PersistentDataContainer<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PersistentDataContainer from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/persistence/PersistentDataContainer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PersistentDataContainer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Get a set of keys present on this <a href="PersistentDataContainer.html" title="interface in org.bukkit.persistence"><code>PersistentDataContainer</code></a> instance. Any changes made to the returned set will not be reflected on the instance.
    pub fn keys(&mut self) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getKeys", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns if the persistent metadata provider has metadata registered matching the provided parameters.
    /// <p>This method will only return if the found value has the same primitive data type as the provided key.</p>
    /// <p>Storing a value using a custom <a title="interface in org.bukkit.persistence" href="PersistentDataType.html"><code>PersistentDataType</code></a> implementation will not store the complex data type. Therefore storing a UUID (by storing a byte[]) will match has("key" , <a href="PersistentDataType.html#BYTE_ARRAY"><code>PersistentDataType.BYTE_ARRAY</code></a>). Likewise a stored byte[] will always match your UUID <a href="PersistentDataType.html" title="interface in org.bukkit.persistence"><code>PersistentDataType</code></a> even if it is not 16 bytes long.</p>
    /// <p>This method is only usable for custom object keys. Overwriting existing tags, like the the display name, will not work as the values are stored using your namespace.</p>
    pub fn has(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
        arg1: impl Into<&'mc crate::persistence::PersistentDataType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "has",
            "(Lorg/bukkit/NamespacedKey;Lorg/bukkit/persistence/PersistentDataType;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns the adapter context this tag container uses.
    pub fn adapter_context(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataAdapterContext<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAdapterContext",
            "()Lorg/bukkit/persistence/PersistentDataAdapterContext;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataAdapterContext::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Removes a custom key from the <a title="interface in org.bukkit.persistence" href="PersistentDataHolder.html"><code>PersistentDataHolder</code></a> instance.
    pub fn remove(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/NamespacedKey;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns the metadata value that is stored on the <a href="PersistentDataHolder.html" title="interface in org.bukkit.persistence"><code>PersistentDataHolder</code></a> instance.
    /// Returns the metadata value that is stored on the <a href="PersistentDataHolder.html" title="interface in org.bukkit.persistence"><code>PersistentDataHolder</code></a> instance. If the value does not exist in the container, the default value provided is returned.
    /// Get a set of keys present on this <a title="interface in org.bukkit.persistence" href="PersistentDataContainer.html"><code>PersistentDataContainer</code></a> instance. Any changes made to the returned set will not be reflected on the instance.
    /// Returns the adapter context this tag container uses.
    pub fn get(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
        arg1: impl Into<&'mc crate::persistence::PersistentDataType<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"get","(Lorg/bukkit/NamespacedKey;Lorg/bukkit/persistence/PersistentDataType;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    /// Returns if the container instance is empty, therefore has no entries inside it.
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Stores a metadata value on the <a href="PersistentDataHolder.html" title="interface in org.bukkit.persistence"><code>PersistentDataHolder</code></a> instance.
    /// <p>This API cannot be used to manipulate minecraft data, as the values will be stored using your namespace. This method will override any existing value the <a href="PersistentDataHolder.html" title="interface in org.bukkit.persistence"><code>PersistentDataHolder</code></a> may have stored under the provided key.</p>
    pub fn set(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
        arg1: impl Into<&'mc crate::persistence::PersistentDataType<'mc>>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = arg2;
        let res = self.jni_ref().call_method(&self.jni_object(),"set","(Lorg/bukkit/NamespacedKey;Lorg/bukkit/persistence/PersistentDataType;Ljava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns the metadata value that is stored on the <a href="PersistentDataHolder.html" title="interface in org.bukkit.persistence"><code>PersistentDataHolder</code></a> instance. If the value does not exist in the container, the default value provided is returned.
    pub fn get_or_default(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
        arg1: impl Into<&'mc crate::persistence::PersistentDataType<'mc>>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = arg2;
        let res = self.jni_ref().call_method(&self.jni_object(),"getOrDefault","(Lorg/bukkit/NamespacedKey;Lorg/bukkit/persistence/PersistentDataType;Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for PersistentDataContainer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
