#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct PersistentDataContainer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PersistentDataContainer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PersistentDataContainer<'mc> {
    fn from_raw(
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
}

impl<'mc> PersistentDataContainer<'mc> {
    /// Stores a metadata value on the {@link PersistentDataHolder} instance.
    ///
    /// This API cannot be used to manipulate minecraft data, as the values will
    /// be stored using your namespace. This method will override any existing
    /// value the {@link PersistentDataHolder} may have stored under the provided
    /// key.
    pub fn set(
        &self,
        key: impl Into<crate::NamespacedKey<'mc>>,
        val_type: impl Into<crate::persistence::PersistentDataType<'mc>>,
        value: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/NamespacedKey;Lorg/bukkit/persistence/PersistentDataType;LC;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(key.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns if the persistent metadata provider has metadata registered
    /// matching the provided parameters.
    ///
    /// This method will only return true if the found value has the same primitive
    /// data type as the provided key.
    ///
    /// Storing a value using a custom {@link PersistentDataType} implementation
    /// will not store the complex data type. Therefore storing a UUID (by
    /// storing a byte[]) will match has("key" ,
    /// {@link PersistentDataType#BYTE_ARRAY}). Likewise a stored byte[] will
    /// always match your UUID {@link PersistentDataType} even if it is not 16
    /// bytes long.
    ///
    /// This method is only usable for custom object keys. Overwriting existing
    /// tags, like the display name, will not work as the values are stored
    /// using your namespace.
    pub fn has(
        &self,
        key: impl Into<crate::NamespacedKey<'mc>>,
        val_type: std::option::Option<impl Into<crate::persistence::PersistentDataType<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/NamespacedKey;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(key.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = val_type {
            sig += "Lorg/bukkit/persistence/PersistentDataType;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "has", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the metadata value that is stored on the
    /// {@link PersistentDataHolder} instance.
    pub fn get(
        &self,
        key: impl Into<crate::NamespacedKey<'mc>>,
        val_type: impl Into<crate::persistence::PersistentDataType<'mc>>,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/NamespacedKey;Lorg/bukkit/persistence/PersistentDataType;)LC;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(key.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    /// Returns the metadata value that is stored on the
    /// {@link PersistentDataHolder} instance. If the value does not exist in the
    /// container, the default value provided is returned.
    pub fn get_or_default(
        &self,
        key: impl Into<crate::NamespacedKey<'mc>>,
        val_type: impl Into<crate::persistence::PersistentDataType<'mc>>,
        default_value: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/NamespacedKey;Lorg/bukkit/persistence/PersistentDataType;LC;)LC;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(key.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(default_value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    /// Get the set of keys present on this {@link PersistentDataContainer}
    /// instance.
    /// Any changes made to the returned set will not be reflected on the
    /// instance.
    pub fn keys(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKeys", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Removes a custom key from the {@link PersistentDataHolder} instance.
    pub fn remove(
        &self,
        key: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/NamespacedKey;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(key.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns if the container instance is empty, therefore has no entries
    /// inside it.
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Copies all values from this {@link PersistentDataContainer} to the provided
    /// container.
    ///
    /// This method only copies custom object keys. Existing tags, like the display
    /// name, will not be copied as the values are stored using your namespace.
    pub fn copy_to(
        &self,
        other: impl Into<crate::persistence::PersistentDataContainer<'mc>>,
        replace: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/persistence/PersistentDataContainer;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(replace.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns the adapter context this tag container uses.
    pub fn adapter_context(
        &self,
    ) -> Result<crate::persistence::PersistentDataAdapterContext<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/persistence/PersistentDataAdapterContext;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAdapterContext",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataAdapterContext::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct ListPersistentDataType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ListPersistentDataType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ListPersistentDataType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ListPersistentDataType from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/persistence/ListPersistentDataType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ListPersistentDataType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ListPersistentDataType<'mc> {
    /// Provides the persistent data type of the elements found in the list.
    pub fn element_type(
        &self,
    ) -> Result<crate::persistence::PersistentDataType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/PersistentDataType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "elementType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the primitive data type of this tag.
    pub fn primitive_type(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrimitiveType",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    /// Returns the complex object type the primitive value resembles.
    pub fn complex_type(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getComplexType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    /// Returns the primitive data that resembles the complex object passed to
    /// this method.
    pub fn to_primitive(
        &self,
        complex: jni::objects::JObject<'mc>,
        context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(LC;Lorg/bukkit/persistence/PersistentDataAdapterContext;)LP;");
        let val_1 = jni::objects::JValueGen::Object(complex);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(context.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toPrimitive",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    /// Creates a complex object based of the passed primitive value
    pub fn from_primitive(
        &self,
        primitive: jni::objects::JObject<'mc>,
        context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(LP;Lorg/bukkit/persistence/PersistentDataAdapterContext;)LC;");
        let val_1 = jni::objects::JValueGen::Object(primitive);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(context.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "fromPrimitive",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::persistence::PersistentDataType<'mc>> for ListPersistentDataType<'mc> {
    fn into(self) -> crate::persistence::PersistentDataType<'mc> {
        crate::persistence::PersistentDataType::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting ListPersistentDataType into crate::persistence::PersistentDataType",
        )
    }
}
#[repr(C)]
pub struct PersistentDataHolder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PersistentDataHolder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PersistentDataHolder<'mc> {
    fn from_raw(
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
}

impl<'mc> PersistentDataHolder<'mc> {
    /// Returns a custom tag container capable of storing tags on the object.
    /// Note that the tags stored on this container are all stored under their
    /// own custom namespace therefore modifying default tags using this
    /// {@link PersistentDataHolder} is impossible.
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/persistence/PersistentDataContainer;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct ListPersistentDataTypeProvider<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ListPersistentDataTypeProvider<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ListPersistentDataTypeProvider<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ListPersistentDataTypeProvider from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/persistence/ListPersistentDataTypeProvider",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ListPersistentDataTypeProvider object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ListPersistentDataTypeProvider<'mc> {
    /// Provides a shared {@link ListPersistentDataType} that is capable of
    /// storing lists of bytes.
    pub fn bytes(
        &self,
    ) -> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/ListPersistentDataType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "bytes", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Provides a shared {@link ListPersistentDataType} that is capable of
    /// storing lists of shorts.
    pub fn shorts(
        &self,
    ) -> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/ListPersistentDataType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "shorts", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Provides a shared {@link ListPersistentDataType} that is capable of
    /// storing lists of integers.
    pub fn integers(
        &self,
    ) -> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/ListPersistentDataType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "integers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Provides a shared {@link ListPersistentDataType} that is capable of
    /// storing lists of longs.
    pub fn longs(
        &self,
    ) -> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/ListPersistentDataType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "longs", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Provides a shared {@link ListPersistentDataType} that is capable of
    /// storing lists of floats.
    pub fn floats(
        &self,
    ) -> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/ListPersistentDataType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "floats", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Provides a shared {@link ListPersistentDataType} that is capable of
    /// storing lists of doubles.
    pub fn doubles(
        &self,
    ) -> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/ListPersistentDataType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "doubles", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Provides a shared {@link ListPersistentDataType} that is capable of
    /// storing lists of booleans.
    pub fn booleans(
        &self,
    ) -> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/ListPersistentDataType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "booleans", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Provides a shared {@link ListPersistentDataType} that is capable of
    /// storing lists of strings.
    pub fn strings(
        &self,
    ) -> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/ListPersistentDataType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "strings", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Provides a shared {@link ListPersistentDataType} that is capable of
    /// storing lists of byte arrays.
    pub fn byte_arrays(
        &self,
    ) -> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/ListPersistentDataType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "byteArrays", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Provides a shared {@link ListPersistentDataType} that is capable of
    /// storing lists of int arrays.
    pub fn integer_arrays(
        &self,
    ) -> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/ListPersistentDataType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "integerArrays", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Provides a shared {@link ListPersistentDataType} that is capable of
    /// storing lists of long arrays.
    pub fn long_arrays(
        &self,
    ) -> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/ListPersistentDataType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "longArrays", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Provides a shared {@link ListPersistentDataType} that is capable of
    /// persistent data containers..
    pub fn data_containers(
        &self,
    ) -> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/ListPersistentDataType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "dataContainers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Constructs a new list persistent data type given any persistent data type
    /// for its elements.
    pub fn list_type_from(
        &self,
        element_type: impl Into<crate::persistence::PersistentDataType<'mc>>,
    ) -> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/persistence/PersistentDataType;)Lorg/bukkit/persistence/ListPersistentDataType;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(element_type.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "listTypeFrom",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct PersistentDataType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PersistentDataType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PersistentDataType<'mc> {
    fn from_raw(
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
}

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
    /// Returns the primitive data type of this tag.
    pub fn primitive_type(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrimitiveType",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    /// Returns the complex object type the primitive value resembles.
    pub fn complex_type(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getComplexType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    /// Returns the primitive data that resembles the complex object passed to
    /// this method.
    pub fn to_primitive(
        &self,
        complex: jni::objects::JObject<'mc>,
        context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(LC;Lorg/bukkit/persistence/PersistentDataAdapterContext;)LP;");
        let val_1 = jni::objects::JValueGen::Object(complex);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(context.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toPrimitive",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    /// Creates a complex object based of the passed primitive value
    pub fn from_primitive(
        &self,
        primitive: jni::objects::JObject<'mc>,
        context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(LP;Lorg/bukkit/persistence/PersistentDataAdapterContext;)LC;");
        let val_1 = jni::objects::JValueGen::Object(primitive);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(context.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "fromPrimitive",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct PersistentDataTypePrimitivePersistentDataType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PersistentDataTypePrimitivePersistentDataType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PersistentDataTypePrimitivePersistentDataType<'mc> {
    fn from_raw(
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
            "org/bukkit/persistence/PersistentDataType/PrimitivePersistentDataType",
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
}

impl<'mc> PersistentDataTypePrimitivePersistentDataType<'mc> {
    pub fn primitive_type(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrimitiveType",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn complex_type(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getComplexType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn to_primitive(
        &self,
        complex: jni::objects::JObject<'mc>,
        context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(LP;Lorg/bukkit/persistence/PersistentDataAdapterContext;)LP;");
        let val_1 = jni::objects::JValueGen::Object(complex);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(context.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toPrimitive",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn from_primitive(
        &self,
        primitive: jni::objects::JObject<'mc>,
        context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(LP;Lorg/bukkit/persistence/PersistentDataAdapterContext;)LP;");
        let val_1 = jni::objects::JValueGen::Object(primitive);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(context.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "fromPrimitive",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::persistence::PersistentDataType<'mc>>
    for PersistentDataTypePrimitivePersistentDataType<'mc>
{
    fn into(self) -> crate::persistence::PersistentDataType<'mc> {
        crate::persistence::PersistentDataType::from_raw(&self.jni_ref(), self.1).expect("Error converting PersistentDataTypePrimitivePersistentDataType into crate::persistence::PersistentDataType")
    }
}
#[repr(C)]
pub struct PersistentDataTypeBooleanPersistentDataType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PersistentDataTypeBooleanPersistentDataType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PersistentDataTypeBooleanPersistentDataType<'mc> {
    fn from_raw(
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
            "org/bukkit/persistence/PersistentDataType/BooleanPersistentDataType",
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
}

impl<'mc> PersistentDataTypeBooleanPersistentDataType<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        crate::persistence::PersistentDataTypeBooleanPersistentDataType<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()V");
        let cls =
            jni.find_class("org/bukkit/persistence/PersistentDataType/BooleanPersistentDataType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::persistence::PersistentDataTypeBooleanPersistentDataType::from_raw(&jni, res)
    }

    pub fn primitive_type(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrimitiveType",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn complex_type(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getComplexType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn to_primitive(
        &self,
        complex: bool,
        context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;Lorg/bukkit/persistence/PersistentDataAdapterContext;)Ljava/lang/Byte;");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Ljava/Lang/Object;)V",
            vec![complex.into()],
        )?);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(context.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toPrimitive",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn from_primitive(
        &self,
        primitive: i8,
        context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Byte;Lorg/bukkit/persistence/PersistentDataAdapterContext;)Ljava/lang/Boolean;");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Byte",
            "(Ljava/Lang/Object;)V",
            vec![primitive.into()],
        )?);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(context.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "fromPrimitive",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::persistence::PersistentDataType<'mc>>
    for PersistentDataTypeBooleanPersistentDataType<'mc>
{
    fn into(self) -> crate::persistence::PersistentDataType<'mc> {
        crate::persistence::PersistentDataType::from_raw(&self.jni_ref(), self.1).expect("Error converting PersistentDataTypeBooleanPersistentDataType into crate::persistence::PersistentDataType")
    }
}
#[repr(C)]
pub struct PersistentDataAdapterContext<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PersistentDataAdapterContext<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PersistentDataAdapterContext<'mc> {
    fn from_raw(
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
}

impl<'mc> PersistentDataAdapterContext<'mc> {
    /// Creates a new and empty meta container instance.
    pub fn new_persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/PersistentDataContainer;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "newPersistentDataContainer",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
