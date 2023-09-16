#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Interface to the inventory of a Cartography table.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct CartographyInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CartographyInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CartographyInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> CartographyInventory<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for CartographyInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CartographyInventory into crate::inventory::Inventory")
    }
}
/// Represents some type of crafting recipe.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Recipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Recipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Recipe<'mc> {
    fn from_raw(
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
}

impl<'mc> Recipe<'mc> {
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResult", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a Stonecutting recipe.
#[repr(C)]
pub struct StonecuttingRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for StonecuttingRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for StonecuttingRecipe<'mc> {
    fn from_raw(
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
}

impl<'mc> StonecuttingRecipe<'mc> {
    pub fn new_with_namespaced_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::StonecuttingRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/NamespacedKey;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/inventory/RecipeChoice;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        args.push(val_3);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/StonecuttingRecipe");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::StonecuttingRecipe::from_raw(&jni, res)
    }

    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResult", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_input(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<crate::inventory::StonecuttingRecipe<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Material;)Lorg/bukkit/inventory/StonecuttingRecipe;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInput",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::StonecuttingRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn input(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getInput", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_input_choice(
        &self,
        arg0: impl Into<crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::StonecuttingRecipe<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/inventory/RecipeChoice;)Lorg/bukkit/inventory/StonecuttingRecipe;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInputChoice",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::StonecuttingRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn input_choice(
        &self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/RecipeChoice;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInputChoice", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn group(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_group(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGroup",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Recipe<'mc>> for StonecuttingRecipe<'mc> {
    fn into(self) -> crate::inventory::Recipe<'mc> {
        crate::inventory::Recipe::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting StonecuttingRecipe into crate::inventory::Recipe")
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for StonecuttingRecipe<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting StonecuttingRecipe into crate::Keyed")
    }
}
/// Represents a furnace recipe.
#[repr(C)]
pub struct FurnaceRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FurnaceRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FurnaceRecipe<'mc> {
    fn from_raw(
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
}

impl<'mc> FurnaceRecipe<'mc> {
    #[deprecated]

    pub fn new_with_item_stack(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: impl Into<crate::Material<'mc>>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::inventory::FurnaceRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/Material;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/FurnaceRecipe");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::FurnaceRecipe::from_raw(&jni, res)
    }
    #[deprecated]

    pub fn new_with_namespaced_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg3: std::option::Option<i32>,
        arg4: std::option::Option<f32>,
        arg5: std::option::Option<i32>,
    ) -> Result<crate::inventory::FurnaceRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/NamespacedKey;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/Material;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        if let Some(a) = arg4 {
            sig += "F";
            let val_5 = jni::objects::JValueGen::Float(a);
            args.push(val_5);
        }
        if let Some(a) = arg5 {
            sig += "I";
            let val_6 = jni::objects::JValueGen::Int(a);
            args.push(val_6);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/FurnaceRecipe");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::FurnaceRecipe::from_raw(&jni, res)
    }

    pub fn set_input_with_material_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<crate::inventory::FurnaceRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/material/MaterialData;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Lorg/bukkit/inventory/FurnaceRecipe;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setInput", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::FurnaceRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_input_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::inventory::FurnaceRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/inventory/FurnaceRecipe;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setInput", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::FurnaceRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_input_choice_with_recipe_choice(
        &self,
        arg0: impl Into<crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::FurnaceRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/RecipeChoice;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Lorg/bukkit/inventory/FurnaceRecipe;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setInputChoice", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::FurnaceRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: CookingRecipe
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CookingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Recipe = temp_clone.into();
        real.result()
    }
    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CookingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Keyed = temp_clone.into();
        real.key()
    }
    pub fn input(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CookingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CookingRecipe = temp_clone.into();
        real.input()
    }
    pub fn category(
        &self,
    ) -> Result<crate::inventory::recipe::CookingBookCategory<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::inventory::CookingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CookingRecipe = temp_clone.into();
        real.category()
    }
    pub fn input_choice(
        &self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CookingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CookingRecipe = temp_clone.into();
        real.input_choice()
    }
    pub fn group(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CookingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CookingRecipe = temp_clone.into();
        real.group()
    }
    pub fn set_group(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CookingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CookingRecipe = temp_clone.into();
        real.set_group(arg0)
    }
    pub fn set_category(
        &self,
        arg0: impl Into<crate::inventory::recipe::CookingBookCategory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CookingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CookingRecipe = temp_clone.into();
        real.set_category(arg0)
    }
    pub fn experience(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CookingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CookingRecipe = temp_clone.into();
        real.experience()
    }
    pub fn set_experience(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CookingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CookingRecipe = temp_clone.into();
        real.set_experience(arg0)
    }
    pub fn set_cooking_time(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CookingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CookingRecipe = temp_clone.into();
        real.set_cooking_time(arg0)
    }
    pub fn cooking_time(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CookingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CookingRecipe = temp_clone.into();
        real.cooking_time()
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::CookingRecipe<'mc>> for FurnaceRecipe<'mc> {
    fn into(self) -> crate::inventory::CookingRecipe<'mc> {
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FurnaceRecipe into crate::inventory::CookingRecipe")
    }
}
/// An interface to the inventory of a Horse.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct HorseInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HorseInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HorseInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> HorseInventory<'mc> {
    pub fn armor(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getArmor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_armor(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setArmor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_saddle(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = HorseInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::AbstractHorseInventory = temp_clone.into();
        real.set_saddle(arg0)
    }
    pub fn saddle(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = HorseInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::AbstractHorseInventory = temp_clone.into();
        real.saddle()
    }
    // SUPER CLASS: Inventory
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_item(arg0)
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_item(arg0, arg1)
    }
    pub fn holder(
        &self,
    ) -> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.holder()
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::AbstractHorseInventory<'mc>> for HorseInventory<'mc> {
    fn into(self) -> crate::inventory::AbstractHorseInventory<'mc> {
        crate::inventory::AbstractHorseInventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HorseInventory into crate::inventory::AbstractHorseInventory")
    }
}
/// Represents a complex recipe which has imperative server-defined behavior, eg armor dyeing. Note: Since a complex recipe has dynamic outputs, <a href="Recipe.html#getResult()"><code>Recipe.getResult()</code></a> will sometimes return an AIR ItemStack.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct ComplexRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ComplexRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ComplexRecipe<'mc> {
    fn from_raw(
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
}

impl<'mc> ComplexRecipe<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Recipe<'mc>> for ComplexRecipe<'mc> {
    fn into(self) -> crate::inventory::Recipe<'mc> {
        crate::inventory::Recipe::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ComplexRecipe into crate::inventory::Recipe")
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for ComplexRecipe<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ComplexRecipe into crate::Keyed")
    }
}
/// Interface to the inventory of a Double Chest.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct DoubleChestInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DoubleChestInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DoubleChestInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> DoubleChestInventory<'mc> {
    pub fn holder(
        &self,
    ) -> Result<Option<crate::block::DoubleChest<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/DoubleChest;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHolder", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::DoubleChest::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn left_side(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLeftSide", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn right_side(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRightSide", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = DoubleChestInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_item(arg0)
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = DoubleChestInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = DoubleChestInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_item(arg0, arg1)
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = DoubleChestInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = DoubleChestInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = DoubleChestInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = DoubleChestInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = DoubleChestInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = DoubleChestInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = DoubleChestInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = DoubleChestInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = DoubleChestInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }
    // SUPER CLASS: Iterable

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for DoubleChestInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting DoubleChestInventory into crate::inventory::Inventory")
    }
}
/// Represents a block inventory holder - either a BlockState, or a regular Block.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct BlockInventoryHolder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockInventoryHolder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockInventoryHolder<'mc> {
    fn from_raw(
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
}

impl<'mc> BlockInventoryHolder<'mc> {
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BlockInventoryHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::InventoryHolder = temp_clone.into();
        real.inventory()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for BlockInventoryHolder<'mc> {
    fn into(self) -> crate::inventory::InventoryHolder<'mc> {
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockInventoryHolder into crate::inventory::InventoryHolder")
    }
}
/// Interface to the inventory of a Jukebox.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct JukeboxInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JukeboxInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for JukeboxInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> JukeboxInventory<'mc> {
    pub fn holder(&self) -> Result<Option<crate::block::Jukebox<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/Jukebox;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHolder", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::Jukebox::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_record(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRecord",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn record(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRecord", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = JukeboxInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_item(arg0)
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = JukeboxInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = JukeboxInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_item(arg0, arg1)
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = JukeboxInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = JukeboxInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = JukeboxInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = JukeboxInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = JukeboxInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = JukeboxInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = JukeboxInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = JukeboxInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = JukeboxInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }
    // SUPER CLASS: Iterable

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for JukeboxInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JukeboxInventory into crate::inventory::Inventory")
    }
}
/// Represents a smithing recipe.
#[repr(C)]
pub struct SmithingRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SmithingRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SmithingRecipe<'mc> {
    fn from_raw(
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
}

impl<'mc> SmithingRecipe<'mc> {
    #[deprecated]

    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<crate::inventory::RecipeChoice<'mc>>,
        arg3: impl Into<crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::SmithingRecipe<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/RecipeChoice;Lorg/bukkit/inventory/RecipeChoice;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/inventory/SmithingRecipe");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::SmithingRecipe::from_raw(&jni, res)
    }

    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResult", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn base(&self) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/RecipeChoice;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBase", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn addition(
        &self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/RecipeChoice;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAddition", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Recipe<'mc>> for SmithingRecipe<'mc> {
    fn into(self) -> crate::inventory::Recipe<'mc> {
        crate::inventory::Recipe::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SmithingRecipe into crate::inventory::Recipe")
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for SmithingRecipe<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SmithingRecipe into crate::Keyed")
    }
}
/// Represents a shaped or shapeless crafting recipe.
#[repr(C)]
pub struct CraftingRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CraftingRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CraftingRecipe<'mc> {
    fn from_raw(
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
}

