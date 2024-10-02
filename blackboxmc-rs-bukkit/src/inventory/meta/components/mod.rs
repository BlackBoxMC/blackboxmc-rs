#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct FoodComponent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FoodComponent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FoodComponent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate FoodComponent from null object.").into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/components/FoodComponent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FoodComponent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FoodComponent<'mc> {
    /// Gets the food restored by this item when eaten.
    pub fn nutrition(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNutrition", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the food restored by this item when eaten.
    pub fn set_nutrition(&self, nutrition: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(nutrition);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNutrition",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the saturation restored by this item when eaten.
    pub fn saturation(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSaturation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the saturation restored by this item when eaten.
    pub fn set_saturation(&self, saturation: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(saturation);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSaturation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if this item can be eaten even when not hungry.
    pub fn can_always_eat(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "canAlwaysEat", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets if this item can be eaten even when not hungry.
    pub fn set_can_always_eat(
        &self,
        can_always_eat: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(can_always_eat.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCanAlwaysEat",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the time in seconds it will take for this item to be eaten.
    pub fn eat_seconds(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEatSeconds", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the time in seconds it will take for this item to be eaten.
    pub fn set_eat_seconds(&self, eat_seconds: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(eat_seconds);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEatSeconds",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the item this food will convert to once eaten.
    pub fn using_converts_to(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getUsingConvertsTo",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the item this food will convert to once eaten.
    pub fn set_using_converts_to(
        &self,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUsingConvertsTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the effects which may be applied by this item when eaten.
    pub fn effects(
        &self,
    ) -> Result<
        Vec<crate::inventory::meta::components::FoodComponentFoodEffect<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEffects", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                crate::inventory::meta::components::FoodComponentFoodEffect::from_raw(
                    &self.jni_ref(),
                    obj,
                )?,
            );
        }
        Ok(new_vec)
    }
    /// Sets the effects which may be applied by this item when eaten.
    pub fn set_effects(
        &self,
        effects: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)V");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in effects {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEffects",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Adds an effect which may be applied by this item when eaten.
    pub fn add_effect(
        &self,
        effect: impl Into<crate::potion::PotionEffect<'mc>>,
        probability: f32,
    ) -> Result<
        crate::inventory::meta::components::FoodComponentFoodEffect<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("(Lorg/bukkit/potion/PotionEffect;F)Lorg/bukkit/inventory/meta/components/FoodComponent/FoodEffect;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(effect.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Float(probability);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEffect",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::components::FoodComponentFoodEffect::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }
    /// Creates a Map representation of this class.
    ///
    /// This class must provide a method to restore this class, as defined in
    /// the {@link ConfigurationSerializable} interface javadocs.
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "serialize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for FoodComponent<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting FoodComponent into crate::configuration::serialization::ConfigurationSerializable")
    }
}
#[repr(C)]
pub struct FoodComponentFoodEffect<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FoodComponentFoodEffect<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FoodComponentFoodEffect<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate FoodComponentFoodEffect from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/inventory/meta/components/FoodComponent/FoodEffect",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FoodComponentFoodEffect object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FoodComponentFoodEffect<'mc> {
    /// Gets the effect which may be applied.
    pub fn effect(&self) -> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffect;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEffect", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffect::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the effect which may be applied.
    pub fn set_effect(
        &self,
        effect: impl Into<crate::potion::PotionEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionEffect;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(effect.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the probability of this effect being applied.
    pub fn probability(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getProbability", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the probability of this effect being applied.
    pub fn set_probability(&self, probability: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(probability);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProbability",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a Map representation of this class.
    ///
    /// This class must provide a method to restore this class, as defined in
    /// the {@link ConfigurationSerializable} interface javadocs.
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "serialize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for FoodComponentFoodEffect<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting FoodComponentFoodEffect into crate::configuration::serialization::ConfigurationSerializable")
    }
}
#[repr(C)]
pub struct JukeboxPlayableComponent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JukeboxPlayableComponent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for JukeboxPlayableComponent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JukeboxPlayableComponent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/inventory/meta/components/JukeboxPlayableComponent",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JukeboxPlayableComponent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JukeboxPlayableComponent<'mc> {
    /// Gets the song assigned to this component.
    pub fn song(&self) -> Result<Option<crate::JukeboxSong<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/JukeboxSong;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSong", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::JukeboxSong::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the key of the song assigned to this component.
    pub fn song_key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/NamespacedKey;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSongKey", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::NamespacedKey::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the song assigned to this component.
    pub fn set_song(
        &self,
        song: impl Into<crate::JukeboxSong<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/JukeboxSong;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(song.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSong",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the key of the song assigned to this component.
    pub fn set_song_key(
        &self,
        song: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/NamespacedKey;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(song.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSongKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the song will show in the item tooltip.
    pub fn is_show_in_tooltip(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isShowInTooltip", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets if the song will show in the item tooltip.
    pub fn set_show_in_tooltip(&self, show: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(show.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setShowInTooltip",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a Map representation of this class.
    ///
    /// This class must provide a method to restore this class, as defined in
    /// the {@link ConfigurationSerializable} interface javadocs.
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "serialize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for JukeboxPlayableComponent<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting JukeboxPlayableComponent into crate::configuration::serialization::ConfigurationSerializable")
    }
}
#[repr(C)]
pub struct ToolComponent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ToolComponent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ToolComponent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ToolComponent from null object.").into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/components/ToolComponent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ToolComponent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ToolComponent<'mc> {
    /// Get the default mining speed of this tool. This value is used by the tool
    /// if no rule explicitly overrides it. 1.0 is standard mining speed.
    pub fn default_mining_speed(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultMiningSpeed",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Set the default mining speed of this tool. This value is used by the tool
    /// if no rule explicitly overrides it. 1.0 is standard mining speed.
    pub fn set_default_mining_speed(&self, speed: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(speed);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDefaultMiningSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the amount of durability to be removed from the tool each time a
    /// block is broken.
    pub fn damage_per_block(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDamagePerBlock",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the amount of durability to be removed from the tool each time a
    /// block is broken.
    pub fn set_damage_per_block(&self, damage: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(damage);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamagePerBlock",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the list of {@link ToolRule ToolRules} that apply to this tool.
    pub fn rules(
        &self,
    ) -> Result<
        Vec<crate::inventory::meta::components::ToolComponentToolRule<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRules", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                crate::inventory::meta::components::ToolComponentToolRule::from_raw(
                    &self.jni_ref(),
                    obj,
                )?,
            );
        }
        Ok(new_vec)
    }
    /// Set the list of {@link ToolRule ToolRules} to apply to this tool. This
    /// will remove any existing tool rules.
    pub fn set_rules(
        &self,
        rules: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)V");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in rules {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRules",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Add a new rule to this tool component, which provides further information
    /// about a collection of block types represented by a block {@link Tag}.
    pub fn add_rule(
        &self,
        tag: impl Into<crate::Tag<'mc>>,
        speed: f32,
        correct_for_drops: bool,
    ) -> Result<
        crate::inventory::meta::components::ToolComponentToolRule<'mc>,
        Box<dyn std::error::Error>,
    > {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Tag;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tag.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/Float;";
        let val_2 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Float",
            "(Ljava/Lang/Object;)V",
            vec![speed.into()],
        )?);
        args.push(val_2);
        sig += "Ljava/lang/Boolean;";
        let val_3 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Ljava/Lang/Object;)V",
            vec![correct_for_drops.into()],
        )?);
        args.push(val_3);
        sig += ")Lorg/bukkit/inventory/meta/components/ToolComponent/ToolRule;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addRule", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::components::ToolComponentToolRule::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }
    /// Remove the given {@link ToolRule} from this tool.
    pub fn remove_rule(
        &self,
        rule: impl Into<crate::inventory::meta::components::ToolComponentToolRule<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/meta/components/ToolComponent/ToolRule;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rule.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeRule",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Creates a Map representation of this class.
    ///
    /// This class must provide a method to restore this class, as defined in
    /// the {@link ConfigurationSerializable} interface javadocs.
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "serialize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for ToolComponent<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting ToolComponent into crate::configuration::serialization::ConfigurationSerializable")
    }
}
#[repr(C)]
pub struct ToolComponentToolRule<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ToolComponentToolRule<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ToolComponentToolRule<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ToolComponentToolRule from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/inventory/meta/components/ToolComponent/ToolRule",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ToolComponentToolRule object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ToolComponentToolRule<'mc> {
    /// Get a collection of the block types to which this tool rule applies.
    pub fn blocks(&self) -> Result<Vec<crate::Material<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlocks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::Material::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Set the block types (represented as a block {@link Tag}) to which
    /// this rule applies.
    pub fn set_blocks(
        &self,
        tag: impl Into<crate::Tag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Tag;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tag.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setBlocks", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the mining speed of this rule. If non-null, this speed value is
    /// used in lieu of the default speed value of the tool. 1.0 is standard
    /// mining speed.
    pub fn speed(&self) -> Result<Option<f32>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Float;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSpeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(Some(res.f()?))
    }
    /// Set the mining speed of this rule. 1.0 is standard mining speed.
    pub fn set_speed(&self, speed: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Float;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Float",
            "(Ljava/Lang/Object;)V",
            vec![speed.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get whether or not this rule is considered the optimal tool for the
    /// blocks listed by this rule and will drop items. If non-null, this
    /// value is used in lieu of the default tool checking behavior defined
    /// by Minecraft.
    pub fn is_correct_for_drops(&self) -> Result<Option<bool>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCorrectForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(Some(res.z()?))
    }
    /// Set whether or not this rule is considered the optimal tool for the
    /// blocks listed by this rule and will drop items.
    pub fn set_correct_for_drops(&self, correct: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Ljava/Lang/Object;)V",
            vec![correct.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCorrectForDrops",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a Map representation of this class.
    ///
    /// This class must provide a method to restore this class, as defined in
    /// the {@link ConfigurationSerializable} interface javadocs.
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "serialize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for ToolComponentToolRule<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting ToolComponentToolRule into crate::configuration::serialization::ConfigurationSerializable")
    }
}
