#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// A simple wrapper for ease of selecting <a title="class in org.bukkit.enchantments" href="Enchantment.html"><code>Enchantment</code></a>s
#[repr(C)]
pub struct EnchantmentWrapper<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EnchantmentWrapper<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentWrapper<'mc> {
    fn from_raw(
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
}

impl<'mc> EnchantmentWrapper<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::enchantments::EnchantmentWrapper<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("org/bukkit/enchantments/EnchantmentWrapper");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::enchantments::EnchantmentWrapper::from_raw(&jni, res)
    }

    pub fn enchantment(
        &self,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/enchantments/Enchantment;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchantment", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::enchantments::Enchantment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn start_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getStartLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn max_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn can_enchant_item(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "canEnchantItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn item_target(
        &self,
    ) -> Result<crate::enchantments::EnchantmentTarget<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/enchantments/EnchantmentTarget;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemTarget", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::enchantments::EnchantmentTarget::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_treasure(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isTreasure", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    #[deprecated]

    pub fn is_cursed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCursed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn conflicts_with(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "conflictsWith",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
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
    // SUPER CLASS: Enchantment
    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::enchantments::Enchantment::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Keyed = temp_clone.into();
        real.key()
    }
    pub fn register_enchantment(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        crate::enchantments::Enchantment::register_enchantment(jni, arg0)
    }
    pub fn get_by_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        crate::enchantments::Enchantment::get_by_key(jni, arg0)
    }
    pub fn stop_accepting_registrations(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "stopAcceptingRegistrations", sig.as_str(), vec![]);
        jni.translate_error(res)?;
        Ok(())
    }
    pub fn is_accepting_registrations(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isAcceptingRegistrations", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::enchantments::Enchantment::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::enchantments::Enchantment = temp_clone.into();
        real.equals(arg0)
    }
    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::enchantments::Enchantment::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::enchantments::Enchantment = temp_clone.into();
        real.internal_to_string()
    }
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<Vec<crate::enchantments::Enchantment<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/enchantments/Enchantment;");
        let cls = jni.find_class("org/bukkit/enchantments/Enchantment");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::enchantments::Enchantment::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::enchantments::Enchantment::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::enchantments::Enchantment = temp_clone.into();
        real.hash_code()
    }
    pub fn get_by_name(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        crate::enchantments::Enchantment::get_by_name(jni, arg0)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::enchantments::Enchantment<'mc>> for EnchantmentWrapper<'mc> {
    fn into(self) -> crate::enchantments::Enchantment<'mc> {
        crate::enchantments::Enchantment::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EnchantmentWrapper into crate::enchantments::Enchantment")
    }
}
/// A class for the available enchantment offers in the enchantment table.
#[repr(C)]
pub struct EnchantmentOffer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EnchantmentOffer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentOffer<'mc> {
    fn from_raw(
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
}

