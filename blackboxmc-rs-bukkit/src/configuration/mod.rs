#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Various settings for controlling the input and output of a <a title="class in org.bukkit.configuration" href="MemoryConfiguration.html"><code>MemoryConfiguration</code></a>
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
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="ConfigurationOptions.html#pathSeparator(char)">ConfigurationOptions</a></code></span>
    /// Sets the char that will be used to separate <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>s
    /// <p>This value does not affect how the <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> is stored, only in how you access the data. The default value is '.'.</p>
    pub fn path_separator_with_char(
        &self,
        arg0: u16,
    ) -> Result<crate::configuration::MemoryConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += ")Lorg/bukkit/configuration/MemoryConfigurationOptions;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "pathSeparator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="ConfigurationOptions.html#copyDefaults(boolean)">ConfigurationOptions</a></code></span>
    /// Sets if the <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> should copy values from its default <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> directly.
    /// <p>If this is true, all values in the default Configuration will be directly copied, making it impossible to distinguish between values that were set and values that are provided by default. As a result, <a href="ConfigurationSection.html#contains(java.lang.String)"><code>ConfigurationSection.contains(java.lang.String)</code></a> will always return the same value as <a href="ConfigurationSection.html#isSet(java.lang.String)"><code>ConfigurationSection.isSet(java.lang.String)</code></a>. The default value is false.</p>
    pub fn copy_defaults_with_boolean(
        &self,
        arg0: bool,
    ) -> Result<crate::configuration::MemoryConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        args.push(val_1);
        sig += ")Lorg/bukkit/configuration/MemoryConfigurationOptions;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyDefaults", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn configuration(
        &self,
    ) -> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/configuration/Configuration;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "configuration", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::Configuration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: ConfigurationOptions
    // SUPER CLASS: Object

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
/// A type of <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> that is stored in memory.
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
    pub fn get_string_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getKeys(boolean)">ConfigurationSection</a></code></span>
    /// Gets a set containing all keys in this section.
    /// <p>If deep is set to true, then this will contain all the keys within any child <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>s (and their children, etc). These will be in a valid path notation for you to use.</p>
    /// <p>If deep is set to false, then this will contain only the keys of any direct children, and not their own children.</p>
    pub fn get_keys(
        &self,
        arg0: bool,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Set;");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
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

    pub fn get_color_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::Color<'mc>>>,
    ) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item_stack_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_offline_player_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::OfflinePlayer<'mc>>>,
    ) -> Result<crate::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::OfflinePlayer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getValues(boolean)">ConfigurationSection</a></code></span>
    /// Gets a Map containing all keys and their values for this section.
    /// <p>If deep is set to true, then this will contain all the keys and values within any child <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>s (and their children, etc). These keys will be in a valid path notation for you to use.</p>
    /// <p>If deep is set to false, then this will contain only the keys and values of any direct children, and not their own children.</p>
    pub fn get_values(
        &self,
        arg0: bool,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Map;");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
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

    pub fn create_path_with_configuration_section(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::configuration::ConfigurationSection<'mc>>,
        arg1: impl Into<String>,
        arg2: std::option::Option<impl Into<crate::configuration::ConfigurationSection<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/configuration/ConfigurationSection;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg1.into())?,
        ));
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/configuration/ConfigurationSection;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "createPath", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
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

    pub fn add_default(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(arg1);
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

    pub fn is_configuration_section(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn get_configuration_section(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/String;)Lorg/bukkit/configuration/ConfigurationSection;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfigurationSection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn create_section_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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

    pub fn is_string(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_int(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_boolean(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_double(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_long(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_list(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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
        arg0: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn get_serializable_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JClass<'mc>,
        arg2: std::option::Option<
            impl Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>,
        >,
    ) -> Result<
        crate::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/Class;";
        let val_2 = jni::objects::JValueGen::Object(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSerializable", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn get_vector_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::util::Vector<'mc>>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_vector(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_offline_player(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_item_stack(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_color(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_location(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn get_comments(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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
        arg0: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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
        arg0: impl Into<String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            let map_val_0 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(v.into())?,
            ));
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
        arg0: impl Into<String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            let map_val_0 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(v.into())?,
            ));
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

    pub fn get_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "get", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
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

    pub fn get_boolean_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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

    pub fn get_int_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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

    pub fn get_long_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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

    pub fn get_double_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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

    pub fn contains_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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

    pub fn get_location_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn parent(
        &self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/configuration/ConfigurationSection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getParent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(arg1);
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

    pub fn is_set(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn get_object_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JClass<'mc>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/Class;";
        let val_2 = jni::objects::JValueGen::Object(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getObject", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    // SUPER CLASS: Object

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
/// Represents a source of configurable options and settings
///
/// This is a representation of an abstract class.
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
    pub fn add_default(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(arg1);
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

    pub fn add_defaults_with_configuration(
        &self,
        arg0: impl Into<crate::configuration::Configuration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/configuration/Configuration;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
        arg0: impl Into<crate::configuration::Configuration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/configuration/Configuration;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn get_string_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn get_keys(
        &self,
        arg0: bool,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Set;");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
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
    pub fn get_color_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::Color<'mc>>>,
    ) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_item_stack_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_offline_player_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::OfflinePlayer<'mc>>>,
    ) -> Result<crate::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::OfflinePlayer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_values(
        &self,
        arg0: bool,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Map;");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
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
    pub fn default_section(
        &self,
    ) -> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>
    {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.default_section()
    }
    pub fn current_path(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.current_path()
    }
    pub fn is_configuration_section(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_configuration_section(arg0)
    }
    pub fn get_configuration_section(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.get_configuration_section(arg0)
    }
    pub fn create_section_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
    pub fn is_string(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_string(arg0)
    }
    pub fn is_int(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_int(arg0)
    }
    pub fn is_boolean(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_boolean(arg0)
    }
    pub fn is_double(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_double(arg0)
    }
    pub fn is_long(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_long(arg0)
    }
    pub fn is_list(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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
        arg0: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.get_string_list(arg0)
    }
    pub fn get_serializable_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JClass<'mc>,
        arg2: std::option::Option<
            impl Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>,
        >,
    ) -> Result<
        crate::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/Class;";
        let val_2 = jni::objects::JValueGen::Object(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSerializable", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }
    pub fn get_vector_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::util::Vector<'mc>>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn is_vector(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_vector(arg0)
    }
    pub fn is_offline_player(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_offline_player(arg0)
    }
    pub fn is_item_stack(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_item_stack(arg0)
    }
    pub fn is_color(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_color(arg0)
    }
    pub fn is_location(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_location(arg0)
    }
    pub fn get_comments(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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
    pub fn get_inline_comments(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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
    pub fn set_comments(
        &self,
        arg0: impl Into<String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            let map_val_0 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(v.into())?,
            ));
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
        arg0: impl Into<String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            let map_val_0 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(v.into())?,
            ));
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
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.name()
    }
    pub fn get_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "get", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    pub fn get_boolean_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
    pub fn get_int_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
    pub fn get_long_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
    pub fn get_double_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
    pub fn contains_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
    pub fn get_location_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn parent(
        &self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.parent()
    }
    pub fn set(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.set(arg0, arg1)
    }
    pub fn is_set(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_set(arg0)
    }
    pub fn root(
        &self,
    ) -> Result<Option<crate::configuration::Configuration<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = Configuration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.root()
    }
    pub fn get_object_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JClass<'mc>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/Class;";
        let val_2 = jni::objects::JValueGen::Object(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getObject", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
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
/// This is a <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> implementation that does not save or load from any source, and stores all values in memory only. This is useful for temporary Configurations for providing defaults.
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
    pub fn new_with_configuration(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::configuration::Configuration<'mc>>>,
    ) -> Result<crate::configuration::MemoryConfiguration<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
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

    pub fn add_default(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(arg1);
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

    pub fn add_defaults_with_map(
        &self,
        arg0: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/Map;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
        arg0: impl Into<crate::configuration::Configuration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/configuration/Configuration;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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

    pub fn parent(
        &self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/configuration/ConfigurationSection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getParent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn options(
        &self,
    ) -> Result<crate::configuration::MemoryConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/configuration/MemoryConfigurationOptions;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "options", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: ConfigurationSection
    pub fn get_string_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn get_keys(
        &self,
        arg0: bool,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Set;");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
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
    pub fn get_color_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::Color<'mc>>>,
    ) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_item_stack_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_offline_player_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::OfflinePlayer<'mc>>>,
    ) -> Result<crate::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::OfflinePlayer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_values(
        &self,
        arg0: bool,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Map;");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
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
    pub fn default_section(
        &self,
    ) -> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.default_section()
    }
    pub fn current_path(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.current_path()
    }
    pub fn is_configuration_section(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_configuration_section(arg0)
    }
    pub fn get_configuration_section(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.get_configuration_section(arg0)
    }
    pub fn create_section_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
    pub fn is_string(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_string(arg0)
    }
    pub fn is_int(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_int(arg0)
    }
    pub fn is_boolean(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_boolean(arg0)
    }
    pub fn is_double(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_double(arg0)
    }
    pub fn is_long(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_long(arg0)
    }
    pub fn is_list(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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
        arg0: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.get_string_list(arg0)
    }
    pub fn get_serializable_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JClass<'mc>,
        arg2: std::option::Option<
            impl Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>,
        >,
    ) -> Result<
        crate::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/Class;";
        let val_2 = jni::objects::JValueGen::Object(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSerializable", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }
    pub fn get_vector_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::util::Vector<'mc>>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn is_vector(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_vector(arg0)
    }
    pub fn is_offline_player(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_offline_player(arg0)
    }
    pub fn is_item_stack(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_item_stack(arg0)
    }
    pub fn is_color(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_color(arg0)
    }
    pub fn is_location(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_location(arg0)
    }
    pub fn get_comments(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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
        arg0: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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
        arg0: impl Into<String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            let map_val_0 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(v.into())?,
            ));
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
        arg0: impl Into<String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            let map_val_0 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(v.into())?,
            ));
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
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.name()
    }
    pub fn get_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "get", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    pub fn get_boolean_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
    pub fn get_int_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
    pub fn get_long_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
    pub fn get_double_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
    pub fn contains_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
    pub fn get_location_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn set(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.set(arg0, arg1)
    }
    pub fn is_set(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.is_set(arg0)
    }
    pub fn root(
        &self,
    ) -> Result<Option<crate::configuration::Configuration<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::ConfigurationSection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::ConfigurationSection = temp_clone.into();
        real.root()
    }
    pub fn get_object_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JClass<'mc>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/Class;";
        let val_2 = jni::objects::JValueGen::Object(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getObject", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    // SUPER CLASS: MemorySection
    pub fn create_path_with_configuration_section(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::configuration::ConfigurationSection<'mc>>,
        arg1: impl Into<String>,
        arg2: std::option::Option<impl Into<crate::configuration::ConfigurationSection<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/configuration/ConfigurationSection;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg1.into())?,
        ));
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/configuration/ConfigurationSection;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "createPath", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemorySection::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemorySection = temp_clone.into();
        real.internal_to_string()
    }
    // SUPER CLASS: Object

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
/// Various settings for controlling the input and output of a <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>
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
    /// Sets the char that will be used to separate <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>s
    /// <p>This value does not affect how the <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> is stored, only in how you access the data. The default value is '.'.</p>
    pub fn path_separator_with_char(
        &self,
        arg0: std::option::Option<u16>,
    ) -> Result<crate::configuration::ConfigurationOptions<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
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
    /// Sets if the <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> should copy values from its default <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> directly.
    /// <p>If this is true, all values in the default Configuration will be directly copied, making it impossible to distinguish between values that were set and values that are provided by default. As a result, <a href="ConfigurationSection.html#contains(java.lang.String)"><code>ConfigurationSection.contains(java.lang.String)</code></a> will always return the same value as <a href="ConfigurationSection.html#isSet(java.lang.String)"><code>ConfigurationSection.isSet(java.lang.String)</code></a>. The default value is false.</p>
    pub fn copy_defaults_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
    ) -> Result<crate::configuration::ConfigurationOptions<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a section of a <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>
