#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct MemorySection<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MemorySection<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MemorySection<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MemorySection from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/configuration/MemorySection")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MemorySection object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MemorySection<'mc> {
    pub fn get_keys(
        &self,
        deep: bool,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Set;");
        let val_1 = jni::objects::JValueGen::Bool(deep.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKeys",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_values(
        &self,
        deep: bool,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Map;");
        let val_1 = jni::objects::JValueGen::Bool(deep.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getValues",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains(
        &self,
        path: impl Into<String>,
        ignore_default: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = ignore_default {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_set(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSet",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn current_path(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCurrentPath", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

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

    pub fn root(
        &self,
    ) -> Result<Option<crate::configuration::Configuration<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/configuration/Configuration;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRoot", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::Configuration::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn parent(
        &self,
    ) -> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/configuration/ConfigurationSection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getParent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::ConfigurationSection::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn add_default(
        &self,
        path: impl Into<String>,
        value: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addDefault",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn default_section(
        &self,
    ) -> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/configuration/ConfigurationSection;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultSection",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::ConfigurationSection::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set(
        &self,
        path: impl Into<String>,
        value: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get(
        &self,
        path: impl Into<String>,
        def: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "get", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }

    pub fn create_section(
        &self,
        path: impl Into<String>,
        map: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = map {
            sig += "Ljava/util/Map;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/configuration/ConfigurationSection;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "createSection", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_string(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<String>>,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getString", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            self.jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
                .to_string_lossy()
                .to_string(),
        ))
    }

    pub fn is_string(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_int(
        &self,
        path: impl Into<String>,
        def: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_int(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_boolean(
        &self,
        path: impl Into<String>,
        def: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBoolean", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_boolean(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBoolean",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_double(
        &self,
        path: impl Into<String>,
        def: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a);
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn is_double(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isDouble",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_long(
        &self,
        path: impl Into<String>,
        def: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a);
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn is_long(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLong",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_list(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isList",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_string_list(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStringList",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn get_object(
        &self,
        path: impl Into<String>,
        clazz: jni::objects::JClass<'mc>,
        def: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/Class;";
        let val_2 = jni::objects::JValueGen::Object(clazz.into());
        args.push(val_2);
        if let Some(a) = def {
            sig += "LT;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")LT;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getObject", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }

    pub fn get_serializable(
        &self,
        path: impl Into<String>,
        clazz: jni::objects::JClass<'mc>,
        def: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/Class;";
        let val_2 = jni::objects::JValueGen::Object(clazz.into());
        args.push(val_2);
        if let Some(a) = def {
            sig += "LT;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")LT;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSerializable", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }

    pub fn get_vector(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::util::Vector<'mc>>>,
    ) -> Result<Option<crate::util::Vector<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Lorg/bukkit/util/Vector;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/util/Vector;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getVector", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::util::Vector::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn is_vector(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVector",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_offline_player(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::OfflinePlayer<'mc>>>,
    ) -> Result<Option<crate::OfflinePlayer<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Lorg/bukkit/OfflinePlayer;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/OfflinePlayer;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOfflinePlayer", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::OfflinePlayer::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn is_offline_player(
        &self,
        path: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isOfflinePlayer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_item_stack(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Lorg/bukkit/inventory/ItemStack;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn is_item_stack(
        &self,
        path: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isItemStack",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_color(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::Color<'mc>>>,
    ) -> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Lorg/bukkit/Color;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/Color;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }

    pub fn is_color(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_location(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Lorg/bukkit/Location;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/Location;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLocation", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }

    pub fn is_location(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLocation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_configuration_section(
        &self,
        path: impl Into<String>,
    ) -> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>
    {
        let sig =
            String::from("(Ljava/lang/String;)Lorg/bukkit/configuration/ConfigurationSection;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfigurationSection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::ConfigurationSection::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn is_configuration_section(
        &self,
        path: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isConfigurationSection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Creates a relative path to the given {@link ConfigurationSection} from
    /// the given relative section.
    ///
    /// You may use this method for any given {@link ConfigurationSection}, not
    /// only {@link MemorySection}.
    pub fn create_path(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        section: impl Into<crate::configuration::ConfigurationSection<'mc>>,
        key: impl Into<String>,
        relative_to: std::option::Option<
            impl Into<crate::configuration::ConfigurationSection<'mc>>,
        >,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/configuration/ConfigurationSection;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(section.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(key.into())?,
        ));
        args.push(val_2);
        if let Some(a) = relative_to {
            sig += "Lorg/bukkit/configuration/ConfigurationSection;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("org/bukkit/configuration/MemorySection");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "createPath", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_comments(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComments",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn get_inline_comments(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInlineComments",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn set_comments(
        &self,
        path: impl Into<String>,
        comments: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in comments {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setComments",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_inline_comments(
        &self,
        path: impl Into<String>,
        comments: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in comments {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInlineComments",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> std::string::ToString for MemorySection<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling MemorySection.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::configuration::ConfigurationSection<'mc>> for MemorySection<'mc> {
    fn into(self) -> crate::configuration::ConfigurationSection<'mc> {
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting MemorySection into crate::configuration::ConfigurationSection",
        )
    }
}
#[repr(C)]
pub struct MemoryConfiguration<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MemoryConfiguration<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MemoryConfiguration<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MemoryConfiguration from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/configuration/MemoryConfiguration")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MemoryConfiguration object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MemoryConfiguration<'mc> {
    /// Creates an empty {@link MemoryConfiguration} using the specified {@link
    /// Configuration} as a source for all default values.
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        defaults: std::option::Option<impl Into<crate::configuration::Configuration<'mc>>>,
    ) -> Result<crate::configuration::MemoryConfiguration<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = defaults {
            sig += "Lorg/bukkit/configuration/Configuration;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/configuration/MemoryConfiguration");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::configuration::MemoryConfiguration::from_raw(&jni, res)
    }

    pub fn add_defaults(
        &self,
        defaults: impl Into<crate::configuration::Configuration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/configuration/Configuration;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(defaults.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addDefaults", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_defaults(
        &self,
        defaults: impl Into<crate::configuration::Configuration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/configuration/Configuration;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(defaults.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDefaults",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn defaults(
        &self,
    ) -> Result<Option<crate::configuration::Configuration<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/configuration/Configuration;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDefaults", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::Configuration::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn parent(
        &self,
    ) -> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/configuration/ConfigurationSection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getParent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::ConfigurationSection::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn options(
        &self,
    ) -> Result<crate::configuration::MemoryConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/configuration/MemoryConfigurationOptions;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "options", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the default value of the given path as provided.
    ///
    /// If no source {@link Configuration} was provided as a default
    /// collection, then a new {@link MemoryConfiguration} will be created to
    /// hold the new default value.
    ///
    /// If value is null, the value will be removed from the default
    /// Configuration source.
    pub fn add_default(
        &self,
        path: impl Into<String>,
        value: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addDefault",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: org.bukkit.configuration.MemorySection ( ['addDefaults', 'setDefaults', 'getDefaults', 'getParent', 'options', 'addDefault'])

    pub fn get_keys(
        &self,
        deep: bool,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Set;");
        let val_1 = jni::objects::JValueGen::Bool(deep.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKeys",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_values(
        &self,
        deep: bool,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Map;");
        let val_1 = jni::objects::JValueGen::Bool(deep.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getValues",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains(
        &self,
        path: impl Into<String>,
        ignore_default: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = ignore_default {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_set(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.is_set(path)
    }

    pub fn current_path(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.current_path()
    }

    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.name()
    }

    pub fn root(
        &self,
    ) -> Result<Option<crate::configuration::Configuration<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.root()
    }

    pub fn default_section(
        &self,
    ) -> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.default_section()
    }

    pub fn set(
        &self,
        path: impl Into<String>,
        value: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.set(path, value)
    }

    pub fn get(
        &self,
        path: impl Into<String>,
        def: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.get(path, def)
    }

    pub fn create_section(
        &self,
        path: impl Into<String>,
        map: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.create_section(path, map)
    }

    pub fn get_string(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<String>>,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.get_string(path, def)
    }

    pub fn is_string(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.is_string(path)
    }

    pub fn get_int(
        &self,
        path: impl Into<String>,
        def: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.get_int(path, def)
    }

    pub fn is_int(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.is_int(path)
    }

    pub fn get_boolean(
        &self,
        path: impl Into<String>,
        def: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.get_boolean(path, def)
    }

    pub fn is_boolean(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.is_boolean(path)
    }

    pub fn get_double(
        &self,
        path: impl Into<String>,
        def: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.get_double(path, def)
    }

    pub fn is_double(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.is_double(path)
    }

    pub fn get_long(
        &self,
        path: impl Into<String>,
        def: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.get_long(path, def)
    }

    pub fn is_long(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.is_long(path)
    }

    pub fn is_list(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.is_list(path)
    }

    pub fn get_string_list(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.get_string_list(path)
    }

    pub fn get_object(
        &self,
        path: impl Into<String>,
        clazz: jni::objects::JClass<'mc>,
        def: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.get_object(path, clazz, def)
    }

    pub fn get_serializable(
        &self,
        path: impl Into<String>,
        clazz: jni::objects::JClass<'mc>,
        def: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.get_serializable(path, clazz, def)
    }

    pub fn get_vector(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::util::Vector<'mc>>>,
    ) -> Result<Option<crate::util::Vector<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.get_vector(path, def)
    }

    pub fn is_vector(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.is_vector(path)
    }

    pub fn get_offline_player(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::OfflinePlayer<'mc>>>,
    ) -> Result<Option<crate::OfflinePlayer<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.get_offline_player(path, def)
    }

    pub fn is_offline_player(
        &self,
        path: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.is_offline_player(path)
    }

    pub fn get_item_stack(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.get_item_stack(path, def)
    }

    pub fn is_item_stack(
        &self,
        path: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.is_item_stack(path)
    }

    pub fn get_color(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::Color<'mc>>>,
    ) -> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.get_color(path, def)
    }

    pub fn is_color(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.is_color(path)
    }

    pub fn get_location(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.get_location(path, def)
    }

    pub fn is_location(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.is_location(path)
    }

    pub fn get_configuration_section(
        &self,
        path: impl Into<String>,
    ) -> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.get_configuration_section(path)
    }

    pub fn is_configuration_section(
        &self,
        path: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.is_configuration_section(path)
    }
    /// Creates a relative path to the given {@link ConfigurationSection} from
    /// the given relative section.
    ///
    /// You may use this method for any given {@link ConfigurationSection}, not
    /// only {@link MemorySection}.
    pub fn create_path(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        section: impl Into<crate::configuration::ConfigurationSection<'mc>>,
        key: impl Into<String>,
        relative_to: std::option::Option<
            impl Into<crate::configuration::ConfigurationSection<'mc>>,
        >,
    ) -> Result<String, Box<dyn std::error::Error>> {
        crate::configuration::MemorySection::create_path(jni, section, key, relative_to)
    }

    pub fn get_comments(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComments",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn get_inline_comments(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInlineComments",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn set_comments(
        &self,
        path: impl Into<String>,
        comments: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in comments {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setComments",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_inline_comments(
        &self,
        path: impl Into<String>,
        comments: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in comments {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInlineComments",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.internal_to_string()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::Configuration<'mc>> for MemoryConfiguration<'mc> {
    fn into(self) -> crate::configuration::Configuration<'mc> {
        crate::configuration::Configuration::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MemoryConfiguration into crate::configuration::Configuration")
    }
}
impl<'mc> Into<crate::configuration::MemorySection<'mc>> for MemoryConfiguration<'mc> {
    fn into(self) -> crate::configuration::MemorySection<'mc> {
        crate::configuration::MemorySection::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MemoryConfiguration into crate::configuration::MemorySection")
    }
}
#[repr(C)]
pub struct InvalidConfigurationException<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InvalidConfigurationException<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InvalidConfigurationException<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InvalidConfigurationException from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/configuration/InvalidConfigurationException",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InvalidConfigurationException object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InvalidConfigurationException<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct ConfigurationOptions<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ConfigurationOptions<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConfigurationOptions<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ConfigurationOptions from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/configuration/ConfigurationOptions")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ConfigurationOptions object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ConfigurationOptions<'mc> {
    /// Returns the {@link Configuration} that this object is responsible for.
    pub fn configuration(
        &self,
    ) -> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/configuration/Configuration;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "configuration", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::Configuration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the char that will be used to separate {@link
    /// ConfigurationSection}s
    ///
    /// This value does not affect how the {@link Configuration} is stored,
    /// only in how you access the data. The default value is '.'.
    pub fn path_separator(
        &self,
        value: std::option::Option<u16>,
    ) -> Result<crate::configuration::ConfigurationOptions<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = value {
            sig += "C";
            let val_1 = jni::objects::JValueGen::Char(a);
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/configuration/ConfigurationOptions;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "pathSeparator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets if the {@link Configuration} should copy values from its default
    /// {@link Configuration} directly.
    ///
    /// If this is true, all values in the default Configuration will be
    /// directly copied, making it impossible to distinguish between values
    /// that were set and values that are provided by default. As a result,
    /// {@link ConfigurationSection#contains(java.lang.String)} will always
    /// return the same value as {@link
    /// ConfigurationSection#isSet(java.lang.String)}. The default value is
    /// false.
    pub fn copy_defaults(
        &self,
        value: std::option::Option<bool>,
    ) -> Result<crate::configuration::ConfigurationOptions<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = value {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/configuration/ConfigurationOptions;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyDefaults", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct ConfigurationSection<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ConfigurationSection<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConfigurationSection<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ConfigurationSection from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/configuration/ConfigurationSection")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ConfigurationSection object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ConfigurationSection<'mc> {
    /// Gets a set containing all keys in this section.
    ///
    /// If deep is set to true, then this will contain all the keys within any
    /// child {@link ConfigurationSection}s (and their children, etc). These
    /// will be in a valid path notation for you to use.
    ///
    /// If deep is set to false, then this will contain only the keys of any
    /// direct children, and not their own children.
    pub fn get_keys(
        &self,
        deep: bool,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Set;");
        let val_1 = jni::objects::JValueGen::Bool(deep.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKeys",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a Map containing all keys and their values for this section.
    ///
    /// If deep is set to true, then this will contain all the keys and values
    /// within any child {@link ConfigurationSection}s (and their children,
    /// etc). These keys will be in a valid path notation for you to use.
    ///
    /// If deep is set to false, then this will contain only the keys and
    /// values of any direct children, and not their own children.
    pub fn get_values(
        &self,
        deep: bool,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Map;");
        let val_1 = jni::objects::JValueGen::Bool(deep.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getValues",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this {@link ConfigurationSection} contains the given path.
    ///
    /// If the value for the requested path does not exist, the boolean parameter
    /// of true has been specified, a default value for the path exists, this
    /// will return true.
    ///
    /// If a boolean parameter of false has been specified, true will only be
    /// returned if there is a set value for the specified path.
    pub fn contains(
        &self,
        path: impl Into<String>,
        ignore_default: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = ignore_default {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if this {@link ConfigurationSection} has a value set for the
    /// given path.
    ///
    /// If the value for the requested path does not exist but a default value
    /// has been specified, this will still return false.
    pub fn is_set(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSet",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the path of this {@link ConfigurationSection} from its root {@link
    /// Configuration}
    ///
    /// For any {@link Configuration} themselves, this will return an empty
    /// string.
    ///
    /// If the section is no longer contained within its root for any reason,
    /// such as being replaced with a different value, this may return null.
    ///
    /// To retrieve the single name of this section, that is, the final part of
    /// the path returned by this method, you may use {@link #getName()}.
    pub fn current_path(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCurrentPath", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            self.jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
                .to_string_lossy()
                .to_string(),
        ))
    }
    /// Gets the name of this individual {@link ConfigurationSection}, in the
    /// path.
    ///
    /// This will always be the final part of {@link #getCurrentPath()}, unless
    /// the section is orphaned.
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
    /// Gets the root {@link Configuration} that contains this {@link
    /// ConfigurationSection}
    ///
    /// For any {@link Configuration} themselves, this will return its own
    /// object.
    ///
    /// If the section is no longer contained within its root for any reason,
    /// such as being replaced with a different value, this may return null.
    pub fn root(
        &self,
    ) -> Result<Option<crate::configuration::Configuration<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/configuration/Configuration;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRoot", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::Configuration::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the parent {@link ConfigurationSection} that directly contains
    /// this {@link ConfigurationSection}.
    ///
    /// For any {@link Configuration} themselves, this will return null.
    ///
    /// If the section is no longer contained within its parent for any reason,
    /// such as being replaced with a different value, this may return null.
    pub fn parent(
        &self,
    ) -> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/configuration/ConfigurationSection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getParent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::ConfigurationSection::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the requested Object by path, returning a default value if not
    /// found.
    ///
    /// If the Object does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get(
        &self,
        path: impl Into<String>,
        def: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "get", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    /// Sets the specified path to the given value.
    ///
    /// If value is null, the entry will be removed. Any existing entry will be
    /// replaced, regardless of what the new value is.
    ///
    /// Some implementations may have limitations on what you may store. See
    /// their individual javadocs for details. No implementations should allow
    /// you to store {@link Configuration}s or {@link ConfigurationSection}s,
    /// please use {@link #createSection(java.lang.String)} for that.
    pub fn set(
        &self,
        path: impl Into<String>,
        value: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a {@link ConfigurationSection} at the specified path, with
    /// specified values.
    ///
    /// Any value that was previously set at this path will be overwritten. If
    /// the previous value was itself a {@link ConfigurationSection}, it will
    /// be orphaned.
    pub fn create_section(
        &self,
        path: impl Into<String>,
        map: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = map {
            sig += "Ljava/util/Map;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/configuration/ConfigurationSection;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "createSection", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the requested String by path, returning a default value if not
    /// found.
    ///
    /// If the String does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_string(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<String>>,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getString", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            self.jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
                .to_string_lossy()
                .to_string(),
        ))
    }
    /// Checks if the specified path is a String.
    ///
    /// If the path exists but is not a String, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a String and return appropriately.
    pub fn is_string(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested int by path, returning a default value if not found.
    ///
    /// If the int does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_int(
        &self,
        path: impl Into<String>,
        def: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Checks if the specified path is an int.
    ///
    /// If the path exists but is not a int, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a int and return appropriately.
    pub fn is_int(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested boolean by path, returning a default value if not
    /// found.
    ///
    /// If the boolean does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_boolean(
        &self,
        path: impl Into<String>,
        def: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBoolean", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if the specified path is a boolean.
    ///
    /// If the path exists but is not a boolean, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a boolean and return appropriately.
    pub fn is_boolean(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBoolean",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested double by path, returning a default value if not
    /// found.
    ///
    /// If the double does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_double(
        &self,
        path: impl Into<String>,
        def: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a);
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Checks if the specified path is a double.
    ///
    /// If the path exists but is not a double, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a double and return appropriately.
    pub fn is_double(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isDouble",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested long by path, returning a default value if not
    /// found.
    ///
    /// If the long does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_long(
        &self,
        path: impl Into<String>,
        def: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a);
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    /// Checks if the specified path is a long.
    ///
    /// If the path exists but is not a long, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a long and return appropriately.
    pub fn is_long(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLong",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if the specified path is a List.
    ///
    /// If the path exists but is not a List, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a List and return appropriately.
    pub fn is_list(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isList",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested List of String by path.
    ///
    /// If the List does not exist but a default value has been specified, this
    /// will return the default value. If the List does not exist and no
    /// default value was specified, this will return an empty List.
    ///
    /// This method will attempt to cast any values into a String if possible,
    /// but may miss any values out if they are not compatible.
    pub fn get_string_list(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStringList",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Gets the requested object at the given path, returning a default value if
    /// not found
    /// If the Object does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    /// <b>Note:</b> For example #getObject(path, String.class, def) is
    /// <b>not</b> equivalent to
    /// {@link #getString(String, String) #getString(path, def)} because
    /// {@link #getString(String, String) #getString(path, def)} converts
    /// internally all Objects to Strings. However, #getObject(path,
    /// Boolean.class, def) is equivalent to {@link #getBoolean(String, boolean) #getBoolean(path,
    /// def)} for example.
    pub fn get_object(
        &self,
        path: impl Into<String>,
        clazz: jni::objects::JClass<'mc>,
        def: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/Class;";
        let val_2 = jni::objects::JValueGen::Object(clazz.into());
        args.push(val_2);
        if let Some(a) = def {
            sig += "LT;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")LT;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getObject", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    /// Gets the requested {@link ConfigurationSerializable} object at the given
    /// path, returning a default value if not found
    /// If the Object does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_serializable(
        &self,
        path: impl Into<String>,
        clazz: jni::objects::JClass<'mc>,
        def: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/Class;";
        let val_2 = jni::objects::JValueGen::Object(clazz.into());
        args.push(val_2);
        if let Some(a) = def {
            sig += "LT;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")LT;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSerializable", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    /// Gets the requested {@link Vector} by path, returning a default value if
    /// not found.
    ///
    /// If the Vector does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_vector(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::util::Vector<'mc>>>,
    ) -> Result<Option<crate::util::Vector<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Lorg/bukkit/util/Vector;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/util/Vector;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getVector", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::util::Vector::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Checks if the specified path is a Vector.
    ///
    /// If the path exists but is not a Vector, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a Vector and return appropriately.
    pub fn is_vector(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVector",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested {@link OfflinePlayer} by path, returning a default
    /// value if not found.
    ///
    /// If the OfflinePlayer does not exist then the specified default value
    /// will returned regardless of if a default has been identified in the
    /// root {@link Configuration}.
    pub fn get_offline_player(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::OfflinePlayer<'mc>>>,
    ) -> Result<Option<crate::OfflinePlayer<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Lorg/bukkit/OfflinePlayer;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/OfflinePlayer;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOfflinePlayer", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::OfflinePlayer::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Checks if the specified path is an OfflinePlayer.
    ///
    /// If the path exists but is not a OfflinePlayer, this will return false.
    /// If the path does not exist, this will return false. If the path does
    /// not exist but a default value has been specified, this will check if
    /// that default value is a OfflinePlayer and return appropriately.
    pub fn is_offline_player(
        &self,
        path: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isOfflinePlayer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested {@link ItemStack} by path, returning a default value
    /// if not found.
    ///
    /// If the ItemStack does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_item_stack(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Lorg/bukkit/inventory/ItemStack;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Checks if the specified path is an ItemStack.
    ///
    /// If the path exists but is not a ItemStack, this will return false. If
    /// the path does not exist, this will return false. If the path does not
    /// exist but a default value has been specified, this will check if that
    /// default value is a ItemStack and return appropriately.
    pub fn is_item_stack(
        &self,
        path: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isItemStack",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested {@link Color} by path, returning a default value if
    /// not found.
    ///
    /// If the Color does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_color(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::Color<'mc>>>,
    ) -> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Lorg/bukkit/Color;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/Color;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Checks if the specified path is a Color.
    ///
    /// If the path exists but is not a Color, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a Color and return appropriately.
    pub fn is_color(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested {@link Location} by path, returning a default value if
    /// not found.
    ///
    /// If the Location does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_location(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Lorg/bukkit/Location;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/Location;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLocation", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Checks if the specified path is a Location.
    ///
    /// If the path exists but is not a Location, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a Location and return appropriately.
    pub fn is_location(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLocation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested ConfigurationSection by path.
    ///
    /// If the ConfigurationSection does not exist but a default value has been
    /// specified, this will return the default value. If the
    /// ConfigurationSection does not exist and no default value was specified,
    /// this will return null.
    pub fn get_configuration_section(
        &self,
        path: impl Into<String>,
    ) -> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>
    {
        let sig =
            String::from("(Ljava/lang/String;)Lorg/bukkit/configuration/ConfigurationSection;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfigurationSection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::ConfigurationSection::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Checks if the specified path is a ConfigurationSection.
    ///
    /// If the path exists but is not a ConfigurationSection, this will return
    /// false. If the path does not exist, this will return false. If the path
    /// does not exist but a default value has been specified, this will check
    /// if that default value is a ConfigurationSection and return
    /// appropriately.
    pub fn is_configuration_section(
        &self,
        path: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isConfigurationSection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the equivalent {@link ConfigurationSection} from the default
    /// {@link Configuration} defined in {@link #getRoot()}.
    ///
    /// If the root contains no defaults, or the defaults doesn't contain a
    /// value for this path, or the value at this path is not a {@link
    /// ConfigurationSection} then this will return null.
    pub fn default_section(
        &self,
    ) -> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/configuration/ConfigurationSection;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultSection",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::ConfigurationSection::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the default value in the root at the given path as provided.
    ///
    /// If no source {@link Configuration} was provided as a default
    /// collection, then a new {@link MemoryConfiguration} will be created to
    /// hold the new default value.
    ///
    /// If value is null, the value will be removed from the default
    /// Configuration source.
    ///
    /// If the value as returned by {@link #getDefaultSection()} is null, then
    /// this will create a new section at the path, replacing anything that may
    /// have existed there previously.
    pub fn add_default(
        &self,
        path: impl Into<String>,
        value: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addDefault",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the requested comment list by path.
    ///
    /// If no comments exist, an empty list will be returned. A null entry
    /// represents an empty line and an empty String represents an empty comment
    /// line.
    pub fn get_comments(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComments",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Gets the requested inline comment list by path.
    ///
    /// If no comments exist, an empty list will be returned. A null entry
    /// represents an empty line and an empty String represents an empty comment
    /// line.
    pub fn get_inline_comments(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInlineComments",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Sets the comment list at the specified path.
    ///
    /// If value is null, the comments will be removed. A null entry is an empty
    /// line and an empty String entry is an empty comment line. If the path does
    /// not exist, no comments will be set. Any existing comments will be
    /// replaced, regardless of what the new comments are.
    ///
    /// Some implementations may have limitations on what persists. See their
    /// individual javadocs for details.
    pub fn set_comments(
        &self,
        path: impl Into<String>,
        comments: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in comments {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setComments",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the inline comment list at the specified path.
    ///
    /// If value is null, the comments will be removed. A null entry is an empty
    /// line and an empty String entry is an empty comment line. If the path does
    /// not exist, no comment will be set. Any existing comments will be
    /// replaced, regardless of what the new comments are.
    ///
    /// Some implementations may have limitations on what persists. See their
    /// individual javadocs for details.
    pub fn set_inline_comments(
        &self,
        path: impl Into<String>,
        comments: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in comments {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInlineComments",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct Configuration<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Configuration<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Configuration<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Configuration from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/configuration/Configuration")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Configuration object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Configuration<'mc> {
    /// Sets the default value of the given path as provided.
    ///
    /// If no source {@link Configuration} was provided as a default
    /// collection, then a new {@link MemoryConfiguration} will be created to
    /// hold the new default value.
    ///
    /// If value is null, the value will be removed from the default
    /// Configuration source.
    pub fn add_default(
        &self,
        path: impl Into<String>,
        value: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addDefault",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the default values of the given paths as provided.
    ///
    /// If no source {@link Configuration} was provided as a default
    /// collection, then a new {@link MemoryConfiguration} will be created to
    /// hold the new default value.
    ///
    /// This method will not hold a reference to the specified Configuration,
    /// nor will it automatically update if that Configuration ever changes. If
    /// you require this, you should set the default source with {@link
    /// #setDefaults(org.bukkit.configuration.Configuration)}.
    pub fn add_defaults(
        &self,
        defaults: impl Into<crate::configuration::Configuration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/configuration/Configuration;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(defaults.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addDefaults", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the source of all default values for this {@link Configuration}.
    ///
    /// If a previous source was set, or previous default values were defined,
    /// then they will not be copied to the new source.
    pub fn set_defaults(
        &self,
        defaults: impl Into<crate::configuration::Configuration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/configuration/Configuration;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(defaults.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDefaults",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the source {@link Configuration} for this configuration.
    ///
    /// If no configuration source was set, but default values were added, then
    /// a {@link MemoryConfiguration} will be returned. If no source was set
    /// and no defaults were set, then this method will return null.
    pub fn defaults(
        &self,
    ) -> Result<Option<crate::configuration::Configuration<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/configuration/Configuration;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDefaults", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::Configuration::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the {@link ConfigurationOptions} for this {@link Configuration}.
    ///
    /// All setters through this method are chainable.
    pub fn options(
        &self,
    ) -> Result<crate::configuration::ConfigurationOptions<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/configuration/ConfigurationOptions;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "options", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a set containing all keys in this section.
    ///
    /// If deep is set to true, then this will contain all the keys within any
    /// child {@link ConfigurationSection}s (and their children, etc). These
    /// will be in a valid path notation for you to use.
    ///
    /// If deep is set to false, then this will contain only the keys of any
    /// direct children, and not their own children.
    pub fn get_keys(
        &self,
        deep: bool,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Set;");
        let val_1 = jni::objects::JValueGen::Bool(deep.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKeys",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a Map containing all keys and their values for this section.
    ///
    /// If deep is set to true, then this will contain all the keys and values
    /// within any child {@link ConfigurationSection}s (and their children,
    /// etc). These keys will be in a valid path notation for you to use.
    ///
    /// If deep is set to false, then this will contain only the keys and
    /// values of any direct children, and not their own children.
    pub fn get_values(
        &self,
        deep: bool,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Map;");
        let val_1 = jni::objects::JValueGen::Bool(deep.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getValues",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this {@link ConfigurationSection} contains the given path.
    ///
    /// If the value for the requested path does not exist, the boolean parameter
    /// of true has been specified, a default value for the path exists, this
    /// will return true.
    ///
    /// If a boolean parameter of false has been specified, true will only be
    /// returned if there is a set value for the specified path.
    pub fn contains(
        &self,
        path: impl Into<String>,
        ignore_default: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = ignore_default {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if this {@link ConfigurationSection} has a value set for the
    /// given path.
    ///
    /// If the value for the requested path does not exist but a default value
    /// has been specified, this will still return false.
    pub fn is_set(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSet",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the path of this {@link ConfigurationSection} from its root {@link
    /// Configuration}
    ///
    /// For any {@link Configuration} themselves, this will return an empty
    /// string.
    ///
    /// If the section is no longer contained within its root for any reason,
    /// such as being replaced with a different value, this may return null.
    ///
    /// To retrieve the single name of this section, that is, the final part of
    /// the path returned by this method, you may use {@link #getName()}.
    pub fn current_path(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCurrentPath", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            self.jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
                .to_string_lossy()
                .to_string(),
        ))
    }
    /// Gets the name of this individual {@link ConfigurationSection}, in the
    /// path.
    ///
    /// This will always be the final part of {@link #getCurrentPath()}, unless
    /// the section is orphaned.
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
    /// Gets the root {@link Configuration} that contains this {@link
    /// ConfigurationSection}
    ///
    /// For any {@link Configuration} themselves, this will return its own
    /// object.
    ///
    /// If the section is no longer contained within its root for any reason,
    /// such as being replaced with a different value, this may return null.
    pub fn root(
        &self,
    ) -> Result<Option<crate::configuration::Configuration<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/configuration/Configuration;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRoot", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::Configuration::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the parent {@link ConfigurationSection} that directly contains
    /// this {@link ConfigurationSection}.
    ///
    /// For any {@link Configuration} themselves, this will return null.
    ///
    /// If the section is no longer contained within its parent for any reason,
    /// such as being replaced with a different value, this may return null.
    pub fn parent(
        &self,
    ) -> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/configuration/ConfigurationSection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getParent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::ConfigurationSection::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the requested Object by path, returning a default value if not
    /// found.
    ///
    /// If the Object does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get(
        &self,
        path: impl Into<String>,
        def: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "get", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    /// Sets the specified path to the given value.
    ///
    /// If value is null, the entry will be removed. Any existing entry will be
    /// replaced, regardless of what the new value is.
    ///
    /// Some implementations may have limitations on what you may store. See
    /// their individual javadocs for details. No implementations should allow
    /// you to store {@link Configuration}s or {@link ConfigurationSection}s,
    /// please use {@link #createSection(java.lang.String)} for that.
    pub fn set(
        &self,
        path: impl Into<String>,
        value: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a {@link ConfigurationSection} at the specified path, with
    /// specified values.
    ///
    /// Any value that was previously set at this path will be overwritten. If
    /// the previous value was itself a {@link ConfigurationSection}, it will
    /// be orphaned.
    pub fn create_section(
        &self,
        path: impl Into<String>,
        map: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = map {
            sig += "Ljava/util/Map;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/configuration/ConfigurationSection;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "createSection", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the requested String by path, returning a default value if not
    /// found.
    ///
    /// If the String does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_string(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<String>>,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getString", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            self.jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
                .to_string_lossy()
                .to_string(),
        ))
    }
    /// Checks if the specified path is a String.
    ///
    /// If the path exists but is not a String, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a String and return appropriately.
    pub fn is_string(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested int by path, returning a default value if not found.
    ///
    /// If the int does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_int(
        &self,
        path: impl Into<String>,
        def: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Checks if the specified path is an int.
    ///
    /// If the path exists but is not a int, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a int and return appropriately.
    pub fn is_int(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested boolean by path, returning a default value if not
    /// found.
    ///
    /// If the boolean does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_boolean(
        &self,
        path: impl Into<String>,
        def: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBoolean", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if the specified path is a boolean.
    ///
    /// If the path exists but is not a boolean, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a boolean and return appropriately.
    pub fn is_boolean(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBoolean",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested double by path, returning a default value if not
    /// found.
    ///
    /// If the double does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_double(
        &self,
        path: impl Into<String>,
        def: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a);
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Checks if the specified path is a double.
    ///
    /// If the path exists but is not a double, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a double and return appropriately.
    pub fn is_double(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isDouble",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested long by path, returning a default value if not
    /// found.
    ///
    /// If the long does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_long(
        &self,
        path: impl Into<String>,
        def: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a);
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    /// Checks if the specified path is a long.
    ///
    /// If the path exists but is not a long, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a long and return appropriately.
    pub fn is_long(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLong",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if the specified path is a List.
    ///
    /// If the path exists but is not a List, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a List and return appropriately.
    pub fn is_list(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isList",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested List of String by path.
    ///
    /// If the List does not exist but a default value has been specified, this
    /// will return the default value. If the List does not exist and no
    /// default value was specified, this will return an empty List.
    ///
    /// This method will attempt to cast any values into a String if possible,
    /// but may miss any values out if they are not compatible.
    pub fn get_string_list(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStringList",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Gets the requested object at the given path, returning a default value if
    /// not found
    /// If the Object does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    /// <b>Note:</b> For example #getObject(path, String.class, def) is
    /// <b>not</b> equivalent to
    /// {@link #getString(String, String) #getString(path, def)} because
    /// {@link #getString(String, String) #getString(path, def)} converts
    /// internally all Objects to Strings. However, #getObject(path,
    /// Boolean.class, def) is equivalent to {@link #getBoolean(String, boolean) #getBoolean(path,
    /// def)} for example.
    pub fn get_object(
        &self,
        path: impl Into<String>,
        clazz: jni::objects::JClass<'mc>,
        def: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/Class;";
        let val_2 = jni::objects::JValueGen::Object(clazz.into());
        args.push(val_2);
        if let Some(a) = def {
            sig += "LT;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")LT;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getObject", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    /// Gets the requested {@link ConfigurationSerializable} object at the given
    /// path, returning a default value if not found
    /// If the Object does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_serializable(
        &self,
        path: impl Into<String>,
        clazz: jni::objects::JClass<'mc>,
        def: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/Class;";
        let val_2 = jni::objects::JValueGen::Object(clazz.into());
        args.push(val_2);
        if let Some(a) = def {
            sig += "LT;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")LT;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSerializable", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    /// Gets the requested {@link Vector} by path, returning a default value if
    /// not found.
    ///
    /// If the Vector does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_vector(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::util::Vector<'mc>>>,
    ) -> Result<Option<crate::util::Vector<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Lorg/bukkit/util/Vector;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/util/Vector;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getVector", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::util::Vector::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Checks if the specified path is a Vector.
    ///
    /// If the path exists but is not a Vector, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a Vector and return appropriately.
    pub fn is_vector(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVector",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested {@link OfflinePlayer} by path, returning a default
    /// value if not found.
    ///
    /// If the OfflinePlayer does not exist then the specified default value
    /// will returned regardless of if a default has been identified in the
    /// root {@link Configuration}.
    pub fn get_offline_player(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::OfflinePlayer<'mc>>>,
    ) -> Result<Option<crate::OfflinePlayer<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Lorg/bukkit/OfflinePlayer;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/OfflinePlayer;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOfflinePlayer", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::OfflinePlayer::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Checks if the specified path is an OfflinePlayer.
    ///
    /// If the path exists but is not a OfflinePlayer, this will return false.
    /// If the path does not exist, this will return false. If the path does
    /// not exist but a default value has been specified, this will check if
    /// that default value is a OfflinePlayer and return appropriately.
    pub fn is_offline_player(
        &self,
        path: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isOfflinePlayer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested {@link ItemStack} by path, returning a default value
    /// if not found.
    ///
    /// If the ItemStack does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_item_stack(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Lorg/bukkit/inventory/ItemStack;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Checks if the specified path is an ItemStack.
    ///
    /// If the path exists but is not a ItemStack, this will return false. If
    /// the path does not exist, this will return false. If the path does not
    /// exist but a default value has been specified, this will check if that
    /// default value is a ItemStack and return appropriately.
    pub fn is_item_stack(
        &self,
        path: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isItemStack",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested {@link Color} by path, returning a default value if
    /// not found.
    ///
    /// If the Color does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_color(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::Color<'mc>>>,
    ) -> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Lorg/bukkit/Color;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/Color;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Checks if the specified path is a Color.
    ///
    /// If the path exists but is not a Color, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a Color and return appropriately.
    pub fn is_color(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested {@link Location} by path, returning a default value if
    /// not found.
    ///
    /// If the Location does not exist then the specified default value will
    /// returned regardless of if a default has been identified in the root
    /// {@link Configuration}.
    pub fn get_location(
        &self,
        path: impl Into<String>,
        def: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        args.push(val_1);
        if let Some(a) = def {
            sig += "Lorg/bukkit/Location;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/Location;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLocation", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Checks if the specified path is a Location.
    ///
    /// If the path exists but is not a Location, this will return false. If the
    /// path does not exist, this will return false. If the path does not exist
    /// but a default value has been specified, this will check if that default
    /// value is a Location and return appropriately.
    pub fn is_location(&self, path: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLocation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the requested ConfigurationSection by path.
    ///
    /// If the ConfigurationSection does not exist but a default value has been
    /// specified, this will return the default value. If the
    /// ConfigurationSection does not exist and no default value was specified,
    /// this will return null.
    pub fn get_configuration_section(
        &self,
        path: impl Into<String>,
    ) -> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>
    {
        let sig =
            String::from("(Ljava/lang/String;)Lorg/bukkit/configuration/ConfigurationSection;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfigurationSection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::ConfigurationSection::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Checks if the specified path is a ConfigurationSection.
    ///
    /// If the path exists but is not a ConfigurationSection, this will return
    /// false. If the path does not exist, this will return false. If the path
    /// does not exist but a default value has been specified, this will check
    /// if that default value is a ConfigurationSection and return
    /// appropriately.
    pub fn is_configuration_section(
        &self,
        path: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isConfigurationSection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the equivalent {@link ConfigurationSection} from the default
    /// {@link Configuration} defined in {@link #getRoot()}.
    ///
    /// If the root contains no defaults, or the defaults doesn't contain a
    /// value for this path, or the value at this path is not a {@link
    /// ConfigurationSection} then this will return null.
    pub fn default_section(
        &self,
    ) -> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/configuration/ConfigurationSection;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultSection",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::ConfigurationSection::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the requested comment list by path.
    ///
    /// If no comments exist, an empty list will be returned. A null entry
    /// represents an empty line and an empty String represents an empty comment
    /// line.
    pub fn get_comments(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComments",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Gets the requested inline comment list by path.
    ///
    /// If no comments exist, an empty list will be returned. A null entry
    /// represents an empty line and an empty String represents an empty comment
    /// line.
    pub fn get_inline_comments(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInlineComments",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Sets the comment list at the specified path.
    ///
    /// If value is null, the comments will be removed. A null entry is an empty
    /// line and an empty String entry is an empty comment line. If the path does
    /// not exist, no comments will be set. Any existing comments will be
    /// replaced, regardless of what the new comments are.
    ///
    /// Some implementations may have limitations on what persists. See their
    /// individual javadocs for details.
    pub fn set_comments(
        &self,
        path: impl Into<String>,
        comments: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in comments {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setComments",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the inline comment list at the specified path.
    ///
    /// If value is null, the comments will be removed. A null entry is an empty
    /// line and an empty String entry is an empty comment line. If the path does
    /// not exist, no comment will be set. Any existing comments will be
    /// replaced, regardless of what the new comments are.
    ///
    /// Some implementations may have limitations on what persists. See their
    /// individual javadocs for details.
    pub fn set_inline_comments(
        &self,
        path: impl Into<String>,
        comments: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(path.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in comments {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInlineComments",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::ConfigurationSection<'mc>> for Configuration<'mc> {
    fn into(self) -> crate::configuration::ConfigurationSection<'mc> {
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting Configuration into crate::configuration::ConfigurationSection",
        )
    }
}
#[repr(C)]
pub struct MemoryConfigurationOptions<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MemoryConfigurationOptions<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MemoryConfigurationOptions<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate MemoryConfigurationOptions from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/configuration/MemoryConfigurationOptions")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MemoryConfigurationOptions object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MemoryConfigurationOptions<'mc> {
    pub fn configuration(
        &self,
    ) -> Result<crate::configuration::MemoryConfiguration<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/configuration/MemoryConfiguration;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "configuration", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::MemoryConfiguration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn copy_defaults(
        &self,
        value: bool,
    ) -> Result<crate::configuration::MemoryConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Z)Lorg/bukkit/configuration/MemoryConfigurationOptions;");
        let val_1 = jni::objects::JValueGen::Bool(value.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyDefaults",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn path_separator(
        &self,
        value: u16,
    ) -> Result<crate::configuration::MemoryConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(C)Lorg/bukkit/configuration/MemoryConfigurationOptions;");
        let val_1 = jni::objects::JValueGen::Char(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pathSeparator",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.configuration.ConfigurationOptions ( ['configuration', 'copyDefaults', 'pathSeparator'])

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::ConfigurationOptions<'mc>>
    for MemoryConfigurationOptions<'mc>
{
    fn into(self) -> crate::configuration::ConfigurationOptions<'mc> {
        crate::configuration::ConfigurationOptions::from_raw(&self.jni_ref(), self.1).expect("Error converting MemoryConfigurationOptions into crate::configuration::ConfigurationOptions")
    }
}
pub mod file;
pub mod serialization;
