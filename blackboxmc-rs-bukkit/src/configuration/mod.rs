#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Various settings for controlling the input and output of a <a title="class in org.bukkit.configuration" href="MemoryConfiguration.html"><code>MemoryConfiguration</code></a>
pub struct MemoryConfigurationOptions<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MemoryConfigurationOptions<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MemoryConfigurationOptions<'mc> {
    pub fn from_raw(
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
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="ConfigurationOptions.html#pathSeparator(char)">ConfigurationOptions</a></code></span>
    /// Sets the char that will be used to separate <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>s
    /// <p>This value does not affect how the <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> is stored, only in how you access the data. The default value is '.'.</p>
    pub fn path_separator(
        &mut self,
        arg0: std::option::Option<u16>,
    ) -> Result<crate::configuration::MemoryConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = jni::objects::JValueGen::Char(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pathSeparator",
            "(C)Lorg/bukkit/configuration/MemoryConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="ConfigurationOptions.html#copyDefaults(boolean)">ConfigurationOptions</a></code></span>
    /// Sets if the <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> should copy values from its default <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> directly.
    /// <p>If this is true, all values in the default Configuration will be directly copied, making it impossible to distinguish between values that were set and values that are provided by default. As a result, <a href="ConfigurationSection.html#contains(java.lang.String)"><code>ConfigurationSection.contains(java.lang.String)</code></a> will always return the same value as <a href="ConfigurationSection.html#isSet(java.lang.String)"><code>ConfigurationSection.isSet(java.lang.String)</code></a>. The default value is false.</p>
    pub fn copy_defaults(
        &mut self,
        arg0: std::option::Option<bool>,
    ) -> Result<crate::configuration::MemoryConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyDefaults",
            "(Z)Lorg/bukkit/configuration/MemoryConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="ConfigurationOptions.html#configuration()">ConfigurationOptions</a></code></span>
    /// Returns the <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> that this object is responsible for.
    pub fn configuration(
        &mut self,
    ) -> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "configuration",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::Configuration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
impl<'mc> Into<crate::configuration::ConfigurationOptions<'mc>>
    for MemoryConfigurationOptions<'mc>
{
    fn into(self) -> crate::configuration::ConfigurationOptions<'mc> {
        crate::configuration::ConfigurationOptions::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// A type of <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> that is stored in memory.
pub struct MemorySection<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MemorySection<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MemorySection<'mc> {
    pub fn from_raw(
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
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getString(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested String by path.
    /// <p>If the String does not exist but a default value has been specified, this will return the default value. If the String does not exist and no default value was specified, this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getString(java.lang.String,java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested String by path, returning a default value if not found.
    /// <p>If the String does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getStringList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of String by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a String if possible, but may miss any values out if they are not compatible.</p>
    pub fn get_string_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getString",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
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
        &mut self,
        arg0: bool,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKeys",
            "(Z)Ljava/util/Set;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getOfflinePlayer(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested OfflinePlayer by path.
    /// <p>If the OfflinePlayer does not exist but a default value has been specified, this will return the default value. If the OfflinePlayer does not exist and no default value was specified, this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getOfflinePlayer(java.lang.String,org.bukkit.OfflinePlayer)">ConfigurationSection</a></code></span>
    /// Gets the requested <a href="../OfflinePlayer.html" title="interface in org.bukkit"><code>OfflinePlayer</code></a> by path, returning a default value if not found.
    /// <p>If the OfflinePlayer does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    pub fn get_offline_player_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::OfflinePlayer<'mc>>>,
    ) -> Result<crate::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOfflinePlayer",
            "(Ljava/lang/String;Lorg/bukkit/OfflinePlayer;)Lorg/bukkit/OfflinePlayer;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::OfflinePlayer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getColor(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested Color by path.
    /// <p>If the Color does not exist but a default value has been specified, this will return the default value. If the Color does not exist and no default value was specified, this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getColor(java.lang.String,org.bukkit.Color)">ConfigurationSection</a></code></span>
    /// Gets the requested <a href="../Color.html" title="class in org.bukkit"><code>Color</code></a> by path, returning a default value if not found.
    /// <p>If the Color does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    pub fn get_color_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::Color<'mc>>>,
    ) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "(Ljava/lang/String;Lorg/bukkit/Color;)Lorg/bukkit/Color;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getItemStack(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested ItemStack by path.
    /// <p>If the ItemStack does not exist but a default value has been specified, this will return the default value. If the ItemStack does not exist and no default value was specified, this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getItemStack(java.lang.String,org.bukkit.inventory.ItemStack)">ConfigurationSection</a></code></span>
    /// Gets the requested <a href="../inventory/ItemStack.html" title="class in org.bukkit.inventory"><code>ItemStack</code></a> by path, returning a default value if not found.
    /// <p>If the ItemStack does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    pub fn get_item_stack_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemStack",
            "(Ljava/lang/String;Lorg/bukkit/inventory/ItemStack;)Lorg/bukkit/inventory/ItemStack;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getValues(boolean)">ConfigurationSection</a></code></span>
    /// Gets a Map containing all keys and their values for this section.
    /// <p>If deep is set to true, then this will contain all the keys and values within any child <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>s (and their children, etc). These keys will be in a valid path notation for you to use.</p>
    /// <p>If deep is set to false, then this will contain only the keys and values of any direct children, and not their own children.</p>
    pub fn get_values(
        &mut self,
        arg0: bool,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getValues",
            "(Z)Ljava/util/Map;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Creates a full path to the given <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> from its root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.
    /// <p>You may use this method for any given <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>, not only <a href="MemorySection.html" title="class in org.bukkit.configuration"><code>MemorySection</code></a>.</p>
    /// Creates a relative path to the given <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> from the given relative section.
    /// <p>You may use this method for any given <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>, not only <a href="MemorySection.html" title="class in org.bukkit.configuration"><code>MemorySection</code></a>.</p>
    pub fn create_path_with_configuration_section(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::configuration::ConfigurationSection<'mc>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
        arg2: std::option::Option<impl Into<&'mc crate::configuration::ConfigurationSection<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let val_3 =
            unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(cls,"createPath",
"(Lorg/bukkit/configuration/ConfigurationSection;Ljava/lang/String;Lorg/bukkit/configuration/ConfigurationSection;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getDefaultSection()">ConfigurationSection</a></code></span>
    /// Gets the equivalent <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> from the default <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> defined in <a href="ConfigurationSection.html#getRoot()"><code>ConfigurationSection.getRoot()</code></a>.
    /// <p>If the root contains no defaults, or the defaults doesn't contain a value for this path, or the value at this path is not a <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> then this will return null.</p>
    pub fn default_section(
        &mut self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultSection",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getCurrentPath()">ConfigurationSection</a></code></span>
    /// Gets the path of this <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> from its root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>
    /// <p>For any <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> themselves, this will return an empty string.</p>
    /// <p>If the section is no longer contained within its root for any reason, such as being replaced with a different value, this may return null.</p>
    /// <p>To retrieve the single name of this section, that is, the final part of the path returned by this method, you may use <a href="ConfigurationSection.html#getName()"><code>ConfigurationSection.getName()</code></a>.</p>
    pub fn current_path(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCurrentPath",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#addDefault(java.lang.String,java.lang.Object)">ConfigurationSection</a></code></span>
    /// Sets the default value in the root at the given path as provided.
    /// <p>If no source <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> was provided as a default collection, then a new <a title="class in org.bukkit.configuration" href="MemoryConfiguration.html"><code>MemoryConfiguration</code></a> will be created to hold the new default value.</p>
    /// <p>If value is null, the value will be removed from the default Configuration source.</p>
    /// <p>If the value as returned by <a href="ConfigurationSection.html#getDefaultSection()"><code>ConfigurationSection.getDefaultSection()</code></a> is null, then this will create a new section at the path, replacing anything that may have existed there previously.</p>
    pub fn add_default(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addDefault",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#isConfigurationSection(java.lang.String)">ConfigurationSection</a></code></span>
    /// Checks if the specified path is a ConfigurationSection.
    /// <p>If the path exists but is not a ConfigurationSection, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a ConfigurationSection and return appropriately.</p>
    pub fn is_configuration_section(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isConfigurationSection",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getConfigurationSection(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested ConfigurationSection by path.
    /// <p>If the ConfigurationSection does not exist but a default value has been specified, this will return the default value. If the ConfigurationSection does not exist and no default value was specified, this will return null.</p>
    pub fn get_configuration_section(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfigurationSection",
            "(Ljava/lang/String;)Lorg/bukkit/configuration/ConfigurationSection;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#createSection(java.lang.String)">ConfigurationSection</a></code></span>
    /// Creates an empty <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> at the specified path.
    /// <p>Any value that was previously set at this path will be overwritten. If the previous value was itself a <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>, it will be orphaned.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#createSection(java.lang.String,java.util.Map)">ConfigurationSection</a></code></span>
    /// Creates a <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> at the specified path, with specified values.
    /// <p>Any value that was previously set at this path will be overwritten. If the previous value was itself a <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>, it will be orphaned.</p>
    pub fn create_section_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc>>>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createSection",
            "(Ljava/lang/String;Ljava/util/Map;)Lorg/bukkit/configuration/ConfigurationSection;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#isString(java.lang.String)">ConfigurationSection</a></code></span>
    /// Checks if the specified path is a String.
    /// <p>If the path exists but is not a String, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a String and return appropriately.</p>
    pub fn is_string(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isString",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#isInt(java.lang.String)">ConfigurationSection</a></code></span>
    /// Checks if the specified path is an int.
    /// <p>If the path exists but is not a int, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a int and return appropriately.</p>
    pub fn is_int(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInt",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#isBoolean(java.lang.String)">ConfigurationSection</a></code></span>
    /// Checks if the specified path is a boolean.
    /// <p>If the path exists but is not a boolean, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a boolean and return appropriately.</p>
    pub fn is_boolean(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBoolean",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#isDouble(java.lang.String)">ConfigurationSection</a></code></span>
    /// Checks if the specified path is a double.
    /// <p>If the path exists but is not a double, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a double and return appropriately.</p>
    pub fn is_double(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isDouble",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#isLong(java.lang.String)">ConfigurationSection</a></code></span>
    /// Checks if the specified path is a long.
    /// <p>If the path exists but is not a long, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a long and return appropriately.</p>
    pub fn is_long(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLong",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#isList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Checks if the specified path is a List.
    /// <p>If the path exists but is not a List, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a List and return appropriately.</p>
    pub fn is_list(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isList",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getStringList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of String by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a String if possible, but may miss any values out if they are not compatible.</p>
    pub fn get_string_list(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStringList",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getSerializable(java.lang.String,java.lang.Class)">ConfigurationSection</a></code></span>
    /// Gets the requested <a title="interface in org.bukkit.configuration.serialization" href="serialization/ConfigurationSerializable.html"><code>ConfigurationSerializable</code></a> object at the given path. If the Object does not exist but a default value has been specified, this will return the default value. If the Object does not exist and no default value was specified, this will return null.
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getSerializable(java.lang.String,java.lang.Class,T)">ConfigurationSection</a></code></span>
    /// Gets the requested <a href="serialization/ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a> object at the given path, returning a default value if not found If the Object does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.
    pub fn get_serializable_with_string(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<
            impl Into<&'mc crate::configuration::serialization::ConfigurationSerializable<'mc>>,
        >,
    ) -> Result<
        crate::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1.unwrap();
        let val_3 =
            unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"getSerializable","(Ljava/lang/String;Ljava/lang/Class;Lorg/bukkit/configuration/serialization/ConfigurationSerializable;)Lorg/bukkit/configuration/serialization/ConfigurationSerializable;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getVector(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested Vector by path.
    /// <p>If the Vector does not exist but a default value has been specified, this will return the default value. If the Vector does not exist and no default value was specified, this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getVector(java.lang.String,org.bukkit.util.Vector)">ConfigurationSection</a></code></span>
    /// Gets the requested <a href="../util/Vector.html" title="class in org.bukkit.util"><code>Vector</code></a> by path, returning a default value if not found.
    /// <p>If the Vector does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    pub fn get_vector_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVector",
            "(Ljava/lang/String;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#isVector(java.lang.String)">ConfigurationSection</a></code></span>
    /// Checks if the specified path is a Vector.
    /// <p>If the path exists but is not a Vector, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a Vector and return appropriately.</p>
    pub fn is_vector(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVector",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#isOfflinePlayer(java.lang.String)">ConfigurationSection</a></code></span>
    /// Checks if the specified path is an OfflinePlayer.
    /// <p>If the path exists but is not a OfflinePlayer, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a OfflinePlayer and return appropriately.</p>
    pub fn is_offline_player(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isOfflinePlayer",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#isItemStack(java.lang.String)">ConfigurationSection</a></code></span>
    /// Checks if the specified path is an ItemStack.
    /// <p>If the path exists but is not a ItemStack, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a ItemStack and return appropriately.</p>
    pub fn is_item_stack(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isItemStack",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#isColor(java.lang.String)">ConfigurationSection</a></code></span>
    /// Checks if the specified path is a Color.
    /// <p>If the path exists but is not a Color, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a Color and return appropriately.</p>
    pub fn is_color(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isColor",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#isLocation(java.lang.String)">ConfigurationSection</a></code></span>
    /// Checks if the specified path is a Location.
    /// <p>If the path exists but is not a Location, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a Location and return appropriately.</p>
    pub fn is_location(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLocation",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getComments(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested comment list by path.
    /// <p>If no comments exist, an empty list will be returned. A null entry represents an empty line and an empty String represents an empty comment line.</p>
    pub fn get_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComments",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getInlineComments(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested inline comment list by path.
    /// <p>If no comments exist, an empty list will be returned. A null entry represents an empty line and an empty String represents an empty comment line.</p>
    pub fn get_inline_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInlineComments",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#setComments(java.lang.String,java.util.List)">ConfigurationSection</a></code></span>
    /// Sets the comment list at the specified path.
    /// <p>If value is null, the comments will be removed. A null entry is an empty line and an empty String entry is an empty comment line. If the path does not exist, no comments will be set. Any existing comments will be replaced, regardless of what the new comments are.</p>
    /// <p>Some implementations may have limitations on what persists. See their individual javadocs for details.</p>
    pub fn set_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#setInlineComments(java.lang.String,java.util.List)">ConfigurationSection</a></code></span>
    /// Sets the inline comment list at the specified path.
    /// <p>If value is null, the comments will be removed. A null entry is an empty line and an empty String entry is an empty comment line. If the path does not exist, no comment will be set. Any existing comments will be replaced, regardless of what the new comments are.</p>
    /// <p>Some implementations may have limitations on what persists. See their individual javadocs for details.</p>
    pub fn set_inline_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInlineComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getName()">ConfigurationSection</a></code></span>
    /// Gets the name of this individual <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>, in the path.
    /// <p>This will always be the final part of <a href="ConfigurationSection.html#getCurrentPath()"><code>ConfigurationSection.getCurrentPath()</code></a>, unless the section is orphaned.</p>
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
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getKeys(boolean)">ConfigurationSection</a></code></span>
    /// Gets a set containing all keys in this section.
    /// <p>If deep is set to true, then this will contain all the keys within any child <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>s (and their children, etc). These will be in a valid path notation for you to use.</p>
    /// <p>If deep is set to false, then this will contain only the keys of any direct children, and not their own children.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getValues(boolean)">ConfigurationSection</a></code></span>
    /// Gets a Map containing all keys and their values for this section.
    /// <p>If deep is set to true, then this will contain all the keys and values within any child <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>s (and their children, etc). These keys will be in a valid path notation for you to use.</p>
    /// <p>If deep is set to false, then this will contain only the keys and values of any direct children, and not their own children.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getCurrentPath()">ConfigurationSection</a></code></span>
    /// Gets the path of this <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> from its root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>
    /// <p>For any <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> themselves, this will return an empty string.</p>
    /// <p>If the section is no longer contained within its root for any reason, such as being replaced with a different value, this may return null.</p>
    /// <p>To retrieve the single name of this section, that is, the final part of the path returned by this method, you may use <a href="ConfigurationSection.html#getName()"><code>ConfigurationSection.getName()</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getName()">ConfigurationSection</a></code></span>
    /// Gets the name of this individual <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>, in the path.
    /// <p>This will always be the final part of <a href="ConfigurationSection.html#getCurrentPath()"><code>ConfigurationSection.getCurrentPath()</code></a>, unless the section is orphaned.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getRoot()">ConfigurationSection</a></code></span>
    /// Gets the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> that contains this <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>
    /// <p>For any <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> themselves, this will return its own object.</p>
    /// <p>If the section is no longer contained within its root for any reason, such as being replaced with a different value, this may return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getParent()">ConfigurationSection</a></code></span>
    /// Gets the parent <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> that directly contains this <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>.
    /// <p>For any <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> themselves, this will return null.</p>
    /// <p>If the section is no longer contained within its parent for any reason, such as being replaced with a different value, this may return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getDefaultSection()">ConfigurationSection</a></code></span>
    /// Gets the equivalent <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> from the default <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> defined in <a href="ConfigurationSection.html#getRoot()"><code>ConfigurationSection.getRoot()</code></a>.
    /// <p>If the root contains no defaults, or the defaults doesn't contain a value for this path, or the value at this path is not a <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a> then this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#get(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested Object by path.
    /// <p>If the Object does not exist but a default value has been specified, this will return the default value. If the Object does not exist and no default value was specified, this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#get(java.lang.String,java.lang.Object)">ConfigurationSection</a></code></span>
    /// Gets the requested Object by path, returning a default value if not found.
    /// <p>If the Object does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getString(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested String by path.
    /// <p>If the String does not exist but a default value has been specified, this will return the default value. If the String does not exist and no default value was specified, this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getString(java.lang.String,java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested String by path, returning a default value if not found.
    /// <p>If the String does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getInt(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested int by path.
    /// <p>If the int does not exist but a default value has been specified, this will return the default value. If the int does not exist and no default value was specified, this will return 0.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getInt(java.lang.String,int)">ConfigurationSection</a></code></span>
    /// Gets the requested int by path, returning a default value if not found.
    /// <p>If the int does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getBoolean(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested boolean by path.
    /// <p>If the boolean does not exist but a default value has been specified, this will return the default value. If the boolean does not exist and no default value was specified, this will return false.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getBoolean(java.lang.String,boolean)">ConfigurationSection</a></code></span>
    /// Gets the requested boolean by path, returning a default value if not found.
    /// <p>If the boolean does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getDouble(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested double by path.
    /// <p>If the double does not exist but a default value has been specified, this will return the default value. If the double does not exist and no default value was specified, this will return 0.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getDouble(java.lang.String,double)">ConfigurationSection</a></code></span>
    /// Gets the requested double by path, returning a default value if not found.
    /// <p>If the double does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getLong(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested long by path.
    /// <p>If the long does not exist but a default value has been specified, this will return the default value. If the long does not exist and no default value was specified, this will return 0.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getLong(java.lang.String,long)">ConfigurationSection</a></code></span>
    /// Gets the requested long by path, returning a default value if not found.
    /// <p>If the long does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getList(java.lang.String,java.util.List)">ConfigurationSection</a></code></span>
    /// Gets the requested List by path, returning a default value if not found.
    /// <p>If the List does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getStringList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of String by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a String if possible, but may miss any values out if they are not compatible.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getIntegerList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of Integer by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Integer if possible, but may miss any values out if they are not compatible.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getBooleanList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of Boolean by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Boolean if possible, but may miss any values out if they are not compatible.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getDoubleList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of Double by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Double if possible, but may miss any values out if they are not compatible.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getFloatList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of Float by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Float if possible, but may miss any values out if they are not compatible.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getLongList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of Long by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Long if possible, but may miss any values out if they are not compatible.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getByteList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of Byte by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Byte if possible, but may miss any values out if they are not compatible.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getCharacterList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of Character by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Character if possible, but may miss any values out if they are not compatible.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getShortList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of Short by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Short if possible, but may miss any values out if they are not compatible.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getMapList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of Maps by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Map if possible, but may miss any values out if they are not compatible.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getObject(java.lang.String,java.lang.Class)">ConfigurationSection</a></code></span>
    /// Gets the requested object at the given path. If the Object does not exist but a default value has been specified, this will return the default value. If the Object does not exist and no default value was specified, this will return null. <b>Note:</b> For example #getObject(path, String.class) is <b>not</b> equivalent to <a href="ConfigurationSection.html#getString(java.lang.String)"><code>#getString(path)</code></a> because <a href="ConfigurationSection.html#getString(java.lang.String)"><code>#getString(path)</code></a> converts internally all Objects to Strings. However, #getObject(path, Boolean.class) is equivalent to <a href="ConfigurationSection.html#getBoolean(java.lang.String)"><code>#getBoolean(path)</code></a> for example.
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getObject(java.lang.String,java.lang.Class,T)">ConfigurationSection</a></code></span>
    /// Gets the requested object at the given path, returning a default value if not found If the Object does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>. <b>Note:</b> For example #getObject(path, String.class, def) is <b>not</b> equivalent to <a href="ConfigurationSection.html#getString(java.lang.String,java.lang.String)"><code>#getString(path, def)</code></a> because <a href="ConfigurationSection.html#getString(java.lang.String,java.lang.String)"><code>#getString(path, def)</code></a> converts internally all Objects to Strings. However, #getObject(path, Boolean.class, def) is equivalent to <a href="ConfigurationSection.html#getBoolean(java.lang.String,boolean)"><code>#getBoolean(path, def)</code></a> for example.
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getSerializable(java.lang.String,java.lang.Class)">ConfigurationSection</a></code></span>
    /// Gets the requested <a title="interface in org.bukkit.configuration.serialization" href="serialization/ConfigurationSerializable.html"><code>ConfigurationSerializable</code></a> object at the given path. If the Object does not exist but a default value has been specified, this will return the default value. If the Object does not exist and no default value was specified, this will return null.
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getSerializable(java.lang.String,java.lang.Class,T)">ConfigurationSection</a></code></span>
    /// Gets the requested <a href="serialization/ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a> object at the given path, returning a default value if not found If the Object does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getVector(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested Vector by path.
    /// <p>If the Vector does not exist but a default value has been specified, this will return the default value. If the Vector does not exist and no default value was specified, this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getVector(java.lang.String,org.bukkit.util.Vector)">ConfigurationSection</a></code></span>
    /// Gets the requested <a title="class in org.bukkit.util" href="../util/Vector.html"><code>Vector</code></a> by path, returning a default value if not found.
    /// <p>If the Vector does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getOfflinePlayer(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested OfflinePlayer by path.
    /// <p>If the OfflinePlayer does not exist but a default value has been specified, this will return the default value. If the OfflinePlayer does not exist and no default value was specified, this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getOfflinePlayer(java.lang.String,org.bukkit.OfflinePlayer)">ConfigurationSection</a></code></span>
    /// Gets the requested <a href="../OfflinePlayer.html" title="interface in org.bukkit"><code>OfflinePlayer</code></a> by path, returning a default value if not found.
    /// <p>If the OfflinePlayer does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getItemStack(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested ItemStack by path.
    /// <p>If the ItemStack does not exist but a default value has been specified, this will return the default value. If the ItemStack does not exist and no default value was specified, this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getItemStack(java.lang.String,org.bukkit.inventory.ItemStack)">ConfigurationSection</a></code></span>
    /// Gets the requested <a title="class in org.bukkit.inventory" href="../inventory/ItemStack.html"><code>ItemStack</code></a> by path, returning a default value if not found.
    /// <p>If the ItemStack does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getColor(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested Color by path.
    /// <p>If the Color does not exist but a default value has been specified, this will return the default value. If the Color does not exist and no default value was specified, this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getColor(java.lang.String,org.bukkit.Color)">ConfigurationSection</a></code></span>
    /// Gets the requested <a title="class in org.bukkit" href="../Color.html"><code>Color</code></a> by path, returning a default value if not found.
    /// <p>If the Color does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getLocation(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested Location by path.
    /// <p>If the Location does not exist but a default value has been specified, this will return the default value. If the Location does not exist and no default value was specified, this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getLocation(java.lang.String,org.bukkit.Location)">ConfigurationSection</a></code></span>
    /// Gets the requested <a href="../Location.html" title="class in org.bukkit"><code>Location</code></a> by path, returning a default value if not found.
    /// <p>If the Location does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getConfigurationSection(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested ConfigurationSection by path.
    /// <p>If the ConfigurationSection does not exist but a default value has been specified, this will return the default value. If the ConfigurationSection does not exist and no default value was specified, this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getComments(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested comment list by path.
    /// <p>If no comments exist, an empty list will be returned. A null entry represents an empty line and an empty String represents an empty comment line.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getInlineComments(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested inline comment list by path.
    /// <p>If no comments exist, an empty list will be returned. A null entry represents an empty line and an empty String represents an empty comment line.</p>
    pub fn get_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
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
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getBoolean(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested boolean by path.
    /// <p>If the boolean does not exist but a default value has been specified, this will return the default value. If the boolean does not exist and no default value was specified, this will return false.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getBoolean(java.lang.String,boolean)">ConfigurationSection</a></code></span>
    /// Gets the requested boolean by path, returning a default value if not found.
    /// <p>If the boolean does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getBooleanList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of Boolean by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Boolean if possible, but may miss any values out if they are not compatible.</p>
    pub fn get_boolean_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoolean",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getInt(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested int by path.
    /// <p>If the int does not exist but a default value has been specified, this will return the default value. If the int does not exist and no default value was specified, this will return 0.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getInt(java.lang.String,int)">ConfigurationSection</a></code></span>
    /// Gets the requested int by path, returning a default value if not found.
    /// <p>If the int does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getIntegerList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of Integer by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Integer if possible, but may miss any values out if they are not compatible.</p>
    pub fn get_int_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInt",
            "(Ljava/lang/String;I)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getLong(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested long by path.
    /// <p>If the long does not exist but a default value has been specified, this will return the default value. If the long does not exist and no default value was specified, this will return 0.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getLong(java.lang.String,long)">ConfigurationSection</a></code></span>
    /// Gets the requested long by path, returning a default value if not found.
    /// <p>If the long does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getLongList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of Long by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Long if possible, but may miss any values out if they are not compatible.</p>
    pub fn get_long_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLong",
            "(Ljava/lang/String;J)J",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getDouble(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested double by path.
    /// <p>If the double does not exist but a default value has been specified, this will return the default value. If the double does not exist and no default value was specified, this will return 0.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getDouble(java.lang.String,double)">ConfigurationSection</a></code></span>
    /// Gets the requested double by path, returning a default value if not found.
    /// <p>If the double does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getDoubleList(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested List of Double by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Double if possible, but may miss any values out if they are not compatible.</p>
    pub fn get_double_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDouble",
            "(Ljava/lang/String;D)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#contains(java.lang.String)">ConfigurationSection</a></code></span>
    /// Checks if this <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a> contains the given path.
    /// <p>If the value for the requested path does not exist but a default value has been specified, this will return true.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#contains(java.lang.String,boolean)">ConfigurationSection</a></code></span>
    /// Checks if this <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> contains the given path.
    /// <p>If the value for the requested path does not exist, the boolean parameter of true has been specified, a default value for the path exists, this will return true.</p>
    /// <p>If a boolean parameter of false has been specified, true will only be returned if there is a set value for the specified path.</p>
    pub fn contains_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getLocation(java.lang.String)">ConfigurationSection</a></code></span>
    /// Gets the requested Location by path.
    /// <p>If the Location does not exist but a default value has been specified, this will return the default value. If the Location does not exist and no default value was specified, this will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getLocation(java.lang.String,org.bukkit.Location)">ConfigurationSection</a></code></span>
    /// Gets the requested <a title="class in org.bukkit" href="../Location.html"><code>Location</code></a> by path, returning a default value if not found.
    /// <p>If the Location does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    pub fn get_location_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::Location<'mc>>>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Ljava/lang/String;Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getParent()">ConfigurationSection</a></code></span>
    /// Gets the parent <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> that directly contains this <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>.
    /// <p>For any <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> themselves, this will return null.</p>
    /// <p>If the section is no longer contained within its parent for any reason, such as being replaced with a different value, this may return null.</p>
    pub fn parent(
        &mut self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getParent",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#set(java.lang.String,java.lang.Object)">ConfigurationSection</a></code></span>
    /// Sets the specified path to the given value.
    /// <p>If value is null, the entry will be removed. Any existing entry will be replaced, regardless of what the new value is.</p>
    /// <p>Some implementations may have limitations on what you may store. See their individual javadocs for details. No implementations should allow you to store <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>s or <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>s, please use <a href="ConfigurationSection.html#createSection(java.lang.String)"><code>ConfigurationSection.createSection(java.lang.String)</code></a> for that.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#setComments(java.lang.String,java.util.List)">ConfigurationSection</a></code></span>
    /// Sets the comment list at the specified path.
    /// <p>If value is null, the comments will be removed. A null entry is an empty line and an empty String entry is an empty comment line. If the path does not exist, no comments will be set. Any existing comments will be replaced, regardless of what the new comments are.</p>
    /// <p>Some implementations may have limitations on what persists. See their individual javadocs for details.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#setInlineComments(java.lang.String,java.util.List)">ConfigurationSection</a></code></span>
    /// Sets the inline comment list at the specified path.
    /// <p>If value is null, the comments will be removed. A null entry is an empty line and an empty String entry is an empty comment line. If the path does not exist, no comment will be set. Any existing comments will be replaced, regardless of what the new comments are.</p>
    /// <p>Some implementations may have limitations on what persists. See their individual javadocs for details.</p>
    pub fn set(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#isSet(java.lang.String)">ConfigurationSection</a></code></span>
    /// Checks if this <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> has a value set for the given path.
    /// <p>If the value for the requested path does not exist but a default value has been specified, this will still return false.</p>
    pub fn is_set(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSet",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getRoot()">ConfigurationSection</a></code></span>
    /// Gets the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> that contains this <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>
    /// <p>For any <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> themselves, this will return its own object.</p>
    /// <p>If the section is no longer contained within its root for any reason, such as being replaced with a different value, this may return null.</p>
    pub fn root(
        &mut self,
    ) -> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRoot",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::Configuration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getObject(java.lang.String,java.lang.Class)">ConfigurationSection</a></code></span>
    /// Gets the requested object at the given path. If the Object does not exist but a default value has been specified, this will return the default value. If the Object does not exist and no default value was specified, this will return null. <b>Note:</b> For example #getObject(path, String.class) is <b>not</b> equivalent to <a href="ConfigurationSection.html#getString(java.lang.String)"><code>#getString(path)</code></a> because <a href="ConfigurationSection.html#getString(java.lang.String)"><code>#getString(path)</code></a> converts internally all Objects to Strings. However, #getObject(path, Boolean.class) is equivalent to <a href="ConfigurationSection.html#getBoolean(java.lang.String)"><code>#getBoolean(path)</code></a> for example.
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getObject(java.lang.String,java.lang.Class,T)">ConfigurationSection</a></code></span>
    /// Gets the requested object at the given path, returning a default value if not found If the Object does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>. <b>Note:</b> For example #getObject(path, String.class, def) is <b>not</b> equivalent to <a href="ConfigurationSection.html#getString(java.lang.String,java.lang.String)"><code>#getString(path, def)</code></a> because <a href="ConfigurationSection.html#getString(java.lang.String,java.lang.String)"><code>#getString(path, def)</code></a> converts internally all Objects to Strings. However, #getObject(path, Boolean.class, def) is equivalent to <a href="ConfigurationSection.html#getBoolean(java.lang.String,boolean)"><code>#getBoolean(path, def)</code></a> for example.
    pub fn get_object_with_string(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1.unwrap();
        let val_3 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getObject",
            "(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
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
impl<'mc> Into<crate::configuration::ConfigurationSection<'mc>> for MemorySection<'mc> {
    fn into(self) -> crate::configuration::ConfigurationSection<'mc> {
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a source of configurable options and settings
///
/// This is a representation of an abstract class.
pub struct Configuration<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Configuration<'mc> {
    pub fn from_raw(
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
    /// Sets the default value of the given path as provided.
    /// <p>If no source <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> was provided as a default collection, then a new <a href="MemoryConfiguration.html" title="class in org.bukkit.configuration"><code>MemoryConfiguration</code></a> will be created to hold the new default value.</p>
    /// <p>If value is null, the value will be removed from the default Configuration source.</p>
    /// Sets the default values of the given paths as provided.
    /// <p>If no source <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> was provided as a default collection, then a new <a title="class in org.bukkit.configuration" href="MemoryConfiguration.html"><code>MemoryConfiguration</code></a> will be created to hold the new default values.</p>
    /// Sets the default values of the given paths as provided.
    /// <p>If no source <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> was provided as a default collection, then a new <a title="class in org.bukkit.configuration" href="MemoryConfiguration.html"><code>MemoryConfiguration</code></a> will be created to hold the new default value.</p>
    /// <p>This method will not hold a reference to the specified Configuration, nor will it automatically update if that Configuration ever changes. If you require this, you should set the default source with <a href="#setDefaults(org.bukkit.configuration.Configuration)"><code>setDefaults(org.bukkit.configuration.Configuration)</code></a>.</p>
    pub fn add_default(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addDefault",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the source <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> for this configuration.
    /// <p>If no configuration source was set, but default values were added, then a <a href="MemoryConfiguration.html" title="class in org.bukkit.configuration"><code>MemoryConfiguration</code></a> will be returned. If no source was set and no defaults were set, then this method will return null.</p>
    pub fn defaults(
        &mut self,
    ) -> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaults",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::Configuration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the default values of the given paths as provided.
    /// <p>If no source <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> was provided as a default collection, then a new <a title="class in org.bukkit.configuration" href="MemoryConfiguration.html"><code>MemoryConfiguration</code></a> will be created to hold the new default values.</p>
    /// Sets the default values of the given paths as provided.
    /// <p>If no source <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> was provided as a default collection, then a new <a href="MemoryConfiguration.html" title="class in org.bukkit.configuration"><code>MemoryConfiguration</code></a> will be created to hold the new default value.</p>
    /// <p>This method will not hold a reference to the specified Configuration, nor will it automatically update if that Configuration ever changes. If you require this, you should set the default source with <a href="#setDefaults(org.bukkit.configuration.Configuration)"><code>setDefaults(org.bukkit.configuration.Configuration)</code></a>.</p>
    pub fn add_defaults_with_map(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::configuration::Configuration<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addDefaults",
            "(Lorg/bukkit/configuration/Configuration;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the source of all default values for this <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.
    /// <p>If a previous source was set, or previous default values were defined, then they will not be copied to the new source.</p>
    pub fn set_defaults(
        &mut self,
        arg0: impl Into<&'mc crate::configuration::Configuration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDefaults",
            "(Lorg/bukkit/configuration/Configuration;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the <a title="class in org.bukkit.configuration" href="ConfigurationOptions.html"><code>ConfigurationOptions</code></a> for this <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.
    /// <p>All setters through this method are chainable.</p>
    pub fn options(
        &mut self,
    ) -> Result<crate::configuration::ConfigurationOptions<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "options",
            "()Lorg/bukkit/configuration/ConfigurationOptions;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_string_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getString",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_keys(
        &mut self,
        arg0: bool,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKeys",
            "(Z)Ljava/util/Set;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_offline_player_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::OfflinePlayer<'mc>>>,
    ) -> Result<crate::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOfflinePlayer",
            "(Ljava/lang/String;Lorg/bukkit/OfflinePlayer;)Lorg/bukkit/OfflinePlayer;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::OfflinePlayer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_color_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::Color<'mc>>>,
    ) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "(Ljava/lang/String;Lorg/bukkit/Color;)Lorg/bukkit/Color;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item_stack_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemStack",
            "(Ljava/lang/String;Lorg/bukkit/inventory/ItemStack;)Lorg/bukkit/inventory/ItemStack;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_values(
        &mut self,
        arg0: bool,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getValues",
            "(Z)Ljava/util/Map;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn default_section(
        &mut self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultSection",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn current_path(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCurrentPath",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn is_configuration_section(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isConfigurationSection",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_configuration_section(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfigurationSection",
            "(Ljava/lang/String;)Lorg/bukkit/configuration/ConfigurationSection;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn create_section_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc>>>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createSection",
            "(Ljava/lang/String;Ljava/util/Map;)Lorg/bukkit/configuration/ConfigurationSection;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_string(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isString",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_int(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInt",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_boolean(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBoolean",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_double(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isDouble",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_long(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLong",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_list(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isList",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_string_list(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStringList",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
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
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<
            impl Into<&'mc crate::configuration::serialization::ConfigurationSerializable<'mc>>,
        >,
    ) -> Result<
        crate::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1.unwrap();
        let val_3 =
            unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"getSerializable","(Ljava/lang/String;Ljava/lang/Class;Lorg/bukkit/configuration/serialization/ConfigurationSerializable;)Lorg/bukkit/configuration/serialization/ConfigurationSerializable;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn get_vector_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVector",
            "(Ljava/lang/String;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_vector(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVector",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_offline_player(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isOfflinePlayer",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_item_stack(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isItemStack",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_color(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isColor",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_location(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLocation",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComments",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
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
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInlineComments",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
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
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_inline_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInlineComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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
    /// Gets the source <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> for this configuration.
    /// <p>If no configuration source was set, but default values were added, then a <a title="class in org.bukkit.configuration" href="MemoryConfiguration.html"><code>MemoryConfiguration</code></a> will be returned. If no source was set and no defaults were set, then this method will return null.</p>
    pub fn get_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn get_boolean_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoolean",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_int_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInt",
            "(Ljava/lang/String;I)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn get_long_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLong",
            "(Ljava/lang/String;J)J",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn get_double_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDouble",
            "(Ljava/lang/String;D)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn contains_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_location_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::Location<'mc>>>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Ljava/lang/String;Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn parent(
        &mut self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getParent",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the source of all default values for this <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.
    /// <p>If a previous source was set, or previous default values were defined, then they will not be copied to the new source.</p>
    pub fn set(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_set(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSet",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn root(
        &mut self,
    ) -> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRoot",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::Configuration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_object_with_string(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1.unwrap();
        let val_3 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getObject",
            "(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for Configuration<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::configuration::ConfigurationSection<'mc>> for Configuration<'mc> {
    fn into(self) -> crate::configuration::ConfigurationSection<'mc> {
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// This is a <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> implementation that does not save or load from any source, and stores all values in memory only. This is useful for temporary Configurations for providing defaults.
pub struct MemoryConfiguration<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MemoryConfiguration<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MemoryConfiguration<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::configuration::Configuration<'mc>>>,
    ) -> Result<crate::configuration::MemoryConfiguration<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/configuration/MemoryConfiguration")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/configuration/Configuration;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::configuration::MemoryConfiguration::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#addDefault(java.lang.String,java.lang.Object)">ConfigurationSection</a></code></span>
    /// Sets the default value in the root at the given path as provided.
    /// <p>If no source <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> was provided as a default collection, then a new <a title="class in org.bukkit.configuration" href="MemoryConfiguration.html"><code>MemoryConfiguration</code></a> will be created to hold the new default value.</p>
    /// <p>If value is null, the value will be removed from the default Configuration source.</p>
    /// <p>If the value as returned by <a href="ConfigurationSection.html#getDefaultSection()"><code>ConfigurationSection.getDefaultSection()</code></a> is null, then this will create a new section at the path, replacing anything that may have existed there previously.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Configuration.html#addDefaults(java.util.Map)">Configuration</a></code></span>
    /// Sets the default values of the given paths as provided.
    /// <p>If no source <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> was provided as a default collection, then a new <a title="class in org.bukkit.configuration" href="MemoryConfiguration.html"><code>MemoryConfiguration</code></a> will be created to hold the new default values.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Configuration.html#addDefaults(org.bukkit.configuration.Configuration)">Configuration</a></code></span>
    /// Sets the default values of the given paths as provided.
    /// <p>If no source <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> was provided as a default collection, then a new <a href="MemoryConfiguration.html" title="class in org.bukkit.configuration"><code>MemoryConfiguration</code></a> will be created to hold the new default value.</p>
    /// <p>This method will not hold a reference to the specified Configuration, nor will it automatically update if that Configuration ever changes. If you require this, you should set the default source with <a href="Configuration.html#setDefaults(org.bukkit.configuration.Configuration)"><code>Configuration.setDefaults(org.bukkit.configuration.Configuration)</code></a>.</p>
    pub fn add_default(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addDefault",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Configuration.html#getDefaults()">Configuration</a></code></span>
    /// Gets the source <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> for this configuration.
    /// <p>If no configuration source was set, but default values were added, then a <a href="MemoryConfiguration.html" title="class in org.bukkit.configuration"><code>MemoryConfiguration</code></a> will be returned. If no source was set and no defaults were set, then this method will return null.</p>
    pub fn defaults(
        &mut self,
    ) -> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaults",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::Configuration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Configuration.html#addDefaults(java.util.Map)">Configuration</a></code></span>
    /// Sets the default values of the given paths as provided.
    /// <p>If no source <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> was provided as a default collection, then a new <a href="MemoryConfiguration.html" title="class in org.bukkit.configuration"><code>MemoryConfiguration</code></a> will be created to hold the new default values.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Configuration.html#addDefaults(org.bukkit.configuration.Configuration)">Configuration</a></code></span>
    /// Sets the default values of the given paths as provided.
    /// <p>If no source <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> was provided as a default collection, then a new <a title="class in org.bukkit.configuration" href="MemoryConfiguration.html"><code>MemoryConfiguration</code></a> will be created to hold the new default value.</p>
    /// <p>This method will not hold a reference to the specified Configuration, nor will it automatically update if that Configuration ever changes. If you require this, you should set the default source with <a href="Configuration.html#setDefaults(org.bukkit.configuration.Configuration)"><code>Configuration.setDefaults(org.bukkit.configuration.Configuration)</code></a>.</p>
    pub fn add_defaults_with_configuration(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addDefaults",
            "(Ljava/util/Map;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Configuration.html#setDefaults(org.bukkit.configuration.Configuration)">Configuration</a></code></span>
    /// Sets the source of all default values for this <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.
    /// <p>If a previous source was set, or previous default values were defined, then they will not be copied to the new source.</p>
    pub fn set_defaults(
        &mut self,
        arg0: impl Into<&'mc crate::configuration::Configuration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDefaults",
            "(Lorg/bukkit/configuration/Configuration;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getParent()">ConfigurationSection</a></code></span>
    /// Gets the parent <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a> that directly contains this <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>.
    /// <p>For any <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> themselves, this will return null.</p>
    /// <p>If the section is no longer contained within its parent for any reason, such as being replaced with a different value, this may return null.</p>
    pub fn parent(
        &mut self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getParent",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Configuration.html#options()">Configuration</a></code></span>
    /// Gets the <a href="ConfigurationOptions.html" title="class in org.bukkit.configuration"><code>ConfigurationOptions</code></a> for this <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.
    /// <p>All setters through this method are chainable.</p>
    pub fn options(
        &mut self,
    ) -> Result<crate::configuration::MemoryConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "options",
            "()Lorg/bukkit/configuration/MemoryConfigurationOptions;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_string_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getString",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_keys(
        &mut self,
        arg0: bool,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKeys",
            "(Z)Ljava/util/Set;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_offline_player_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::OfflinePlayer<'mc>>>,
    ) -> Result<crate::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOfflinePlayer",
            "(Ljava/lang/String;Lorg/bukkit/OfflinePlayer;)Lorg/bukkit/OfflinePlayer;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::OfflinePlayer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_color_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::Color<'mc>>>,
    ) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "(Ljava/lang/String;Lorg/bukkit/Color;)Lorg/bukkit/Color;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item_stack_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemStack",
            "(Ljava/lang/String;Lorg/bukkit/inventory/ItemStack;)Lorg/bukkit/inventory/ItemStack;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_values(
        &mut self,
        arg0: bool,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getValues",
            "(Z)Ljava/util/Map;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn create_path_with_configuration_section(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::configuration::ConfigurationSection<'mc>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
        arg2: std::option::Option<impl Into<&'mc crate::configuration::ConfigurationSection<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let val_3 =
            unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(cls,"createPath",
"(Lorg/bukkit/configuration/ConfigurationSection;Ljava/lang/String;Lorg/bukkit/configuration/ConfigurationSection;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn default_section(
        &mut self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultSection",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn current_path(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCurrentPath",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn is_configuration_section(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isConfigurationSection",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_configuration_section(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfigurationSection",
            "(Ljava/lang/String;)Lorg/bukkit/configuration/ConfigurationSection;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn create_section_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc>>>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createSection",
            "(Ljava/lang/String;Ljava/util/Map;)Lorg/bukkit/configuration/ConfigurationSection;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_string(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isString",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_int(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInt",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_boolean(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBoolean",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_double(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isDouble",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_long(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLong",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_list(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isList",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_string_list(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStringList",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
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
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<
            impl Into<&'mc crate::configuration::serialization::ConfigurationSerializable<'mc>>,
        >,
    ) -> Result<
        crate::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1.unwrap();
        let val_3 =
            unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"getSerializable","(Ljava/lang/String;Ljava/lang/Class;Lorg/bukkit/configuration/serialization/ConfigurationSerializable;)Lorg/bukkit/configuration/serialization/ConfigurationSerializable;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn get_vector_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVector",
            "(Ljava/lang/String;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_vector(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVector",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_offline_player(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isOfflinePlayer",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_item_stack(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isItemStack",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_color(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isColor",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_location(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLocation",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComments",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
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
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInlineComments",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
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
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_inline_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInlineComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Configuration.html#getDefaults()">Configuration</a></code></span>
    /// Gets the source <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> for this configuration.
    /// <p>If no configuration source was set, but default values were added, then a <a href="MemoryConfiguration.html" title="class in org.bukkit.configuration"><code>MemoryConfiguration</code></a> will be returned. If no source was set and no defaults were set, then this method will return null.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ConfigurationSection.html#getParent()">ConfigurationSection</a></code></span>
    /// Gets the parent <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> that directly contains this <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>.
    /// <p>For any <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> themselves, this will return null.</p>
    /// <p>If the section is no longer contained within its parent for any reason, such as being replaced with a different value, this may return null.</p>
    pub fn get_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
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

    pub fn get_boolean_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoolean",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_int_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInt",
            "(Ljava/lang/String;I)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn get_long_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLong",
            "(Ljava/lang/String;J)J",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn get_double_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDouble",
            "(Ljava/lang/String;D)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn contains_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_location_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::Location<'mc>>>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Ljava/lang/String;Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Configuration.html#setDefaults(org.bukkit.configuration.Configuration)">Configuration</a></code></span>
    /// Sets the source of all default values for this <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.
    /// <p>If a previous source was set, or previous default values were defined, then they will not be copied to the new source.</p>
    pub fn set(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_set(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSet",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn root(
        &mut self,
    ) -> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRoot",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::Configuration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_object_with_string(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1.unwrap();
        let val_3 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getObject",
            "(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
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
impl<'mc> Into<crate::configuration::Configuration<'mc>> for MemoryConfiguration<'mc> {
    fn into(self) -> crate::configuration::Configuration<'mc> {
        crate::configuration::Configuration::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::configuration::MemorySection<'mc>> for MemoryConfiguration<'mc> {
    fn into(self) -> crate::configuration::MemorySection<'mc> {
        crate::configuration::MemorySection::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Various settings for controlling the input and output of a <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>
pub struct ConfigurationOptions<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ConfigurationOptions<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ConfigurationOptions<'mc> {
    pub fn from_raw(
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
    /// Gets the char that will be used to separate <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>s
    /// <p>This value does not affect how the <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> is stored, only in how you access the data. The default value is '.'.</p>
    /// Sets the char that will be used to separate <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>s
    /// <p>This value does not affect how the <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> is stored, only in how you access the data. The default value is '.'.</p>
    pub fn path_separator(
        &mut self,
        arg0: std::option::Option<u16>,
    ) -> Result<crate::configuration::ConfigurationOptions<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Char(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pathSeparator",
            "(C)Lorg/bukkit/configuration/ConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> should copy values from its default <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> directly.
    /// <p>If this is true, all values in the default Configuration will be directly copied, making it impossible to distinguish between values that were set and values that are provided by default. As a result, <a href="ConfigurationSection.html#contains(java.lang.String)"><code>ConfigurationSection.contains(java.lang.String)</code></a> will always return the same value as <a href="ConfigurationSection.html#isSet(java.lang.String)"><code>ConfigurationSection.isSet(java.lang.String)</code></a>. The default value is false.</p>
    /// Sets if the <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> should copy values from its default <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> directly.
    /// <p>If this is true, all values in the default Configuration will be directly copied, making it impossible to distinguish between values that were set and values that are provided by default. As a result, <a href="ConfigurationSection.html#contains(java.lang.String)"><code>ConfigurationSection.contains(java.lang.String)</code></a> will always return the same value as <a href="ConfigurationSection.html#isSet(java.lang.String)"><code>ConfigurationSection.isSet(java.lang.String)</code></a>. The default value is false.</p>
    pub fn copy_defaults(
        &mut self,
        arg0: std::option::Option<bool>,
    ) -> Result<crate::configuration::ConfigurationOptions<'mc>, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyDefaults",
            "(Z)Lorg/bukkit/configuration/ConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> that this object is responsible for.
    pub fn configuration(
        &mut self,
    ) -> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "configuration",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::Configuration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
/// Represents a section of a <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>
///
/// This is a representation of an abstract class.
pub struct ConfigurationSection<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ConfigurationSection<'mc> {
    pub fn from_raw(
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
    /// Gets the requested String by path.
    /// <p>If the String does not exist but a default value has been specified, this will return the default value. If the String does not exist and no default value was specified, this will return null.</p>
    /// Gets the requested String by path, returning a default value if not found.
    /// <p>If the String does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// Gets the requested List of String by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a String if possible, but may miss any values out if they are not compatible.</p>
    pub fn get_string_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getString",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
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
        &mut self,
        arg0: bool,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKeys",
            "(Z)Ljava/util/Set;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the requested OfflinePlayer by path.
    /// <p>If the OfflinePlayer does not exist but a default value has been specified, this will return the default value. If the OfflinePlayer does not exist and no default value was specified, this will return null.</p>
    /// Gets the requested <a title="interface in org.bukkit" href="../OfflinePlayer.html"><code>OfflinePlayer</code></a> by path, returning a default value if not found.
    /// <p>If the OfflinePlayer does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    pub fn get_offline_player_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::OfflinePlayer<'mc>>>,
    ) -> Result<crate::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOfflinePlayer",
            "(Ljava/lang/String;Lorg/bukkit/OfflinePlayer;)Lorg/bukkit/OfflinePlayer;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::OfflinePlayer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the requested Color by path.
    /// <p>If the Color does not exist but a default value has been specified, this will return the default value. If the Color does not exist and no default value was specified, this will return null.</p>
    /// Gets the requested <a href="../Color.html" title="class in org.bukkit"><code>Color</code></a> by path, returning a default value if not found.
    /// <p>If the Color does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    pub fn get_color_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::Color<'mc>>>,
    ) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "(Ljava/lang/String;Lorg/bukkit/Color;)Lorg/bukkit/Color;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the requested ItemStack by path.
    /// <p>If the ItemStack does not exist but a default value has been specified, this will return the default value. If the ItemStack does not exist and no default value was specified, this will return null.</p>
    /// Gets the requested <a href="../inventory/ItemStack.html" title="class in org.bukkit.inventory"><code>ItemStack</code></a> by path, returning a default value if not found.
    /// <p>If the ItemStack does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    pub fn get_item_stack_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemStack",
            "(Ljava/lang/String;Lorg/bukkit/inventory/ItemStack;)Lorg/bukkit/inventory/ItemStack;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a Map containing all keys and their values for this section.
    /// <p>If deep is set to true, then this will contain all the keys and values within any child <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>s (and their children, etc). These keys will be in a valid path notation for you to use.</p>
    /// <p>If deep is set to false, then this will contain only the keys and values of any direct children, and not their own children.</p>
    pub fn get_values(
        &mut self,
        arg0: bool,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getValues",
            "(Z)Ljava/util/Map;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the equivalent <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a> from the default <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> defined in <a href="#getRoot()"><code>getRoot()</code></a>.
    /// <p>If the root contains no defaults, or the defaults doesn't contain a value for this path, or the value at this path is not a <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> then this will return null.</p>
    pub fn default_section(
        &mut self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultSection",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the path of this <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> from its root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>
    /// <p>For any <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> themselves, this will return an empty string.</p>
    /// <p>If the section is no longer contained within its root for any reason, such as being replaced with a different value, this may return null.</p>
    /// <p>To retrieve the single name of this section, that is, the final part of the path returned by this method, you may use <a href="#getName()"><code>getName()</code></a>.</p>
    pub fn current_path(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCurrentPath",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the default value in the root at the given path as provided.
    /// <p>If no source <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> was provided as a default collection, then a new <a href="MemoryConfiguration.html" title="class in org.bukkit.configuration"><code>MemoryConfiguration</code></a> will be created to hold the new default value.</p>
    /// <p>If value is null, the value will be removed from the default Configuration source.</p>
    /// <p>If the value as returned by <a href="#getDefaultSection()"><code>getDefaultSection()</code></a> is null, then this will create a new section at the path, replacing anything that may have existed there previously.</p>
    pub fn add_default(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addDefault",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks if the specified path is a ConfigurationSection.
    /// <p>If the path exists but is not a ConfigurationSection, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a ConfigurationSection and return appropriately.</p>
    pub fn is_configuration_section(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isConfigurationSection",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the requested ConfigurationSection by path.
    /// <p>If the ConfigurationSection does not exist but a default value has been specified, this will return the default value. If the ConfigurationSection does not exist and no default value was specified, this will return null.</p>
    pub fn get_configuration_section(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfigurationSection",
            "(Ljava/lang/String;)Lorg/bukkit/configuration/ConfigurationSection;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Creates an empty <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a> at the specified path.
    /// <p>Any value that was previously set at this path will be overwritten. If the previous value was itself a <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>, it will be orphaned.</p>
    /// Creates a <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> at the specified path, with specified values.
    /// <p>Any value that was previously set at this path will be overwritten. If the previous value was itself a <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>, it will be orphaned.</p>
    pub fn create_section_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc>>>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createSection",
            "(Ljava/lang/String;Ljava/util/Map;)Lorg/bukkit/configuration/ConfigurationSection;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified path is a String.
    /// <p>If the path exists but is not a String, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a String and return appropriately.</p>
    pub fn is_string(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isString",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks if the specified path is an int.
    /// <p>If the path exists but is not a int, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a int and return appropriately.</p>
    pub fn is_int(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInt",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks if the specified path is a boolean.
    /// <p>If the path exists but is not a boolean, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a boolean and return appropriately.</p>
    pub fn is_boolean(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBoolean",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks if the specified path is a double.
    /// <p>If the path exists but is not a double, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a double and return appropriately.</p>
    pub fn is_double(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isDouble",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks if the specified path is a long.
    /// <p>If the path exists but is not a long, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a long and return appropriately.</p>
    pub fn is_long(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLong",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks if the specified path is a List.
    /// <p>If the path exists but is not a List, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a List and return appropriately.</p>
    pub fn is_list(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isList",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the requested List of String by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a String if possible, but may miss any values out if they are not compatible.</p>
    pub fn get_string_list(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStringList",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Gets the requested <a href="serialization/ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a> object at the given path. If the Object does not exist but a default value has been specified, this will return the default value. If the Object does not exist and no default value was specified, this will return null.
    /// Gets the requested <a title="interface in org.bukkit.configuration.serialization" href="serialization/ConfigurationSerializable.html"><code>ConfigurationSerializable</code></a> object at the given path, returning a default value if not found If the Object does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.
    pub fn get_serializable_with_string(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<
            impl Into<&'mc crate::configuration::serialization::ConfigurationSerializable<'mc>>,
        >,
    ) -> Result<
        crate::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1.unwrap();
        let val_3 =
            unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"getSerializable","(Ljava/lang/String;Ljava/lang/Class;Lorg/bukkit/configuration/serialization/ConfigurationSerializable;)Lorg/bukkit/configuration/serialization/ConfigurationSerializable;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }
    /// Gets the requested Vector by path.
    /// <p>If the Vector does not exist but a default value has been specified, this will return the default value. If the Vector does not exist and no default value was specified, this will return null.</p>
    /// Gets the requested <a title="class in org.bukkit.util" href="../util/Vector.html"><code>Vector</code></a> by path, returning a default value if not found.
    /// <p>If the Vector does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    pub fn get_vector_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVector",
            "(Ljava/lang/String;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified path is a Vector.
    /// <p>If the path exists but is not a Vector, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a Vector and return appropriately.</p>
    pub fn is_vector(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVector",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks if the specified path is an OfflinePlayer.
    /// <p>If the path exists but is not a OfflinePlayer, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a OfflinePlayer and return appropriately.</p>
    pub fn is_offline_player(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isOfflinePlayer",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks if the specified path is an ItemStack.
    /// <p>If the path exists but is not a ItemStack, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a ItemStack and return appropriately.</p>
    pub fn is_item_stack(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isItemStack",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks if the specified path is a Color.
    /// <p>If the path exists but is not a Color, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a Color and return appropriately.</p>
    pub fn is_color(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isColor",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks if the specified path is a Location.
    /// <p>If the path exists but is not a Location, this will return false. If the path does not exist, this will return false. If the path does not exist but a default value has been specified, this will check if that default value is a Location and return appropriately.</p>
    pub fn is_location(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLocation",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the requested comment list by path.
    /// <p>If no comments exist, an empty list will be returned. A null entry represents an empty line and an empty String represents an empty comment line.</p>
    pub fn get_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComments",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
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
    /// <p>If no comments exist, an empty list will be returned. A null entry represents an empty line and an empty String represents an empty comment line.</p>
    pub fn get_inline_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInlineComments",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
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
    /// <p>If value is null, the comments will be removed. A null entry is an empty line and an empty String entry is an empty comment line. If the path does not exist, no comments will be set. Any existing comments will be replaced, regardless of what the new comments are.</p>
    /// <p>Some implementations may have limitations on what persists. See their individual javadocs for details.</p>
    pub fn set_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the inline comment list at the specified path.
    /// <p>If value is null, the comments will be removed. A null entry is an empty line and an empty String entry is an empty comment line. If the path does not exist, no comment will be set. Any existing comments will be replaced, regardless of what the new comments are.</p>
    /// <p>Some implementations may have limitations on what persists. See their individual javadocs for details.</p>
    pub fn set_inline_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInlineComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the name of this individual <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>, in the path.
    /// <p>This will always be the final part of <a href="#getCurrentPath()"><code>getCurrentPath()</code></a>, unless the section is orphaned.</p>
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
    /// Gets a set containing all keys in this section.
    /// <p>If deep is set to true, then this will contain all the keys within any child <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>s (and their children, etc). These will be in a valid path notation for you to use.</p>
    /// <p>If deep is set to false, then this will contain only the keys of any direct children, and not their own children.</p>
    /// Gets a Map containing all keys and their values for this section.
    /// <p>If deep is set to true, then this will contain all the keys and values within any child <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>s (and their children, etc). These keys will be in a valid path notation for you to use.</p>
    /// <p>If deep is set to false, then this will contain only the keys and values of any direct children, and not their own children.</p>
    /// Gets the path of this <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> from its root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>
    /// <p>For any <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> themselves, this will return an empty string.</p>
    /// <p>If the section is no longer contained within its root for any reason, such as being replaced with a different value, this may return null.</p>
    /// <p>To retrieve the single name of this section, that is, the final part of the path returned by this method, you may use <a href="#getName()"><code>getName()</code></a>.</p>
    /// Gets the name of this individual <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>, in the path.
    /// <p>This will always be the final part of <a href="#getCurrentPath()"><code>getCurrentPath()</code></a>, unless the section is orphaned.</p>
    /// Gets the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> that contains this <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>
    /// <p>For any <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> themselves, this will return its own object.</p>
    /// <p>If the section is no longer contained within its root for any reason, such as being replaced with a different value, this may return null.</p>
    /// Gets the parent <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> that directly contains this <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>.
    /// <p>For any <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> themselves, this will return null.</p>
    /// <p>If the section is no longer contained within its parent for any reason, such as being replaced with a different value, this may return null.</p>
    /// Gets the requested Object by path.
    /// <p>If the Object does not exist but a default value has been specified, this will return the default value. If the Object does not exist and no default value was specified, this will return null.</p>
    /// Gets the requested Object by path, returning a default value if not found.
    /// <p>If the Object does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    /// Gets the requested String by path.
    /// <p>If the String does not exist but a default value has been specified, this will return the default value. If the String does not exist and no default value was specified, this will return null.</p>
    /// Gets the requested String by path, returning a default value if not found.
    /// <p>If the String does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// Gets the requested int by path.
    /// <p>If the int does not exist but a default value has been specified, this will return the default value. If the int does not exist and no default value was specified, this will return 0.</p>
    /// Gets the requested int by path, returning a default value if not found.
    /// <p>If the int does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// Gets the requested boolean by path.
    /// <p>If the boolean does not exist but a default value has been specified, this will return the default value. If the boolean does not exist and no default value was specified, this will return false.</p>
    /// Gets the requested boolean by path, returning a default value if not found.
    /// <p>If the boolean does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// Gets the requested double by path.
    /// <p>If the double does not exist but a default value has been specified, this will return the default value. If the double does not exist and no default value was specified, this will return 0.</p>
    /// Gets the requested double by path, returning a default value if not found.
    /// <p>If the double does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// Gets the requested long by path.
    /// <p>If the long does not exist but a default value has been specified, this will return the default value. If the long does not exist and no default value was specified, this will return 0.</p>
    /// Gets the requested long by path, returning a default value if not found.
    /// <p>If the long does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    /// Gets the requested List by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return null.</p>
    /// Gets the requested List by path, returning a default value if not found.
    /// <p>If the List does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// Gets the requested List of String by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a String if possible, but may miss any values out if they are not compatible.</p>
    /// Gets the requested List of Integer by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Integer if possible, but may miss any values out if they are not compatible.</p>
    /// Gets the requested List of Boolean by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Boolean if possible, but may miss any values out if they are not compatible.</p>
    /// Gets the requested List of Double by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Double if possible, but may miss any values out if they are not compatible.</p>
    /// Gets the requested List of Float by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Float if possible, but may miss any values out if they are not compatible.</p>
    /// Gets the requested List of Long by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Long if possible, but may miss any values out if they are not compatible.</p>
    /// Gets the requested List of Byte by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Byte if possible, but may miss any values out if they are not compatible.</p>
    /// Gets the requested List of Character by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Character if possible, but may miss any values out if they are not compatible.</p>
    /// Gets the requested List of Short by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Short if possible, but may miss any values out if they are not compatible.</p>
    /// Gets the requested List of Maps by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Map if possible, but may miss any values out if they are not compatible.</p>
    /// Gets the requested object at the given path. If the Object does not exist but a default value has been specified, this will return the default value. If the Object does not exist and no default value was specified, this will return null. <b>Note:</b> For example #getObject(path, String.class) is <b>not</b> equivalent to <a href="#getString(java.lang.String)"><code>#getString(path)</code></a> because <a href="#getString(java.lang.String)"><code>#getString(path)</code></a> converts internally all Objects to Strings. However, #getObject(path, Boolean.class) is equivalent to <a href="#getBoolean(java.lang.String)"><code>#getBoolean(path)</code></a> for example.
    /// Gets the requested object at the given path, returning a default value if not found If the Object does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>. <b>Note:</b> For example #getObject(path, String.class, def) is <b>not</b> equivalent to <a href="#getString(java.lang.String,java.lang.String)"><code>#getString(path, def)</code></a> because <a href="#getString(java.lang.String,java.lang.String)"><code>#getString(path, def)</code></a> converts internally all Objects to Strings. However, #getObject(path, Boolean.class, def) is equivalent to <a href="#getBoolean(java.lang.String,boolean)"><code>#getBoolean(path, def)</code></a> for example.
    /// Gets the requested <a href="serialization/ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a> object at the given path. If the Object does not exist but a default value has been specified, this will return the default value. If the Object does not exist and no default value was specified, this will return null.
    /// Gets the requested <a title="interface in org.bukkit.configuration.serialization" href="serialization/ConfigurationSerializable.html"><code>ConfigurationSerializable</code></a> object at the given path, returning a default value if not found If the Object does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.
    /// Gets the requested Vector by path.
    /// <p>If the Vector does not exist but a default value has been specified, this will return the default value. If the Vector does not exist and no default value was specified, this will return null.</p>
    /// Gets the requested <a title="class in org.bukkit.util" href="../util/Vector.html"><code>Vector</code></a> by path, returning a default value if not found.
    /// <p>If the Vector does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// Gets the requested OfflinePlayer by path.
    /// <p>If the OfflinePlayer does not exist but a default value has been specified, this will return the default value. If the OfflinePlayer does not exist and no default value was specified, this will return null.</p>
    /// Gets the requested <a href="../OfflinePlayer.html" title="interface in org.bukkit"><code>OfflinePlayer</code></a> by path, returning a default value if not found.
    /// <p>If the OfflinePlayer does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    /// Gets the requested ItemStack by path.
    /// <p>If the ItemStack does not exist but a default value has been specified, this will return the default value. If the ItemStack does not exist and no default value was specified, this will return null.</p>
    /// Gets the requested <a href="../inventory/ItemStack.html" title="class in org.bukkit.inventory"><code>ItemStack</code></a> by path, returning a default value if not found.
    /// <p>If the ItemStack does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    /// Gets the requested Color by path.
    /// <p>If the Color does not exist but a default value has been specified, this will return the default value. If the Color does not exist and no default value was specified, this will return null.</p>
    /// Gets the requested <a href="../Color.html" title="class in org.bukkit"><code>Color</code></a> by path, returning a default value if not found.
    /// <p>If the Color does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    /// Gets the requested Location by path.
    /// <p>If the Location does not exist but a default value has been specified, this will return the default value. If the Location does not exist and no default value was specified, this will return null.</p>
    /// Gets the requested <a href="../Location.html" title="class in org.bukkit"><code>Location</code></a> by path, returning a default value if not found.
    /// <p>If the Location does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    /// Gets the requested ConfigurationSection by path.
    /// <p>If the ConfigurationSection does not exist but a default value has been specified, this will return the default value. If the ConfigurationSection does not exist and no default value was specified, this will return null.</p>
    /// Gets the equivalent <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> from the default <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> defined in <a href="#getRoot()"><code>getRoot()</code></a>.
    /// <p>If the root contains no defaults, or the defaults doesn't contain a value for this path, or the value at this path is not a <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a> then this will return null.</p>
    /// Gets the requested comment list by path.
    /// <p>If no comments exist, an empty list will be returned. A null entry represents an empty line and an empty String represents an empty comment line.</p>
    /// Gets the requested inline comment list by path.
    /// <p>If no comments exist, an empty list will be returned. A null entry represents an empty line and an empty String represents an empty comment line.</p>
    pub fn get_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    /// Gets the requested boolean by path.
    /// <p>If the boolean does not exist but a default value has been specified, this will return the default value. If the boolean does not exist and no default value was specified, this will return false.</p>
    /// Gets the requested boolean by path, returning a default value if not found.
    /// <p>If the boolean does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    /// Gets the requested List of Boolean by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Boolean if possible, but may miss any values out if they are not compatible.</p>
    pub fn get_boolean_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoolean",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the requested int by path.
    /// <p>If the int does not exist but a default value has been specified, this will return the default value. If the int does not exist and no default value was specified, this will return 0.</p>
    /// Gets the requested int by path, returning a default value if not found.
    /// <p>If the int does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    /// Gets the requested List of Integer by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Integer if possible, but may miss any values out if they are not compatible.</p>
    pub fn get_int_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInt",
            "(Ljava/lang/String;I)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets the requested long by path.
    /// <p>If the long does not exist but a default value has been specified, this will return the default value. If the long does not exist and no default value was specified, this will return 0.</p>
    /// Gets the requested long by path, returning a default value if not found.
    /// <p>If the long does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    /// Gets the requested List of Long by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Long if possible, but may miss any values out if they are not compatible.</p>
    pub fn get_long_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLong",
            "(Ljava/lang/String;J)J",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    /// Gets the requested double by path.
    /// <p>If the double does not exist but a default value has been specified, this will return the default value. If the double does not exist and no default value was specified, this will return 0.</p>
    /// Gets the requested double by path, returning a default value if not found.
    /// <p>If the double does not exist then the specified default value will returned regardless of if a default has been identified in the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a>.</p>
    /// Gets the requested List of Double by path.
    /// <p>If the List does not exist but a default value has been specified, this will return the default value. If the List does not exist and no default value was specified, this will return an empty List.</p>
    /// <p>This method will attempt to cast any values into a Double if possible, but may miss any values out if they are not compatible.</p>
    pub fn get_double_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDouble",
            "(Ljava/lang/String;D)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Checks if this <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a> contains the given path.
    /// <p>If the value for the requested path does not exist but a default value has been specified, this will return true.</p>
    /// Checks if this <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> contains the given path.
    /// <p>If the value for the requested path does not exist, the boolean parameter of true has been specified, a default value for the path exists, this will return true.</p>
    /// <p>If a boolean parameter of false has been specified, true will only be returned if there is a set value for the specified path.</p>
    pub fn contains_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the requested Location by path.
    /// <p>If the Location does not exist but a default value has been specified, this will return the default value. If the Location does not exist and no default value was specified, this will return null.</p>
    /// Gets the requested <a href="../Location.html" title="class in org.bukkit"><code>Location</code></a> by path, returning a default value if not found.
    /// <p>If the Location does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.</p>
    pub fn get_location_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::Location<'mc>>>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Ljava/lang/String;Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the parent <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> that directly contains this <a title="interface in org.bukkit.configuration" href="ConfigurationSection.html"><code>ConfigurationSection</code></a>.
    /// <p>For any <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> themselves, this will return null.</p>
    /// <p>If the section is no longer contained within its parent for any reason, such as being replaced with a different value, this may return null.</p>
    pub fn parent(
        &mut self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getParent",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the specified path to the given value.
    /// <p>If value is null, the entry will be removed. Any existing entry will be replaced, regardless of what the new value is.</p>
    /// <p>Some implementations may have limitations on what you may store. See their individual javadocs for details. No implementations should allow you to store <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>s or <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>s, please use <a href="#createSection(java.lang.String)"><code>createSection(java.lang.String)</code></a> for that.</p>
    /// Sets the comment list at the specified path.
    /// <p>If value is null, the comments will be removed. A null entry is an empty line and an empty String entry is an empty comment line. If the path does not exist, no comments will be set. Any existing comments will be replaced, regardless of what the new comments are.</p>
    /// <p>Some implementations may have limitations on what persists. See their individual javadocs for details.</p>
    /// Sets the inline comment list at the specified path.
    /// <p>If value is null, the comments will be removed. A null entry is an empty line and an empty String entry is an empty comment line. If the path does not exist, no comment will be set. Any existing comments will be replaced, regardless of what the new comments are.</p>
    /// <p>Some implementations may have limitations on what persists. See their individual javadocs for details.</p>
    pub fn set(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks if this <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a> has a value set for the given path.
    /// <p>If the value for the requested path does not exist but a default value has been specified, this will still return false.</p>
    pub fn is_set(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSet",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the root <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> that contains this <a href="ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>
    /// <p>For any <a title="interface in org.bukkit.configuration" href="Configuration.html"><code>Configuration</code></a> themselves, this will return its own object.</p>
    /// <p>If the section is no longer contained within its root for any reason, such as being replaced with a different value, this may return null.</p>
    pub fn root(
        &mut self,
    ) -> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRoot",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::Configuration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the requested object at the given path. If the Object does not exist but a default value has been specified, this will return the default value. If the Object does not exist and no default value was specified, this will return null. <b>Note:</b> For example #getObject(path, String.class) is <b>not</b> equivalent to <a href="#getString(java.lang.String)"><code>#getString(path)</code></a> because <a href="#getString(java.lang.String)"><code>#getString(path)</code></a> converts internally all Objects to Strings. However, #getObject(path, Boolean.class) is equivalent to <a href="#getBoolean(java.lang.String)"><code>#getBoolean(path)</code></a> for example.
    /// Gets the requested object at the given path, returning a default value if not found If the Object does not exist then the specified default value will returned regardless of if a default has been identified in the root <a href="Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>. <b>Note:</b> For example #getObject(path, String.class, def) is <b>not</b> equivalent to <a href="#getString(java.lang.String,java.lang.String)"><code>#getString(path, def)</code></a> because <a href="#getString(java.lang.String,java.lang.String)"><code>#getString(path, def)</code></a> converts internally all Objects to Strings. However, #getObject(path, Boolean.class, def) is equivalent to <a href="#getBoolean(java.lang.String,boolean)"><code>#getBoolean(path, def)</code></a> for example.
    pub fn get_object_with_string(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1.unwrap();
        let val_3 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getObject",
            "(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for ConfigurationSection<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub mod file;
pub mod serialization;