impl<'mc> EnchantmentOffer<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: i32,
    ) -> Result<crate::enchantments::EnchantmentOffer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;II)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let val_3 = jni::objects::JValueGen::Int(arg2);
        let cls = jni.find_class("org/bukkit/enchantments/EnchantmentOffer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::enchantments::EnchantmentOffer::from_raw(&jni, res)
    }

    pub fn enchantment(
        &self,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/enchantments/Enchantment;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchantment", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::enchantments::Enchantment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_enchantment(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEnchantment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn enchantment_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantmentLevel",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the level of the enchantment.
    pub fn set_enchantment_level(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEnchantmentLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cost(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCost", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the cost (minimum level) which is displayed as a number on the right hand side of the enchantment offer.
    pub fn set_cost(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCost",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents the applicable target for a <a href="Enchantment.html" title="class in org.bukkit.enchantments"><code>Enchantment</code></a>
pub enum EnchantmentTarget<'mc> {
    All { inner: EnchantmentTargetStruct<'mc> },
    ArmorHead { inner: EnchantmentTargetStruct<'mc> },
    Breakable { inner: EnchantmentTargetStruct<'mc> },
    Armor { inner: EnchantmentTargetStruct<'mc> },
    ArmorLegs { inner: EnchantmentTargetStruct<'mc> },
    FishingRod { inner: EnchantmentTargetStruct<'mc> },
    Bow { inner: EnchantmentTargetStruct<'mc> },
    ArmorFeet { inner: EnchantmentTargetStruct<'mc> },
    Trident { inner: EnchantmentTargetStruct<'mc> },
    Vanishable { inner: EnchantmentTargetStruct<'mc> },
    Wearable { inner: EnchantmentTargetStruct<'mc> },
    ArmorTorso { inner: EnchantmentTargetStruct<'mc> },
    Weapon { inner: EnchantmentTargetStruct<'mc> },
    Crossbow { inner: EnchantmentTargetStruct<'mc> },
    Tool { inner: EnchantmentTargetStruct<'mc> },
}
impl<'mc> std::fmt::Display for EnchantmentTarget<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnchantmentTarget::All { .. } => f.write_str("ALL"),
            EnchantmentTarget::ArmorHead { .. } => f.write_str("ARMOR_HEAD"),
            EnchantmentTarget::Breakable { .. } => f.write_str("BREAKABLE"),
            EnchantmentTarget::Armor { .. } => f.write_str("ARMOR"),
            EnchantmentTarget::ArmorLegs { .. } => f.write_str("ARMOR_LEGS"),
            EnchantmentTarget::FishingRod { .. } => f.write_str("FISHING_ROD"),
            EnchantmentTarget::Bow { .. } => f.write_str("BOW"),
            EnchantmentTarget::ArmorFeet { .. } => f.write_str("ARMOR_FEET"),
            EnchantmentTarget::Trident { .. } => f.write_str("TRIDENT"),
            EnchantmentTarget::Vanishable { .. } => f.write_str("VANISHABLE"),
            EnchantmentTarget::Wearable { .. } => f.write_str("WEARABLE"),
            EnchantmentTarget::ArmorTorso { .. } => f.write_str("ARMOR_TORSO"),
            EnchantmentTarget::Weapon { .. } => f.write_str("WEAPON"),
            EnchantmentTarget::Crossbow { .. } => f.write_str("CROSSBOW"),
            EnchantmentTarget::Tool { .. } => f.write_str("TOOL"),
        }
    }
}

impl<'mc> EnchantmentTarget<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EnchantmentTarget<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/enchantments/EnchantmentTarget");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/enchantments/EnchantmentTarget;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "ALL" => Ok(EnchantmentTarget::All {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "ARMOR_HEAD" => Ok(EnchantmentTarget::ArmorHead {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "BREAKABLE" => Ok(EnchantmentTarget::Breakable {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "ARMOR" => Ok(EnchantmentTarget::Armor {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "ARMOR_LEGS" => Ok(EnchantmentTarget::ArmorLegs {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "FISHING_ROD" => Ok(EnchantmentTarget::FishingRod {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "BOW" => Ok(EnchantmentTarget::Bow {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "ARMOR_FEET" => Ok(EnchantmentTarget::ArmorFeet {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "TRIDENT" => Ok(EnchantmentTarget::Trident {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "VANISHABLE" => Ok(EnchantmentTarget::Vanishable {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "WEARABLE" => Ok(EnchantmentTarget::Wearable {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "ARMOR_TORSO" => Ok(EnchantmentTarget::ArmorTorso {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "WEAPON" => Ok(EnchantmentTarget::Weapon {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "CROSSBOW" => Ok(EnchantmentTarget::Crossbow {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),
            "TOOL" => Ok(EnchantmentTarget::Tool {
                inner: EnchantmentTargetStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EnchantmentTargetStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EnchantmentTarget<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::All { inner } => inner.0.clone(),
            Self::ArmorHead { inner } => inner.0.clone(),
            Self::Breakable { inner } => inner.0.clone(),
            Self::Armor { inner } => inner.0.clone(),
            Self::ArmorLegs { inner } => inner.0.clone(),
            Self::FishingRod { inner } => inner.0.clone(),
            Self::Bow { inner } => inner.0.clone(),
            Self::ArmorFeet { inner } => inner.0.clone(),
            Self::Trident { inner } => inner.0.clone(),
            Self::Vanishable { inner } => inner.0.clone(),
            Self::Wearable { inner } => inner.0.clone(),
            Self::ArmorTorso { inner } => inner.0.clone(),
            Self::Weapon { inner } => inner.0.clone(),
            Self::Crossbow { inner } => inner.0.clone(),
            Self::Tool { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::All { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ArmorHead { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Breakable { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Armor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ArmorLegs { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FishingRod { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Bow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ArmorFeet { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Trident { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Vanishable { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Wearable { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ArmorTorso { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Weapon { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Crossbow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Tool { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentTarget<'mc> {
    fn from_raw(
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
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "ALL" => Ok(EnchantmentTarget::All {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "ARMOR_HEAD" => Ok(EnchantmentTarget::ArmorHead {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "BREAKABLE" => Ok(EnchantmentTarget::Breakable {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "ARMOR" => Ok(EnchantmentTarget::Armor {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "ARMOR_LEGS" => Ok(EnchantmentTarget::ArmorLegs {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "FISHING_ROD" => Ok(EnchantmentTarget::FishingRod {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "BOW" => Ok(EnchantmentTarget::Bow {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "ARMOR_FEET" => Ok(EnchantmentTarget::ArmorFeet {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "TRIDENT" => Ok(EnchantmentTarget::Trident {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "VANISHABLE" => Ok(EnchantmentTarget::Vanishable {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "WEARABLE" => Ok(EnchantmentTarget::Wearable {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "ARMOR_TORSO" => Ok(EnchantmentTarget::ArmorTorso {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "WEAPON" => Ok(EnchantmentTarget::Weapon {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "CROSSBOW" => Ok(EnchantmentTarget::Crossbow {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                "TOOL" => Ok(EnchantmentTarget::Tool {
                    inner: EnchantmentTargetStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EnchantmentTargetStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentTargetStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EnchantmentTargetStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/enchantments/EnchantmentTarget")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantmentTargetStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EnchantmentTargetStruct<'mc> {
    pub fn includes_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "includes", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<Vec<crate::enchantments::EnchantmentTarget<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/enchantments/EnchantmentTarget;");
        let cls = jni.find_class("org/bukkit/enchantments/EnchantmentTarget");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::enchantments::EnchantmentTarget::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// The various type of enchantments that may be added to armour or weapons
#[repr(C)]
pub struct Enchantment<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Enchantment<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Enchantment<'mc> {
    fn from_raw(
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
}

impl<'mc> Enchantment<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/NamespacedKey;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/enchantments/Enchantment");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::enchantments::Enchantment::from_raw(&jni, res)
    }

    pub fn register_enchantment(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "registerEnchantment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        jni.translate_error(res)?;
        Ok(())
    }

    pub fn get_by_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/enchantments/Enchantment;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/enchantments/Enchantment");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::enchantments::Enchantment::from_raw(&jni, obj)
    }

    pub fn start_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getStartLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn max_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn can_enchant_item(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "canEnchantItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn stop_accepting_registrations(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "stopAcceptingRegistrations", sig.as_str(), vec![]);
        jni.translate_error(res)?;
        Ok(())
    }

    pub fn item_target(
        &self,
    ) -> Result<crate::enchantments::EnchantmentTarget<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/enchantments/EnchantmentTarget;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemTarget", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::enchantments::EnchantmentTarget::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_treasure(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isTreasure", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    #[deprecated]

    pub fn is_cursed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCursed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn conflicts_with(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "conflictsWith",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_accepting_registrations(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isAcceptingRegistrations", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
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

    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<Vec<crate::enchantments::Enchantment<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/enchantments/Enchantment;");
        let cls = jni.find_class("org/bukkit/enchantments/Enchantment");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::enchantments::Enchantment::from_raw(&jni, res)? });
        }
        Ok(vec)
    }

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

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

    pub fn get_by_name(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/enchantments/Enchantment;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("org/bukkit/enchantments/Enchantment");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::enchantments::Enchantment::from_raw(&jni, obj)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> std::string::ToString for Enchantment<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling Enchantment.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::Keyed<'mc>> for Enchantment<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Enchantment into crate::Keyed")
    }
}