impl<'mc> CraftingRecipe<'mc> {
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResult", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn category(
        &self,
    ) -> Result<crate::inventory::recipe::CraftingBookCategory<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/inventory/recipe/CraftingBookCategory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCategory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::recipe::CraftingBookCategory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn group(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_group(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGroup",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_category(
        &self,
        arg0: impl Into<crate::inventory::recipe::CraftingBookCategory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/recipe/CraftingBookCategory;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCategory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Recipe<'mc>> for CraftingRecipe<'mc> {
    fn into(self) -> crate::inventory::Recipe<'mc> {
        crate::inventory::Recipe::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CraftingRecipe into crate::inventory::Recipe")
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for CraftingRecipe<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CraftingRecipe into crate::Keyed")
    }
}
/// Represents a view linking two inventories and a single player (whose inventory may or may not be one of the two).
/// <p>Note: If you implement this interface but fail to satisfy the expected contracts of certain methods, there's no guarantee that the game will work as it should.</p>
#[repr(C)]
pub struct InventoryView<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum InventoryViewProperty<'mc> {
    BrewTime {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    FuelTime {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    BurnTime {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    TicksForCurrentFuel {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    CookTime {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    TicksForCurrentSmelting {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    EnchantButton1 {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    EnchantButton2 {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    EnchantButton3 {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    EnchantXpSeed {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    EnchantId1 {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    EnchantId2 {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    EnchantId3 {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    EnchantLevel1 {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    EnchantLevel2 {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    EnchantLevel3 {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    Levels {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    PrimaryEffect {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    SecondaryEffect {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    RepairCost {
        inner: InventoryViewPropertyStruct<'mc>,
    },
    BookPage {
        inner: InventoryViewPropertyStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for InventoryViewProperty<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryViewProperty::BrewTime { .. } => f.write_str("BREW_TIME"),
            InventoryViewProperty::FuelTime { .. } => f.write_str("FUEL_TIME"),
            InventoryViewProperty::BurnTime { .. } => f.write_str("BURN_TIME"),
            InventoryViewProperty::TicksForCurrentFuel { .. } => {
                f.write_str("TICKS_FOR_CURRENT_FUEL")
            }
            InventoryViewProperty::CookTime { .. } => f.write_str("COOK_TIME"),
            InventoryViewProperty::TicksForCurrentSmelting { .. } => {
                f.write_str("TICKS_FOR_CURRENT_SMELTING")
            }
            InventoryViewProperty::EnchantButton1 { .. } => f.write_str("ENCHANT_BUTTON1"),
            InventoryViewProperty::EnchantButton2 { .. } => f.write_str("ENCHANT_BUTTON2"),
            InventoryViewProperty::EnchantButton3 { .. } => f.write_str("ENCHANT_BUTTON3"),
            InventoryViewProperty::EnchantXpSeed { .. } => f.write_str("ENCHANT_XP_SEED"),
            InventoryViewProperty::EnchantId1 { .. } => f.write_str("ENCHANT_ID1"),
            InventoryViewProperty::EnchantId2 { .. } => f.write_str("ENCHANT_ID2"),
            InventoryViewProperty::EnchantId3 { .. } => f.write_str("ENCHANT_ID3"),
            InventoryViewProperty::EnchantLevel1 { .. } => f.write_str("ENCHANT_LEVEL1"),
            InventoryViewProperty::EnchantLevel2 { .. } => f.write_str("ENCHANT_LEVEL2"),
            InventoryViewProperty::EnchantLevel3 { .. } => f.write_str("ENCHANT_LEVEL3"),
            InventoryViewProperty::Levels { .. } => f.write_str("LEVELS"),
            InventoryViewProperty::PrimaryEffect { .. } => f.write_str("PRIMARY_EFFECT"),
            InventoryViewProperty::SecondaryEffect { .. } => f.write_str("SECONDARY_EFFECT"),
            InventoryViewProperty::RepairCost { .. } => f.write_str("REPAIR_COST"),
            InventoryViewProperty::BookPage { .. } => f.write_str("BOOK_PAGE"),
        }
    }
}

impl<'mc> InventoryViewProperty<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<InventoryViewProperty<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/inventory/InventoryView$Property");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/inventory/InventoryView$Property;",
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
            "BREW_TIME" => Ok(InventoryViewProperty::BrewTime {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "FUEL_TIME" => Ok(InventoryViewProperty::FuelTime {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "BURN_TIME" => Ok(InventoryViewProperty::BurnTime {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "TICKS_FOR_CURRENT_FUEL" => Ok(InventoryViewProperty::TicksForCurrentFuel {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "COOK_TIME" => Ok(InventoryViewProperty::CookTime {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "TICKS_FOR_CURRENT_SMELTING" => Ok(InventoryViewProperty::TicksForCurrentSmelting {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_BUTTON1" => Ok(InventoryViewProperty::EnchantButton1 {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_BUTTON2" => Ok(InventoryViewProperty::EnchantButton2 {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_BUTTON3" => Ok(InventoryViewProperty::EnchantButton3 {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_XP_SEED" => Ok(InventoryViewProperty::EnchantXpSeed {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_ID1" => Ok(InventoryViewProperty::EnchantId1 {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_ID2" => Ok(InventoryViewProperty::EnchantId2 {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_ID3" => Ok(InventoryViewProperty::EnchantId3 {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_LEVEL1" => Ok(InventoryViewProperty::EnchantLevel1 {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_LEVEL2" => Ok(InventoryViewProperty::EnchantLevel2 {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_LEVEL3" => Ok(InventoryViewProperty::EnchantLevel3 {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "LEVELS" => Ok(InventoryViewProperty::Levels {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "PRIMARY_EFFECT" => Ok(InventoryViewProperty::PrimaryEffect {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "SECONDARY_EFFECT" => Ok(InventoryViewProperty::SecondaryEffect {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "REPAIR_COST" => Ok(InventoryViewProperty::RepairCost {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),
            "BOOK_PAGE" => Ok(InventoryViewProperty::BookPage {
                inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct InventoryViewPropertyStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InventoryViewProperty<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::BrewTime { inner } => inner.0.clone(),
            Self::FuelTime { inner } => inner.0.clone(),
            Self::BurnTime { inner } => inner.0.clone(),
            Self::TicksForCurrentFuel { inner } => inner.0.clone(),
            Self::CookTime { inner } => inner.0.clone(),
            Self::TicksForCurrentSmelting { inner } => inner.0.clone(),
            Self::EnchantButton1 { inner } => inner.0.clone(),
            Self::EnchantButton2 { inner } => inner.0.clone(),
            Self::EnchantButton3 { inner } => inner.0.clone(),
            Self::EnchantXpSeed { inner } => inner.0.clone(),
            Self::EnchantId1 { inner } => inner.0.clone(),
            Self::EnchantId2 { inner } => inner.0.clone(),
            Self::EnchantId3 { inner } => inner.0.clone(),
            Self::EnchantLevel1 { inner } => inner.0.clone(),
            Self::EnchantLevel2 { inner } => inner.0.clone(),
            Self::EnchantLevel3 { inner } => inner.0.clone(),
            Self::Levels { inner } => inner.0.clone(),
            Self::PrimaryEffect { inner } => inner.0.clone(),
            Self::SecondaryEffect { inner } => inner.0.clone(),
            Self::RepairCost { inner } => inner.0.clone(),
            Self::BookPage { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::BrewTime { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FuelTime { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::BurnTime { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::TicksForCurrentFuel { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CookTime { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::TicksForCurrentSmelting { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantButton1 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantButton2 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantButton3 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantXpSeed { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantId1 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantId2 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantId3 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantLevel1 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantLevel2 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantLevel3 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Levels { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PrimaryEffect { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SecondaryEffect { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RepairCost { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BookPage { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryViewProperty<'mc> {
    fn from_raw(
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
            env.validate_name(&obj, "org/bukkit/inventory/InventoryView$Property")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryViewProperty object, got {}",
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
                "BREW_TIME" => Ok(InventoryViewProperty::BrewTime {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "FUEL_TIME" => Ok(InventoryViewProperty::FuelTime {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "BURN_TIME" => Ok(InventoryViewProperty::BurnTime {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "TICKS_FOR_CURRENT_FUEL" => Ok(InventoryViewProperty::TicksForCurrentFuel {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "COOK_TIME" => Ok(InventoryViewProperty::CookTime {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "TICKS_FOR_CURRENT_SMELTING" => {
                    Ok(InventoryViewProperty::TicksForCurrentSmelting {
                        inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                    })
                }
                "ENCHANT_BUTTON1" => Ok(InventoryViewProperty::EnchantButton1 {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_BUTTON2" => Ok(InventoryViewProperty::EnchantButton2 {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_BUTTON3" => Ok(InventoryViewProperty::EnchantButton3 {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_XP_SEED" => Ok(InventoryViewProperty::EnchantXpSeed {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_ID1" => Ok(InventoryViewProperty::EnchantId1 {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_ID2" => Ok(InventoryViewProperty::EnchantId2 {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_ID3" => Ok(InventoryViewProperty::EnchantId3 {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_LEVEL1" => Ok(InventoryViewProperty::EnchantLevel1 {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_LEVEL2" => Ok(InventoryViewProperty::EnchantLevel2 {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_LEVEL3" => Ok(InventoryViewProperty::EnchantLevel3 {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "LEVELS" => Ok(InventoryViewProperty::Levels {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "PRIMARY_EFFECT" => Ok(InventoryViewProperty::PrimaryEffect {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "SECONDARY_EFFECT" => Ok(InventoryViewProperty::SecondaryEffect {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "REPAIR_COST" => Ok(InventoryViewProperty::RepairCost {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                "BOOK_PAGE" => Ok(InventoryViewProperty::BookPage {
                    inner: InventoryViewPropertyStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for InventoryViewPropertyStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryViewPropertyStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InventoryViewPropertyStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/InventoryView$Property")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryViewPropertyStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InventoryViewPropertyStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<Vec<crate::inventory::InventoryViewProperty<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/InventoryView$Property;");
        let cls = jni.find_class("org/bukkit/inventory/InventoryView$Property");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::InventoryViewProperty::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    pub fn id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for InventoryView<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryView<'mc> {
    fn from_raw(
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
}

impl<'mc> InventoryView<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/inventory/InventoryView");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::InventoryView::from_raw(&jni, res)
    }
    /// Gets one item in this inventory view by its raw slot ID.
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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

    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the inventory corresponding to the given raw slot ID. If the slot ID is <a href="#OUTSIDE"><code>OUTSIDE</code></a> null will be returned, otherwise behaviour for illegal and negative slot IDs is undefined. May be used with <a href="#convertSlot(int)"><code>convertSlot(int)</code></a> to directly index an underlying inventory.
    pub fn get_inventory(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/Inventory;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::Inventory::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn player(
        &self,
    ) -> Result<Option<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::HumanEntity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn top_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTopInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn bottom_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBottomInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Converts a raw slot ID into its local slot ID into whichever of the two inventories the slot points to.
    /// <p>If the raw slot refers to the upper inventory, it will be returned unchanged and thus be suitable for getTopInventory().getItem(); if it refers to the lower inventory, the output will differ from the input and be suitable for getBottomInventory().getItem().</p>
    pub fn convert_slot(&self, arg0: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "convertSlot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn set_cursor(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCursor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cursor(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCursor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn count_slots(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "countSlots", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Determine the type of the slot by its raw slot ID.
    /// <p>If the type of the slot is unknown, then <a href="../event/inventory/InventoryType.SlotType.html#CONTAINER"><code>InventoryType.SlotType.CONTAINER</code></a> will be returned.</p>
    pub fn get_slot_type(
        &self,
        arg0: i32,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(I)Lorg/bukkit/event/inventory/InventoryType$SlotType;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn original_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOriginalTitle",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_title(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_property(
        &self,
        arg0: impl Into<crate::inventory::InventoryViewProperty<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView$Property;I)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProperty",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a campfire recipe.
#[repr(C)]
pub struct CampfireRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CampfireRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CampfireRecipe<'mc> {
    fn from_raw(
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
}

impl<'mc> CampfireRecipe<'mc> {
    pub fn new_with_namespaced_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<crate::inventory::RecipeChoice<'mc>>,
        arg3: f32,
        arg4: i32,
    ) -> Result<crate::inventory::CampfireRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/NamespacedKey;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/inventory/RecipeChoice;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        args.push(val_3);
        sig += "F";
        let val_4 = jni::objects::JValueGen::Float(arg3);
        args.push(val_4);
        sig += "I";
        let val_5 = jni::objects::JValueGen::Int(arg4);
        args.push(val_5);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/CampfireRecipe");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::CampfireRecipe::from_raw(&jni, res)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::CookingRecipe<'mc>> for CampfireRecipe<'mc> {
    fn into(self) -> crate::inventory::CookingRecipe<'mc> {
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CampfireRecipe into crate::inventory::CookingRecipe")
    }
}
/// Represents a stack of items.
/// <p><b>IMPORTANT: An <i>Item</i>Stack is only designed to contain <i>items</i>. Do not use this class to encapsulate Materials for which <a href="../Material.html#isItem()"><code>Material.isItem()</code></a> returns false.</b></p>
#[repr(C)]
pub struct ItemStack<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ItemStack<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ItemStack<'mc> {
    fn from_raw(
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
}

impl<'mc> ItemStack<'mc> {
    pub fn new_with_item_stack(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/ItemStack");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::ItemStack::from_raw(&jni, res)
    }
    #[deprecated]

    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i16>,
        arg3: std::option::Option<i8>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "S";
            let val_3 = jni::objects::JValueGen::Short(a);
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "Ljava/lang/Byte;";
            let val_4 = jni::objects::JValueGen::Object(jni.new_object(
                "java/lang/Byte",
                "(Ljava/Lang/Object;)V",
                vec![a.into()],
            )?);
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/ItemStack");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::ItemStack::from_raw(&jni, res)
    }

    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/material/MaterialData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Material;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn deserialize(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/inventory/ItemStack");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "deserialize",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::inventory::ItemStack::from_raw(&jni, obj)
    }

    pub fn translation_key(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTranslationKey",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxStackSize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn get_enchantment_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)I");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantmentLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    #[deprecated = "durability is now part of ItemMeta. To avoid confusion and misuse, <a href='#getItemMeta()'><code>getItemMeta()</code></a>, <a href='#setItemMeta(org.bukkit.inventory.meta.ItemMeta)'><code>setItemMeta(ItemMeta)</code></a> and <a href='meta/Damageable.html#setDamage(int)'><code>Damageable.setDamage(int)</code></a> should be used instead. This is because any call to this method will be overwritten by subsequent setting of ItemMeta which was created before this call. "]
    /// Sets the durability of this item
    pub fn set_durability(&self, arg0: i16) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(S)V");
        let val_1 = jni::objects::JValueGen::Short(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDurability",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn amount(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn has_item_meta(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasItemMeta", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn item_meta(
        &self,
    ) -> Result<Option<crate::inventory::meta::ItemMeta<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/meta/ItemMeta;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemMeta", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::meta::ItemMeta::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the amount of items in this stack
    pub fn set_amount(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAmount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn durability(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("()S");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDurability", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s()?)
    }

    pub fn set_item_meta(
        &self,
        arg0: impl Into<crate::inventory::meta::ItemMeta<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/meta/ItemMeta;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemMeta",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_similar(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSimilar",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn contains_enchantment(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsEnchantment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn enchantments(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchantments", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchantments(
        &self,
        arg0: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchantments",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_enchantment(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchantment",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_unsafe_enchantment(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addUnsafeEnchantment",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_unsafe_enchantments(
        &self,
        arg0: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addUnsafeEnchantments",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_enchantment(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)I");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchantment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
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

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn clone(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> std::string::ToString for ItemStack<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling ItemStack.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for ItemStack<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting ItemStack into crate::configuration::serialization::ConfigurationSerializable")
    }
}
impl<'mc> Into<crate::Translatable<'mc>> for ItemStack<'mc> {
    fn into(self) -> crate::Translatable<'mc> {
        crate::Translatable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ItemStack into crate::Translatable")
    }
}
/// Represents a choice of multiple matching Materials.
#[repr(C)]
pub struct RecipeChoiceMaterialChoice<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for RecipeChoiceMaterialChoice<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RecipeChoiceMaterialChoice<'mc> {
    fn from_raw(
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
            env.validate_name(&obj, "org/bukkit/inventory/RecipeChoice$MaterialChoice")?;
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
}

impl<'mc> RecipeChoiceMaterialChoice<'mc> {
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<crate::inventory::RecipeChoiceMaterialChoice<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/RecipeChoice$MaterialChoice");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::RecipeChoiceMaterialChoice::from_raw(&jni, res)
    }
    pub fn new_with_materials(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: Vec<crate::Material<'mc>>,
    ) -> Result<crate::inventory::RecipeChoiceMaterialChoice<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[Lorg/bukkit/Material;";
        let arr = jni.new_object_array(
            arg0.len() as i32,
            "org/bukkit/Material",
            jni::objects::JObject::null(),
        );
        let arr = jni.translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/RecipeChoice$MaterialChoice");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::RecipeChoiceMaterialChoice::from_raw(&jni, res)
    }
    pub fn new_with_tag(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Tag<'mc>>,
    ) -> Result<crate::inventory::RecipeChoiceMaterialChoice<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Tag;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/RecipeChoice$MaterialChoice");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::RecipeChoiceMaterialChoice::from_raw(&jni, res)
    }

    pub fn item_stack(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemStack", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn choices(&self) -> Result<Vec<crate::Material<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChoices", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::Material::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
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

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::RecipeChoiceMaterialChoice<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/RecipeChoice$MaterialChoice;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoiceMaterialChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn test_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "test", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> std::string::ToString for RecipeChoiceMaterialChoice<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling RecipeChoiceMaterialChoice.toString: {}", err),
        }
    }
}

pub enum ItemFlag<'mc> {
    HideEnchants { inner: ItemFlagStruct<'mc> },
    HideAttributes { inner: ItemFlagStruct<'mc> },
    HideUnbreakable { inner: ItemFlagStruct<'mc> },
    HideDestroys { inner: ItemFlagStruct<'mc> },
    HidePlacedOn { inner: ItemFlagStruct<'mc> },
    HidePotionEffects { inner: ItemFlagStruct<'mc> },
    HideDye { inner: ItemFlagStruct<'mc> },
    HideArmorTrim { inner: ItemFlagStruct<'mc> },
}
impl<'mc> std::fmt::Display for ItemFlag<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemFlag::HideEnchants { .. } => f.write_str("HIDE_ENCHANTS"),
            ItemFlag::HideAttributes { .. } => f.write_str("HIDE_ATTRIBUTES"),
            ItemFlag::HideUnbreakable { .. } => f.write_str("HIDE_UNBREAKABLE"),
            ItemFlag::HideDestroys { .. } => f.write_str("HIDE_DESTROYS"),
            ItemFlag::HidePlacedOn { .. } => f.write_str("HIDE_PLACED_ON"),
            ItemFlag::HidePotionEffects { .. } => f.write_str("HIDE_POTION_EFFECTS"),
            ItemFlag::HideDye { .. } => f.write_str("HIDE_DYE"),
            ItemFlag::HideArmorTrim { .. } => f.write_str("HIDE_ARMOR_TRIM"),
        }
    }
}

impl<'mc> ItemFlag<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ItemFlag<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/inventory/ItemFlag");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/inventory/ItemFlag;",
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
            "HIDE_ENCHANTS" => Ok(ItemFlag::HideEnchants {
                inner: ItemFlagStruct::from_raw(env, obj)?,
            }),
            "HIDE_ATTRIBUTES" => Ok(ItemFlag::HideAttributes {
                inner: ItemFlagStruct::from_raw(env, obj)?,
            }),
            "HIDE_UNBREAKABLE" => Ok(ItemFlag::HideUnbreakable {
                inner: ItemFlagStruct::from_raw(env, obj)?,
            }),
            "HIDE_DESTROYS" => Ok(ItemFlag::HideDestroys {
                inner: ItemFlagStruct::from_raw(env, obj)?,
            }),
            "HIDE_PLACED_ON" => Ok(ItemFlag::HidePlacedOn {
                inner: ItemFlagStruct::from_raw(env, obj)?,
            }),
            "HIDE_POTION_EFFECTS" => Ok(ItemFlag::HidePotionEffects {
                inner: ItemFlagStruct::from_raw(env, obj)?,
            }),
            "HIDE_DYE" => Ok(ItemFlag::HideDye {
                inner: ItemFlagStruct::from_raw(env, obj)?,
            }),
            "HIDE_ARMOR_TRIM" => Ok(ItemFlag::HideArmorTrim {
                inner: ItemFlagStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ItemFlagStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ItemFlag<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::HideEnchants { inner } => inner.0.clone(),
            Self::HideAttributes { inner } => inner.0.clone(),
            Self::HideUnbreakable { inner } => inner.0.clone(),
            Self::HideDestroys { inner } => inner.0.clone(),
            Self::HidePlacedOn { inner } => inner.0.clone(),
            Self::HidePotionEffects { inner } => inner.0.clone(),
            Self::HideDye { inner } => inner.0.clone(),
            Self::HideArmorTrim { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::HideEnchants { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HideAttributes { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HideUnbreakable { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HideDestroys { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HidePlacedOn { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HidePotionEffects { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HideDye { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::HideArmorTrim { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ItemFlag<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
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
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "HIDE_ENCHANTS" => Ok(ItemFlag::HideEnchants {
                    inner: ItemFlagStruct::from_raw(env, obj)?,
                }),
                "HIDE_ATTRIBUTES" => Ok(ItemFlag::HideAttributes {
                    inner: ItemFlagStruct::from_raw(env, obj)?,
                }),
                "HIDE_UNBREAKABLE" => Ok(ItemFlag::HideUnbreakable {
                    inner: ItemFlagStruct::from_raw(env, obj)?,
                }),
                "HIDE_DESTROYS" => Ok(ItemFlag::HideDestroys {
                    inner: ItemFlagStruct::from_raw(env, obj)?,
                }),
                "HIDE_PLACED_ON" => Ok(ItemFlag::HidePlacedOn {
                    inner: ItemFlagStruct::from_raw(env, obj)?,
                }),
                "HIDE_POTION_EFFECTS" => Ok(ItemFlag::HidePotionEffects {
                    inner: ItemFlagStruct::from_raw(env, obj)?,
                }),
                "HIDE_DYE" => Ok(ItemFlag::HideDye {
                    inner: ItemFlagStruct::from_raw(env, obj)?,
                }),
                "HIDE_ARMOR_TRIM" => Ok(ItemFlag::HideArmorTrim {
                    inner: ItemFlagStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ItemFlagStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ItemFlagStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ItemFlagStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ItemFlag")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemFlagStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ItemFlagStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum MainHand<'mc> {
    Left { inner: MainHandStruct<'mc> },
    Right { inner: MainHandStruct<'mc> },
}
impl<'mc> std::fmt::Display for MainHand<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainHand::Left { .. } => f.write_str("LEFT"),
            MainHand::Right { .. } => f.write_str("RIGHT"),
        }
    }
}

impl<'mc> MainHand<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<MainHand<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/inventory/MainHand");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/inventory/MainHand;",
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
            "LEFT" => Ok(MainHand::Left {
                inner: MainHandStruct::from_raw(env, obj)?,
            }),
            "RIGHT" => Ok(MainHand::Right {
                inner: MainHandStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct MainHandStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MainHand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Left { inner } => inner.0.clone(),
            Self::Right { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Left { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Right { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MainHand<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
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
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "LEFT" => Ok(MainHand::Left {
                    inner: MainHandStruct::from_raw(env, obj)?,
                }),
                "RIGHT" => Ok(MainHand::Right {
                    inner: MainHandStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for MainHandStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MainHandStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MainHandStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/MainHand")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MainHandStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MainHandStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Interface to the various inventories. Behavior relating to <a href="../Material.html#AIR"><code>Material.AIR</code></a> is unspecified.
///
/// <b>Note that whilst <a href="#iterator()"><code>iterator()</code></a> deals with the entire inventory, add / contains / remove methods deal only with the storage contents.</b>
///
/// <b>Consider using <a href="#getContents()"><code>getContents()</code></a> and <a href="#getStorageContents()"><code>getStorageContents()</code></a> for specific iteration.</b>
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Inventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Inventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Inventory<'mc> {
    fn from_raw(
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
}

impl<'mc> Inventory<'mc> {
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the ItemStack found in the slot at the given index
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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

    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn holder(
        &self,
    ) -> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/InventoryHolder;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHolder", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::InventoryHolder::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxStackSize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// This method allows you to change the maximum stack size for an inventory.
    /// <p><b>Caveats:</b></p>
    /// <ul>
    /// <li>Not all inventories respect this value.</li>
    /// <li>Stacks larger than 127 may be clipped when the world is saved.</li>
    /// <li>This value is not guaranteed to be preserved; be sure to set it before every time you want to set a slot over the max stack size.</li>
    /// <li>Stacks larger than the default max size for this type of inventory may not display correctly in the client.</li>
    /// </ul>
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxStackSize",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }

    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }

    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;I)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAtLeast",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "firstEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Clears out a particular slot in the index.
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns an iterator starting at the given index. If the index is positive, then the first call to next() will return the item at that index; if it is negative, the first call to previous will return the item at index (getSize() + index).
    pub fn iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a choice that will be valid only one of the stacks is exactly matched (aside from stack size).
///
/// <b>Only valid for shaped recipes</b>
#[repr(C)]
pub struct RecipeChoiceExactChoice<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for RecipeChoiceExactChoice<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RecipeChoiceExactChoice<'mc> {
    fn from_raw(
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
            env.validate_name(&obj, "org/bukkit/inventory/RecipeChoice$ExactChoice")?;
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
}

impl<'mc> RecipeChoiceExactChoice<'mc> {
    pub fn new_with_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: Vec<impl Into<crate::inventory::RecipeChoiceExactChoice<'mc>>>,
    ) -> Result<crate::inventory::RecipeChoiceExactChoice<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/List;";
        let raw_val_1 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            sig += "Lorg/bukkit/inventory/crate::inventory::RecipeChoiceExactChoice;";
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            jni.call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/RecipeChoice$ExactChoice");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::RecipeChoiceExactChoice::from_raw(&jni, res)
    }
    pub fn new_with_item_stacks(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::inventory::RecipeChoiceExactChoice<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[Lorg/bukkit/inventory/ItemStack;";
        let arr = jni.new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = jni.translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/RecipeChoice$ExactChoice");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::RecipeChoiceExactChoice::from_raw(&jni, res)
    }

    pub fn item_stack(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemStack", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn choices(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChoices", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
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

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn test_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "test", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> std::string::ToString for RecipeChoiceExactChoice<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling RecipeChoiceExactChoice.toString: {}", err),
        }
    }
}

/// Interface to the inventory of a Lectern.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct LecternInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LecternInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LecternInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> LecternInventory<'mc> {
    pub fn holder(
        &self,
    ) -> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/InventoryHolder;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHolder", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::InventoryHolder::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = LecternInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_item(arg0)
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = LecternInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = LecternInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_item(arg0, arg1)
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = LecternInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = LecternInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = LecternInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = LecternInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = LecternInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = LecternInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = LecternInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = LecternInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = LecternInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }
    // SUPER CLASS: Iterable

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for LecternInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LecternInventory into crate::inventory::Inventory")
    }
}
/// Interface to the inventory of a Brewing Stand.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct BrewerInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BrewerInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BrewerInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> BrewerInventory<'mc> {
    pub fn set_fuel(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn fuel(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFuel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn holder(
        &self,
    ) -> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/InventoryHolder;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHolder", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::InventoryHolder::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn ingredient(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getIngredient", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_ingredient(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setIngredient",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = BrewerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_item(arg0)
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BrewerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BrewerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_item(arg0, arg1)
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BrewerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BrewerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BrewerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BrewerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BrewerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BrewerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BrewerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BrewerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BrewerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }
    // SUPER CLASS: Iterable

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for BrewerInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BrewerInventory into crate::inventory::Inventory")
    }
}
/// Interface to the inventory of a Loom.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct LoomInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LoomInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LoomInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> LoomInventory<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for LoomInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LoomInventory into crate::inventory::Inventory")
    }
}
/// Represents a shaped (ie normal) crafting recipe.
#[repr(C)]
pub struct ShapedRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ShapedRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ShapedRecipe<'mc> {
    fn from_raw(
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
}

impl<'mc> ShapedRecipe<'mc> {
    pub fn new_with_namespaced_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
        arg1: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::inventory::ShapedRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/NamespacedKey;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/inventory/ItemStack;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/ShapedRecipe");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::ShapedRecipe::from_raw(&jni, res)
    }

    pub fn shape_with_strings(
        &self,
        arg0: std::option::Option<Vec<String>>,
    ) -> Result<crate::inventory::ShapedRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[Ljava/lang/String;";
            let arr = self.jni_ref().new_object_array(
                a.len() as i32,
                "java/lang/String",
                jni::objects::JObject::null(),
            );
            let arr = self.jni_ref().translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                    self.jni_ref().new_string(a.get(i).unwrap().clone())?,
                ));
                self.jni_ref()
                    .set_object_array_element(&arr, i as i32, val_1.l()?)?;
            }
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
        sig += ")Lorg/bukkit/inventory/ShapedRecipe;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getShape", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapedRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_ingredient_with_char(
        &self,
        arg0: u16,
        arg1: impl Into<crate::Material<'mc>>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::inventory::ShapedRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += "Lorg/bukkit/Material;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")Lorg/bukkit/inventory/ShapedRecipe;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setIngredient", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapedRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn ingredient_map(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getIngredientMap",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn choice_map(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChoiceMap", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: CraftingRecipe
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CraftingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Recipe = temp_clone.into();
        real.result()
    }
    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CraftingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Keyed = temp_clone.into();
        real.key()
    }
    pub fn category(
        &self,
    ) -> Result<crate::inventory::recipe::CraftingBookCategory<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::inventory::CraftingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CraftingRecipe = temp_clone.into();
        real.category()
    }
    pub fn group(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CraftingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CraftingRecipe = temp_clone.into();
        real.group()
    }
    pub fn set_group(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CraftingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CraftingRecipe = temp_clone.into();
        real.set_group(arg0)
    }
    pub fn set_category(
        &self,
        arg0: impl Into<crate::inventory::recipe::CraftingBookCategory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CraftingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CraftingRecipe = temp_clone.into();
        real.set_category(arg0)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::CraftingRecipe<'mc>> for ShapedRecipe<'mc> {
    fn into(self) -> crate::inventory::CraftingRecipe<'mc> {
        crate::inventory::CraftingRecipe::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ShapedRecipe into crate::inventory::CraftingRecipe")
    }
}
/// Represents a smithing transform recipe.
#[repr(C)]
pub struct SmithingTransformRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SmithingTransformRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SmithingTransformRecipe<'mc> {
    fn from_raw(
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
}

impl<'mc> SmithingTransformRecipe<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<crate::inventory::RecipeChoice<'mc>>,
        arg3: impl Into<crate::inventory::RecipeChoice<'mc>>,
        arg4: impl Into<crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::SmithingTransformRecipe<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/RecipeChoice;Lorg/bukkit/inventory/RecipeChoice;Lorg/bukkit/inventory/RecipeChoice;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg4.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/inventory/SmithingTransformRecipe");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::SmithingTransformRecipe::from_raw(&jni, res)
    }

    pub fn template(
        &self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/RecipeChoice;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTemplate", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: SmithingRecipe
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::SmithingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Recipe = temp_clone.into();
        real.result()
    }
    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::SmithingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Keyed = temp_clone.into();
        real.key()
    }
    pub fn base(&self) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::SmithingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::SmithingRecipe = temp_clone.into();
        real.base()
    }
    pub fn addition(
        &self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::SmithingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::SmithingRecipe = temp_clone.into();
        real.addition()
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::SmithingRecipe<'mc>> for SmithingTransformRecipe<'mc> {
    fn into(self) -> crate::inventory::SmithingRecipe<'mc> {
        crate::inventory::SmithingRecipe::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting SmithingTransformRecipe into crate::inventory::SmithingRecipe",
        )
    }
}
pub enum EquipmentSlot<'mc> {
    Hand { inner: EquipmentSlotStruct<'mc> },
    OffHand { inner: EquipmentSlotStruct<'mc> },
    Feet { inner: EquipmentSlotStruct<'mc> },
    Legs { inner: EquipmentSlotStruct<'mc> },
    Chest { inner: EquipmentSlotStruct<'mc> },
    Head { inner: EquipmentSlotStruct<'mc> },
}
impl<'mc> std::fmt::Display for EquipmentSlot<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EquipmentSlot::Hand { .. } => f.write_str("HAND"),
            EquipmentSlot::OffHand { .. } => f.write_str("OFF_HAND"),
            EquipmentSlot::Feet { .. } => f.write_str("FEET"),
            EquipmentSlot::Legs { .. } => f.write_str("LEGS"),
            EquipmentSlot::Chest { .. } => f.write_str("CHEST"),
            EquipmentSlot::Head { .. } => f.write_str("HEAD"),
        }
    }
}

impl<'mc> EquipmentSlot<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EquipmentSlot<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/inventory/EquipmentSlot");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/inventory/EquipmentSlot;",
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
            "HAND" => Ok(EquipmentSlot::Hand {
                inner: EquipmentSlotStruct::from_raw(env, obj)?,
            }),
            "OFF_HAND" => Ok(EquipmentSlot::OffHand {
                inner: EquipmentSlotStruct::from_raw(env, obj)?,
            }),
            "FEET" => Ok(EquipmentSlot::Feet {
                inner: EquipmentSlotStruct::from_raw(env, obj)?,
            }),
            "LEGS" => Ok(EquipmentSlot::Legs {
                inner: EquipmentSlotStruct::from_raw(env, obj)?,
            }),
            "CHEST" => Ok(EquipmentSlot::Chest {
                inner: EquipmentSlotStruct::from_raw(env, obj)?,
            }),
            "HEAD" => Ok(EquipmentSlot::Head {
                inner: EquipmentSlotStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EquipmentSlotStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EquipmentSlot<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Hand { inner } => inner.0.clone(),
            Self::OffHand { inner } => inner.0.clone(),
            Self::Feet { inner } => inner.0.clone(),
            Self::Legs { inner } => inner.0.clone(),
            Self::Chest { inner } => inner.0.clone(),
            Self::Head { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Hand { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::OffHand { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Feet { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Legs { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Chest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Head { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EquipmentSlot<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
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
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "HAND" => Ok(EquipmentSlot::Hand {
                    inner: EquipmentSlotStruct::from_raw(env, obj)?,
                }),
                "OFF_HAND" => Ok(EquipmentSlot::OffHand {
                    inner: EquipmentSlotStruct::from_raw(env, obj)?,
                }),
                "FEET" => Ok(EquipmentSlot::Feet {
                    inner: EquipmentSlotStruct::from_raw(env, obj)?,
                }),
                "LEGS" => Ok(EquipmentSlot::Legs {
                    inner: EquipmentSlotStruct::from_raw(env, obj)?,
                }),
                "CHEST" => Ok(EquipmentSlot::Chest {
                    inner: EquipmentSlotStruct::from_raw(env, obj)?,
                }),
                "HEAD" => Ok(EquipmentSlot::Head {
                    inner: EquipmentSlotStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EquipmentSlotStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EquipmentSlotStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EquipmentSlotStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/EquipmentSlot")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EquipmentSlotStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EquipmentSlotStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a merchant. A merchant is a special type of inventory which can facilitate custom trades between items.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Merchant<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Merchant<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Merchant<'mc> {
    fn from_raw(
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
}

impl<'mc> Merchant<'mc> {
    pub fn recipes(
        &self,
    ) -> Result<Vec<crate::inventory::MerchantRecipe<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRecipes", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::MerchantRecipe::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn set_recipes(
        &self,
        arg0: Vec<impl Into<crate::inventory::MerchantRecipe<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)V");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
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
            "setRecipes",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the recipe at a certain index of this merchant's trade list.
    pub fn get_recipe(
        &self,
        arg0: i32,
    ) -> Result<crate::inventory::MerchantRecipe<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/MerchantRecipe;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRecipe",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::MerchantRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_recipe(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::MerchantRecipe<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/inventory/MerchantRecipe;)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRecipe",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn recipe_count(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRecipeCount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_trading(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isTrading", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn trader(
        &self,
    ) -> Result<Option<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTrader", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::HumanEntity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// An instance of the ItemFactory can be obtained with <a href="../Server.html#getItemFactory()"><code>Server.getItemFactory()</code></a>.
/// <p>The ItemFactory is solely responsible for creating item meta containers to apply on item stacks.</p>
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct ItemFactory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ItemFactory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ItemFactory<'mc> {
    fn from_raw(
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
}

impl<'mc> ItemFactory<'mc> {
    pub fn get_item_meta(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<crate::inventory::meta::ItemMeta<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Material;)Lorg/bukkit/inventory/meta/ItemMeta;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemMeta",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn as_meta_for_with_item_meta(
        &self,
        arg0: impl Into<crate::inventory::meta::ItemMeta<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::inventory::meta::ItemMeta<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/meta/ItemMeta;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")Lorg/bukkit/inventory/meta/ItemMeta;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asMetaFor", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_applicable_with_item_meta(
        &self,
        arg0: impl Into<crate::inventory::meta::ItemMeta<'mc>>,
        arg1: impl Into<crate::Material<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/meta/ItemMeta;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/Material;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isApplicable", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn update_material(
        &self,
        arg0: impl Into<crate::inventory::meta::ItemMeta<'mc>>,
        arg1: impl Into<crate::Material<'mc>>,
    ) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/inventory/meta/ItemMeta;Lorg/bukkit/Material;)Lorg/bukkit/Material;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "updateMaterial",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn default_leather_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultLeatherColor",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn create_item_stack(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createItemStack",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_spawn_egg(
        &self,
        arg0: impl Into<crate::entity::EntityType<'mc>>,
    ) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/EntityType;)Lorg/bukkit/Material;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnEgg",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn equals(
        &self,
        arg0: impl Into<crate::inventory::meta::ItemMeta<'mc>>,
        arg1: impl Into<crate::inventory::meta::ItemMeta<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/inventory/meta/ItemMeta;Lorg/bukkit/inventory/meta/ItemMeta;)Z",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
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
/// An interface to a creatures inventory
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct EntityEquipment<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityEquipment<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityEquipment<'mc> {
    fn from_raw(
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
}

impl<'mc> EntityEquipment<'mc> {
    pub fn get_item(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/inventory/EquipmentSlot;)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setItem", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn holder(&self) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHolder", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    #[deprecated]

    pub fn item_in_hand(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemInHand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item_in_hand(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInHand",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn boots(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBoots", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_boots_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setBoots", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn leggings(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLeggings", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_leggings_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setLeggings", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn chestplate(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChestplate", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_chestplate_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setChestplate", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn helmet(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHelmet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_helmet_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setHelmet", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_in_main_hand(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemInMainHand",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item_in_main_hand_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setItemInMainHand", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_in_off_hand(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemInOffHand",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item_in_off_hand_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setItemInOffHand", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn armor_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getArmorContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }

    pub fn set_armor_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setArmorContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn item_in_hand_drop_chance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemInHandDropChance",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    #[deprecated = "entities can duel wield now use the methods for the specific hand instead "]

    pub fn set_item_in_hand_drop_chance(
        &self,
        arg0: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInHandDropChance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_in_main_hand_drop_chance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemInMainHandDropChance",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the chance of the item this creature is currently holding in their main hand being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_item_in_main_hand_drop_chance(
        &self,
        arg0: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInMainHandDropChance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_in_off_hand_drop_chance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemInOffHandDropChance",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the chance of the off hand item being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_item_in_off_hand_drop_chance(
        &self,
        arg0: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInOffHandDropChance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn helmet_drop_chance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHelmetDropChance",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the chance of the helmet being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_helmet_drop_chance(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHelmetDropChance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn chestplate_drop_chance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getChestplateDropChance",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the chance of the chest plate being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_chestplate_drop_chance(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setChestplateDropChance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn leggings_drop_chance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLeggingsDropChance",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the chance of the leggings being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_leggings_drop_chance(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLeggingsDropChance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn boots_drop_chance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBootsDropChance",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the chance of the boots being dropped upon this creature's death.
    /// <ul>
    /// <li>A drop chance of 0.0F will never drop</li>
    /// <li>A drop chance of 1.0F will always drop</li>
    /// </ul>
    pub fn set_boots_drop_chance(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBootsDropChance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Interface to the inventory of a chiseled bookshelf.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct ChiseledBookshelfInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ChiseledBookshelfInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ChiseledBookshelfInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> ChiseledBookshelfInventory<'mc> {
    pub fn holder(
        &self,
    ) -> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/InventoryHolder;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHolder", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::InventoryHolder::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = ChiseledBookshelfInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_item(arg0)
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ChiseledBookshelfInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ChiseledBookshelfInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_item(arg0, arg1)
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = ChiseledBookshelfInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ChiseledBookshelfInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ChiseledBookshelfInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = ChiseledBookshelfInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = ChiseledBookshelfInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = ChiseledBookshelfInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ChiseledBookshelfInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ChiseledBookshelfInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = ChiseledBookshelfInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }
    // SUPER CLASS: Iterable

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for ChiseledBookshelfInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ChiseledBookshelfInventory into crate::inventory::Inventory")
    }
}
/// Represents a shapeless recipe, where the arrangement of the ingredients on the crafting grid does not matter.
#[repr(C)]
pub struct ShapelessRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ShapelessRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ShapelessRecipe<'mc> {
    fn from_raw(
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
}

impl<'mc> ShapelessRecipe<'mc> {
    pub fn new_with_namespaced_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
        arg1: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/NamespacedKey;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/inventory/ItemStack;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/ShapelessRecipe");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::ShapelessRecipe::from_raw(&jni, res)
    }

    pub fn add_ingredient_with_material_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/material/MaterialData;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Lorg/bukkit/inventory/ShapelessRecipe;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addIngredient", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapelessRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_ingredient_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/inventory/ShapelessRecipe;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addIngredient", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapelessRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_ingredient_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/Material;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")Lorg/bukkit/inventory/ShapelessRecipe;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addIngredient", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapelessRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn remove_ingredient_with_recipe_choice(
        &self,
        arg0: impl Into<crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/RecipeChoice;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Lorg/bukkit/inventory/ShapelessRecipe;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeIngredient", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapelessRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn remove_ingredient_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/inventory/ShapelessRecipe;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeIngredient", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapelessRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn remove_ingredient_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/Material;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")Lorg/bukkit/inventory/ShapelessRecipe;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeIngredient", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ShapelessRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn ingredient_list(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getIngredientList",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn choice_list(
        &self,
    ) -> Result<Vec<crate::inventory::RecipeChoice<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChoiceList", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::RecipeChoice::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    // SUPER CLASS: CraftingRecipe
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CraftingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Recipe = temp_clone.into();
        real.result()
    }
    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CraftingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Keyed = temp_clone.into();
        real.key()
    }
    pub fn category(
        &self,
    ) -> Result<crate::inventory::recipe::CraftingBookCategory<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::inventory::CraftingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CraftingRecipe = temp_clone.into();
        real.category()
    }
    pub fn group(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CraftingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CraftingRecipe = temp_clone.into();
        real.group()
    }
    pub fn set_group(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CraftingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CraftingRecipe = temp_clone.into();
        real.set_group(arg0)
    }
    pub fn set_category(
        &self,
        arg0: impl Into<crate::inventory::recipe::CraftingBookCategory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::CraftingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::CraftingRecipe = temp_clone.into();
        real.set_category(arg0)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::CraftingRecipe<'mc>> for ShapelessRecipe<'mc> {
    fn into(self) -> crate::inventory::CraftingRecipe<'mc> {
        crate::inventory::CraftingRecipe::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ShapelessRecipe into crate::inventory::CraftingRecipe")
    }
}
/// Interface to the inventory of an Anvil.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct AnvilInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for AnvilInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AnvilInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> AnvilInventory<'mc> {
    pub fn rename_text(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRenameText", sig.as_str(), vec![]);
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

    pub fn repair_cost_amount(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRepairCostAmount",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the item cost (in amount) to complete the current repair.
    pub fn set_repair_cost_amount(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRepairCostAmount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn repair_cost(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRepairCost", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the item cost (in amount) to complete the current repair.
    /// Set the experience cost (in levels) to complete the current repair.
    pub fn set_repair_cost(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRepairCost",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn maximum_repair_cost(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaximumRepairCost",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the maximum experience cost (in levels) to be allowed by the current repair. The default value set by vanilla Minecraft is 40.
    pub fn set_maximum_repair_cost(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaximumRepairCost",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = AnvilInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_item(arg0)
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = AnvilInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = AnvilInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_item(arg0, arg1)
    }
    pub fn holder(
        &self,
    ) -> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = AnvilInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.holder()
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = AnvilInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = AnvilInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = AnvilInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AnvilInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = AnvilInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AnvilInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = AnvilInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = AnvilInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = AnvilInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }
    // SUPER CLASS: Iterable

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for AnvilInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting AnvilInventory into crate::inventory::Inventory")
    }
}
/// Represents a potential item match within a recipe. All choices within a recipe must be satisfied for it to be craftable. <b>This class is not legal for implementation by plugins!</b>
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct RecipeChoice<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for RecipeChoice<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RecipeChoice<'mc> {
    fn from_raw(
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
}

impl<'mc> RecipeChoice<'mc> {
    pub fn item_stack(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemStack", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn clone(&self) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/RecipeChoice;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn test_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "test", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn or(
        &self,
        arg0: impl Into<blackboxmc_java::util::function::JavaPredicate<'mc>>,
    ) -> Result<blackboxmc_java::util::function::JavaPredicate<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = RecipeChoice::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: blackboxmc_java::util::function::JavaPredicate = temp_clone.into();
        real.or(arg0)
    }
    pub fn is_equal(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<blackboxmc_java::util::function::JavaPredicate<'mc>, Box<dyn std::error::Error>>
    {
        blackboxmc_java::util::function::JavaPredicate::is_equal(jni, arg0)
    }
    pub fn negate(
        &self,
    ) -> Result<blackboxmc_java::util::function::JavaPredicate<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = RecipeChoice::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: blackboxmc_java::util::function::JavaPredicate = temp_clone.into();
        real.negate()
    }
    pub fn and(
        &self,
        arg0: impl Into<blackboxmc_java::util::function::JavaPredicate<'mc>>,
    ) -> Result<blackboxmc_java::util::function::JavaPredicate<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = RecipeChoice::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: blackboxmc_java::util::function::JavaPredicate = temp_clone.into();
        real.and(arg0)
    }
    pub fn not(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<blackboxmc_java::util::function::JavaPredicate<'mc>>,
    ) -> Result<blackboxmc_java::util::function::JavaPredicate<'mc>, Box<dyn std::error::Error>>
    {
        blackboxmc_java::util::function::JavaPredicate::not(jni, arg0)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<blackboxmc_java::util::function::JavaPredicate<'mc>> for RecipeChoice<'mc> {
    fn into(self) -> blackboxmc_java::util::function::JavaPredicate<'mc> {
        blackboxmc_java::util::function::JavaPredicate::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting RecipeChoice into blackboxmc_java::util::function::JavaPredicate",
        )
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct InventoryHolder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InventoryHolder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryHolder<'mc> {
    fn from_raw(
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
}

impl<'mc> InventoryHolder<'mc> {
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum Property<'mc> {
    BrewTime { inner: PropertyStruct<'mc> },
    FuelTime { inner: PropertyStruct<'mc> },
    BurnTime { inner: PropertyStruct<'mc> },
    TicksForCurrentFuel { inner: PropertyStruct<'mc> },
    CookTime { inner: PropertyStruct<'mc> },
    TicksForCurrentSmelting { inner: PropertyStruct<'mc> },
    EnchantButton1 { inner: PropertyStruct<'mc> },
    EnchantButton2 { inner: PropertyStruct<'mc> },
    EnchantButton3 { inner: PropertyStruct<'mc> },
    EnchantXpSeed { inner: PropertyStruct<'mc> },
    EnchantId1 { inner: PropertyStruct<'mc> },
    EnchantId2 { inner: PropertyStruct<'mc> },
    EnchantId3 { inner: PropertyStruct<'mc> },
    EnchantLevel1 { inner: PropertyStruct<'mc> },
    EnchantLevel2 { inner: PropertyStruct<'mc> },
    EnchantLevel3 { inner: PropertyStruct<'mc> },
    Levels { inner: PropertyStruct<'mc> },
    PrimaryEffect { inner: PropertyStruct<'mc> },
    SecondaryEffect { inner: PropertyStruct<'mc> },
    RepairCost { inner: PropertyStruct<'mc> },
    BookPage { inner: PropertyStruct<'mc> },
}
impl<'mc> std::fmt::Display for Property<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::BrewTime { .. } => f.write_str("BREW_TIME"),
            Property::FuelTime { .. } => f.write_str("FUEL_TIME"),
            Property::BurnTime { .. } => f.write_str("BURN_TIME"),
            Property::TicksForCurrentFuel { .. } => f.write_str("TICKS_FOR_CURRENT_FUEL"),
            Property::CookTime { .. } => f.write_str("COOK_TIME"),
            Property::TicksForCurrentSmelting { .. } => f.write_str("TICKS_FOR_CURRENT_SMELTING"),
            Property::EnchantButton1 { .. } => f.write_str("ENCHANT_BUTTON1"),
            Property::EnchantButton2 { .. } => f.write_str("ENCHANT_BUTTON2"),
            Property::EnchantButton3 { .. } => f.write_str("ENCHANT_BUTTON3"),
            Property::EnchantXpSeed { .. } => f.write_str("ENCHANT_XP_SEED"),
            Property::EnchantId1 { .. } => f.write_str("ENCHANT_ID1"),
            Property::EnchantId2 { .. } => f.write_str("ENCHANT_ID2"),
            Property::EnchantId3 { .. } => f.write_str("ENCHANT_ID3"),
            Property::EnchantLevel1 { .. } => f.write_str("ENCHANT_LEVEL1"),
            Property::EnchantLevel2 { .. } => f.write_str("ENCHANT_LEVEL2"),
            Property::EnchantLevel3 { .. } => f.write_str("ENCHANT_LEVEL3"),
            Property::Levels { .. } => f.write_str("LEVELS"),
            Property::PrimaryEffect { .. } => f.write_str("PRIMARY_EFFECT"),
            Property::SecondaryEffect { .. } => f.write_str("SECONDARY_EFFECT"),
            Property::RepairCost { .. } => f.write_str("REPAIR_COST"),
            Property::BookPage { .. } => f.write_str("BOOK_PAGE"),
        }
    }
}

impl<'mc> Property<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Property<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/inventory/Property");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/inventory/Property;",
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
            "BREW_TIME" => Ok(Property::BrewTime {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "FUEL_TIME" => Ok(Property::FuelTime {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "BURN_TIME" => Ok(Property::BurnTime {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "TICKS_FOR_CURRENT_FUEL" => Ok(Property::TicksForCurrentFuel {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "COOK_TIME" => Ok(Property::CookTime {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "TICKS_FOR_CURRENT_SMELTING" => Ok(Property::TicksForCurrentSmelting {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_BUTTON1" => Ok(Property::EnchantButton1 {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_BUTTON2" => Ok(Property::EnchantButton2 {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_BUTTON3" => Ok(Property::EnchantButton3 {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_XP_SEED" => Ok(Property::EnchantXpSeed {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_ID1" => Ok(Property::EnchantId1 {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_ID2" => Ok(Property::EnchantId2 {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_ID3" => Ok(Property::EnchantId3 {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_LEVEL1" => Ok(Property::EnchantLevel1 {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_LEVEL2" => Ok(Property::EnchantLevel2 {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "ENCHANT_LEVEL3" => Ok(Property::EnchantLevel3 {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "LEVELS" => Ok(Property::Levels {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "PRIMARY_EFFECT" => Ok(Property::PrimaryEffect {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "SECONDARY_EFFECT" => Ok(Property::SecondaryEffect {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "REPAIR_COST" => Ok(Property::RepairCost {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),
            "BOOK_PAGE" => Ok(Property::BookPage {
                inner: PropertyStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct PropertyStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Property<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::BrewTime { inner } => inner.0.clone(),
            Self::FuelTime { inner } => inner.0.clone(),
            Self::BurnTime { inner } => inner.0.clone(),
            Self::TicksForCurrentFuel { inner } => inner.0.clone(),
            Self::CookTime { inner } => inner.0.clone(),
            Self::TicksForCurrentSmelting { inner } => inner.0.clone(),
            Self::EnchantButton1 { inner } => inner.0.clone(),
            Self::EnchantButton2 { inner } => inner.0.clone(),
            Self::EnchantButton3 { inner } => inner.0.clone(),
            Self::EnchantXpSeed { inner } => inner.0.clone(),
            Self::EnchantId1 { inner } => inner.0.clone(),
            Self::EnchantId2 { inner } => inner.0.clone(),
            Self::EnchantId3 { inner } => inner.0.clone(),
            Self::EnchantLevel1 { inner } => inner.0.clone(),
            Self::EnchantLevel2 { inner } => inner.0.clone(),
            Self::EnchantLevel3 { inner } => inner.0.clone(),
            Self::Levels { inner } => inner.0.clone(),
            Self::PrimaryEffect { inner } => inner.0.clone(),
            Self::SecondaryEffect { inner } => inner.0.clone(),
            Self::RepairCost { inner } => inner.0.clone(),
            Self::BookPage { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::BrewTime { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FuelTime { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::BurnTime { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::TicksForCurrentFuel { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CookTime { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::TicksForCurrentSmelting { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantButton1 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantButton2 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantButton3 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantXpSeed { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantId1 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantId2 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantId3 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantLevel1 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantLevel2 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnchantLevel3 { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Levels { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PrimaryEffect { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SecondaryEffect { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RepairCost { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BookPage { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Property<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Property from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/Property")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Property object, got {}",
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
                "BREW_TIME" => Ok(Property::BrewTime {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "FUEL_TIME" => Ok(Property::FuelTime {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "BURN_TIME" => Ok(Property::BurnTime {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "TICKS_FOR_CURRENT_FUEL" => Ok(Property::TicksForCurrentFuel {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "COOK_TIME" => Ok(Property::CookTime {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "TICKS_FOR_CURRENT_SMELTING" => Ok(Property::TicksForCurrentSmelting {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_BUTTON1" => Ok(Property::EnchantButton1 {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_BUTTON2" => Ok(Property::EnchantButton2 {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_BUTTON3" => Ok(Property::EnchantButton3 {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_XP_SEED" => Ok(Property::EnchantXpSeed {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_ID1" => Ok(Property::EnchantId1 {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_ID2" => Ok(Property::EnchantId2 {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_ID3" => Ok(Property::EnchantId3 {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_LEVEL1" => Ok(Property::EnchantLevel1 {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_LEVEL2" => Ok(Property::EnchantLevel2 {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "ENCHANT_LEVEL3" => Ok(Property::EnchantLevel3 {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "LEVELS" => Ok(Property::Levels {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "PRIMARY_EFFECT" => Ok(Property::PrimaryEffect {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "SECONDARY_EFFECT" => Ok(Property::SecondaryEffect {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "REPAIR_COST" => Ok(Property::RepairCost {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                "BOOK_PAGE" => Ok(Property::BookPage {
                    inner: PropertyStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for PropertyStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PropertyStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PropertyStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/Property")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PropertyStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PropertyStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Interface to the inventory of an Enchantment Table.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct EnchantingInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EnchantingInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnchantingInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> EnchantingInventory<'mc> {
    pub fn item(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getItem", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn secondary(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSecondary", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_secondary(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSecondary",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = EnchantingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn holder(
        &self,
    ) -> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = EnchantingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.holder()
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = EnchantingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = EnchantingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = EnchantingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = EnchantingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = EnchantingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = EnchantingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = EnchantingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = EnchantingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = EnchantingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }
    // SUPER CLASS: Iterable

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for EnchantingInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EnchantingInventory into crate::inventory::Inventory")
    }
}
/// Represents a campfire recipe.
#[repr(C)]
pub struct BlastingRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlastingRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlastingRecipe<'mc> {
    fn from_raw(
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
}

impl<'mc> BlastingRecipe<'mc> {
    pub fn new_with_namespaced_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<crate::inventory::RecipeChoice<'mc>>,
        arg3: f32,
        arg4: i32,
    ) -> Result<crate::inventory::BlastingRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/NamespacedKey;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/inventory/RecipeChoice;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        args.push(val_3);
        sig += "F";
        let val_4 = jni::objects::JValueGen::Float(arg3);
        args.push(val_4);
        sig += "I";
        let val_5 = jni::objects::JValueGen::Int(arg4);
        args.push(val_5);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/BlastingRecipe");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::BlastingRecipe::from_raw(&jni, res)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::CookingRecipe<'mc>> for BlastingRecipe<'mc> {
    fn into(self) -> crate::inventory::CookingRecipe<'mc> {
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlastingRecipe into crate::inventory::CookingRecipe")
    }
}
/// Represents a campfire recipe.
#[repr(C)]
pub struct SmokingRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SmokingRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SmokingRecipe<'mc> {
    fn from_raw(
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
}

impl<'mc> SmokingRecipe<'mc> {
    pub fn new_with_namespaced_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<crate::inventory::RecipeChoice<'mc>>,
        arg3: f32,
        arg4: i32,
    ) -> Result<crate::inventory::SmokingRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/NamespacedKey;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/inventory/RecipeChoice;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        args.push(val_3);
        sig += "F";
        let val_4 = jni::objects::JValueGen::Float(arg3);
        args.push(val_4);
        sig += "I";
        let val_5 = jni::objects::JValueGen::Int(arg4);
        args.push(val_5);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/SmokingRecipe");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::SmokingRecipe::from_raw(&jni, res)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::CookingRecipe<'mc>> for SmokingRecipe<'mc> {
    fn into(self) -> crate::inventory::CookingRecipe<'mc> {
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SmokingRecipe into crate::inventory::CookingRecipe")
    }
}
/// Interface to the crafting inventories
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct CraftingInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CraftingInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CraftingInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> CraftingInventory<'mc> {
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResult", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_result(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn recipe(
        &self,
    ) -> Result<Option<crate::inventory::Recipe<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Recipe;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRecipe", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::Recipe::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn matrix(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMatrix", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }

    pub fn set_matrix(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMatrix",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = CraftingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_item(arg0)
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CraftingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CraftingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_item(arg0, arg1)
    }
    pub fn holder(
        &self,
    ) -> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = CraftingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.holder()
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = CraftingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CraftingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CraftingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CraftingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = CraftingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CraftingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CraftingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CraftingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = CraftingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }
    // SUPER CLASS: Iterable

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for CraftingInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CraftingInventory into crate::inventory::Inventory")
    }
}
/// Represents a merchant's trade.
/// <p>Trades can take one or two ingredients, and provide one result. The ingredients' ItemStack amounts are respected in the trade.</p>
/// <p>A trade has a maximum number of uses. A <a href="../entity/Villager.html" title="interface in org.bukkit.entity"><code>Villager</code></a> may periodically replenish its trades by resetting the <a href="#getUses()"><code>uses</code></a> of its merchant recipes to <code>0</code>, allowing them to be used again.</p>
/// <p>A trade may or may not reward experience for being completed.</p>
/// <p>During trades, the <a href="MerchantRecipe.html" title="class in org.bukkit.inventory"><code>MerchantRecipe</code></a> dynamically adjusts the amount of its first ingredient based on the following criteria:</p>
/// <ul>
/// <li><a href="#getDemand()"><code>Demand</code></a>: This value is periodically updated by the villager that owns this merchant recipe based on how often the recipe has been used since it has been last restocked in relation to its <a href="#getMaxUses()"><code>maximum uses</code></a>. The amount by which the demand influences the amount of the first ingredient is scaled by the recipe's <a href="#getPriceMultiplier()"><code>price multiplier</code></a>, and can never be below zero.</li>
/// <li><a href="#getSpecialPrice()"><code>Special price</code></a>: This value is dynamically updated whenever a player starts and stops trading with a villager that owns this merchant recipe. It is based on the player's individual reputation with the villager, and the player's currently active status effects (see <a href="../potion/PotionEffectType.html#HERO_OF_THE_VILLAGE"><code>PotionEffectType.HERO_OF_THE_VILLAGE</code></a>). The influence of the player's reputation on the special price is scaled by the recipe's <a href="#getPriceMultiplier()"><code>price multiplier</code></a>.</li>
/// </ul> The adjusted amount of the first ingredient is calculated by adding up the original amount of the first ingredient, the demand scaled by the recipe's <a href="#getPriceMultiplier()"><code>price multiplier</code></a> and truncated to the next lowest integer value greater than or equal to 0, and the special price, and then constraining the resulting value between <code>1</code> and the item stack's <a href="ItemStack.html#getMaxStackSize()"><code>maximum stack size</code></a>.
#[repr(C)]
pub struct MerchantRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MerchantRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MerchantRecipe<'mc> {
    fn from_raw(
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
}

impl<'mc> MerchantRecipe<'mc> {
    pub fn new_with_item_stack(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<bool>,
        arg4: std::option::Option<i32>,
        arg5: std::option::Option<f32>,
        arg6: std::option::Option<i32>,
        arg7: std::option::Option<i32>,
    ) -> Result<crate::inventory::MerchantRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1);
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "Z";
            let val_4 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_4);
        }
        if let Some(a) = arg4 {
            sig += "I";
            let val_5 = jni::objects::JValueGen::Int(a);
            args.push(val_5);
        }
        if let Some(a) = arg5 {
            sig += "F";
            let val_6 = jni::objects::JValueGen::Float(a);
            args.push(val_6);
        }
        if let Some(a) = arg6 {
            sig += "I";
            let val_7 = jni::objects::JValueGen::Int(a);
            args.push(val_7);
        }
        if let Some(a) = arg7 {
            sig += "I";
            let val_8 = jni::objects::JValueGen::Int(a);
            args.push(val_8);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/MerchantRecipe");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::MerchantRecipe::from_raw(&jni, res)
    }

    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResult", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_ingredient(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addIngredient",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_ingredient(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIngredient",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_ingredients(
        &self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)V");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
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
            "setIngredients",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn ingredients(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getIngredients", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn adjusted_ingredient1(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAdjustedIngredient1",
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

    pub fn adjust(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "adjust",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn demand(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDemand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn price_multiplier(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPriceMultiplier",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn special_price(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpecialPrice", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the demand for this trade.
    pub fn set_demand(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDemand",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Set the special price for this trade.
    pub fn set_special_price(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpecialPrice",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn uses(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getUses", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the number of times this trade has been used.
    pub fn set_uses(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUses",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn max_uses(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxUses", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the maximum number of uses this trade has.
    pub fn set_max_uses(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxUses",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_experience_reward(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasExperienceReward",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Set whether to reward experience to the player for the trade.
    pub fn set_experience_reward(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExperienceReward",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn villager_experience(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVillagerExperience",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the amount of experience the villager earns from this trade.
    pub fn set_villager_experience(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVillagerExperience",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the price multiplier for the cost of this trade.
    pub fn set_price_multiplier(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPriceMultiplier",
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
impl<'mc> Into<crate::inventory::Recipe<'mc>> for MerchantRecipe<'mc> {
    fn into(self) -> crate::inventory::Recipe<'mc> {
        crate::inventory::Recipe::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MerchantRecipe into crate::inventory::Recipe")
    }
}
/// Represents a trading inventory between a player and a merchant.
///
/// The holder of this Inventory is the owning Villager, or null if the player is trading with a merchant created by a plugin.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct MerchantInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MerchantInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MerchantInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> MerchantInventory<'mc> {
    pub fn selected_recipe_index(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSelectedRecipeIndex",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn selected_recipe(
        &self,
    ) -> Result<Option<crate::inventory::MerchantRecipe<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/MerchantRecipe;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSelectedRecipe",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::MerchantRecipe::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn merchant(&self) -> Result<crate::inventory::Merchant<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Merchant;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMerchant", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Merchant::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = MerchantInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_item(arg0)
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = MerchantInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MerchantInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_item(arg0, arg1)
    }
    pub fn holder(
        &self,
    ) -> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = MerchantInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.holder()
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = MerchantInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MerchantInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = MerchantInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MerchantInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = MerchantInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MerchantInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = MerchantInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = MerchantInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = MerchantInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }
    // SUPER CLASS: Iterable

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for MerchantInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MerchantInventory into crate::inventory::Inventory")
    }
}
/// An interface to the inventory of a <a href="../entity/Llama.html" title="interface in org.bukkit.entity"><code>Llama</code></a>.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct LlamaInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LlamaInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LlamaInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> LlamaInventory<'mc> {
    pub fn decor(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDecor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_decor(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDecor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_saddle(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = LlamaInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::AbstractHorseInventory = temp_clone.into();
        real.set_saddle(arg0)
    }
    pub fn saddle(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = LlamaInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::AbstractHorseInventory = temp_clone.into();
        real.saddle()
    }
    // SUPER CLASS: Inventory
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_item(arg0)
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_item(arg0, arg1)
    }
    pub fn holder(
        &self,
    ) -> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.holder()
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Inventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::AbstractHorseInventory<'mc>> for LlamaInventory<'mc> {
    fn into(self) -> crate::inventory::AbstractHorseInventory<'mc> {
        crate::inventory::AbstractHorseInventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LlamaInventory into crate::inventory::AbstractHorseInventory")
    }
}
/// Interface to the inventory of a Grindstone.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct GrindstoneInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for GrindstoneInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for GrindstoneInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> GrindstoneInventory<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for GrindstoneInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting GrindstoneInventory into crate::inventory::Inventory")
    }
}
/// Represents a cooking recipe.
#[repr(C)]
pub struct CookingRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CookingRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CookingRecipe<'mc> {
    fn from_raw(
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
}

impl<'mc> CookingRecipe<'mc> {
    pub fn new_with_namespaced_key(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<crate::inventory::RecipeChoice<'mc>>,
        arg3: f32,
        arg4: i32,
    ) -> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/NamespacedKey;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/inventory/RecipeChoice;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        args.push(val_3);
        sig += "F";
        let val_4 = jni::objects::JValueGen::Float(arg3);
        args.push(val_4);
        sig += "I";
        let val_5 = jni::objects::JValueGen::Int(arg4);
        args.push(val_5);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/inventory/CookingRecipe");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::CookingRecipe::from_raw(&jni, res)
    }

    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResult", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_input(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Material;)Lorg/bukkit/inventory/CookingRecipe;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInput",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn input(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getInput", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn category(
        &self,
    ) -> Result<crate::inventory::recipe::CookingBookCategory<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/inventory/recipe/CookingBookCategory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCategory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::recipe::CookingBookCategory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_input_choice(
        &self,
        arg0: impl Into<crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/inventory/RecipeChoice;)Lorg/bukkit/inventory/CookingRecipe;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInputChoice",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn input_choice(
        &self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/RecipeChoice;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInputChoice", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn group(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_group(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGroup",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_category(
        &self,
        arg0: impl Into<crate::inventory::recipe::CookingBookCategory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/recipe/CookingBookCategory;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCategory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn experience(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getExperience", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the experience given by this recipe.
    pub fn set_experience(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExperience",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Set the cooking time for this recipe in ticks.
    pub fn set_cooking_time(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookingTime",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cooking_time(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCookingTime", sig.as_str(), vec![]);
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Recipe<'mc>> for CookingRecipe<'mc> {
    fn into(self) -> crate::inventory::Recipe<'mc> {
        crate::inventory::Recipe::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CookingRecipe into crate::inventory::Recipe")
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for CookingRecipe<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CookingRecipe into crate::Keyed")
    }
}
/// Interface to the inventory of a Stonecutter.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct StonecutterInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for StonecutterInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for StonecutterInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> StonecutterInventory<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for StonecutterInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting StonecutterInventory into crate::inventory::Inventory")
    }
}
pub enum CreativeCategory<'mc> {
    BuildingBlocks { inner: CreativeCategoryStruct<'mc> },
    Decorations { inner: CreativeCategoryStruct<'mc> },
    Redstone { inner: CreativeCategoryStruct<'mc> },
    Transportation { inner: CreativeCategoryStruct<'mc> },
    Misc { inner: CreativeCategoryStruct<'mc> },
    Food { inner: CreativeCategoryStruct<'mc> },
    Tools { inner: CreativeCategoryStruct<'mc> },
    Combat { inner: CreativeCategoryStruct<'mc> },
    Brewing { inner: CreativeCategoryStruct<'mc> },
}
impl<'mc> std::fmt::Display for CreativeCategory<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreativeCategory::BuildingBlocks { .. } => f.write_str("BUILDING_BLOCKS"),
            CreativeCategory::Decorations { .. } => f.write_str("DECORATIONS"),
            CreativeCategory::Redstone { .. } => f.write_str("REDSTONE"),
            CreativeCategory::Transportation { .. } => f.write_str("TRANSPORTATION"),
            CreativeCategory::Misc { .. } => f.write_str("MISC"),
            CreativeCategory::Food { .. } => f.write_str("FOOD"),
            CreativeCategory::Tools { .. } => f.write_str("TOOLS"),
            CreativeCategory::Combat { .. } => f.write_str("COMBAT"),
            CreativeCategory::Brewing { .. } => f.write_str("BREWING"),
        }
    }
}

impl<'mc> CreativeCategory<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<CreativeCategory<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/inventory/CreativeCategory");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/inventory/CreativeCategory;",
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
            "BUILDING_BLOCKS" => Ok(CreativeCategory::BuildingBlocks {
                inner: CreativeCategoryStruct::from_raw(env, obj)?,
            }),
            "DECORATIONS" => Ok(CreativeCategory::Decorations {
                inner: CreativeCategoryStruct::from_raw(env, obj)?,
            }),
            "REDSTONE" => Ok(CreativeCategory::Redstone {
                inner: CreativeCategoryStruct::from_raw(env, obj)?,
            }),
            "TRANSPORTATION" => Ok(CreativeCategory::Transportation {
                inner: CreativeCategoryStruct::from_raw(env, obj)?,
            }),
            "MISC" => Ok(CreativeCategory::Misc {
                inner: CreativeCategoryStruct::from_raw(env, obj)?,
            }),
            "FOOD" => Ok(CreativeCategory::Food {
                inner: CreativeCategoryStruct::from_raw(env, obj)?,
            }),
            "TOOLS" => Ok(CreativeCategory::Tools {
                inner: CreativeCategoryStruct::from_raw(env, obj)?,
            }),
            "COMBAT" => Ok(CreativeCategory::Combat {
                inner: CreativeCategoryStruct::from_raw(env, obj)?,
            }),
            "BREWING" => Ok(CreativeCategory::Brewing {
                inner: CreativeCategoryStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct CreativeCategoryStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CreativeCategory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::BuildingBlocks { inner } => inner.0.clone(),
            Self::Decorations { inner } => inner.0.clone(),
            Self::Redstone { inner } => inner.0.clone(),
            Self::Transportation { inner } => inner.0.clone(),
            Self::Misc { inner } => inner.0.clone(),
            Self::Food { inner } => inner.0.clone(),
            Self::Tools { inner } => inner.0.clone(),
            Self::Combat { inner } => inner.0.clone(),
            Self::Brewing { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::BuildingBlocks { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Decorations { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Redstone { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Transportation { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Misc { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Food { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Tools { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Combat { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Brewing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CreativeCategory<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
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
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "BUILDING_BLOCKS" => Ok(CreativeCategory::BuildingBlocks {
                    inner: CreativeCategoryStruct::from_raw(env, obj)?,
                }),
                "DECORATIONS" => Ok(CreativeCategory::Decorations {
                    inner: CreativeCategoryStruct::from_raw(env, obj)?,
                }),
                "REDSTONE" => Ok(CreativeCategory::Redstone {
                    inner: CreativeCategoryStruct::from_raw(env, obj)?,
                }),
                "TRANSPORTATION" => Ok(CreativeCategory::Transportation {
                    inner: CreativeCategoryStruct::from_raw(env, obj)?,
                }),
                "MISC" => Ok(CreativeCategory::Misc {
                    inner: CreativeCategoryStruct::from_raw(env, obj)?,
                }),
                "FOOD" => Ok(CreativeCategory::Food {
                    inner: CreativeCategoryStruct::from_raw(env, obj)?,
                }),
                "TOOLS" => Ok(CreativeCategory::Tools {
                    inner: CreativeCategoryStruct::from_raw(env, obj)?,
                }),
                "COMBAT" => Ok(CreativeCategory::Combat {
                    inner: CreativeCategoryStruct::from_raw(env, obj)?,
                }),
                "BREWING" => Ok(CreativeCategory::Brewing {
                    inner: CreativeCategoryStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for CreativeCategoryStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CreativeCategoryStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CreativeCategoryStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/CreativeCategory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CreativeCategoryStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CreativeCategoryStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Interface to the inventory of a Smithing table.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct SmithingInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SmithingInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SmithingInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> SmithingInventory<'mc> {
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResult", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_result(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn recipe(
        &self,
    ) -> Result<Option<crate::inventory::Recipe<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Recipe;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRecipe", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::Recipe::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = SmithingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_item(arg0)
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = SmithingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SmithingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_item(arg0, arg1)
    }
    pub fn holder(
        &self,
    ) -> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = SmithingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.holder()
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = SmithingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SmithingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = SmithingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SmithingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = SmithingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SmithingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = SmithingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = SmithingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = SmithingInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }
    // SUPER CLASS: Iterable

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for SmithingInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SmithingInventory into crate::inventory::Inventory")
    }
}
/// Interface to the inventory of a Player, including the four armor slots and any extra slots.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct PlayerInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PlayerInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PlayerInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> PlayerInventory<'mc> {
    pub fn get_item(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/inventory/EquipmentSlot;)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item_with_int(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setItem", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn holder(
        &self,
    ) -> Result<Option<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/HumanEntity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHolder", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::HumanEntity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    #[deprecated]

    pub fn item_in_hand(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemInHand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item_in_hand(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInHand",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn boots(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBoots", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_boots(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBoots",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn leggings(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLeggings", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_leggings(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLeggings",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn chestplate(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChestplate", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_chestplate(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setChestplate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn helmet(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHelmet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_helmet(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHelmet",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_in_main_hand(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemInMainHand",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item_in_main_hand(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInMainHand",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_in_off_hand(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemInOffHand",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item_in_off_hand(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItemInOffHand",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn armor_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getArmorContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }

    pub fn set_armor_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setArmorContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn extra_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getExtraContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }

    pub fn set_extra_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExtraContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn held_item_slot(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHeldItemSlot", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the slot number of the currently held item.
    /// <p>This validates whether the slot is between 0 and 8 inclusive.</p>
    pub fn set_held_item_slot(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHeldItemSlot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = PlayerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = PlayerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PlayerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = PlayerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = PlayerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = PlayerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = PlayerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = PlayerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = PlayerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = PlayerInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }
    // SUPER CLASS: Iterable

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for PlayerInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PlayerInventory into crate::inventory::Inventory")
    }
}
/// Interface to the inventory of a Furnace.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct FurnaceInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FurnaceInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FurnaceInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> FurnaceInventory<'mc> {
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResult", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_result(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_fuel(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn fuel(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFuel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn smelting(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSmelting", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_smelting(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSmelting",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn holder(
        &self,
    ) -> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/InventoryHolder;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHolder", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::InventoryHolder::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = FurnaceInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_item(arg0)
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = FurnaceInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FurnaceInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_item(arg0, arg1)
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = FurnaceInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FurnaceInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = FurnaceInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FurnaceInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = FurnaceInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FurnaceInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = FurnaceInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = FurnaceInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = FurnaceInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }
    // SUPER CLASS: Iterable

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for FurnaceInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FurnaceInventory into crate::inventory::Inventory")
    }
}
/// An interface to the inventory of an <a href="../entity/AbstractHorse.html" title="interface in org.bukkit.entity"><code>AbstractHorse</code></a>.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct AbstractHorseInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for AbstractHorseInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AbstractHorseInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> AbstractHorseInventory<'mc> {
    pub fn set_saddle(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSaddle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn saddle(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSaddle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = AbstractHorseInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_item(arg0)
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = AbstractHorseInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = AbstractHorseInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_item(arg0, arg1)
    }
    pub fn holder(
        &self,
    ) -> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = AbstractHorseInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.holder()
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = AbstractHorseInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = AbstractHorseInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = AbstractHorseInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AbstractHorseInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = AbstractHorseInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AbstractHorseInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = AbstractHorseInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = AbstractHorseInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = AbstractHorseInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }
    // SUPER CLASS: Iterable

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for AbstractHorseInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting AbstractHorseInventory into crate::inventory::Inventory")
    }
}
/// Interface to the inventory of a Beacon.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct BeaconInventory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BeaconInventory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BeaconInventory<'mc> {
    fn from_raw(
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
}

impl<'mc> BeaconInventory<'mc> {
    pub fn item(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getItem", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn all_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Ljava/util/HashMap;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "all", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BeaconInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.add_item(arg0)
    }
    pub fn holder(
        &self,
    ) -> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = BeaconInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.holder()
    }
    pub fn max_stack_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BeaconInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.max_stack_size()
    }
    pub fn set_max_stack_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BeaconInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.set_max_stack_size(arg0)
    }
    pub fn remove_item(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BeaconInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.remove_item(arg0)
    }
    pub fn contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn storage_contents(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/inventory/ItemStack;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStorageContents",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::ItemStack::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn set_storage_contents(
        &self,
        arg0: Vec<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemStack;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemStack",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStorageContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn contains_at_least(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BeaconInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.contains_at_least(arg0, arg1)
    }
    pub fn first_empty(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BeaconInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.first_empty()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn remove_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BeaconInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.is_empty()
    }
    pub fn iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn contains_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn contains_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "contains", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn first_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BeaconInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.location()
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BeaconInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.get_type()
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BeaconInventory::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Inventory = temp_clone.into();
        real.size()
    }
    // SUPER CLASS: Iterable

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for BeaconInventory<'mc> {
    fn into(self) -> crate::inventory::Inventory<'mc> {
        crate::inventory::Inventory::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BeaconInventory into crate::inventory::Inventory")
    }
}
/// Represents a smithing trim recipe.
#[repr(C)]
pub struct SmithingTrimRecipe<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SmithingTrimRecipe<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SmithingTrimRecipe<'mc> {
    fn from_raw(
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
}

impl<'mc> SmithingTrimRecipe<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
        arg1: impl Into<crate::inventory::RecipeChoice<'mc>>,
        arg2: impl Into<crate::inventory::RecipeChoice<'mc>>,
        arg3: impl Into<crate::inventory::RecipeChoice<'mc>>,
    ) -> Result<crate::inventory::SmithingTrimRecipe<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/RecipeChoice;Lorg/bukkit/inventory/RecipeChoice;Lorg/bukkit/inventory/RecipeChoice;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/inventory/SmithingTrimRecipe");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::SmithingTrimRecipe::from_raw(&jni, res)
    }

    pub fn template(
        &self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/RecipeChoice;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTemplate", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: Recipe
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::Recipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::Recipe = temp_clone.into();
        real.result()
    }
    // SUPER CLASS: Keyed
    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::Keyed::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Keyed = temp_clone.into();
        real.key()
    }
    // SUPER CLASS: SmithingRecipe
    pub fn base(&self) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::SmithingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::SmithingRecipe = temp_clone.into();
        real.base()
    }
    pub fn addition(
        &self,
    ) -> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::SmithingRecipe::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::SmithingRecipe = temp_clone.into();
        real.addition()
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::ComplexRecipe<'mc>> for SmithingTrimRecipe<'mc> {
    fn into(self) -> crate::inventory::ComplexRecipe<'mc> {
        crate::inventory::ComplexRecipe::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SmithingTrimRecipe into crate::inventory::ComplexRecipe")
    }
}
impl<'mc> Into<crate::inventory::SmithingRecipe<'mc>> for SmithingTrimRecipe<'mc> {
    fn into(self) -> crate::inventory::SmithingRecipe<'mc> {
        crate::inventory::SmithingRecipe::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SmithingTrimRecipe into crate::inventory::SmithingRecipe")
    }
}
pub mod meta;
pub mod recipe;