///
/// This is a representation of an abstract class.
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
    pub fn get_string_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets a set containing all keys in this section.
    /// <p>If deep is set to true, then this will contain all the keys within any child <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>s (and their children, etc). These will be in a valid path notation for you to use.</p>
    /// <p>If deep is set to false, then this will contain only the keys of any direct children, and not their own children.</p>
    pub fn get_keys(
        &self,
        arg0: bool,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Set;");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
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

    pub fn get_color_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::Color<'mc>>>,
    ) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item_stack_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_offline_player_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::OfflinePlayer<'mc>>>,
    ) -> Result<crate::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::OfflinePlayer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a Map containing all keys and their values for this section.
    /// <p>If deep is set to true, then this will contain all the keys and values within any child <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>s (and their children, etc). These keys will be in a valid path notation for you to use.</p>
    /// <p>If deep is set to false, then this will contain only the keys and values of any direct children, and not their own children.</p>
    pub fn get_values(
        &self,
        arg0: bool,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/util/Map;");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
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

    pub fn add_default(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(arg1);
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

    pub fn is_configuration_section(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn get_configuration_section(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/String;)Lorg/bukkit/configuration/ConfigurationSection;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfigurationSection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn create_section_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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

    pub fn is_string(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_int(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_boolean(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_double(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_long(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_list(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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
        arg0: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn get_serializable_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JClass<'mc>,
        arg2: std::option::Option<
            impl Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>,
        >,
    ) -> Result<
        crate::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/Class;";
        let val_2 = jni::objects::JValueGen::Object(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSerializable", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn get_vector_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::util::Vector<'mc>>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_vector(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_offline_player(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_item_stack(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_color(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn is_location(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn get_comments(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn get_inline_comments(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn set_comments(
        &self,
        arg0: impl Into<String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            let map_val_0 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(v.into())?,
            ));
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
        arg0: impl Into<String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            let map_val_0 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(v.into())?,
            ));
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

    pub fn get_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "get", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn get_boolean_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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

    pub fn get_int_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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

    pub fn get_long_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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

    pub fn get_double_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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

    pub fn contains_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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

    pub fn get_location_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
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
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn parent(
        &self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/configuration/ConfigurationSection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getParent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(arg1);
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

    pub fn is_set(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
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

    pub fn get_object_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: jni::objects::JClass<'mc>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/Class;";
        let val_2 = jni::objects::JValueGen::Object(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getObject", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub mod file;
pub mod serialization;
