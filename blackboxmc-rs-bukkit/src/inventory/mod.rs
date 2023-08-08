#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Interface to the inventory of a Cartography table.
///
/// This is a representation of an abstract class.
pub struct CartographyInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CartographyInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CartographyInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/CartographyInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CartographyInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for CartographyInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for CartographyInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents some type of crafting recipe.
///
/// This is a representation of an abstract class.
pub struct Recipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Recipe<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Recipe from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/Recipe")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Recipe object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Get the result of this recipe.
    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for Recipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents a Stonecutting recipe.
pub struct StonecuttingRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for StonecuttingRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> StonecuttingRecipe<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate StonecuttingRecipe from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/StonecuttingRecipe")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StonecuttingRecipe object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_namespaced_key(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg2: std::option::Option<impl Into<&'mc crate::inventory::RecipeChoice<'mc>>>,
    ) -> Result<crate::inventory::StonecuttingRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 =
            unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/inventory/StonecuttingRecipe")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/RecipeChoice;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        crate::inventory::StonecuttingRecipe::from_raw(&jni, res)
    }
    /// Get the result of this recipe.
    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the input material.
    /// Get the input choice.
    pub fn input(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInput",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the input of this cooking recipe.
    /// Sets the input of this cooking recipe.
    pub fn set_input(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<crate::inventory::StonecuttingRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInput",
            "(Lorg/bukkit/Material;)Lorg/bukkit/inventory/StonecuttingRecipe;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::StonecuttingRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the input of this cooking recipe.
    pub fn set_input_choice(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::StonecuttingRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInputChoice",
            "(Lorg/bukkit/inventory/RecipeChoice;)Lorg/bukkit/inventory/StonecuttingRecipe;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::StonecuttingRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the input choice.
    pub fn input_choice(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInputChoice",
            "()Lorg/bukkit/inventory/RecipeChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the group of this recipe. Recipes with the same group may be grouped together when displayed in the client.
    pub fn group(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getGroup", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Set the group of this recipe. Recipes with the same group may be grouped together when displayed in the client.
    pub fn set_group(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGroup",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
impl<'mc> Into<crate::inventory::Recipe<'mc>> for StonecuttingRecipe<'mc> {
    fn into(self) -> crate::inventory::Recipe<'mc> {
        crate::inventory::Recipe::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for StonecuttingRecipe<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a furnace recipe.
pub struct FurnaceRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FurnaceRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FurnaceRecipe<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate FurnaceRecipe from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/FurnaceRecipe")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FurnaceRecipe object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    #[deprecated]
    pub fn new_with_item_stack(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: impl Into<&'mc crate::Material<'mc>>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::inventory::FurnaceRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let cls = &jni.find_class("org/bukkit/inventory/FurnaceRecipe")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/Material;I)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        crate::inventory::FurnaceRecipe::from_raw(&jni, res)
    }
    #[deprecated]
    pub fn new_with_namespaced_key(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<&'mc crate::Material<'mc>>,
        arg3: i32,
        arg4: f32,
        arg5: std::option::Option<i32>,
    ) -> Result<crate::inventory::FurnaceRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = jni::objects::JValueGen::Float(arg4.into());
        let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
        let cls = &jni.find_class("org/bukkit/inventory/FurnaceRecipe")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/Material;IFI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
                jni::objects::JValueGen::from(&val_6),
            ],
        )?;
        crate::inventory::FurnaceRecipe::from_raw(&jni, res)
    }
    #[deprecated]
    /// Sets the input of this furnace recipe.
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="CookingRecipe.html#setInput(org.bukkit.Material)">CookingRecipe</a></code></span>
    /// Sets the input of this cooking recipe.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Sets the input of this furnace recipe.
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="CookingRecipe.html#setInputChoice(org.bukkit.inventory.RecipeChoice)">CookingRecipe</a></code></span>
    /// Sets the input of this cooking recipe.
    pub fn set_input_with_material(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::inventory::FurnaceRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInput",
            "(Lorg/bukkit/Material;I)Lorg/bukkit/inventory/FurnaceRecipe;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::FurnaceRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="CookingRecipe.html#setInputChoice(org.bukkit.inventory.RecipeChoice)">CookingRecipe</a></code></span>
    /// Sets the input of this cooking recipe.
    pub fn set_input_choice_with_recipe_choice(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::inventory::RecipeChoice<'mc>>>,
    ) -> Result<crate::inventory::FurnaceRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInputChoice",
            "(Lorg/bukkit/inventory/RecipeChoice;)Lorg/bukkit/inventory/FurnaceRecipe;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::FurnaceRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn input(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInput",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn category(
        &mut self,
    ) -> Result<crate::inventory::recipe::CookingBookCategory<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCategory",
            "()Lorg/bukkit/inventory/recipe/CookingBookCategory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::inventory::recipe::CookingBookCategory::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::inventory::recipe::CookingBookCategory::from_string(variant_str).unwrap(),
        )
    }

    pub fn input_choice(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInputChoice",
            "()Lorg/bukkit/inventory/RecipeChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn group(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getGroup", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_group(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGroup",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_category(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::recipe::CookingBookCategory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCategory",
            "(Lorg/bukkit/inventory/recipe/CookingBookCategory;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn experience(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getExperience", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }

    pub fn set_experience(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExperience",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_cooking_time(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookingTime",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cooking_time(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCookingTime", "()I", &[]);
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
impl<'mc> Into<crate::inventory::CookingRecipe<'mc>> for FurnaceRecipe<'mc> {
    fn into(self) -> crate::inventory::CookingRecipe<'mc> {
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An interface to the inventory of a Horse.
///
/// This is a representation of an abstract class.
pub struct HorseInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> HorseInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate HorseInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/HorseInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HorseInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Gets the item in the horse's armor slot.
    pub fn armor(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getArmor",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the item in the horse's armor slot.
    pub fn set_armor(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setArmor",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_saddle(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSaddle",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn saddle(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSaddle",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for HorseInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::AbstractHorseInventory<'mc>> for HorseInventory<'mc> {
    fn into(self) -> crate::inventory::AbstractHorseInventory<'mc> {
        crate::inventory::AbstractHorseInventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Interface to the inventory of a Double Chest.
///
/// This is a representation of an abstract class.
pub struct DoubleChestInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> DoubleChestInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DoubleChestInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/DoubleChestInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DoubleChestInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Inventory.html#getHolder()">Inventory</a></code></span>
    /// Gets the block or entity belonging to the open inventory
    pub fn holder(&mut self) -> Result<crate::block::DoubleChest<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/block/DoubleChest;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::DoubleChest::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the left half of this double chest.
    pub fn left_side(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLeftSide",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the right side of this double chest.
    pub fn right_side(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRightSide",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for DoubleChestInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for DoubleChestInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a complex recipe which has imperative server-defined behavior, eg armor dyeing. Note: Since a complex recipe has dynamic outputs, <a href="Recipe.html#getResult()"><code>Recipe.getResult()</code></a> will sometimes return an AIR ItemStack.
///
/// This is a representation of an abstract class.
pub struct ComplexRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ComplexRecipe<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ComplexRecipe from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ComplexRecipe")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ComplexRecipe object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }

    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
}
impl<'mc> JNIRaw<'mc> for ComplexRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Recipe<'mc>> for ComplexRecipe<'mc> {
    fn into(self) -> crate::inventory::Recipe<'mc> {
        crate::inventory::Recipe::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for ComplexRecipe<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a block inventory holder - either a BlockState, or a regular Block.
///
/// This is a representation of an abstract class.
pub struct BlockInventoryHolder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BlockInventoryHolder<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockInventoryHolder from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/BlockInventoryHolder")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockInventoryHolder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Gets the block associated with this holder.
    pub fn block(&mut self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlock",
            "()Lorg/bukkit/block/Block;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for BlockInventoryHolder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for BlockInventoryHolder<'mc> {
    fn into(self) -> crate::inventory::InventoryHolder<'mc> {
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Interface to the inventory of a Jukebox.
///
/// This is a representation of an abstract class.
pub struct JukeboxInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JukeboxInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JukeboxInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/JukeboxInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JukeboxInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Inventory.html#getHolder()">Inventory</a></code></span>
    /// Gets the block or entity belonging to the open inventory
    pub fn holder(&mut self) -> Result<crate::block::Jukebox<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/block/Jukebox;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Jukebox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the record in the jukebox.
    /// <p>This will immediately start playing the inserted item or stop playing if the item provided is null. If the provided item is not a record (according to <a href="../Tag.html#ITEMS_MUSIC_DISCS"><code>Tag.ITEMS_MUSIC_DISCS</code></a>), this method will do nothing and not set the item in the inventory.</p>
    pub fn set_record(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRecord",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the record in the jukebox.
    pub fn record(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRecord",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JukeboxInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for JukeboxInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a smithing recipe.
pub struct SmithingRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SmithingRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SmithingRecipe<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SmithingRecipe from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/SmithingRecipe")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SmithingRecipe object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    #[deprecated]
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
        arg3: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::SmithingRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/inventory/SmithingRecipe")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/RecipeChoice;Lorg/bukkit/inventory/RecipeChoice;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
        crate::inventory::SmithingRecipe::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Recipe.html#getResult()">Recipe</a></code></span>
    /// Get the result of this recipe.
    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the base recipe item.
    pub fn base(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBase",
            "()Lorg/bukkit/inventory/RecipeChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the addition recipe item.
    pub fn addition(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAddition",
            "()Lorg/bukkit/inventory/RecipeChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
impl<'mc> Into<crate::inventory::Recipe<'mc>> for SmithingRecipe<'mc> {
    fn into(self) -> crate::inventory::Recipe<'mc> {
        crate::inventory::Recipe::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for SmithingRecipe<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a shaped or shapeless crafting recipe.
pub struct CraftingRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for CraftingRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CraftingRecipe<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CraftingRecipe from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/CraftingRecipe")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CraftingRecipe object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Get the result of this recipe.
    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the category which this recipe will appear in the recipe book under. Defaults to <a href="recipe/CraftingBookCategory.html#MISC"><code>CraftingBookCategory.MISC</code></a> if not set.
    pub fn category(
        &mut self,
    ) -> Result<crate::inventory::recipe::CraftingBookCategory<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCategory",
            "()Lorg/bukkit/inventory/recipe/CraftingBookCategory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::inventory::recipe::CraftingBookCategory::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::inventory::recipe::CraftingBookCategory::from_string(variant_str).unwrap(),
        )
    }
    /// Get the group of this recipe. Recipes with the same group may be grouped together when displayed in the client.
    pub fn group(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getGroup", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Set the group of this recipe. Recipes with the same group may be grouped together when displayed in the client.
    pub fn set_group(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGroup",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the category which this recipe will appear in the recipe book under. Defaults to <a href="recipe/CraftingBookCategory.html#MISC"><code>CraftingBookCategory.MISC</code></a> if not set.
    pub fn set_category(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::recipe::CraftingBookCategory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCategory",
            "(Lorg/bukkit/inventory/recipe/CraftingBookCategory;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
impl<'mc> Into<crate::inventory::Recipe<'mc>> for CraftingRecipe<'mc> {
    fn into(self) -> crate::inventory::Recipe<'mc> {
        crate::inventory::Recipe::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for CraftingRecipe<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a view linking two inventories and a single player (whose inventory may or may not be one of the two).
/// <p>Note: If you implement this interface but fail to satisfy the expected contracts of certain methods, there's no guarantee that the game will work as it should.</p>
pub struct InventoryView<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub struct InventoryViewProperty<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryViewProperty<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryViewProperty<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InventoryViewProperty from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/InventoryViewProperty")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryViewProperty object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
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
    #[deprecated]
    pub fn id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getId", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
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
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryView<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryView<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate InventoryView from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/InventoryView")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryView object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/inventory/InventoryView")?;
        let res = jni.new_object(cls, "()V", &[])?;
        crate::inventory::InventoryView::from_raw(&jni, res)
    }
    /// Gets one item in this inventory view by its raw slot ID.
    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets one item in this inventory view by its raw slot ID.
    /// <p>Note: If slot ID -999 is chosen, it may be expected that the item is dropped on the ground. This is not required behaviour, however.</p>
    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the inventory corresponding to the given raw slot ID. If the slot ID is <a href="#OUTSIDE"><code>OUTSIDE</code></a> null will be returned, otherwise behaviour for illegal and negative slot IDs is undefined. May be used with <a href="#convertSlot(int)"><code>convertSlot(int)</code></a> to directly index an underlying inventory.
    pub fn get_inventory(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "(I)Lorg/bukkit/inventory/Inventory;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the title of this inventory window.
    pub fn title(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTitle", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Get the player viewing.
    pub fn player(
        &mut self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlayer",
            "()Lorg/bukkit/entity/HumanEntity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the upper inventory involved in this transaction.
    pub fn top_inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTopInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the lower inventory involved in this transaction.
    pub fn bottom_inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBottomInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Converts a raw slot ID into its local slot ID into whichever of the two inventories the slot points to.
    /// <p>If the raw slot refers to the upper inventory, it will be returned unchanged and thus be suitable for getTopInventory().getItem(); if it refers to the lower inventory, the output will differ from the input and be suitable for getBottomInventory().getItem().</p>
    pub fn convert_slot(&mut self, arg0: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "convertSlot",
            "(I)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Sets the item on the cursor of one of the viewing players.
    pub fn set_cursor(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCursor",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the item on the cursor of one of the viewing players.
    pub fn cursor(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCursor",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Check the total number of slots in this view, combining the upper and lower inventories.
    /// <p>Note though that it's possible for this to be greater than the sum of the two inventories if for example some slots are not being used.</p>
    pub fn count_slots(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "countSlots", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Determine the type of the slot by its raw slot ID.
    /// <p>If the type of the slot is unknown, then <a href="../event/inventory/InventoryType.SlotType.html#CONTAINER"><code>InventoryType.SlotType.CONTAINER</code></a> will be returned.</p>
    pub fn get_slot_type(
        &mut self,
        arg0: i32,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            "(I)Lorg/bukkit/event/inventory/InventoryType$SlotType;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the original title of this inventory window, before any changes were made using <a href="#setTitle(java.lang.String)"><code>setTitle(String)</code></a>.
    pub fn original_title(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOriginalTitle",
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
    /// Sets the title of this inventory window to the specified title if the inventory window supports it.
    /// <p>Note if the inventory does not support titles that can be changed (ie, it is not creatable or viewed by a player), then this method will throw an exception.</p>
    pub fn set_title(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets an extra property of this inventory if supported by that inventory, for example the state of a progress bar.
    pub fn set_property(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::InventoryViewProperty<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProperty",
            "(Lorg/bukkit/inventory/InventoryView$Property;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Closes the inventory view.
    pub fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Determine the type of inventory involved in the transaction. This indicates the window style being shown. It will never return PLAYER, since that is common to all windows.
    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
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
/// Represents a campfire recipe.
pub struct CampfireRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for CampfireRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CampfireRecipe<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CampfireRecipe from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/CampfireRecipe")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CampfireRecipe object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_namespaced_key(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
        arg3: f32,
        arg4: std::option::Option<i32>,
    ) -> Result<crate::inventory::CampfireRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = jni::objects::JValueGen::Float(arg3.into());
        let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
        let cls = &jni.find_class("org/bukkit/inventory/CampfireRecipe")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/RecipeChoice;FI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
        crate::inventory::CampfireRecipe::from_raw(&jni, res)
    }

    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn input(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInput",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_input(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInput",
            "(Lorg/bukkit/Material;)Lorg/bukkit/inventory/CookingRecipe;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn category(
        &mut self,
    ) -> Result<crate::inventory::recipe::CookingBookCategory<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCategory",
            "()Lorg/bukkit/inventory/recipe/CookingBookCategory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::inventory::recipe::CookingBookCategory::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::inventory::recipe::CookingBookCategory::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_input_choice(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInputChoice",
            "(Lorg/bukkit/inventory/RecipeChoice;)Lorg/bukkit/inventory/CookingRecipe;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn input_choice(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInputChoice",
            "()Lorg/bukkit/inventory/RecipeChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn group(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getGroup", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_group(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGroup",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_category(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::recipe::CookingBookCategory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCategory",
            "(Lorg/bukkit/inventory/recipe/CookingBookCategory;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn experience(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getExperience", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }

    pub fn set_experience(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExperience",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_cooking_time(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookingTime",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cooking_time(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCookingTime", "()I", &[]);
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
impl<'mc> Into<crate::inventory::CookingRecipe<'mc>> for CampfireRecipe<'mc> {
    fn into(self) -> crate::inventory::CookingRecipe<'mc> {
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a stack of items.
/// <p><b>IMPORTANT: An <i>Item</i>Stack is only designed to contain <i>items</i>. Do not use this class to encapsulate Materials for which <a href="../Material.html#isItem()"><code>Material.isItem()</code></a> returns false.</b></p>
pub struct ItemStack<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ItemStack<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ItemStack<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ItemStack from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ItemStack")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemStack object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    #[deprecated]
    pub fn new_with_material(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: i32,
        arg2: i16,
        arg3: std::option::Option<i8>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Short(arg2.into());
        let val_4 = jni::objects::JValueGen::Byte(arg3.unwrap().into());
        let cls = &jni.find_class("org/bukkit/inventory/ItemStack")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;ISB)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        )?;
        crate::inventory::ItemStack::from_raw(&jni, res)
    }
    /// Sets the type of this item
    /// <p>Note that in doing so you will reset the MaterialData for this stack.</p>
    /// <p><b>IMPORTANT: An <i>Item</i>Stack is only designed to contain <i>items</i>. Do not use this class to encapsulate Materials for which <a href="../Material.html#isItem()"><code>Material.isItem()</code></a> returns false.</b></p>
    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../configuration/serialization/ConfigurationSerializable.html#serialize()">ConfigurationSerializable</a></code></span>
    /// Creates a Map representation of this class.
    /// <p>This class must provide a method to restore this class, as defined in the <a href="../configuration/serialization/ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a> interface javadocs.</p>
    pub fn serialize(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "serialize", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Required method for configuration serialization
    pub fn deserialize(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc blackboxmc_java::JavaMap<'mc>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/inventory/ItemStack")?;
        let res = jni.call_static_method(
            cls,
            "deserialize",
            "(Ljava/util/Map;)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        let obj = res.l()?;
        crate::inventory::ItemStack::from_raw(&jni, obj)
    }
    /// Gets the MaterialData for this stack of items
    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the MaterialData for this stack of items
    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Translatable.html#getTranslationKey()">Translatable</a></code></span>
    /// Get the translation key, suitable for use in a translation component.
    pub fn translation_key(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTranslationKey",
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
    /// Get the maximum stacksize for the material hold in this ItemStack. (Returns -1 if it has no idea)
    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets the level of the specified enchantment on this item stack
    pub fn get_enchantment_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantmentLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// durability is now part of ItemMeta. To avoid confusion and misuse, <a href="#getItemMeta()"><code>getItemMeta()</code></a>, <a href="#setItemMeta(org.bukkit.inventory.meta.ItemMeta)"><code>setItemMeta(ItemMeta)</code></a> and <a href="meta/Damageable.html#setDamage(int)"><code>Damageable.setDamage(int)</code></a> should be used instead. This is because any call to this method will be overwritten by subsequent setting of ItemMeta which was created before this call.
    /// </div>
    /// durability is now part of ItemMeta. To avoid confusion and misuse, <a href="#getItemMeta()"><code>getItemMeta()</code></a>, <a href="#setItemMeta(org.bukkit.inventory.meta.ItemMeta)"><code>setItemMeta(ItemMeta)</code></a> and <a href="meta/Damageable.html#setDamage(int)"><code>Damageable.setDamage(int)</code></a> should be used instead. This is because any call to this method will be overwritten by subsequent setting of ItemMeta which was created before this call.
    ///
    /// Sets the durability of this item
    pub fn set_durability(&mut self, arg0: i16) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Short(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDurability",
            "(S)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the amount of items in this stack
    pub fn amount(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAmount", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Checks to see if any meta data has been defined.
    pub fn has_item_meta(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasItemMeta", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Get a copy of this ItemStack's <a href="meta/ItemMeta.html" title="interface in org.bukkit.inventory.meta"><code>ItemMeta</code></a>.
    pub fn item_meta(
        &mut self,
    ) -> Result<crate::inventory::meta::ItemMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemMeta",
            "()Lorg/bukkit/inventory/meta/ItemMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the amount of items in this stack
    pub fn set_amount(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAmount",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// see <a href="#setDurability(short)"><code>setDurability(short)</code></a>
    /// </div>
    /// see <a href="#setDurability(short)"><code>setDurability(short)</code></a>
    ///
    /// Gets the durability of this item
    pub fn durability(&mut self) -> Result<i16, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDurability", "()S", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s().unwrap())
    }
    /// Set the ItemMeta of this ItemStack.
    pub fn set_item_meta(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::meta::ItemMeta<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemMeta",
            "(Lorg/bukkit/inventory/meta/ItemMeta;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// This method is the same as equals, but does not consider stack size (amount).
    pub fn is_similar(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSimilar",
            "(Lorg/bukkit/inventory/ItemStack;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks if this ItemStack contains the given <a href="../enchantments/Enchantment.html" title="class in org.bukkit.enchantments"><code>Enchantment</code></a>
    pub fn contains_enchantment(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsEnchantment",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets a map containing all enchantments and their levels on this item.
    pub fn enchantments(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantments",
            "()Ljava/util/Map;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Adds the specified enchantments to this item stack.
    /// <p>This method is the same as calling <a href="#addEnchantment(org.bukkit.enchantments.Enchantment,int)"><code>addEnchantment(org.bukkit.enchantments.Enchantment, int)</code></a> for each element of the map.</p>
    pub fn add_enchantments(
        &mut self,
        arg0: impl Into<&'mc blackboxmc_java::JavaMap<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchantments",
            "(Ljava/util/Map;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Adds the specified enchantments to this item stack.
    /// <p>This method is the same as calling <a href="#addEnchantment(org.bukkit.enchantments.Enchantment,int)"><code>addEnchantment(org.bukkit.enchantments.Enchantment, int)</code></a> for each element of the map.</p>
    /// Adds the specified <a href="../enchantments/Enchantment.html" title="class in org.bukkit.enchantments"><code>Enchantment</code></a> to this item stack.
    /// <p>If this item stack already contained the given enchantment (at any level), it will be replaced.</p>
    pub fn add_enchantment(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchantment",
            "(Lorg/bukkit/enchantments/Enchantment;I)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Adds the specified enchantments to this item stack in an unsafe manner.
    /// <p>This method is the same as calling <a href="#addUnsafeEnchantment(org.bukkit.enchantments.Enchantment,int)"><code>addUnsafeEnchantment(org.bukkit.enchantments.Enchantment, int)</code></a> for each element of the map.</p>
    /// Adds the specified <a title="class in org.bukkit.enchantments" href="../enchantments/Enchantment.html"><code>Enchantment</code></a> to this item stack.
    /// <p>If this item stack already contained the given enchantment (at any level), it will be replaced.</p>
    /// <p>This method is unsafe and will ignore level restrictions or item type. Use at your own discretion.</p>
    pub fn add_unsafe_enchantment(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addUnsafeEnchantment",
            "(Lorg/bukkit/enchantments/Enchantment;I)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Adds the specified enchantments to this item stack in an unsafe manner.
    /// <p>This method is the same as calling <a href="#addUnsafeEnchantment(org.bukkit.enchantments.Enchantment,int)"><code>addUnsafeEnchantment(org.bukkit.enchantments.Enchantment, int)</code></a> for each element of the map.</p>
    pub fn add_unsafe_enchantments(
        &mut self,
        arg0: impl Into<&'mc blackboxmc_java::JavaMap<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addUnsafeEnchantments",
            "(Ljava/util/Map;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Removes the specified <a href="../enchantments/Enchantment.html" title="class in org.bukkit.enchantments"><code>Enchantment</code></a> if it exists on this ItemStack
    pub fn remove_enchantment(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchantment",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
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

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the type of this item
    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
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
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for ItemStack<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            self.1,
        )
        .unwrap()
    }
}
impl<'mc> Into<crate::Translatable<'mc>> for ItemStack<'mc> {
    fn into(self) -> crate::Translatable<'mc> {
        crate::Translatable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a choice of multiple matching Materials.
pub struct RecipeChoiceMaterialChoice<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for RecipeChoiceMaterialChoice<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RecipeChoiceMaterialChoice<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate RecipeChoiceMaterialChoice from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/RecipeChoiceMaterialChoice")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RecipeChoiceMaterialChoice object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<crate::inventory::RecipeChoiceMaterialChoice<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/inventory/RecipeChoice$MaterialChoice")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::inventory::RecipeChoiceMaterialChoice::from_raw(&jni, res)
    }
    pub fn new_with_materials(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::Tag<'mc>>>,
    ) -> Result<crate::inventory::RecipeChoiceMaterialChoice<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/inventory/RecipeChoice$MaterialChoice")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Tag;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::inventory::RecipeChoiceMaterialChoice::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="RecipeChoice.html#getItemStack()">RecipeChoice</a></code></span>
    /// Gets a single item stack representative of this stack choice.
    pub fn item_stack(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemStack",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn choices(&mut self) -> Result<Vec<crate::Material<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChoices", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            let variant =
                self.0
                    .call_method(list.get(i)?, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            new_vec.push(crate::Material::from_raw(
                &self.0,
                obj,
                crate::Material::from_string(variant_str).unwrap(),
            )?);
        }
        Ok(new_vec)
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

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoiceMaterialChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/RecipeChoice$MaterialChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoiceMaterialChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn test_with_object(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "test",
            "(Lorg/bukkit/inventory/ItemStack;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
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
pub enum MainHandEnum {
    Left,
    Right,
}
impl std::fmt::Display for MainHandEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            MainHandEnum::Left => f.write_str("LEFT"),
            MainHandEnum::Right => f.write_str("RIGHT"),
        }
    }
}
pub struct MainHand<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub MainHandEnum,
);
impl<'mc> std::ops::Deref for MainHand<'mc> {
    type Target = MainHandEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for MainHand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MainHand<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: MainHandEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MainHand from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/MainHand")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MainHand object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const LEFT: MainHandEnum = MainHandEnum::Left;
    pub const RIGHT: MainHandEnum = MainHandEnum::Right;
    pub fn from_string(str: String) -> std::option::Option<MainHandEnum> {
        match str.as_str() {
            "LEFT" => Some(MainHandEnum::Left),
            "RIGHT" => Some(MainHandEnum::Right),
            _ => None,
        }
    }
}
pub enum ItemFlagEnum {
    HideEnchants,
    HideAttributes,
    HideUnbreakable,
    HideDestroys,
    HidePlacedOn,
    HidePotionEffects,
    HideDye,
    HideArmorTrim,
}
impl std::fmt::Display for ItemFlagEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ItemFlagEnum::HideEnchants => f.write_str("HIDE_ENCHANTS"),
            ItemFlagEnum::HideAttributes => f.write_str("HIDE_ATTRIBUTES"),
            ItemFlagEnum::HideUnbreakable => f.write_str("HIDE_UNBREAKABLE"),
            ItemFlagEnum::HideDestroys => f.write_str("HIDE_DESTROYS"),
            ItemFlagEnum::HidePlacedOn => f.write_str("HIDE_PLACED_ON"),
            ItemFlagEnum::HidePotionEffects => f.write_str("HIDE_POTION_EFFECTS"),
            ItemFlagEnum::HideDye => f.write_str("HIDE_DYE"),
            ItemFlagEnum::HideArmorTrim => f.write_str("HIDE_ARMOR_TRIM"),
        }
    }
}
pub struct ItemFlag<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub ItemFlagEnum,
);
impl<'mc> std::ops::Deref for ItemFlag<'mc> {
    type Target = ItemFlagEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for ItemFlag<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ItemFlag<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: ItemFlagEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ItemFlag from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ItemFlag")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemFlag object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const HIDE_ENCHANTS: ItemFlagEnum = ItemFlagEnum::HideEnchants;
    pub const HIDE_ATTRIBUTES: ItemFlagEnum = ItemFlagEnum::HideAttributes;
    pub const HIDE_UNBREAKABLE: ItemFlagEnum = ItemFlagEnum::HideUnbreakable;
    pub const HIDE_DESTROYS: ItemFlagEnum = ItemFlagEnum::HideDestroys;
    pub const HIDE_PLACED_ON: ItemFlagEnum = ItemFlagEnum::HidePlacedOn;
    pub const HIDE_POTION_EFFECTS: ItemFlagEnum = ItemFlagEnum::HidePotionEffects;
    pub const HIDE_DYE: ItemFlagEnum = ItemFlagEnum::HideDye;
    pub const HIDE_ARMOR_TRIM: ItemFlagEnum = ItemFlagEnum::HideArmorTrim;
    pub fn from_string(str: String) -> std::option::Option<ItemFlagEnum> {
        match str.as_str() {
            "HIDE_ENCHANTS" => Some(ItemFlagEnum::HideEnchants),
            "HIDE_ATTRIBUTES" => Some(ItemFlagEnum::HideAttributes),
            "HIDE_UNBREAKABLE" => Some(ItemFlagEnum::HideUnbreakable),
            "HIDE_DESTROYS" => Some(ItemFlagEnum::HideDestroys),
            "HIDE_PLACED_ON" => Some(ItemFlagEnum::HidePlacedOn),
            "HIDE_POTION_EFFECTS" => Some(ItemFlagEnum::HidePotionEffects),
            "HIDE_DYE" => Some(ItemFlagEnum::HideDye),
            "HIDE_ARMOR_TRIM" => Some(ItemFlagEnum::HideArmorTrim),
            _ => None,
        }
    }
}
/// Interface to the various inventories. Behavior relating to <a href="../Material.html#AIR"><code>Material.AIR</code></a> is unspecified.
///
/// <b>Note that whilst <a href="#iterator()"><code>iterator()</code></a> deals with the entire inventory, add / contains / remove methods deal only with the storage contents.</b>
///
/// <b>Consider using <a href="#getContents()"><code>getContents()</code></a> and <a href="#getStorageContents()"><code>getStorageContents()</code></a> for specific iteration.</b>
///
/// This is a representation of an abstract class.
pub struct Inventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Inventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Inventory from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/Inventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Inventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Returns a HashMap with all slots and ItemStacks in the inventory with the given Material.
    /// <p>The HashMap contains entries where, the key is the slot index, and the value is the ItemStack in that slot. If no matching ItemStack with the given Material is found, an empty map is returned.</p>
    /// Finds all slots in the inventory containing any ItemStacks with the given ItemStack. This will only match slots if both the type and the amount of the stack match
    /// <p>The HashMap contains entries where, the key is the slot index, and the value is the ItemStack in that slot. If no matching ItemStack with the given Material is found, an empty map is returned.</p>
    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the ItemStack found in the slot at the given index
    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// This method allows you to change the maximum stack size for an inventory.
    /// <p><b>Caveats:</b></p>
    /// <ul>
    /// <li>Not all inventories respect this value.</li>
    /// <li>Stacks larger than 127 may be clipped when the world is saved.</li>
    /// <li>This value is not guaranteed to be preserved; be sure to set it before every time you want to set a slot over the max stack size.</li>
    /// <li>Stacks larger than the default max size for this type of inventory may not display correctly in the client.</li>
    /// </ul>
    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Stores the given ItemStacks in the inventory. This will try to fill existing stacks and empty slots as well as it can.
    /// <p>The returned HashMap contains what it couldn't store, where the key is the index of the parameter, and the value is the ItemStack at that index of the varargs parameter. If all items are stored, it will return an empty HashMap.</p>
    /// <p>If you pass in ItemStacks which exceed the maximum stack size for the Material, first they will be added to partial stacks where Material.getMaxStackSize() is not exceeded, up to Material.getMaxStackSize(). When there are no partial stacks left stacks will be split on Inventory.getMaxStackSize() allowing you to exceed the maximum stack size for that material.</p>
    /// <p>It is known that in some implementations this method will also set the inputted argument amount to the number of that item not placed in slots.</p>
    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Stores the ItemStack at the given index of the inventory.
    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the block or entity belonging to the open inventory
    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the maximum stack size for an ItemStack in this inventory.
    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Removes the given ItemStacks from the inventory.
    /// <p>It will try to remove 'as much as possible' from the types and amounts you give as arguments.</p>
    /// <p>The returned HashMap contains what it couldn't remove, where the key is the index of the parameter, and the value is the ItemStack at that index of the varargs parameter. If all the given ItemStacks are removed, it will return an empty HashMap.</p>
    /// <p>It is known that in some implementations this method will also set the inputted argument amount to the number of that item not removed from slots.</p>
    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Completely replaces the inventory's contents. Removes all existing contents and replaces it with the ItemStacks given in the array.
    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Put the given ItemStacks into the storage slots
    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks if the inventory contains ItemStacks matching the given ItemStack whose amounts sum to at least the minimum amount specified.
    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns the first empty Slot.
    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets a list of players viewing the inventory. Note that a player is considered to be viewing their own inventory and internal crafting screen even when said inventory is not open. They will normally be considered to be viewing their inventory even when they have a different inventory screen open, but it's possible for customized inventory screens to exclude the viewer's inventory, so this should never be assumed to be non-empty.
    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Removes the given ItemStacks from the inventory.
    /// <p>It will try to remove 'as much as possible' from the types and amounts you give as arguments.</p>
    /// <p>The returned HashMap contains what it couldn't remove, where the key is the index of the parameter, and the value is the ItemStack at that index of the varargs parameter. If all the given ItemStacks are removed, it will return an empty HashMap.</p>
    /// <p>It is known that in some implementations this method will also set the inputted argument amount to the number of that item not removed from slots.</p>
    /// Removes all stacks in the inventory matching the given material.
    /// Removes all stacks in the inventory matching the given stack.
    /// <p>This will only match a slot if both the type and the amount of the stack match</p>
    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Clears out a particular slot in the index.
    /// Clears out the whole Inventory.
    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Check whether or not this inventory is empty. An inventory is considered to be empty if there are no ItemStacks in any slot of this inventory.
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns an iterator starting at the given index. If the index is positive, then the first call to next() will return the item at that index; if it is negative, the first call to previous will return the item at index (getSize() + index).
    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the inventory contains any ItemStacks with the given material.
    /// Checks if the inventory contains any ItemStacks matching the given ItemStack.
    /// <p>This will only return true if both the type and the amount of the stack match.</p>
    /// Checks if the inventory contains any ItemStacks with the given material, adding to at least the minimum amount specified.
    /// Checks if the inventory contains at least the minimum amount specified of exactly matching ItemStacks.
    /// <p>An ItemStack only counts if both the type and the amount of the stack match.</p>
    /// Checks if the inventory contains ItemStacks matching the given ItemStack whose amounts sum to at least the minimum amount specified.
    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Finds the first slot in the inventory containing an ItemStack with the given material
    /// Returns the first slot in the inventory containing an ItemStack with the given stack. This will only match a slot if both the type and the amount of the stack match
    /// Returns the first empty Slot.
    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Get the location of the block or entity which corresponds to this inventory. May return null if this container was custom created or is a virtual / subcontainer.
    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns what type of inventory this is.
    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }
    /// Returns the size of the inventory
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for Inventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents a choice that will be valid only one of the stacks is exactly matched (aside from stack size).
///
/// <b>Only valid for shaped recipes</b>
pub struct RecipeChoiceExactChoice<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for RecipeChoiceExactChoice<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RecipeChoiceExactChoice<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate RecipeChoiceExactChoice from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/RecipeChoiceExactChoice")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RecipeChoiceExactChoice object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_item_stack(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<impl Into<crate::inventory::ItemStack<'mc>>>>,
    ) -> Result<crate::inventory::RecipeChoiceExactChoice<'mc>, Box<dyn std::error::Error>> {
        let raw_val_1 = jni.new_object("java/util/ArrayList", "()V", &[]).unwrap();
        for v in arg0.unwrap() {
            let map_val_0 =
                unsafe { jni::objects::JObject::from_raw(v.into().jni_object().clone()) };
            jni.call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let cls = &jni.find_class("org/bukkit/inventory/RecipeChoice$ExactChoice")?;
        let res = jni.new_object(
            cls,
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::inventory::RecipeChoiceExactChoice::from_raw(&jni, res)
    }
    pub fn new_with_item_stacks(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<impl Into<crate::inventory::ItemStack<'mc>>>>,
    ) -> Result<crate::inventory::RecipeChoiceExactChoice<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/inventory/RecipeChoice$ExactChoice")?;
        let res = jni.new_object(cls, "(Lorg/bukkit/inventory/ItemStack;)V", &[])?;
        crate::inventory::RecipeChoiceExactChoice::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="RecipeChoice.html#getItemStack()">RecipeChoice</a></code></span>
    /// Gets a single item stack representative of this stack choice.
    pub fn item_stack(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemStack",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn choices(
        &mut self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChoices", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
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

    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn test_with_object(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "test",
            "(Lorg/bukkit/inventory/ItemStack;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
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
/// Interface to the inventory of a Lectern.
///
/// This is a representation of an abstract class.
pub struct LecternInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> LecternInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LecternInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/LecternInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LecternInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Inventory.html#getHolder()">Inventory</a></code></span>
    /// Gets the block or entity belonging to the open inventory
    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for LecternInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for LecternInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Interface to the inventory of a Loom.
///
/// This is a representation of an abstract class.
pub struct LoomInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> LoomInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LoomInventory from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/LoomInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LoomInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for LoomInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for LoomInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Interface to the inventory of a Brewing Stand.
///
/// This is a representation of an abstract class.
pub struct BrewerInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BrewerInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BrewerInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/BrewerInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BrewerInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Set the current fuel for brewing. Generally only <a href="../Material.html#BLAZE_POWDER"><code>Material.BLAZE_POWDER</code></a> will be of use.
    pub fn set_fuel(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuel",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the current fuel for brewing.
    pub fn fuel(&mut self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFuel",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Inventory.html#getHolder()">Inventory</a></code></span>
    /// Gets the block or entity belonging to the open inventory
    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the current ingredient for brewing.
    pub fn ingredient(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getIngredient",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the current ingredient for brewing.
    pub fn set_ingredient(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setIngredient",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for BrewerInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for BrewerInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a shaped (ie normal) crafting recipe.
pub struct ShapedRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ShapedRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ShapedRecipe<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ShapedRecipe from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ShapedRecipe")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ShapedRecipe object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_item_stack(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::NamespacedKey<'mc>>>,
        arg1: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::inventory::ShapedRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/inventory/ShapedRecipe")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        crate::inventory::ShapedRecipe::from_raw(&jni, res)
    }
    /// Set the shape of this recipe to the specified rows. Each character represents a different ingredient; exactly what each character represents is set separately. The first row supplied corresponds with the upper most part of the recipe on the workbench e.g. if all three rows are supplies the first string represents the top row on the workbench.
    pub fn shape(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<crate::inventory::ShapedRecipe<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getShape",
            "(Ljava/lang/String;)Lorg/bukkit/inventory/ShapedRecipe;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapedRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// Sets the material that a character in the recipe shape refers to.
    /// <p>Note that before an ingredient can be set, the recipe's shape must be defined with <a href="#shape(java.lang.String...)"><code>shape(String...)</code></a>.</p>
    /// Sets the material that a character in the recipe shape refers to.
    /// <p>Note that before an ingredient can be set, the recipe's shape must be defined with <a href="#shape(java.lang.String...)"><code>shape(String...)</code></a>.</p>
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Sets the material that a character in the recipe shape refers to.
    /// <p>Note that before an ingredient can be set, the recipe's shape must be defined with <a href="#shape(java.lang.String...)"><code>shape(String...)</code></a>.</p>
    /// Sets the <a href="RecipeChoice.html" title="interface in org.bukkit.inventory"><code>RecipeChoice</code></a> that a character in the recipe shape refers to.
    /// <p>Note that before an ingredient can be set, the recipe's shape must be defined with <a href="#shape(java.lang.String...)"><code>shape(String...)</code></a>.</p>
    pub fn set_ingredient_with_char(
        &mut self,
        arg0: u16,
        arg1: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::inventory::ShapedRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Char(arg0.into());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setIngredient",
            "(CLorg/bukkit/Material;I)Lorg/bukkit/inventory/ShapedRecipe;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapedRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get a copy of the ingredients map.
    pub fn ingredient_map(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getIngredientMap",
            "()Ljava/util/Map;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get a copy of the choice map.
    pub fn choice_map(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getChoiceMap",
            "()Ljava/util/Map;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the result.
    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the category which this recipe will appear in the recipe book under. Defaults to <a href="recipe/CraftingBookCategory.html#MISC"><code>CraftingBookCategory.MISC</code></a> if not set.
    pub fn category(
        &mut self,
    ) -> Result<crate::inventory::recipe::CraftingBookCategory<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCategory",
            "()Lorg/bukkit/inventory/recipe/CraftingBookCategory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::inventory::recipe::CraftingBookCategory::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::inventory::recipe::CraftingBookCategory::from_string(variant_str).unwrap(),
        )
    }
    /// Get the group of this recipe. Recipes with the same group may be grouped together when displayed in the client.
    pub fn group(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getGroup", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Set the group of this recipe. Recipes with the same group may be grouped together when displayed in the client.
    pub fn set_group(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGroup",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the category which this recipe will appear in the recipe book under. Defaults to <a href="recipe/CraftingBookCategory.html#MISC"><code>CraftingBookCategory.MISC</code></a> if not set.
    pub fn set_category(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::recipe::CraftingBookCategory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCategory",
            "(Lorg/bukkit/inventory/recipe/CraftingBookCategory;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
impl<'mc> Into<crate::inventory::CraftingRecipe<'mc>> for ShapedRecipe<'mc> {
    fn into(self) -> crate::inventory::CraftingRecipe<'mc> {
        crate::inventory::CraftingRecipe::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a smithing transform recipe.
pub struct SmithingTransformRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SmithingTransformRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SmithingTransformRecipe<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate SmithingTransformRecipe from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/SmithingTransformRecipe")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SmithingTransformRecipe object, got {}",
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
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
        arg3: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
        arg4: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::SmithingTransformRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/inventory/SmithingTransformRecipe")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/RecipeChoice;Lorg/bukkit/inventory/RecipeChoice;Lorg/bukkit/inventory/RecipeChoice;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
        crate::inventory::SmithingTransformRecipe::from_raw(&jni, res)
    }
    /// Get the template recipe item.
    pub fn template(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTemplate",
            "()Lorg/bukkit/inventory/RecipeChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn base(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBase",
            "()Lorg/bukkit/inventory/RecipeChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn addition(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAddition",
            "()Lorg/bukkit/inventory/RecipeChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
impl<'mc> Into<crate::inventory::SmithingRecipe<'mc>> for SmithingTransformRecipe<'mc> {
    fn into(self) -> crate::inventory::SmithingRecipe<'mc> {
        crate::inventory::SmithingRecipe::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum EquipmentSlotEnum {
    Hand,
    OffHand,
    Feet,
    Legs,
    Chest,
    Head,
}
impl std::fmt::Display for EquipmentSlotEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            EquipmentSlotEnum::Hand => f.write_str("HAND"),
            EquipmentSlotEnum::OffHand => f.write_str("OFF_HAND"),
            EquipmentSlotEnum::Feet => f.write_str("FEET"),
            EquipmentSlotEnum::Legs => f.write_str("LEGS"),
            EquipmentSlotEnum::Chest => f.write_str("CHEST"),
            EquipmentSlotEnum::Head => f.write_str("HEAD"),
        }
    }
}
pub struct EquipmentSlot<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub EquipmentSlotEnum,
);
impl<'mc> std::ops::Deref for EquipmentSlot<'mc> {
    type Target = EquipmentSlotEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for EquipmentSlot<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EquipmentSlot<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: EquipmentSlotEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EquipmentSlot from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/EquipmentSlot")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EquipmentSlot object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const HAND: EquipmentSlotEnum = EquipmentSlotEnum::Hand;
    pub const OFF_HAND: EquipmentSlotEnum = EquipmentSlotEnum::OffHand;
    pub const FEET: EquipmentSlotEnum = EquipmentSlotEnum::Feet;
    pub const LEGS: EquipmentSlotEnum = EquipmentSlotEnum::Legs;
    pub const CHEST: EquipmentSlotEnum = EquipmentSlotEnum::Chest;
    pub const HEAD: EquipmentSlotEnum = EquipmentSlotEnum::Head;
    pub fn from_string(str: String) -> std::option::Option<EquipmentSlotEnum> {
        match str.as_str() {
            "HAND" => Some(EquipmentSlotEnum::Hand),
            "OFF_HAND" => Some(EquipmentSlotEnum::OffHand),
            "FEET" => Some(EquipmentSlotEnum::Feet),
            "LEGS" => Some(EquipmentSlotEnum::Legs),
            "CHEST" => Some(EquipmentSlotEnum::Chest),
            "HEAD" => Some(EquipmentSlotEnum::Head),
            _ => None,
        }
    }
}
/// Represents a merchant. A merchant is a special type of inventory which can facilitate custom trades between items.
///
/// This is a representation of an abstract class.
pub struct Merchant<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Merchant<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Merchant from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/Merchant")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Merchant object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Get a list of trades currently available from this merchant.
    pub fn recipes(
        &mut self,
    ) -> Result<Vec<crate::inventory::MerchantRecipe<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRecipes", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::inventory::MerchantRecipe::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    /// Set the list of trades currently available from this merchant.
    ///
    /// This will not change the selected trades of players currently trading with this merchant.
    pub fn set_recipes(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::MerchantRecipe<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                unsafe { jni::objects::JObject::from_raw(v.into().jni_object().clone()) };
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRecipes",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get a list of trades currently available from this merchant.
    /// Get the recipe at a certain index of this merchant's trade list.
    /// Get the number of trades this merchant currently has available.
    pub fn get_recipe(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::MerchantRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRecipe",
            "(I)Lorg/bukkit/inventory/MerchantRecipe;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::MerchantRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the list of trades currently available from this merchant.
    ///
    /// This will not change the selected trades of players currently trading with this merchant.
    /// Set the recipe at a certain index of this merchant's trade list.
    pub fn set_recipe(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::MerchantRecipe<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRecipe",
            "(ILorg/bukkit/inventory/MerchantRecipe;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the number of trades this merchant currently has available.
    pub fn recipe_count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRecipeCount", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets whether this merchant is currently trading.
    pub fn is_trading(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isTrading", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the player this merchant is trading with, or null if it is not currently trading.
    pub fn trader(
        &mut self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTrader",
            "()Lorg/bukkit/entity/HumanEntity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for Merchant<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instance of the ItemFactory can be obtained with <a href="../Server.html#getItemFactory()"><code>Server.getItemFactory()</code></a>.
/// <p>The ItemFactory is solely responsible for creating item meta containers to apply on item stacks.</p>
///
/// This is a representation of an abstract class.
pub struct ItemFactory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ItemFactory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ItemFactory from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ItemFactory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemFactory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// This creates a new item meta for the material.
    pub fn get_item_meta(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<crate::inventory::meta::ItemMeta<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemMeta",
            "(Lorg/bukkit/Material;)Lorg/bukkit/inventory/meta/ItemMeta;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns an appropriate item meta for the specified stack.
    /// <p>The item meta returned will always be a valid meta for a given ItemStack of the specified material. It may be a more or less specific meta, and could also be the same meta or meta type as the parameter. The item meta returned will also always be the most appropriate meta.</p>
    /// <p>Example, if a <a title="interface in org.bukkit.inventory.meta" href="meta/SkullMeta.html"><code>SkullMeta</code></a> is being applied to a book, this method would return a <a title="interface in org.bukkit.inventory.meta" href="meta/BookMeta.html"><code>BookMeta</code></a> containing all information in the specified meta that is applicable to an <a href="meta/ItemMeta.html" title="interface in org.bukkit.inventory.meta"><code>ItemMeta</code></a>, the highest common interface.</p>
    /// Returns an appropriate item meta for the specified material.
    /// <p>The item meta returned will always be a valid meta for a given ItemStack of the specified material. It may be a more or less specific meta, and could also be the same meta or meta type as the parameter. The item meta returned will also always be the most appropriate meta.</p>
    /// <p>Example, if a <a href="meta/SkullMeta.html" title="interface in org.bukkit.inventory.meta"><code>SkullMeta</code></a> is being applied to a book, this method would return a <a href="meta/BookMeta.html" title="interface in org.bukkit.inventory.meta"><code>BookMeta</code></a> containing all information in the specified meta that is applicable to an <a href="meta/ItemMeta.html" title="interface in org.bukkit.inventory.meta"><code>ItemMeta</code></a>, the highest common interface.</p>
    pub fn as_meta_for_with_item_meta(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::meta::ItemMeta<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::inventory::meta::ItemMeta<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"asMetaFor","(Lorg/bukkit/inventory/meta/ItemMeta;Lorg/bukkit/inventory/ItemStack;)Lorg/bukkit/inventory/meta/ItemMeta;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// This method checks the item meta to confirm that it is applicable (no data lost if applied) to the specified ItemStack.
    /// <p>A <a title="interface in org.bukkit.inventory.meta" href="meta/SkullMeta.html"><code>SkullMeta</code></a> would not be valid for a sword, but a normal <a href="meta/ItemMeta.html" title="interface in org.bukkit.inventory.meta"><code>ItemMeta</code></a> from an enchanted dirt block would.</p>
    /// This method checks the item meta to confirm that it is applicable (no data lost if applied) to the specified Material.
    /// <p>A <a href="meta/SkullMeta.html" title="interface in org.bukkit.inventory.meta"><code>SkullMeta</code></a> would not be valid for a sword, but a normal <a title="interface in org.bukkit.inventory.meta" href="meta/ItemMeta.html"><code>ItemMeta</code></a> from an enchanted dirt block would.</p>
    pub fn is_applicable_with_item_meta(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::meta::ItemMeta<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isApplicable",
            "(Lorg/bukkit/inventory/meta/ItemMeta;Lorg/bukkit/Material;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// for internal use only
    /// </div>
    /// for internal use only
    ///
    /// Apply a material change for an item meta. Do not use under any circumstances.
    pub fn update_material(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::meta::ItemMeta<'mc>>,
        arg1: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "updateMaterial",
            "(Lorg/bukkit/inventory/meta/ItemMeta;Lorg/bukkit/Material;)Lorg/bukkit/Material;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }
    /// Returns the default color for all leather armor.
    pub fn default_leather_color(
        &mut self,
    ) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultLeatherColor",
            "()Lorg/bukkit/Color;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Create a new <a href="ItemStack.html" title="class in org.bukkit.inventory"><code>ItemStack</code></a> given the supplied input.
    /// <p>The input should match the same input as expected by Minecraft's <code>/give</code> command. For example, "minecraft:diamond_sword{Enchantments:[{id:"minecraft:sharpness", lvl:3}]}" would yield an ItemStack of <a href="../Material.html#DIAMOND_SWORD"><code>Material.DIAMOND_SWORD</code></a> with an <a title="interface in org.bukkit.inventory.meta" href="meta/ItemMeta.html"><code>ItemMeta</code></a> containing a level 3 <a href="../enchantments/Enchantment.html#DAMAGE_ALL"><code>Enchantment.DAMAGE_ALL</code></a> enchantment.</p>
    pub fn create_item_stack(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createItemStack",
            "(Ljava/lang/String;)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a <a href="../Material.html" title="enum in org.bukkit"><code>Material</code></a> representing the spawn egg for the provided <a href="../entity/EntityType.html" title="enum in org.bukkit.entity"><code>EntityType</code></a>.
    ///
    /// Will return null for EntityTypes that do not have a corresponding spawn egg.
    pub fn get_spawn_egg(
        &mut self,
        arg0: impl Into<&'mc crate::entity::EntityType<'mc>>,
    ) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnEgg",
            "(Lorg/bukkit/entity/EntityType;)Lorg/bukkit/Material;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }
    /// This method is used to compare two item meta data objects.
    pub fn equals(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::meta::ItemMeta<'mc>>,
        arg1: impl Into<&'mc crate::inventory::meta::ItemMeta<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Lorg/bukkit/inventory/meta/ItemMeta;Lorg/bukkit/inventory/meta/ItemMeta;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for ItemFactory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An interface to a creatures inventory
///
/// This is a representation of an abstract class.
pub struct EntityEquipment<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EntityEquipment<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityEquipment from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/EntityEquipment")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityEquipment object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Gets the ItemStack at the given equipment slot in the inventory.
    /// Gets a copy of the item the entity is currently holding in their main hand.
    /// Gets a copy of the item the entity is currently holding in their off hand.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// entities can duel wield now use the methods for the specific hand instead
    /// </div>
    /// entities can duel wield now use the methods for the specific hand instead
    ///
    /// Gets a copy of the item the entity is currently holding
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// entities can duel wield now use the methods for the specific hand instead
    /// </div>
    /// entities can duel wield now use the methods for the specific hand instead
    ///
    /// Gets the chance of the main hand item being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    /// Gets the chance of the off hand item being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn get_item(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(Lorg/bukkit/inventory/EquipmentSlot;)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Stores the ItemStack at the given equipment slot in the inventory.
    /// Stores the ItemStack at the given equipment slot in the inventory.
    /// Sets the item the entity is holding in their main hand.
    /// Sets the item the entity is holding in their main hand.
    /// Sets the item the entity is holding in their off hand.
    /// Sets the item the entity is holding in their off hand.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// entities can duel wield now use the methods for the specific hand instead
    /// </div>
    /// entities can duel wield now use the methods for the specific hand instead
    ///
    /// Sets the item the entity is holding
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// entities can duel wield now use the methods for the specific hand instead
    /// </div>
    /// entities can duel wield now use the methods for the specific hand instead
    ///
    /// Sets the chance of the item this creature is currently holding in their main hand being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    /// Sets the chance of the off hand item being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_item_with_equipment_slot(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        // 1
        let val_3 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(Lorg/bukkit/inventory/EquipmentSlot;Lorg/bukkit/inventory/ItemStack;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the entity this EntityEquipment belongs to
    pub fn holder(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// entities can duel wield now use the methods for the specific hand instead
    /// </div>
    /// entities can duel wield now use the methods for the specific hand instead
    ///
    /// Gets a copy of the item the entity is currently holding
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// entities can duel wield now use the methods for the specific hand instead
    /// </div>
    /// entities can duel wield now use the methods for the specific hand instead
    ///
    pub fn item_in_hand(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemInHand",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// entities can duel wield now use the methods for the specific hand instead
    /// </div>
    /// entities can duel wield now use the methods for the specific hand instead
    ///
    /// Sets the item the entity is holding
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// entities can duel wield now use the methods for the specific hand instead
    /// </div>
    /// entities can duel wield now use the methods for the specific hand instead
    ///
    pub fn set_item_in_hand(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInHand",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a copy of the boots currently being worn by the entity
    /// Gets the chance of the boots being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn boots(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoots",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the boots worn by the entity
    /// Sets the boots worn by the entity
    /// Sets the chance of the boots being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_boots_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBoots",
            "(Lorg/bukkit/inventory/ItemStack;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a copy of the leggings currently being worn by the entity
    /// Gets the chance of the leggings being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn leggings(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLeggings",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the leggings worn by the entity
    /// Sets the leggings worn by the entity
    /// Sets the chance of the leggings being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_leggings_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLeggings",
            "(Lorg/bukkit/inventory/ItemStack;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a copy of the chest plate currently being worn by the entity
    /// Gets the chance of the chest plate being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn chestplate(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getChestplate",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the chest plate worn by the entity
    /// Sets the chest plate worn by the entity
    /// Sets the chance of the chest plate being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_chestplate_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setChestplate",
            "(Lorg/bukkit/inventory/ItemStack;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a copy of the helmet currently being worn by the entity
    /// Gets the chance of the helmet being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn helmet(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHelmet",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the helmet worn by the entity
    /// Sets the helmet worn by the entity
    /// Sets the chance of the helmet being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_helmet_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHelmet",
            "(Lorg/bukkit/inventory/ItemStack;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a copy of the item the entity is currently holding in their main hand.
    /// Gets the chance of the main hand item being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn item_in_main_hand(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemInMainHand",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the item the entity is holding in their main hand.
    /// Sets the item the entity is holding in their main hand.
    /// Sets the chance of the item this creature is currently holding in their main hand being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_item_in_main_hand_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInMainHand",
            "(Lorg/bukkit/inventory/ItemStack;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a copy of the item the entity is currently holding in their off hand.
    /// Gets the chance of the off hand item being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn item_in_off_hand(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemInOffHand",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the item the entity is holding in their off hand.
    /// Sets the item the entity is holding in their off hand.
    /// Sets the chance of the off hand item being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_item_in_off_hand_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInOffHand",
            "(Lorg/bukkit/inventory/ItemStack;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the entities armor to the provided array of ItemStacks
    pub fn set_armor_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setArmorContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// entities can duel wield now use the methods for the specific hand instead
    /// </div>
    /// entities can duel wield now use the methods for the specific hand instead
    ///
    pub fn item_in_hand_drop_chance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemInHandDropChance", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// entities can duel wield now use the methods for the specific hand instead
    /// </div>
    /// entities can duel wield now use the methods for the specific hand instead
    ///
    pub fn set_item_in_hand_drop_chance(
        &mut self,
        arg0: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInHandDropChance",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the chance of the main hand item being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn item_in_main_hand_drop_chance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemInMainHandDropChance",
            "()F",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    /// Sets the chance of the item this creature is currently holding in their main hand being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_item_in_main_hand_drop_chance(
        &mut self,
        arg0: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInMainHandDropChance",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the chance of the off hand item being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn item_in_off_hand_drop_chance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemInOffHandDropChance",
            "()F",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    /// Sets the chance of the off hand item being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_item_in_off_hand_drop_chance(
        &mut self,
        arg0: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInOffHandDropChance",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the chance of the helmet being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn helmet_drop_chance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHelmetDropChance", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    /// Sets the chance of the helmet being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_helmet_drop_chance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHelmetDropChance",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the chance of the chest plate being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn chestplate_drop_chance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChestplateDropChance", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    /// Sets the chance of the chest plate being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_chestplate_drop_chance(
        &mut self,
        arg0: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setChestplateDropChance",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the chance of the leggings being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn leggings_drop_chance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLeggingsDropChance", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    /// Sets the chance of the leggings being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_leggings_drop_chance(
        &mut self,
        arg0: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLeggingsDropChance",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the chance of the boots being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn boots_drop_chance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBootsDropChance", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    /// Sets the chance of the boots being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_boots_drop_chance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBootsDropChance",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Clears the entity of all armor and held items
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for EntityEquipment<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Interface to the inventory of a chiseled bookshelf.
///
/// This is a representation of an abstract class.
pub struct ChiseledBookshelfInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ChiseledBookshelfInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ChiseledBookshelfInventory from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/ChiseledBookshelfInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChiseledBookshelfInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Inventory.html#getHolder()">Inventory</a></code></span>
    /// Gets the block or entity belonging to the open inventory
    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for ChiseledBookshelfInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for ChiseledBookshelfInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a shapeless recipe, where the arrangement of the ingredients on the crafting grid does not matter.
pub struct ShapelessRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ShapelessRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ShapelessRecipe<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ShapelessRecipe from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ShapelessRecipe")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ShapelessRecipe object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_item_stack(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::NamespacedKey<'mc>>>,
        arg1: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/inventory/ShapelessRecipe")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        crate::inventory::ShapelessRecipe::from_raw(&jni, res)
    }
    /// Adds the specified ingredient.
    /// Adds the specified ingredient.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Adds the specified ingredient.
    /// Adds multiples of the specified ingredient.
    /// Adds multiples of the specified ingredient.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Adds multiples of the specified ingredient.
    pub fn add_ingredient_with_recipe_choice(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::material::MaterialData<'mc>>>,
    ) -> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addIngredient",
            "(Lorg/bukkit/material/MaterialData;)Lorg/bukkit/inventory/ShapelessRecipe;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapelessRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// Adds the specified ingredient.
    /// Adds the specified ingredient.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Adds the specified ingredient.
    /// Adds multiples of the specified ingredient.
    /// Adds multiples of the specified ingredient.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Adds multiples of the specified ingredient.
    pub fn add_ingredient_with_material(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addIngredient",
            "(Lorg/bukkit/Material;I)Lorg/bukkit/inventory/ShapelessRecipe;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapelessRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// Adds the specified ingredient.
    /// Adds the specified ingredient.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Adds the specified ingredient.
    /// Adds multiples of the specified ingredient.
    /// Adds multiples of the specified ingredient.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Adds multiples of the specified ingredient.
    pub fn add_ingredient_with_int(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::Material<'mc>>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addIngredient",
            "(ILorg/bukkit/Material;I)Lorg/bukkit/inventory/ShapelessRecipe;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapelessRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Removes an ingredient from the list.
    /// Removes an ingredient from the list. If the ingredient occurs multiple times, only one instance of it is removed. Only removes exact matches, with a data value of 0.
    /// Removes an ingredient from the list. If the ingredient occurs multiple times, only one instance of it is removed. If the data value is -1, only ingredients with a -1 data value will be removed.
    /// Removes multiple instances of an ingredient from the list. If there are less instances then specified, all will be removed. Only removes exact matches, with a data value of 0.
    /// Removes multiple instances of an ingredient from the list. If there are less instances then specified, all will be removed. If the data value is -1, only ingredients with a -1 data value will be removed.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Removes an ingredient from the list. If the ingredient occurs multiple times, only one instance of it is removed. If the data value is -1, only ingredients with a -1 data value will be removed.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Removes multiple instances of an ingredient from the list. If there are less instances then specified, all will be removed. If the data value is -1, only ingredients with a -1 data value will be removed.
    pub fn remove_ingredient_with_material_data(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::inventory::RecipeChoice<'mc>>>,
    ) -> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIngredient",
            "(Lorg/bukkit/inventory/RecipeChoice;)Lorg/bukkit/inventory/ShapelessRecipe;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapelessRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Removes an ingredient from the list.
    /// Removes an ingredient from the list. If the ingredient occurs multiple times, only one instance of it is removed. Only removes exact matches, with a data value of 0.
    /// Removes an ingredient from the list. If the ingredient occurs multiple times, only one instance of it is removed. If the data value is -1, only ingredients with a -1 data value will be removed.
    /// Removes multiple instances of an ingredient from the list. If there are less instances then specified, all will be removed. Only removes exact matches, with a data value of 0.
    /// Removes multiple instances of an ingredient from the list. If there are less instances then specified, all will be removed. If the data value is -1, only ingredients with a -1 data value will be removed.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Removes an ingredient from the list. If the ingredient occurs multiple times, only one instance of it is removed. If the data value is -1, only ingredients with a -1 data value will be removed.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Removes multiple instances of an ingredient from the list. If there are less instances then specified, all will be removed. If the data value is -1, only ingredients with a -1 data value will be removed.
    pub fn remove_ingredient_with_material(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIngredient",
            "(ILorg/bukkit/Material;)Lorg/bukkit/inventory/ShapelessRecipe;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapelessRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// Removes an ingredient from the list.
    /// Removes an ingredient from the list. If the ingredient occurs multiple times, only one instance of it is removed. Only removes exact matches, with a data value of 0.
    /// Removes an ingredient from the list. If the ingredient occurs multiple times, only one instance of it is removed. If the data value is -1, only ingredients with a -1 data value will be removed.
    /// Removes multiple instances of an ingredient from the list. If there are less instances then specified, all will be removed. Only removes exact matches, with a data value of 0.
    /// Removes multiple instances of an ingredient from the list. If there are less instances then specified, all will be removed. If the data value is -1, only ingredients with a -1 data value will be removed.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Removes an ingredient from the list. If the ingredient occurs multiple times, only one instance of it is removed. If the data value is -1, only ingredients with a -1 data value will be removed.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Removes multiple instances of an ingredient from the list. If there are less instances then specified, all will be removed. If the data value is -1, only ingredients with a -1 data value will be removed.
    pub fn remove_ingredient_with_int(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::Material<'mc>>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIngredient",
            "(ILorg/bukkit/Material;I)Lorg/bukkit/inventory/ShapelessRecipe;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapelessRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the list of ingredients used for this recipe.
    pub fn ingredient_list(
        &mut self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getIngredientList",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn choice_list(
        &mut self,
    ) -> Result<Vec<crate::inventory::RecipeChoice<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getChoiceList",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::inventory::RecipeChoice::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    /// Get the result of this recipe.
    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the category which this recipe will appear in the recipe book under. Defaults to <a href="recipe/CraftingBookCategory.html#MISC"><code>CraftingBookCategory.MISC</code></a> if not set.
    pub fn category(
        &mut self,
    ) -> Result<crate::inventory::recipe::CraftingBookCategory<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCategory",
            "()Lorg/bukkit/inventory/recipe/CraftingBookCategory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::inventory::recipe::CraftingBookCategory::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::inventory::recipe::CraftingBookCategory::from_string(variant_str).unwrap(),
        )
    }
    /// Get the group of this recipe. Recipes with the same group may be grouped together when displayed in the client.
    pub fn group(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getGroup", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Set the group of this recipe. Recipes with the same group may be grouped together when displayed in the client.
    pub fn set_group(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGroup",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the category which this recipe will appear in the recipe book under. Defaults to <a href="recipe/CraftingBookCategory.html#MISC"><code>CraftingBookCategory.MISC</code></a> if not set.
    pub fn set_category(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::recipe::CraftingBookCategory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCategory",
            "(Lorg/bukkit/inventory/recipe/CraftingBookCategory;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
impl<'mc> Into<crate::inventory::CraftingRecipe<'mc>> for ShapelessRecipe<'mc> {
    fn into(self) -> crate::inventory::CraftingRecipe<'mc> {
        crate::inventory::CraftingRecipe::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Interface to the inventory of an Anvil.
///
/// This is a representation of an abstract class.
pub struct AnvilInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> AnvilInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AnvilInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/AnvilInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AnvilInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Get the name to be applied to the repaired item. An empty string denotes the default item name.
    pub fn rename_text(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRenameText",
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
    /// Get the item cost (in amount) to complete the current repair.
    pub fn repair_cost_amount(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRepairCostAmount", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the item cost (in amount) to complete the current repair.
    pub fn set_repair_cost_amount(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRepairCostAmount",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the item cost (in amount) to complete the current repair.
    /// Get the experience cost (in levels) to complete the current repair.
    pub fn repair_cost(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRepairCost", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the item cost (in amount) to complete the current repair.
    /// Set the experience cost (in levels) to complete the current repair.
    pub fn set_repair_cost(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRepairCost",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the maximum experience cost (in levels) to be allowed by the current repair. If the result of <a href="#getRepairCost()"><code>getRepairCost()</code></a> exceeds the returned value, the repair result will be air to due being "too expensive".
    /// <p>By default, this level is set to 40. Players in creative mode ignore the maximum repair cost.</p>
    pub fn maximum_repair_cost(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumRepairCost", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the maximum experience cost (in levels) to be allowed by the current repair. The default value set by vanilla Minecraft is 40.
    pub fn set_maximum_repair_cost(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaximumRepairCost",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for AnvilInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for AnvilInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a potential item match within a recipe. All choices within a recipe must be satisfied for it to be craftable. <b>This class is not legal for implementation by plugins!</b>
///
/// This is a representation of an abstract class.
pub struct RecipeChoice<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> RecipeChoice<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate RecipeChoice from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/RecipeChoice")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RecipeChoice object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// for compatibility only
    /// </div>
    /// for compatibility only
    ///
    /// Gets a single item stack representative of this stack choice.
    pub fn item_stack(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemStack",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/RecipeChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn test_with_object(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "test",
            "(Lorg/bukkit/inventory/ItemStack;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for RecipeChoice<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

///
/// This is a representation of an abstract class.
pub struct InventoryHolder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> InventoryHolder<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate InventoryHolder from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/InventoryHolder")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryHolder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Get the object's inventory.
    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for InventoryHolder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Interface to the inventory of an Enchantment Table.
///
/// This is a representation of an abstract class.
pub struct EnchantingInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EnchantingInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnchantingInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/EnchantingInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantingInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Get the item being enchanted.
    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the item being enchanted.
    pub fn set_item_with_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the secondary item being used for the enchant.
    pub fn secondary(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSecondary",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the secondary item being used for the enchant.
    pub fn set_secondary(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSecondary",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for EnchantingInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for EnchantingInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a campfire recipe.
pub struct BlastingRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BlastingRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BlastingRecipe<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlastingRecipe from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/BlastingRecipe")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlastingRecipe object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_namespaced_key(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
        arg3: f32,
        arg4: std::option::Option<i32>,
    ) -> Result<crate::inventory::BlastingRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = jni::objects::JValueGen::Float(arg3.into());
        let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
        let cls = &jni.find_class("org/bukkit/inventory/BlastingRecipe")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/RecipeChoice;FI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
        crate::inventory::BlastingRecipe::from_raw(&jni, res)
    }

    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn input(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInput",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_input(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInput",
            "(Lorg/bukkit/Material;)Lorg/bukkit/inventory/CookingRecipe;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn category(
        &mut self,
    ) -> Result<crate::inventory::recipe::CookingBookCategory<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCategory",
            "()Lorg/bukkit/inventory/recipe/CookingBookCategory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::inventory::recipe::CookingBookCategory::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::inventory::recipe::CookingBookCategory::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_input_choice(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInputChoice",
            "(Lorg/bukkit/inventory/RecipeChoice;)Lorg/bukkit/inventory/CookingRecipe;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn input_choice(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInputChoice",
            "()Lorg/bukkit/inventory/RecipeChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn group(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getGroup", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_group(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGroup",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_category(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::recipe::CookingBookCategory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCategory",
            "(Lorg/bukkit/inventory/recipe/CookingBookCategory;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn experience(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getExperience", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }

    pub fn set_experience(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExperience",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_cooking_time(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookingTime",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cooking_time(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCookingTime", "()I", &[]);
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
impl<'mc> Into<crate::inventory::CookingRecipe<'mc>> for BlastingRecipe<'mc> {
    fn into(self) -> crate::inventory::CookingRecipe<'mc> {
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a campfire recipe.
pub struct SmokingRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SmokingRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SmokingRecipe<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SmokingRecipe from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/SmokingRecipe")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SmokingRecipe object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_namespaced_key(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
        arg3: f32,
        arg4: std::option::Option<i32>,
    ) -> Result<crate::inventory::SmokingRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = jni::objects::JValueGen::Float(arg3.into());
        let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
        let cls = &jni.find_class("org/bukkit/inventory/SmokingRecipe")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/RecipeChoice;FI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
        crate::inventory::SmokingRecipe::from_raw(&jni, res)
    }

    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn input(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInput",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_input(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInput",
            "(Lorg/bukkit/Material;)Lorg/bukkit/inventory/CookingRecipe;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn category(
        &mut self,
    ) -> Result<crate::inventory::recipe::CookingBookCategory<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCategory",
            "()Lorg/bukkit/inventory/recipe/CookingBookCategory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::inventory::recipe::CookingBookCategory::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::inventory::recipe::CookingBookCategory::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_input_choice(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInputChoice",
            "(Lorg/bukkit/inventory/RecipeChoice;)Lorg/bukkit/inventory/CookingRecipe;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn input_choice(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInputChoice",
            "()Lorg/bukkit/inventory/RecipeChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn group(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getGroup", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_group(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGroup",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_category(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::recipe::CookingBookCategory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCategory",
            "(Lorg/bukkit/inventory/recipe/CookingBookCategory;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn experience(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getExperience", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }

    pub fn set_experience(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExperience",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_cooking_time(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookingTime",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cooking_time(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCookingTime", "()I", &[]);
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
impl<'mc> Into<crate::inventory::CookingRecipe<'mc>> for SmokingRecipe<'mc> {
    fn into(self) -> crate::inventory::CookingRecipe<'mc> {
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Interface to the crafting inventories
///
/// This is a representation of an abstract class.
pub struct CraftingInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CraftingInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CraftingInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/CraftingInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CraftingInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Check what item is in the result slot of this crafting inventory.
    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the item in the result slot of the crafting inventory.
    pub fn set_result(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the current recipe formed on the crafting inventory, if any.
    pub fn recipe(&mut self) -> Result<crate::inventory::Recipe<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRecipe",
            "()Lorg/bukkit/inventory/Recipe;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Recipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Replace the contents of the crafting matrix
    pub fn set_matrix(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMatrix",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for CraftingInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for CraftingInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a merchant's trade.
/// <p>Trades can take one or two ingredients, and provide one result. The ingredients' ItemStack amounts are respected in the trade.</p>
/// <p>A trade has a maximum number of uses. A <a href="../entity/Villager.html" title="interface in org.bukkit.entity"><code>Villager</code></a> may periodically replenish its trades by resetting the <a href="#getUses()"><code>uses</code></a> of its merchant recipes to <code>0</code>, allowing them to be used again.</p>
/// <p>A trade may or may not reward experience for being completed.</p>
/// <p>During trades, the <a title="class in org.bukkit.inventory" href="MerchantRecipe.html"><code>MerchantRecipe</code></a> dynamically adjusts the amount of its first ingredient based on the following criteria:</p>
/// <ul>
/// <li><a href="#getDemand()"><code>Demand</code></a>: This value is periodically updated by the villager that owns this merchant recipe based on how often the recipe has been used since it has been last restocked in relation to its <a href="#getMaxUses()"><code>maximum uses</code></a>. The amount by which the demand influences the amount of the first ingredient is scaled by the recipe's <a href="#getPriceMultiplier()"><code>price multiplier</code></a>, and can never be below zero.</li>
/// <li><a href="#getSpecialPrice()"><code>Special price</code></a>: This value is dynamically updated whenever a player starts and stops trading with a villager that owns this merchant recipe. It is based on the player's individual reputation with the villager, and the player's currently active status effects (see <a href="../potion/PotionEffectType.html#HERO_OF_THE_VILLAGE"><code>PotionEffectType.HERO_OF_THE_VILLAGE</code></a>). The influence of the player's reputation on the special price is scaled by the recipe's <a href="#getPriceMultiplier()"><code>price multiplier</code></a>.</li>
/// </ul> The adjusted amount of the first ingredient is calculated by adding up the original amount of the first ingredient, the demand scaled by the recipe's <a href="#getPriceMultiplier()"><code>price multiplier</code></a> and truncated to the next lowest integer value greater than or equal to 0, and the special price, and then constraining the resulting value between <code>1</code> and the item stack's <a href="ItemStack.html#getMaxStackSize()"><code>maximum stack size</code></a>.
pub struct MerchantRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MerchantRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MerchantRecipe<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MerchantRecipe from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/MerchantRecipe")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MerchantRecipe object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_item_stack(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<bool>,
        arg4: std::option::Option<i32>,
        arg5: std::option::Option<f32>,
        arg6: std::option::Option<i32>,
        arg7: std::option::Option<i32>,
    ) -> Result<crate::inventory::MerchantRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        // 1
        let val_4 = jni::objects::JValueGen::Bool(arg3.unwrap().into());
        let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
        let val_6 = jni::objects::JValueGen::Float(arg5.unwrap().into());
        let val_7 = jni::objects::JValueGen::Int(arg6.unwrap().into());
        let val_8 = jni::objects::JValueGen::Int(arg7.unwrap().into());
        let cls = &jni.find_class("org/bukkit/inventory/MerchantRecipe")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/inventory/ItemStack;IIZIFII)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
                jni::objects::JValueGen::from(&val_6),
                jni::objects::JValueGen::from(&val_7),
                jni::objects::JValueGen::from(&val_8),
            ],
        )?;
        crate::inventory::MerchantRecipe::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Recipe.html#getResult()">Recipe</a></code></span>
    /// Get the result of this recipe.
    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_ingredient(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addIngredient",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_ingredient(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIngredient",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_ingredients(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                unsafe { jni::objects::JObject::from_raw(v.into().jni_object().clone()) };
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setIngredients",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn ingredients(
        &mut self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getIngredients",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    /// Gets the <a href="#adjust(org.bukkit.inventory.ItemStack)"><code>adjusted</code></a> first ingredient.
    pub fn adjusted_ingredient1(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAdjustedIngredient1",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Modifies the amount of the given <a title="class in org.bukkit.inventory" href="ItemStack.html"><code>ItemStack</code></a> in the same way as MerchantRecipe dynamically adjusts the amount of the first ingredient during trading.
    ///
    /// This is calculated by adding up the original amount of the item, the demand scaled by the recipe's <a href="#getPriceMultiplier()"><code>price multiplier</code></a> and truncated to the next lowest integer value greater than or equal to 0, and the special price, and then constraining the resulting value between <code>1</code> and the <a title="class in org.bukkit.inventory" href="ItemStack.html"><code>ItemStack</code></a>'s <a href="ItemStack.html#getMaxStackSize()"><code>maximum stack size</code></a>.
    pub fn adjust(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "adjust",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the demand for this trade.
    pub fn demand(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDemand", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets the price multiplier for the cost of this trade.
    pub fn price_multiplier(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPriceMultiplier", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    /// Get the special price for this trade.
    pub fn special_price(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSpecialPrice", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the demand for this trade.
    pub fn set_demand(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDemand",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Set the special price for this trade.
    pub fn set_special_price(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpecialPrice",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the number of times this trade has been used.
    pub fn uses(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getUses", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the number of times this trade has been used.
    pub fn set_uses(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUses",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the maximum number of uses this trade has.
    pub fn max_uses(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxUses", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the maximum number of uses this trade has.
    pub fn set_max_uses(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxUses",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Whether to reward experience to the player for the trade.
    pub fn has_experience_reward(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasExperienceReward", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Set whether to reward experience to the player for the trade.
    pub fn set_experience_reward(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExperienceReward",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the amount of experience the villager earns from this trade.
    pub fn villager_experience(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVillagerExperience", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Sets the amount of experience the villager earns from this trade.
    pub fn set_villager_experience(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVillagerExperience",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the price multiplier for the cost of this trade.
    pub fn set_price_multiplier(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPriceMultiplier",
            "(F)V",
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
impl<'mc> Into<crate::inventory::Recipe<'mc>> for MerchantRecipe<'mc> {
    fn into(self) -> crate::inventory::Recipe<'mc> {
        crate::inventory::Recipe::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a trading inventory between a player and a merchant.
///
/// The holder of this Inventory is the owning Villager, or null if the player is trading with a merchant created by a plugin.
///
/// This is a representation of an abstract class.
pub struct MerchantInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> MerchantInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MerchantInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/MerchantInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MerchantInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Get the index of the currently selected recipe.
    pub fn selected_recipe_index(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSelectedRecipeIndex", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Get the index of the currently selected recipe.
    /// Get the currently active recipe.
    /// <p>This will be <code>null</code> if the items provided by the player do not match the ingredients of the selected recipe. This does not necessarily match the recipe selected by the player: If the player has selected the first recipe, the merchant will search all of its offers for a matching recipe to activate.</p>
    pub fn selected_recipe(
        &mut self,
    ) -> Result<crate::inventory::MerchantRecipe<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSelectedRecipe",
            "()Lorg/bukkit/inventory/MerchantRecipe;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::MerchantRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the Merchant associated with this inventory.
    pub fn merchant(
        &mut self,
    ) -> Result<crate::inventory::Merchant<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMerchant",
            "()Lorg/bukkit/inventory/Merchant;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Merchant::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for MerchantInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for MerchantInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An interface to the inventory of a <a href="../entity/Llama.html" title="interface in org.bukkit.entity"><code>Llama</code></a>.
///
/// This is a representation of an abstract class.
pub struct LlamaInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> LlamaInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LlamaInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/LlamaInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LlamaInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Gets the item in the llama's decor slot.
    pub fn decor(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDecor",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the item in the llama's decor slot.
    pub fn set_decor(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDecor",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_saddle(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSaddle",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn saddle(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSaddle",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for LlamaInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::AbstractHorseInventory<'mc>> for LlamaInventory<'mc> {
    fn into(self) -> crate::inventory::AbstractHorseInventory<'mc> {
        crate::inventory::AbstractHorseInventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Interface to the inventory of a Grindstone.
///
/// This is a representation of an abstract class.
pub struct GrindstoneInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> GrindstoneInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate GrindstoneInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/GrindstoneInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a GrindstoneInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for GrindstoneInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for GrindstoneInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a cooking recipe.
pub struct CookingRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for CookingRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CookingRecipe<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CookingRecipe from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/CookingRecipe")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CookingRecipe object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_namespaced_key(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
        arg3: f32,
        arg4: std::option::Option<i32>,
    ) -> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = jni::objects::JValueGen::Float(arg3.into());
        let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
        let cls = &jni.find_class("org/bukkit/inventory/CookingRecipe")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/RecipeChoice;FI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
        crate::inventory::CookingRecipe::from_raw(&jni, res)
    }
    /// Get the result of this recipe.
    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the input material.
    /// Get the input choice.
    pub fn input(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInput",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the input of this cooking recipe.
    /// Sets the input of this cooking recipe.
    pub fn set_input(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInput",
            "(Lorg/bukkit/Material;)Lorg/bukkit/inventory/CookingRecipe;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the category which this recipe will appear in the recipe book under. Defaults to <a href="recipe/CookingBookCategory.html#MISC"><code>CookingBookCategory.MISC</code></a> if not set.
    pub fn category(
        &mut self,
    ) -> Result<crate::inventory::recipe::CookingBookCategory<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCategory",
            "()Lorg/bukkit/inventory/recipe/CookingBookCategory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::inventory::recipe::CookingBookCategory::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::inventory::recipe::CookingBookCategory::from_string(variant_str).unwrap(),
        )
    }
    /// Sets the input of this cooking recipe.
    pub fn set_input_choice(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInputChoice",
            "(Lorg/bukkit/inventory/RecipeChoice;)Lorg/bukkit/inventory/CookingRecipe;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the input choice.
    pub fn input_choice(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInputChoice",
            "()Lorg/bukkit/inventory/RecipeChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the group of this recipe. Recipes with the same group may be grouped together when displayed in the client.
    pub fn group(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getGroup", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Set the group of this recipe. Recipes with the same group may be grouped together when displayed in the client.
    pub fn set_group(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGroup",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the category which this recipe will appear in the recipe book under. Defaults to <a href="recipe/CookingBookCategory.html#MISC"><code>CookingBookCategory.MISC</code></a> if not set.
    pub fn set_category(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::recipe::CookingBookCategory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCategory",
            "(Lorg/bukkit/inventory/recipe/CookingBookCategory;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the experience given by this recipe.
    pub fn experience(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getExperience", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    /// Sets the experience given by this recipe.
    pub fn set_experience(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExperience",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Set the cooking time for this recipe in ticks.
    pub fn set_cooking_time(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookingTime",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the cooking time for this recipe in ticks.
    pub fn cooking_time(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCookingTime", "()I", &[]);
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
impl<'mc> Into<crate::inventory::Recipe<'mc>> for CookingRecipe<'mc> {
    fn into(self) -> crate::inventory::Recipe<'mc> {
        crate::inventory::Recipe::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for CookingRecipe<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Interface to the inventory of a Stonecutter.
///
/// This is a representation of an abstract class.
pub struct StonecutterInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> StonecutterInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate StonecutterInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/StonecutterInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StonecutterInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for StonecutterInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for StonecutterInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum CreativeCategoryEnum {
    BuildingBlocks,
    Decorations,
    Redstone,
    Transportation,
    Misc,
    Food,
    Tools,
    Combat,
    Brewing,
}
impl std::fmt::Display for CreativeCategoryEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CreativeCategoryEnum::BuildingBlocks => f.write_str("BUILDING_BLOCKS"),
            CreativeCategoryEnum::Decorations => f.write_str("DECORATIONS"),
            CreativeCategoryEnum::Redstone => f.write_str("REDSTONE"),
            CreativeCategoryEnum::Transportation => f.write_str("TRANSPORTATION"),
            CreativeCategoryEnum::Misc => f.write_str("MISC"),
            CreativeCategoryEnum::Food => f.write_str("FOOD"),
            CreativeCategoryEnum::Tools => f.write_str("TOOLS"),
            CreativeCategoryEnum::Combat => f.write_str("COMBAT"),
            CreativeCategoryEnum::Brewing => f.write_str("BREWING"),
        }
    }
}
pub struct CreativeCategory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub CreativeCategoryEnum,
);
impl<'mc> std::ops::Deref for CreativeCategory<'mc> {
    type Target = CreativeCategoryEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for CreativeCategory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CreativeCategory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: CreativeCategoryEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CreativeCategory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/CreativeCategory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CreativeCategory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const BUILDING_BLOCKS: CreativeCategoryEnum = CreativeCategoryEnum::BuildingBlocks;
    pub const DECORATIONS: CreativeCategoryEnum = CreativeCategoryEnum::Decorations;
    pub const REDSTONE: CreativeCategoryEnum = CreativeCategoryEnum::Redstone;
    pub const TRANSPORTATION: CreativeCategoryEnum = CreativeCategoryEnum::Transportation;
    pub const MISC: CreativeCategoryEnum = CreativeCategoryEnum::Misc;
    pub const FOOD: CreativeCategoryEnum = CreativeCategoryEnum::Food;
    pub const TOOLS: CreativeCategoryEnum = CreativeCategoryEnum::Tools;
    pub const COMBAT: CreativeCategoryEnum = CreativeCategoryEnum::Combat;
    pub const BREWING: CreativeCategoryEnum = CreativeCategoryEnum::Brewing;
    pub fn from_string(str: String) -> std::option::Option<CreativeCategoryEnum> {
        match str.as_str() {
            "BUILDING_BLOCKS" => Some(CreativeCategoryEnum::BuildingBlocks),
            "DECORATIONS" => Some(CreativeCategoryEnum::Decorations),
            "REDSTONE" => Some(CreativeCategoryEnum::Redstone),
            "TRANSPORTATION" => Some(CreativeCategoryEnum::Transportation),
            "MISC" => Some(CreativeCategoryEnum::Misc),
            "FOOD" => Some(CreativeCategoryEnum::Food),
            "TOOLS" => Some(CreativeCategoryEnum::Tools),
            "COMBAT" => Some(CreativeCategoryEnum::Combat),
            "BREWING" => Some(CreativeCategoryEnum::Brewing),
            _ => None,
        }
    }
}
/// Interface to the inventory of a Smithing table.
///
/// This is a representation of an abstract class.
pub struct SmithingInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SmithingInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SmithingInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/SmithingInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SmithingInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Check what item is in the result slot of this smithing table.
    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the item in the result slot of the smithing table
    pub fn set_result(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the current recipe formed on the smithing table, if any.
    pub fn recipe(&mut self) -> Result<crate::inventory::Recipe<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRecipe",
            "()Lorg/bukkit/inventory/Recipe;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Recipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for SmithingInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for SmithingInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Interface to the inventory of a Player, including the four armor slots and any extra slots.
///
/// This is a representation of an abstract class.
pub struct PlayerInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PlayerInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PlayerInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/PlayerInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PlayerInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Gets the ItemStack at the given equipment slot in the inventory.
    /// Gets a copy of the item the player is currently holding in their main hand.
    /// Gets a copy of the item the player is currently holding in their off hand.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// players can duel wield now use the methods for the specific hand instead
    /// </div>
    /// players can duel wield now use the methods for the specific hand instead
    ///
    /// Gets a copy of the item the player is currently holding
    pub fn get_item_with_equipment_slot(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Stores the ItemStack at the given index of the inventory.
    /// <p>Indexes 0 through 8 refer to the hotbar. 9 through 35 refer to the main inventory, counting up from 9 at the top left corner of the inventory, moving to the right, and moving to the row below it back on the left side when it reaches the end of the row. It follows the same path in the inventory like you would read a book.</p>
    /// <p>Indexes 36 through 39 refer to the armor slots. Though you can set armor with this method using these indexes, you are encouraged to use the provided methods for those slots.</p>
    /// <p>Index 40 refers to the off hand (shield) item slot. Though you can set off hand with this method using this index, you are encouraged to use the provided method for this slot.</p>
    /// <p>If you attempt to use this method with an index less than 0 or greater than 40, an ArrayIndexOutOfBounds exception will be thrown.</p>
    /// Stores the ItemStack at the given equipment slot in the inventory.
    /// Sets the item the player is holding in their main hand.
    /// Sets the item the player is holding in their off hand.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// players can duel wield now use the methods for the specific hand instead
    /// </div>
    /// players can duel wield now use the methods for the specific hand instead
    ///
    /// Sets the item the player is holding
    pub fn set_item_with_equipment_slot(
        &mut self,
        arg0: i32,
        arg1: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Inventory.html#getHolder()">Inventory</a></code></span>
    /// Gets the block or entity belonging to the open inventory
    pub fn holder(
        &mut self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/entity/HumanEntity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// players can duel wield now use the methods for the specific hand instead
    /// </div>
    /// players can duel wield now use the methods for the specific hand instead
    ///
    /// Gets a copy of the item the player is currently holding
    pub fn item_in_hand(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemInHand",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// players can duel wield now use the methods for the specific hand instead
    /// </div>
    /// players can duel wield now use the methods for the specific hand instead
    ///
    /// Sets the item the player is holding
    pub fn set_item_in_hand(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInHand",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Return the ItemStack from the boots slot
    pub fn boots(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoots",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Put the given ItemStack into the boots slot. This does not check if the ItemStack is a boots
    pub fn set_boots(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBoots",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Return the ItemStack from the leg slot
    pub fn leggings(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLeggings",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Put the given ItemStack into the leg slot. This does not check if the ItemStack is a pair of leggings
    pub fn set_leggings(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLeggings",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Return the ItemStack from the chestplate slot
    pub fn chestplate(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getChestplate",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Put the given ItemStack into the chestplate slot. This does not check if the ItemStack is a chestplate
    pub fn set_chestplate(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setChestplate",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Return the ItemStack from the helmet slot
    pub fn helmet(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHelmet",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Put the given ItemStack into the helmet slot. This does not check if the ItemStack is a helmet
    pub fn set_helmet(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHelmet",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a copy of the item the player is currently holding in their main hand.
    pub fn item_in_main_hand(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemInMainHand",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the item the player is holding in their main hand.
    pub fn set_item_in_main_hand(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInMainHand",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a copy of the item the player is currently holding in their off hand.
    pub fn item_in_off_hand(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemInOffHand",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the item the player is holding in their off hand.
    pub fn set_item_in_off_hand(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInOffHand",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Put the given ItemStacks into the armor slots
    pub fn set_armor_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setArmorContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Put the given ItemStacks into the extra slots
    ///
    /// See <a href="#getExtraContents()"><code>getExtraContents()</code></a> for an explanation of extra slots.
    pub fn set_extra_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExtraContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the slot number of the currently held item
    pub fn held_item_slot(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeldItemSlot", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the slot number of the currently held item.
    /// <p>This validates whether the slot is between 0 and 8 inclusive.</p>
    pub fn set_held_item_slot(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHeldItemSlot",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for PlayerInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for PlayerInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Interface to the inventory of a Furnace.
///
/// This is a representation of an abstract class.
pub struct FurnaceInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> FurnaceInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FurnaceInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/FurnaceInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FurnaceInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Get the current item in the result slot.
    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the current item in the result slot.
    pub fn set_result(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Set the current fuel.
    pub fn set_fuel(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuel",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the current fuel.
    pub fn fuel(&mut self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFuel",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the item currently smelting.
    pub fn smelting(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSmelting",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the item currently smelting.
    pub fn set_smelting(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSmelting",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Inventory.html#getHolder()">Inventory</a></code></span>
    /// Gets the block or entity belonging to the open inventory
    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for FurnaceInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for FurnaceInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An interface to the inventory of an <a href="../entity/AbstractHorse.html" title="interface in org.bukkit.entity"><code>AbstractHorse</code></a>.
///
/// This is a representation of an abstract class.
pub struct AbstractHorseInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> AbstractHorseInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate AbstractHorseInventory from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/AbstractHorseInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AbstractHorseInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Sets the item in the horse's saddle slot.
    pub fn set_saddle(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSaddle",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the item in the horse's saddle slot.
    pub fn saddle(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSaddle",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for AbstractHorseInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for AbstractHorseInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Interface to the inventory of a Beacon.
///
/// This is a representation of an abstract class.
pub struct BeaconInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BeaconInventory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BeaconInventory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/BeaconInventory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BeaconInventory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Get the item powering the beacon.
    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the item powering the beacon.
    pub fn set_item_with_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn all_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "all",
            "(Lorg/bukkit/Material;)Ljava/util/HashMap;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_max_stack_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn holder(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHolder",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn max_stack_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxStackSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn remove_item(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<blackboxmc_java::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            "(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_storage_contents(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            "(Lorg/bukkit/inventory/ItemStack;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_empty(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstEmpty", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_item_stack(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/Material;I)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn first_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "first",
            "(Lorg/bukkit/Material;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/InventoryType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryType::from_string(variant_str).unwrap(),
        )
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for BeaconInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for BeaconInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a smithing trim recipe.
pub struct SmithingTrimRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SmithingTrimRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SmithingTrimRecipe<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SmithingTrimRecipe from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/SmithingTrimRecipe")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SmithingTrimRecipe object, got {}",
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
        arg1: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
        arg2: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
        arg3: impl Into<&'mc crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::SmithingTrimRecipe<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/inventory/SmithingTrimRecipe")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/RecipeChoice;Lorg/bukkit/inventory/RecipeChoice;Lorg/bukkit/inventory/RecipeChoice;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
        crate::inventory::SmithingTrimRecipe::from_raw(&jni, res)
    }
    /// Get the template recipe item.
    pub fn template(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTemplate",
            "()Lorg/bukkit/inventory/RecipeChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn base(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBase",
            "()Lorg/bukkit/inventory/RecipeChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn addition(
        &mut self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAddition",
            "()Lorg/bukkit/inventory/RecipeChoice;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
impl<'mc> Into<crate::inventory::ComplexRecipe<'mc>> for SmithingTrimRecipe<'mc> {
    fn into(self) -> crate::inventory::ComplexRecipe<'mc> {
        crate::inventory::ComplexRecipe::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::inventory::SmithingRecipe<'mc>> for SmithingTrimRecipe<'mc> {
    fn into(self) -> crate::inventory::SmithingRecipe<'mc> {
        crate::inventory::SmithingRecipe::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub mod meta;
pub mod recipe;
