#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// A simple wrapper for ease of selecting <a href="Enchantment.html" title="class in org.bukkit.enchantments"><code>Enchantment</code></a>s
pub struct EnchantmentWrapper<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EnchantmentWrapper<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EnchantmentWrapper<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnchantmentWrapper from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/enchantments/EnchantmentWrapper")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantmentWrapper object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::enchantments::EnchantmentWrapper<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("org/bukkit/enchantments/EnchantmentWrapper")?;
        let res = jni.new_object(
            cls,
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::enchantments::EnchantmentWrapper::from_raw(&jni, res)
    }
    /// Gets the enchantment bound to this wrapper
    pub fn enchantment(
        &mut self,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantment",
            "()Lorg/bukkit/enchantments/Enchantment;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::enchantments::Enchantment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="Enchantment.html#getStartLevel()">Enchantment</a></code></span>
    /// Gets the level that this Enchantment should start at
    pub fn start_level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getStartLevel", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="Enchantment.html#getMaxLevel()">Enchantment</a></code></span>
    /// Gets the maximum level that this Enchantment may become.
    pub fn max_level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxLevel", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="Enchantment.html#canEnchantItem(org.bukkit.inventory.ItemStack)">Enchantment</a></code></span>
    /// Checks if this Enchantment may be applied to the given <a href="../inventory/ItemStack.html" title="class in org.bukkit.inventory"><code>ItemStack</code></a>.
    /// <p>This does not check if it conflicts with any enchantments already applied to the item.</p>
    pub fn can_enchant_item(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "canEnchantItem",
            "(Lorg/bukkit/inventory/ItemStack;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="Enchantment.html#getItemTarget()">Enchantment</a></code></span>
    /// Gets the type of <a href="../inventory/ItemStack.html" title="class in org.bukkit.inventory"><code>ItemStack</code></a> that may fit this Enchantment.
    pub fn item_target(
        &mut self,
    ) -> Result<crate::enchantments::EnchantmentTarget<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemTarget",
            "()Lorg/bukkit/enchantments/EnchantmentTarget;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::enchantments::EnchantmentTarget::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="Enchantment.html#isTreasure()">Enchantment</a></code></span>
    /// Checks if this enchantment is a treasure enchantment.
    ///
    /// Treasure enchantments can only be received via looting, trading, or fishing.
    pub fn is_treasure(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isTreasure", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="Enchantment.html#isCursed()">Enchantment</a></code></span>
    /// Checks if this enchantment is a cursed enchantment
    ///
    /// Cursed enchantments are found the same way treasure enchantments are
    pub fn is_cursed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCursed", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="Enchantment.html#conflictsWith(org.bukkit.enchantments.Enchantment)">Enchantment</a></code></span>
    /// Check if this enchantment conflicts with another enchantment.
    pub fn conflicts_with(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "conflictsWith",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="Enchantment.html#getName()">Enchantment</a></code></span>
    /// Gets the unique name of this enchantment
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

    pub fn get_by_key(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/enchantments/Enchantment")?;
        let res = jni.call_static_method(
            cls,
            "getByKey",
            "(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/enchantments/Enchantment;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        let obj = res.l()?;
        crate::enchantments::Enchantment::from_raw(&jni, obj)
    }

    pub fn stop_accepting_registrations(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cls = &jni.find_class("void")?;
        let res = jni.call_static_method(cls, "stopAcceptingRegistrations", "()V", &[])?;
        Ok(())
    }

    pub fn register_enchantment(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("void")?;
        let res = jni.call_static_method(
            cls,
            "registerEnchantment",
            "(Lorg/bukkit/enchantments/Enchantment;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(())
    }

    pub fn is_accepting_registrations(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("boolean")?;
        let res = jni.call_static_method(cls, "isAcceptingRegistrations", "()Z", &[])?;
        Ok(res.z().unwrap())
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
    #[deprecated]

    pub fn get_by_name(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("org/bukkit/enchantments/Enchantment")?;
        let res = jni.call_static_method(
            cls,
            "getByName",
            "(Ljava/lang/String;)Lorg/bukkit/enchantments/Enchantment;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        let obj = res.l()?;
        crate::enchantments::Enchantment::from_raw(&jni, obj)
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
impl<'mc> Into<crate::enchantments::Enchantment<'mc>> for EnchantmentWrapper<'mc> {
    fn into(self) -> crate::enchantments::Enchantment<'mc> {
        crate::enchantments::Enchantment::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// A class for the available enchantment offers in the enchantment table.
pub struct EnchantmentOffer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EnchantmentOffer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EnchantmentOffer<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnchantmentOffer from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/enchantments/EnchantmentOffer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantmentOffer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: i32,
    ) -> Result<crate::enchantments::EnchantmentOffer<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let cls = &jni.find_class("org/bukkit/enchantments/EnchantmentOffer")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/enchantments/Enchantment;II)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        crate::enchantments::EnchantmentOffer::from_raw(&jni, res)
    }
    /// Get the type of the enchantment.
    /// Gets the level of the enchantment.
    pub fn enchantment(
        &mut self,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantment",
            "()Lorg/bukkit/enchantments/Enchantment;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::enchantments::Enchantment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the type of the enchantment.
    /// Sets the level of the enchantment.
    pub fn set_enchantment(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEnchantment",
            "(Lorg/bukkit/enchantments/Enchantment;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the level of the enchantment.
    pub fn enchantment_level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEnchantmentLevel", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Sets the level of the enchantment.
    pub fn set_enchantment_level(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEnchantmentLevel",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the cost (minimum level) which is displayed as a number on the right hand side of the enchantment offer.
    pub fn cost(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCost", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Sets the cost (minimum level) which is displayed as a number on the right hand side of the enchantment offer.
    pub fn set_cost(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCost",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
/// Represents the applicable target for a <a href="Enchantment.html" title="class in org.bukkit.enchantments"><code>Enchantment</code></a>
pub struct EnchantmentTarget<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EnchantmentTarget<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EnchantmentTarget<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnchantmentTarget from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/enchantments/EnchantmentTarget")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantmentTarget object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Check whether this target includes the specified item.
    /// Check whether this target includes the specified item.
    pub fn includes_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "includes",
            "(Lorg/bukkit/Material;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns the enum constant of this type with the specified name. The string must match <i>exactly</i> an identifier used to declare an enum constant in this type. (Extraneous whitespace characters are not permitted.)
    pub fn value_of_with_string(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let cls = &jni.find_class("java/lang/Enum")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let obj = res.l()?;
        Self::from_raw(&jni, obj)
    }

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "name", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
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

    pub fn describe_constable(
        &mut self,
    ) -> Result<blackboxmc_java::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            "()Ljava/util/Optional;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn declaring_class(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDeclaringClass",
            "()Ljava/lang/Class;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
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
/// The various type of enchantments that may be added to armour or weapons
pub struct Enchantment<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Enchantment<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Enchantment<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Enchantment from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/enchantments/Enchantment")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Enchantment object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/enchantments/Enchantment")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/NamespacedKey;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::enchantments::Enchantment::from_raw(&jni, res)
    }
    /// Gets the Enchantment at the specified key
    pub fn get_by_key(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/enchantments/Enchantment")?;
        let res = jni.call_static_method(
            cls,
            "getByKey",
            "(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/enchantments/Enchantment;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        let obj = res.l()?;
        crate::enchantments::Enchantment::from_raw(&jni, obj)
    }
    /// Gets the level that this Enchantment should start at
    pub fn start_level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getStartLevel", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets the maximum level that this Enchantment may become.
    pub fn max_level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxLevel", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Checks if this Enchantment may be applied to the given <a href="../inventory/ItemStack.html" title="class in org.bukkit.inventory"><code>ItemStack</code></a>.
    /// <p>This does not check if it conflicts with any enchantments already applied to the item.</p>
    pub fn can_enchant_item(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "canEnchantItem",
            "(Lorg/bukkit/inventory/ItemStack;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Stops accepting any enchantment registrations
    pub fn stop_accepting_registrations(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cls = &jni.find_class("void")?;
        let res = jni.call_static_method(cls, "stopAcceptingRegistrations", "()V", &[])?;
        Ok(())
    }
    /// Gets the type of <a href="../inventory/ItemStack.html" title="class in org.bukkit.inventory"><code>ItemStack</code></a> that may fit this Enchantment.
    pub fn item_target(
        &mut self,
    ) -> Result<crate::enchantments::EnchantmentTarget<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemTarget",
            "()Lorg/bukkit/enchantments/EnchantmentTarget;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::enchantments::EnchantmentTarget::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this enchantment is a treasure enchantment.
    ///
    /// Treasure enchantments can only be received via looting, trading, or fishing.
    pub fn is_treasure(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isTreasure", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// cursed enchantments are no longer special. Will return true only for <a href="#BINDING_CURSE"><code>BINDING_CURSE</code></a> and <a href="#VANISHING_CURSE"><code>VANISHING_CURSE</code></a>.
    /// </div>
    /// cursed enchantments are no longer special. Will return true only for <a href="#BINDING_CURSE"><code>BINDING_CURSE</code></a> and <a href="#VANISHING_CURSE"><code>VANISHING_CURSE</code></a>.
    ///
    /// Checks if this enchantment is a cursed enchantment
    ///
    /// Cursed enchantments are found the same way treasure enchantments are
    pub fn is_cursed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCursed", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Check if this enchantment conflicts with another enchantment.
    pub fn conflicts_with(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "conflictsWith",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Registers an enchantment with the given ID and object.
    /// <p>Generally not to be used from within a plugin.</p>
    pub fn register_enchantment(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("void")?;
        let res = jni.call_static_method(
            cls,
            "registerEnchantment",
            "(Lorg/bukkit/enchantments/Enchantment;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(())
    }
    /// Checks if this is accepting Enchantment registrations.
    pub fn is_accepting_registrations(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("boolean")?;
        let res = jni.call_static_method(cls, "isAcceptingRegistrations", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// enchantments are badly named, use <a href="#getKey()"><code>getKey()</code></a>.
    /// </div>
    /// enchantments are badly named, use <a href="#getKey()"><code>getKey()</code></a>.
    ///
    /// Gets the unique name of this enchantment
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
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Keyed.html#getKey()">Keyed</a></code></span>
    /// Return the namespaced identifier for this object.
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
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// enchantments are badly named, use <a href="#getByKey(org.bukkit.NamespacedKey)"><code>getByKey(org.bukkit.NamespacedKey)</code></a>.
    /// </div>
    /// enchantments are badly named, use <a href="#getByKey(org.bukkit.NamespacedKey)"><code>getByKey(org.bukkit.NamespacedKey)</code></a>.
    ///
    /// Gets the Enchantment at the specified name
    pub fn get_by_name(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("org/bukkit/enchantments/Enchantment")?;
        let res = jni.call_static_method(
            cls,
            "getByName",
            "(Ljava/lang/String;)Lorg/bukkit/enchantments/Enchantment;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        let obj = res.l()?;
        crate::enchantments::Enchantment::from_raw(&jni, obj)
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
impl<'mc> Into<crate::Keyed<'mc>> for Enchantment<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
