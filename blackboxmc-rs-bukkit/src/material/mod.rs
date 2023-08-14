#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents a chest
pub struct Chest<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Chest<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Chest<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Chest from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Chest")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Chest object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,
    ) -> Result<crate::material::Chest<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/block/BlockFace;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Chest");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Chest::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Chest<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Chest");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Chest::from_raw(&jni, res)
    }
    //

    pub fn clone(&mut self) -> Result<crate::material::Chest<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/Chest;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::Chest::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::DirectionalContainer<'mc>> for Chest<'mc> {
    fn into(self) -> crate::material::DirectionalContainer<'mc> {
        crate::material::DirectionalContainer::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Chest into crate::material::DirectionalContainer")
    }
}
/// Represents a diode/repeater in the on or off state, with a delay and facing in a specific direction.
pub struct Diode<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Diode<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Diode<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Diode from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Diode")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Diode object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<crate::material::Diode<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Diode");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Diode::from_raw(&jni, res)
    }
    pub fn new_with_block_face(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: i32,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::material::Diode<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/BlockFace;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Z";
            // 2
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Diode");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Diode::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn delay(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDelay", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Sets the delay of the repeater.
    pub fn set_delay(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDelay",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Directional<'mc>> for Diode<'mc> {
    fn into(self) -> crate::material::Directional<'mc> {
        crate::material::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Diode into crate::material::Directional")
    }
}
impl<'mc> Into<crate::material::Redstone<'mc>> for Diode<'mc> {
    fn into(self) -> crate::material::Redstone<'mc> {
        crate::material::Redstone::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Diode into crate::material::Redstone")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Diode<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Diode into crate::material::MaterialData")
    }
}
/// Represents a pumpkin.
pub struct Pumpkin<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Pumpkin<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Pumpkin<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Pumpkin from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Pumpkin")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Pumpkin object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<crate::material::Pumpkin<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Pumpkin");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Pumpkin::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Pumpkin<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Pumpkin");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Pumpkin::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_lit(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLit", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Directional<'mc>> for Pumpkin<'mc> {
    fn into(self) -> crate::material::Directional<'mc> {
        crate::material::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Pumpkin into crate::material::Directional")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Pumpkin<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Pumpkin into crate::material::MaterialData")
    }
}
/// Represents a flower pot.
pub struct FlowerPot<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FlowerPot<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FlowerPot<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate FlowerPot from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/FlowerPot")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FlowerPot object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::FlowerPot<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/FlowerPot");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::FlowerPot::from_raw(&jni, res)
    }
    //

    pub fn contents(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/material/MaterialData;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_contents(
        &mut self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::FlowerPot<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/FlowerPot;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::FlowerPot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for FlowerPot<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FlowerPot into crate::material::MaterialData")
    }
}
/// Represents a powered rail
pub struct PoweredRail<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PoweredRail<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PoweredRail<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PoweredRail from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/PoweredRail")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PoweredRail object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::PoweredRail<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/PoweredRail");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::PoweredRail::from_raw(&jni, res)
    }
    //

    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set whether this PoweredRail should be powered or not.
    pub fn set_powered(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::PoweredRail<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/PoweredRail;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::PoweredRail::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDirection",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_curve(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCurve", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn direction(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDirection", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_on_slope(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnSlope", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Redstone<'mc>> for PoweredRail<'mc> {
    fn into(self) -> crate::material::Redstone<'mc> {
        crate::material::Redstone::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PoweredRail into crate::material::Redstone")
    }
}
impl<'mc> Into<crate::material::ExtendedRails<'mc>> for PoweredRail<'mc> {
    fn into(self) -> crate::material::ExtendedRails<'mc> {
        crate::material::ExtendedRails::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PoweredRail into crate::material::ExtendedRails")
    }
}
/// Represents the tripwire hook
pub struct TripwireHook<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for TripwireHook<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TripwireHook<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TripwireHook from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/TripwireHook")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TripwireHook object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::TripwireHook<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/TripwireHook");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::TripwireHook::from_raw(&jni, res)
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn attached_face(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_connected(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isConnected", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set tripwire connection state
    pub fn set_connected(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setConnected",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_activated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isActivated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set hook activated state
    pub fn set_activated(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setActivated",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::SimpleAttachableMaterialData<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/SimpleAttachableMaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Redstone<'mc>> for TripwireHook<'mc> {
    fn into(self) -> crate::material::Redstone<'mc> {
        crate::material::Redstone::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TripwireHook into crate::material::Redstone")
    }
}
impl<'mc> Into<crate::material::SimpleAttachableMaterialData<'mc>> for TripwireHook<'mc> {
    fn into(self) -> crate::material::SimpleAttachableMaterialData<'mc> {
        crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting TripwireHook into crate::material::SimpleAttachableMaterialData",
        )
    }
}
/// Simple utility class for attachable MaterialData subclasses
pub struct SimpleAttachableMaterialData<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SimpleAttachableMaterialData<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SimpleAttachableMaterialData<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate SimpleAttachableMaterialData from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/material/SimpleAttachableMaterialData")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SimpleAttachableMaterialData object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,
    ) -> Result<crate::material::SimpleAttachableMaterialData<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/block/BlockFace;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/SimpleAttachableMaterialData");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::SimpleAttachableMaterialData::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn attached_face(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Attachable<'mc>> for SimpleAttachableMaterialData<'mc> {
    fn into(self) -> crate::material::Attachable<'mc> {
        crate::material::Attachable::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting SimpleAttachableMaterialData into crate::material::Attachable",
        )
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for SimpleAttachableMaterialData<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting SimpleAttachableMaterialData into crate::material::MaterialData",
        )
    }
}

///
/// This is a representation of an abstract class.
pub struct Openable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Openable<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Openable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Openable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Openable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    /// Configure this door to be either open or closed;
    pub fn set_open(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOpen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_open(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOpen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
impl<'mc> JNIRaw<'mc> for Openable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// MaterialData for torches
pub struct Torch<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Torch<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Torch<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Torch from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Torch")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Torch object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Torch<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Torch");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Torch::from_raw(&jni, res)
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn attached_face(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::SimpleAttachableMaterialData<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/SimpleAttachableMaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::SimpleAttachableMaterialData<'mc>> for Torch<'mc> {
    fn into(self) -> crate::material::SimpleAttachableMaterialData<'mc> {
        crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Torch into crate::material::SimpleAttachableMaterialData")
    }
}
/// Represents a comparator in the on or off state, in normal or subtraction mode and facing in a specific direction.
pub struct Comparator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Comparator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Comparator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Comparator from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Comparator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Comparator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<crate::material::Comparator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Comparator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Comparator::from_raw(&jni, res)
    }
    pub fn new_with_block_face(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: bool,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::material::Comparator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/BlockFace;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Z";
        // 2
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Z";
            // 2
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Comparator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Comparator::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Sets whether the comparator is in subtraction mode.
    pub fn set_subtraction_mode(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSubtractionMode",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_subtraction_mode(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSubtractionMode",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_being_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isBeingPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Directional<'mc>> for Comparator<'mc> {
    fn into(self) -> crate::material::Directional<'mc> {
        crate::material::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Comparator into crate::material::Directional")
    }
}
impl<'mc> Into<crate::material::Redstone<'mc>> for Comparator<'mc> {
    fn into(self) -> crate::material::Redstone<'mc> {
        crate::material::Redstone::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Comparator into crate::material::Redstone")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Comparator<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Comparator into crate::material::MaterialData")
    }
}
/// Represents a button
pub struct Button<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Button<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Button<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Button from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Button")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Button object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Button<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Button");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Button::from_raw(&jni, res)
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn attached_face(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Sets the current state of this button
    pub fn set_powered(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::Button<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/Button;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::Button::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Redstone<'mc>> for Button<'mc> {
    fn into(self) -> crate::material::Redstone<'mc> {
        crate::material::Redstone::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Button into crate::material::Redstone")
    }
}
impl<'mc> Into<crate::material::SimpleAttachableMaterialData<'mc>> for Button<'mc> {
    fn into(self) -> crate::material::SimpleAttachableMaterialData<'mc> {
        crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Button into crate::material::SimpleAttachableMaterialData")
    }
}
/// Represents the different types of coals.
pub struct Coal<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Coal<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Coal<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Coal from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Coal")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Coal object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::CoalType<'mc>>>,
    ) -> Result<crate::material::Coal<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/CoalType;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Coal");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Coal::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Coal<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Coal");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Coal::from_raw(&jni, res)
    }
    //

    pub fn set_type(
        &mut self,
        arg0: impl Into<crate::CoalType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/CoalType;)V");
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
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_type(&mut self) -> Result<crate::CoalType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/CoalType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::CoalType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::CoalType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Coal<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Coal into crate::material::MaterialData")
    }
}
/// An object that can be colored.
///
/// This is a representation of an abstract class.
pub struct Colorable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Colorable<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Colorable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Colorable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Colorable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/DyeColor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn color(&mut self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/DyeColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::DyeColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::DyeColor::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
impl<'mc> JNIRaw<'mc> for Colorable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents the different types of Tree block that face a direction.
pub struct Tree<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Tree<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Tree<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Tree from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Tree")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Tree object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<crate::material::Tree<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Tree");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Tree::from_raw(&jni, res)
    }
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<impl Into<crate::TreeSpecies<'mc>>>,
        arg2: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,
    ) -> Result<crate::material::Tree<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/TreeSpecies;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/block/BlockFace;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Tree");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Tree::from_raw(&jni, res)
    }
    //

    pub fn direction(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDirection", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::Wood<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/Wood;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::Wood::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_species(
        &mut self,
        arg0: impl Into<crate::TreeSpecies<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/TreeSpecies;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpecies",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn species(&mut self) -> Result<crate::TreeSpecies<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/TreeSpecies;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpecies", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::TreeSpecies::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::TreeSpecies::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Wood<'mc>> for Tree<'mc> {
    fn into(self) -> crate::material::Wood<'mc> {
        crate::material::Wood::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Tree into crate::material::Wood")
    }
}
/// Material data for the piston base block
pub struct PistonBaseMaterial<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PistonBaseMaterial<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PistonBaseMaterial<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PistonBaseMaterial from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/PistonBaseMaterial")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PistonBaseMaterial object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::PistonBaseMaterial<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/PistonBaseMaterial");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::PistonBaseMaterial::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Sets the current state of this piston
    pub fn set_powered(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_sticky(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSticky", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::PistonBaseMaterial<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/PistonBaseMaterial;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::PistonBaseMaterial::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Directional<'mc>> for PistonBaseMaterial<'mc> {
    fn into(self) -> crate::material::Directional<'mc> {
        crate::material::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PistonBaseMaterial into crate::material::Directional")
    }
}
impl<'mc> Into<crate::material::Redstone<'mc>> for PistonBaseMaterial<'mc> {
    fn into(self) -> crate::material::Redstone<'mc> {
        crate::material::Redstone::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PistonBaseMaterial into crate::material::Redstone")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for PistonBaseMaterial<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PistonBaseMaterial into crate::material::MaterialData")
    }
}
/// Represents a trap door
pub struct TrapDoor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for TrapDoor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TrapDoor<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TrapDoor from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/TrapDoor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TrapDoor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::TrapDoor<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/TrapDoor");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::TrapDoor::from_raw(&jni, res)
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn attached_face(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Openable.html#setOpen(boolean)">Openable</a></code></span>
    /// Configure this door to be either open or closed;
    pub fn set_open(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOpen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_inverted(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInverted", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set trapdoor inverted state
    pub fn set_inverted(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInverted",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::TrapDoor<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/TrapDoor;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::TrapDoor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_open(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOpen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Openable<'mc>> for TrapDoor<'mc> {
    fn into(self) -> crate::material::Openable<'mc> {
        crate::material::Openable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TrapDoor into crate::material::Openable")
    }
}
impl<'mc> Into<crate::material::SimpleAttachableMaterialData<'mc>> for TrapDoor<'mc> {
    fn into(self) -> crate::material::SimpleAttachableMaterialData<'mc> {
        crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TrapDoor into crate::material::SimpleAttachableMaterialData")
    }
}
/// Represents the different types of crops in different states of growth.
pub struct Crops<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Crops<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Crops<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Crops from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Crops")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Crops object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::CropState<'mc>>>,
    ) -> Result<crate::material::Crops<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/CropState;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Crops");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Crops::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Crops<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Crops");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Crops::from_raw(&jni, res)
    }
    //

    pub fn set_state(
        &mut self,
        arg0: impl Into<crate::CropState<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/CropState;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setState",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::Crops<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/Crops;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::Crops::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn state(&mut self) -> Result<crate::CropState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/CropState;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getState", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::CropState::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::CropState::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Crops<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Crops into crate::material::MaterialData")
    }
}
/// Represents a spawn egg that can be used to spawn mobs
pub struct SpawnEgg<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SpawnEgg<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SpawnEgg<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SpawnEgg from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/SpawnEgg")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpawnEgg object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i8>,
    ) -> Result<crate::material::SpawnEgg<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "B";
            let val_1 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/SpawnEgg");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::SpawnEgg::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::SpawnEgg<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/SpawnEgg");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::SpawnEgg::from_raw(&jni, res)
    }
    //

    pub fn spawned_type(
        &mut self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntityType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnedType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::EntityType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::EntityType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_spawned_type(
        &mut self,
        arg0: impl Into<crate::entity::EntityType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/EntityType;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnedType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::SpawnEgg<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/SpawnEgg;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::SpawnEgg::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for SpawnEgg<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SpawnEgg into crate::material::MaterialData")
    }
}
/// Represents the different types of monster eggs
pub struct MonsterEggs<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MonsterEggs<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MonsterEggs<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MonsterEggs from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/MonsterEggs")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MonsterEggs object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::MonsterEggs<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/MonsterEggs");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::MonsterEggs::from_raw(&jni, res)
    }
    //

    pub fn textures(&mut self) -> Result<Vec<crate::Material<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTextures", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            let variant =
                self.0
                    .call_method(list.get(i)?, "toString", "()Ljava/lang/String;", vec![]);
            let variant = self.jni_ref().translate_error(variant)?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            new_vec.push(crate::Material::from_raw(
                &self.0,
                obj,
                crate::Material::from_string(variant_str)
                    .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
            )?);
        }
        Ok(new_vec)
    }
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MonsterEggs<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MonsterEggs;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MonsterEggs::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn material(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_material(
        &mut self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Material;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaterial",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::TexturedMaterial<'mc>> for MonsterEggs<'mc> {
    fn into(self) -> crate::material::TexturedMaterial<'mc> {
        crate::material::TexturedMaterial::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MonsterEggs into crate::material::TexturedMaterial")
    }
}
/// Represents a furnace.
pub struct Furnace<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Furnace<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Furnace<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Furnace from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Furnace")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Furnace object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,
    ) -> Result<crate::material::Furnace<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/block/BlockFace;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Furnace");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Furnace::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Furnace<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Furnace");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Furnace::from_raw(&jni, res)
    }
    //

    pub fn clone(&mut self) -> Result<crate::material::Furnace<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/Furnace;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::Furnace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::FurnaceAndDispenser<'mc>> for Furnace<'mc> {
    fn into(self) -> crate::material::FurnaceAndDispenser<'mc> {
        crate::material::FurnaceAndDispenser::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Furnace into crate::material::FurnaceAndDispenser")
    }
}
/// Represents minecart rails.
pub struct Rails<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Rails<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Rails<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Rails from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Rails")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Rails object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Rails<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Rails");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Rails::from_raw(&jni, res)
    }
    //

    pub fn direction(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDirection", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDirection",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_curve(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCurve", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_on_slope(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnSlope", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Rails<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Rails into crate::material::MaterialData")
    }
}
/// Represents the different types of Tree block that face a direction.
pub struct Sapling<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Sapling<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Sapling<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Sapling from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Sapling")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Sapling object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<crate::material::Sapling<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Sapling");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Sapling::from_raw(&jni, res)
    }
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<impl Into<crate::TreeSpecies<'mc>>>,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::material::Sapling<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/TreeSpecies;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            // 1
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Sapling");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Sapling::from_raw(&jni, res)
    }
    //

    pub fn is_instant_growable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInstantGrowable",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set whether this sapling will grow when next ticked with bonemeal
    pub fn set_is_instant_growable(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setIsInstantGrowable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::Sapling<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/Sapling;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::Sapling::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_species(
        &mut self,
        arg0: impl Into<crate::TreeSpecies<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/TreeSpecies;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpecies",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn species(&mut self) -> Result<crate::TreeSpecies<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/TreeSpecies;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpecies", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::TreeSpecies::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::TreeSpecies::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Wood<'mc>> for Sapling<'mc> {
    fn into(self) -> crate::material::Wood<'mc> {
        crate::material::Wood::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Sapling into crate::material::Wood")
    }
}
/// Indicated a Material that may carry or create a Redstone current
///
/// This is a representation of an abstract class.
pub struct Redstone<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Redstone<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Redstone from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Redstone")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Redstone object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
impl<'mc> JNIRaw<'mc> for Redstone<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Material data for the piston extension block
pub struct PistonExtensionMaterial<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PistonExtensionMaterial<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PistonExtensionMaterial<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PistonExtensionMaterial from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/material/PistonExtensionMaterial")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PistonExtensionMaterial object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::PistonExtensionMaterial<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/PistonExtensionMaterial");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::PistonExtensionMaterial::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn attached_face(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_sticky(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSticky", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Sets whether or not this extension is sticky
    pub fn set_sticky(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSticky",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::PistonExtensionMaterial<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/PistonExtensionMaterial;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::PistonExtensionMaterial::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Attachable<'mc>> for PistonExtensionMaterial<'mc> {
    fn into(self) -> crate::material::Attachable<'mc> {
        crate::material::Attachable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PistonExtensionMaterial into crate::material::Attachable")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for PistonExtensionMaterial<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PistonExtensionMaterial into crate::material::MaterialData")
    }
}
/// Represents a lever
pub struct Lever<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Lever<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Lever<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Lever from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Lever")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Lever object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Lever<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Lever");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Lever::from_raw(&jni, res)
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn attached_face(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set this lever to be powered or not.
    pub fn set_powered(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::Lever<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/Lever;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::Lever::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Redstone<'mc>> for Lever<'mc> {
    fn into(self) -> crate::material::Redstone<'mc> {
        crate::material::Redstone::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Lever into crate::material::Redstone")
    }
}
impl<'mc> Into<crate::material::SimpleAttachableMaterialData<'mc>> for Lever<'mc> {
    fn into(self) -> crate::material::SimpleAttachableMaterialData<'mc> {
        crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Lever into crate::material::SimpleAttachableMaterialData")
    }
}
/// Represents a furnace or a dispenser.
pub struct DirectionalContainer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for DirectionalContainer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DirectionalContainer<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DirectionalContainer from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/DirectionalContainer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DirectionalContainer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::DirectionalContainer<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/DirectionalContainer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::DirectionalContainer::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Directional<'mc>> for DirectionalContainer<'mc> {
    fn into(self) -> crate::material::Directional<'mc> {
        crate::material::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting DirectionalContainer into crate::material::Directional")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for DirectionalContainer<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting DirectionalContainer into crate::material::MaterialData")
    }
}
/// Represents nether wart
pub struct NetherWarts<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for NetherWarts<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> NetherWarts<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate NetherWarts from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/NetherWarts")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a NetherWarts object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<crate::material::NetherWarts<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/NetherWarts");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::NetherWarts::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::NetherWarts<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/NetherWarts");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::NetherWarts::from_raw(&jni, res)
    }
    //

    pub fn set_state(
        &mut self,
        arg0: impl Into<crate::NetherWartsState<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/NetherWartsState;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setState",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::NetherWarts<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/NetherWarts;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::NetherWarts::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn state(&mut self) -> Result<crate::NetherWartsState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/NetherWartsState;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getState", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::NetherWartsState::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::NetherWartsState::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for NetherWarts<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting NetherWarts into crate::material::MaterialData")
    }
}
/// Handles specific metadata for certain items or blocks
pub struct MaterialData<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MaterialData<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MaterialData<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MaterialData from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/MaterialData")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MaterialData object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/MaterialData");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::MaterialData::from_raw(&jni, res)
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Sets the raw data of this material
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Creates a new ItemStack based on this MaterialData
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents the different types of long grasses.
pub struct LongGrass<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for LongGrass<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> LongGrass<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LongGrass from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/LongGrass")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LongGrass object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::GrassSpecies<'mc>>>,
    ) -> Result<crate::material::LongGrass<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/GrassSpecies;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/LongGrass");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::LongGrass::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::LongGrass<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/LongGrass");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::LongGrass::from_raw(&jni, res)
    }
    //

    pub fn set_species(
        &mut self,
        arg0: impl Into<crate::GrassSpecies<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/GrassSpecies;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpecies",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn species(&mut self) -> Result<crate::GrassSpecies<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/GrassSpecies;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpecies", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::GrassSpecies::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::GrassSpecies::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::LongGrass<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/LongGrass;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::LongGrass::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for LongGrass<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LongGrass into crate::material::MaterialData")
    }
}
/// Represents the tripwire
pub struct Tripwire<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Tripwire<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Tripwire<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Tripwire from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Tripwire")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Tripwire object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Tripwire<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Tripwire");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Tripwire::from_raw(&jni, res)
    }
    //

    pub fn is_activated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isActivated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set tripwire activated state
    pub fn set_activated(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setActivated",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_object_triggering(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isObjectTriggering",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set object triggering state for this tripwire
    pub fn set_object_triggering(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setObjectTriggering",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Tripwire<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Tripwire into crate::material::MaterialData")
    }
}
/// Represents textured materials like steps and smooth bricks
pub struct TexturedMaterial<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for TexturedMaterial<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TexturedMaterial<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate TexturedMaterial from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/TexturedMaterial")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TexturedMaterial object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::TexturedMaterial<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/TexturedMaterial");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::TexturedMaterial::from_raw(&jni, res)
    }
    //

    pub fn material(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn textures(&mut self) -> Result<Vec<crate::Material<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTextures", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            let variant =
                self.0
                    .call_method(list.get(i)?, "toString", "()Ljava/lang/String;", vec![]);
            let variant = self.jni_ref().translate_error(variant)?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            new_vec.push(crate::Material::from_raw(
                &self.0,
                obj,
                crate::Material::from_string(variant_str)
                    .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
            )?);
        }
        Ok(new_vec)
    }
    //

    pub fn set_material(
        &mut self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Material;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaterial",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::TexturedMaterial<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/TexturedMaterial;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::TexturedMaterial::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for TexturedMaterial<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TexturedMaterial into crate::material::MaterialData")
    }
}

///
/// This is a representation of an abstract class.
pub struct PressureSensor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PressureSensor<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PressureSensor from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/PressureSensor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PressureSensor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn is_pressed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPressed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
impl<'mc> JNIRaw<'mc> for PressureSensor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents a dispenser.
pub struct Dispenser<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Dispenser<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Dispenser<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Dispenser from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Dispenser")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Dispenser object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<crate::material::Dispenser<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Dispenser");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Dispenser::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Dispenser<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Dispenser");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Dispenser::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::FurnaceAndDispenser<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/FurnaceAndDispenser;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::FurnaceAndDispenser::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::FurnaceAndDispenser<'mc>> for Dispenser<'mc> {
    fn into(self) -> crate::material::FurnaceAndDispenser<'mc> {
        crate::material::FurnaceAndDispenser::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Dispenser into crate::material::FurnaceAndDispenser")
    }
}

pub struct Cake<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Cake<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Cake<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Cake from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Cake")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Cake object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Cake<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Cake");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Cake::from_raw(&jni, res)
    }
    //

    pub fn slices_eaten(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSlicesEaten", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn slices_remaining(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlicesRemaining",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Sets the number of slices eaten from this cake
    pub fn set_slices_eaten(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlicesEaten",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Sets the number of slices remaining on this cake
    pub fn set_slices_remaining(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlicesRemaining",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::Cake<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/Cake;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::Cake::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Cake<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Cake into crate::material::MaterialData")
    }
}
/// Represents a fence gate
pub struct Gate<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Gate<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Gate<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Gate from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Gate")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Gate object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Gate<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Gate");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Gate::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Openable.html#setOpen(boolean)">Openable</a></code></span>
    /// Configure this door to be either open or closed;
    pub fn set_open(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOpen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_open(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOpen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Directional<'mc>> for Gate<'mc> {
    fn into(self) -> crate::material::Directional<'mc> {
        crate::material::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Gate into crate::material::Directional")
    }
}
impl<'mc> Into<crate::material::Openable<'mc>> for Gate<'mc> {
    fn into(self) -> crate::material::Openable<'mc> {
        crate::material::Openable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Gate into crate::material::Openable")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Gate<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Gate into crate::material::MaterialData")
    }
}
#[derive(PartialEq, Eq)]
pub enum CocoaPlantSizeEnum {
    Small,
    Medium,
    Large,
}
impl std::fmt::Display for CocoaPlantSizeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CocoaPlantSizeEnum::Small => f.write_str("SMALL"),
            CocoaPlantSizeEnum::Medium => f.write_str("MEDIUM"),
            CocoaPlantSizeEnum::Large => f.write_str("LARGE"),
        }
    }
}
pub struct CocoaPlantSize<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub CocoaPlantSizeEnum,
);
impl<'mc> std::ops::Deref for CocoaPlantSize<'mc> {
    type Target = CocoaPlantSizeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for CocoaPlantSize<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CocoaPlantSize<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: CocoaPlantSizeEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CocoaPlantSize from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/CocoaPlantSize")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CocoaPlantSize object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const SMALL: CocoaPlantSizeEnum = CocoaPlantSizeEnum::Small;
    pub const MEDIUM: CocoaPlantSizeEnum = CocoaPlantSizeEnum::Medium;
    pub const LARGE: CocoaPlantSizeEnum = CocoaPlantSizeEnum::Large;
    pub fn from_string(str: String) -> std::option::Option<CocoaPlantSizeEnum> {
        match str.as_str() {
            "SMALL" => Some(CocoaPlantSizeEnum::Small),
            "MEDIUM" => Some(CocoaPlantSizeEnum::Medium),
            "LARGE" => Some(CocoaPlantSizeEnum::Large),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<CocoaPlantSize<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/material/CocoaPlantSize");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/material/CocoaPlantSize;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        CocoaPlantSize::from_raw(
            &jni,
            raw_obj,
            CocoaPlantSize::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
/// Represents the different types of steps.
pub struct Step<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Step<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Step<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Step from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Step")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Step object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Step<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Step");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Step::from_raw(&jni, res)
    }
    //

    pub fn is_inverted(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInverted", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set step inverted state
    pub fn set_inverted(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInverted",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn textures(&mut self) -> Result<Vec<crate::Material<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTextures", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            let variant =
                self.0
                    .call_method(list.get(i)?, "toString", "()Ljava/lang/String;", vec![]);
            let variant = self.jni_ref().translate_error(variant)?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            new_vec.push(crate::Material::from_raw(
                &self.0,
                obj,
                crate::Material::from_string(variant_str)
                    .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
            )?);
        }
        Ok(new_vec)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::Step<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/Step;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::Step::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn material(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_material(
        &mut self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Material;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaterial",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::TexturedMaterial<'mc>> for Step<'mc> {
    fn into(self) -> crate::material::TexturedMaterial<'mc> {
        crate::material::TexturedMaterial::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Step into crate::material::TexturedMaterial")
    }
}
/// Represents the different types of smooth bricks.
pub struct SmoothBrick<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SmoothBrick<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SmoothBrick<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SmoothBrick from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/SmoothBrick")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SmoothBrick object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::SmoothBrick<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/SmoothBrick");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::SmoothBrick::from_raw(&jni, res)
    }
    //

    pub fn textures(&mut self) -> Result<Vec<crate::Material<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTextures", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            let variant =
                self.0
                    .call_method(list.get(i)?, "toString", "()Ljava/lang/String;", vec![]);
            let variant = self.jni_ref().translate_error(variant)?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            new_vec.push(crate::Material::from_raw(
                &self.0,
                obj,
                crate::Material::from_string(variant_str)
                    .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
            )?);
        }
        Ok(new_vec)
    }
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::SmoothBrick<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/SmoothBrick;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::SmoothBrick::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn material(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_material(
        &mut self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Material;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaterial",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::TexturedMaterial<'mc>> for SmoothBrick<'mc> {
    fn into(self) -> crate::material::TexturedMaterial<'mc> {
        crate::material::TexturedMaterial::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SmoothBrick into crate::material::TexturedMaterial")
    }
}
/// Represents the different types of leaf block that may be permanent or can decay when too far from a log.
pub struct Leaves<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Leaves<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Leaves<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Leaves from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Leaves")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Leaves object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::TreeSpecies<'mc>>>,
    ) -> Result<crate::material::Leaves<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/TreeSpecies;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Leaves");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Leaves::from_raw(&jni, res)
    }
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<impl Into<crate::TreeSpecies<'mc>>>,
    ) -> Result<crate::material::Leaves<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/TreeSpecies;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Leaves");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Leaves::from_raw(&jni, res)
    }
    pub fn new_with_tree_species(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<impl Into<crate::TreeSpecies<'mc>>>,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::material::Leaves<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/TreeSpecies;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            // 1
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Leaves");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Leaves::from_raw(&jni, res)
    }
    //

    pub fn is_decayable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDecayable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set whether this leaf block will disappear when too far from a log
    pub fn set_decayable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDecayable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_decaying(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDecaying", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set whether this leaf block is in the process of decaying
    pub fn set_decaying(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDecaying",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::Leaves<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/Leaves;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::Leaves::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_species(
        &mut self,
        arg0: impl Into<crate::TreeSpecies<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/TreeSpecies;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpecies",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn species(&mut self) -> Result<crate::TreeSpecies<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/TreeSpecies;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpecies", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::TreeSpecies::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::TreeSpecies::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Wood<'mc>> for Leaves<'mc> {
    fn into(self) -> crate::material::Wood<'mc> {
        crate::material::Wood::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Leaves into crate::material::Wood")
    }
}
/// Represents dye
pub struct Dye<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Dye<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Dye<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Dye from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Dye")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Dye object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<crate::material::Dye<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Dye");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Dye::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Dye<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Dye");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Dye::from_raw(&jni, res)
    }
    //

    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/DyeColor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn color(&mut self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/DyeColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::DyeColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::DyeColor::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::Dye<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/Dye;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::Dye::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Colorable<'mc>> for Dye<'mc> {
    fn into(self) -> crate::material::Colorable<'mc> {
        crate::material::Colorable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Dye into crate::material::Colorable")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Dye<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Dye into crate::material::MaterialData")
    }
}
/// Represents a pressure plate
pub struct PressurePlate<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PressurePlate<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PressurePlate<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PressurePlate from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/PressurePlate")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PressurePlate object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::PressurePlate<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/PressurePlate");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::PressurePlate::from_raw(&jni, res)
    }
    //

    pub fn is_pressed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPressed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::PressureSensor<'mc>> for PressurePlate<'mc> {
    fn into(self) -> crate::material::PressureSensor<'mc> {
        crate::material::PressureSensor::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PressurePlate into crate::material::PressureSensor")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for PressurePlate<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PressurePlate into crate::material::MaterialData")
    }
}
/// Represents an observer.
pub struct Observer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Observer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Observer<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Observer from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Observer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Observer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<crate::material::Observer<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Observer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Observer::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Observer<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Observer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Observer::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Directional<'mc>> for Observer<'mc> {
    fn into(self) -> crate::material::Directional<'mc> {
        crate::material::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Observer into crate::material::Directional")
    }
}
impl<'mc> Into<crate::material::Redstone<'mc>> for Observer<'mc> {
    fn into(self) -> crate::material::Redstone<'mc> {
        crate::material::Redstone::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Observer into crate::material::Redstone")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Observer<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Observer into crate::material::MaterialData")
    }
}
/// Represents stairs.
pub struct Stairs<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Stairs<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Stairs<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Stairs from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Stairs")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Stairs object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Stairs<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Stairs");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Stairs::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_inverted(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInverted", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set step inverted state
    pub fn set_inverted(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInverted",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn ascending_direction(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAscendingDirection",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn descending_direction(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDescendingDirection",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Directional<'mc>> for Stairs<'mc> {
    fn into(self) -> crate::material::Directional<'mc> {
        crate::material::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Stairs into crate::material::Directional")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Stairs<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Stairs into crate::material::MaterialData")
    }
}
/// Represents the different types of sandstone.
pub struct Sandstone<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Sandstone<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Sandstone<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Sandstone from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Sandstone")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Sandstone object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::SandstoneType<'mc>>>,
    ) -> Result<crate::material::Sandstone<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/SandstoneType;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Sandstone");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Sandstone::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Sandstone<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Sandstone");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Sandstone::from_raw(&jni, res)
    }
    //

    pub fn set_type(
        &mut self,
        arg0: impl Into<crate::SandstoneType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/SandstoneType;)V");
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
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_type(&mut self) -> Result<crate::SandstoneType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SandstoneType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::SandstoneType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::SandstoneType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Sandstone<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Sandstone into crate::material::MaterialData")
    }
}
/// Represents an ender chest
pub struct EnderChest<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EnderChest<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EnderChest<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EnderChest from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/EnderChest")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnderChest object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,
    ) -> Result<crate::material::EnderChest<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/block/BlockFace;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/EnderChest");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::EnderChest::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::EnderChest<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/EnderChest");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::EnderChest::from_raw(&jni, res)
    }
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::EnderChest<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/EnderChest;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::EnderChest::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::DirectionalContainer<'mc>> for EnderChest<'mc> {
    fn into(self) -> crate::material::DirectionalContainer<'mc> {
        crate::material::DirectionalContainer::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EnderChest into crate::material::DirectionalContainer")
    }
}
/// Represents a furnace or dispenser, two types of directional containers
pub struct FurnaceAndDispenser<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FurnaceAndDispenser<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FurnaceAndDispenser<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FurnaceAndDispenser from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/FurnaceAndDispenser")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FurnaceAndDispenser object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::FurnaceAndDispenser<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/FurnaceAndDispenser");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::FurnaceAndDispenser::from_raw(&jni, res)
    }
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::FurnaceAndDispenser<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/FurnaceAndDispenser;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::FurnaceAndDispenser::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::DirectionalContainer<'mc>> for FurnaceAndDispenser<'mc> {
    fn into(self) -> crate::material::DirectionalContainer<'mc> {
        crate::material::DirectionalContainer::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting FurnaceAndDispenser into crate::material::DirectionalContainer",
        )
    }
}
/// Represents a bed.
pub struct Bed<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Bed<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Bed<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Bed from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Bed")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Bed object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<crate::material::Bed<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Bed");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Bed::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Bed<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Bed");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Bed::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_head_of_bed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isHeadOfBed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Configure this to be either the head or the foot of the bed
    pub fn set_head_of_bed(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHeadOfBed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Directional<'mc>> for Bed<'mc> {
    fn into(self) -> crate::material::Directional<'mc> {
        crate::material::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Bed into crate::material::Directional")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Bed<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Bed into crate::material::MaterialData")
    }
}
/// Represents redstone wire
pub struct RedstoneWire<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for RedstoneWire<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RedstoneWire<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate RedstoneWire from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/RedstoneWire")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RedstoneWire object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::RedstoneWire<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/RedstoneWire");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::RedstoneWire::from_raw(&jni, res)
    }
    //

    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Redstone<'mc>> for RedstoneWire<'mc> {
    fn into(self) -> crate::material::Redstone<'mc> {
        crate::material::Redstone::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting RedstoneWire into crate::material::Redstone")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for RedstoneWire<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting RedstoneWire into crate::material::MaterialData")
    }
}
/// Represents the different types of wooden steps.
pub struct WoodenStep<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for WoodenStep<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> WoodenStep<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate WoodenStep from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/WoodenStep")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WoodenStep object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::WoodenStep<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/WoodenStep");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::WoodenStep::from_raw(&jni, res)
    }
    //

    pub fn is_inverted(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInverted", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set step inverted state
    pub fn set_inverted(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInverted",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::WoodenStep<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/WoodenStep;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::WoodenStep::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_species(
        &mut self,
        arg0: impl Into<crate::TreeSpecies<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/TreeSpecies;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpecies",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn species(&mut self) -> Result<crate::TreeSpecies<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/TreeSpecies;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpecies", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::TreeSpecies::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::TreeSpecies::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Wood<'mc>> for WoodenStep<'mc> {
    fn into(self) -> crate::material::Wood<'mc> {
        crate::material::Wood::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WoodenStep into crate::material::Wood")
    }
}
/// Represents a vine
pub struct Vine<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Vine<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Vine<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Vine from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Vine")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Vine object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_block_faces(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Vine<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Vine");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Vine::from_raw(&jni, res)
    }
    //

    pub fn is_on_face(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isOnFace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn put_on_face(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putOnFace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_from_face(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeFromFace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Vine<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Vine into crate::material::MaterialData")
    }
}
/// Represents wood blocks of different species.
pub struct Wood<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Wood<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Wood<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Wood from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Wood")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Wood object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<crate::material::Wood<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Wood");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Wood::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Wood<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Wood");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Wood::from_raw(&jni, res)
    }
    //

    pub fn set_species(
        &mut self,
        arg0: impl Into<crate::TreeSpecies<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/TreeSpecies;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpecies",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn species(&mut self) -> Result<crate::TreeSpecies<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/TreeSpecies;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpecies", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::TreeSpecies::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::TreeSpecies::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Wood<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Wood into crate::material::MaterialData")
    }
}

pub struct Banner<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Banner<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Banner<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Banner from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Banner")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Banner object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Banner<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Banner");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Banner::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn attached_face(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_wall_banner(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWallBanner", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Attachable<'mc>> for Banner<'mc> {
    fn into(self) -> crate::material::Attachable<'mc> {
        crate::material::Attachable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Banner into crate::material::Attachable")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Banner<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Banner into crate::material::MaterialData")
    }
}
/// Represents a huge mushroom block with certain combinations of faces set to cap, pores or stem.
pub struct Mushroom<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Mushroom<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Mushroom<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Mushroom from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Mushroom")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Mushroom object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,
    ) -> Result<crate::material::Mushroom<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/block/BlockFace;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Mushroom");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Mushroom::from_raw(&jni, res)
    }
    //

    pub fn is_stem(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isStem", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_stem(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setStem", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn block_texture(
        &mut self,
    ) -> Result<crate::material::types::MushroomBlockTexture<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/material/types/MushroomBlockTexture;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlockTexture", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::material::types::MushroomBlockTexture::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::material::types::MushroomBlockTexture::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_block_texture(
        &mut self,
        arg0: impl Into<crate::material::types::MushroomBlockTexture<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/types/MushroomBlockTexture;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockTexture",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_face_painted(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFacePainted",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_face_painted(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacePainted",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn painted_faces(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPaintedFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::Mushroom<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/Mushroom;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::Mushroom::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Mushroom<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Mushroom into crate::material::MaterialData")
    }
}
/// Represents a redstone torch
pub struct RedstoneTorch<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for RedstoneTorch<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RedstoneTorch<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate RedstoneTorch from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/RedstoneTorch")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RedstoneTorch object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::RedstoneTorch<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/RedstoneTorch");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::RedstoneTorch::from_raw(&jni, res)
    }
    //

    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::RedstoneTorch<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/RedstoneTorch;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::RedstoneTorch::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn attached_face(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Redstone<'mc>> for RedstoneTorch<'mc> {
    fn into(self) -> crate::material::Redstone<'mc> {
        crate::material::Redstone::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting RedstoneTorch into crate::material::Redstone")
    }
}
impl<'mc> Into<crate::material::Torch<'mc>> for RedstoneTorch<'mc> {
    fn into(self) -> crate::material::Torch<'mc> {
        crate::material::Torch::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting RedstoneTorch into crate::material::Torch")
    }
}
/// Represents the cocoa plant
pub struct CocoaPlant<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
#[derive(PartialEq, Eq)]
pub enum CocoaPlantCocoaPlantSizeEnum {
    Small,
    Medium,
    Large,
}
impl std::fmt::Display for CocoaPlantCocoaPlantSizeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CocoaPlantCocoaPlantSizeEnum::Small => f.write_str("SMALL"),
            CocoaPlantCocoaPlantSizeEnum::Medium => f.write_str("MEDIUM"),
            CocoaPlantCocoaPlantSizeEnum::Large => f.write_str("LARGE"),
        }
    }
}
pub struct CocoaPlantCocoaPlantSize<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub CocoaPlantCocoaPlantSizeEnum,
);
impl<'mc> std::ops::Deref for CocoaPlantCocoaPlantSize<'mc> {
    type Target = CocoaPlantCocoaPlantSizeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for CocoaPlantCocoaPlantSize<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CocoaPlantCocoaPlantSize<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: CocoaPlantCocoaPlantSizeEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CocoaPlantCocoaPlantSize from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/material/CocoaPlant$CocoaPlantSize")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CocoaPlantCocoaPlantSize object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const SMALL: CocoaPlantCocoaPlantSizeEnum = CocoaPlantCocoaPlantSizeEnum::Small;
    pub const MEDIUM: CocoaPlantCocoaPlantSizeEnum = CocoaPlantCocoaPlantSizeEnum::Medium;
    pub const LARGE: CocoaPlantCocoaPlantSizeEnum = CocoaPlantCocoaPlantSizeEnum::Large;
    pub fn from_string(str: String) -> std::option::Option<CocoaPlantCocoaPlantSizeEnum> {
        match str.as_str() {
            "SMALL" => Some(CocoaPlantCocoaPlantSizeEnum::Small),
            "MEDIUM" => Some(CocoaPlantCocoaPlantSizeEnum::Medium),
            "LARGE" => Some(CocoaPlantCocoaPlantSizeEnum::Large),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<CocoaPlantCocoaPlantSize<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/material/CocoaPlant$CocoaPlantSize");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/material/CocoaPlant$CocoaPlantSize;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        CocoaPlantCocoaPlantSize::from_raw(
            &jni,
            raw_obj,
            CocoaPlantCocoaPlantSize::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    //
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for CocoaPlant<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CocoaPlant<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CocoaPlant from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/CocoaPlant")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CocoaPlant object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::CocoaPlant<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/CocoaPlant");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::CocoaPlant::from_raw(&jni, res)
    }
    pub fn new_with_cocoa_plantcocoa_plant_size(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::material::CocoaPlantCocoaPlantSize<'mc>>,
        arg1: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,
    ) -> Result<crate::material::CocoaPlant<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/material/CocoaPlant$CocoaPlantSize;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/block/BlockFace;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/CocoaPlant");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::CocoaPlant::from_raw(&jni, res)
    }
    //

    pub fn set_size(
        &mut self,
        arg0: impl Into<crate::material::CocoaPlantCocoaPlantSize<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/CocoaPlant$CocoaPlantSize;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSize",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn attached_face(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn size(
        &mut self,
    ) -> Result<crate::material::CocoaPlantCocoaPlantSize<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/material/CocoaPlant$CocoaPlantSize;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::material::CocoaPlantCocoaPlantSize::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::material::CocoaPlantCocoaPlantSize::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Directional<'mc>> for CocoaPlant<'mc> {
    fn into(self) -> crate::material::Directional<'mc> {
        crate::material::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CocoaPlant into crate::material::Directional")
    }
}
impl<'mc> Into<crate::material::Attachable<'mc>> for CocoaPlant<'mc> {
    fn into(self) -> crate::material::Attachable<'mc> {
        crate::material::Attachable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CocoaPlant into crate::material::Attachable")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for CocoaPlant<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CocoaPlant into crate::material::MaterialData")
    }
}
/// Represents a door. This class was previously deprecated, but has been retrofitted to work with modern doors. Some methods are undefined dependant on <code>isTopHalf()</code> due to Minecraft's internal representation of doors.
pub struct Door<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Door<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Door<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Door from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Door")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Door object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::TreeSpecies<'mc>>>,
        arg1: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,
    ) -> Result<crate::material::Door<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/TreeSpecies;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/block/BlockFace;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Door");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Door::from_raw(&jni, res)
    }
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::TreeSpecies<'mc>>,
        arg1: impl Into<crate::block::BlockFace<'mc>>,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::material::Door<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/TreeSpecies;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/block/BlockFace;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Z";
            // 2
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Door");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Door::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_tree_species(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Door<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Door");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Door::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set whether the door is open. Undefined if <code>isTopHalf()</code> is true.
    pub fn set_open(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOpen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn hinge(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHinge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Set whether the hinge is on the left or right side. Left is false, right is true. Undefined if <code>isTopHalf()</code> is false.
    pub fn set_hinge(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHinge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Configure this part of the door to be either the top or the bottom half
    pub fn set_top_half(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTopHalf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get_wood_door_of_species(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::TreeSpecies<'mc>>,
    ) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/TreeSpecies;)Lorg/bukkit/Material;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/Material");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getWoodDoorOfSpecies",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &jni,
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_top_half(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isTopHalf", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hinge_corner(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHingeCorner", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::Door<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/Door;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::Door::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_open(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOpen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Directional<'mc>> for Door<'mc> {
    fn into(self) -> crate::material::Directional<'mc> {
        crate::material::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Door into crate::material::Directional")
    }
}
impl<'mc> Into<crate::material::Openable<'mc>> for Door<'mc> {
    fn into(self) -> crate::material::Openable<'mc> {
        crate::material::Openable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Door into crate::material::Openable")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Door<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Door into crate::material::MaterialData")
    }
}

///
/// This is a representation of an abstract class.
pub struct Directional<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Directional<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Directional from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Directional")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Directional object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Directional<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// This is the superclass for the <a href="DetectorRail.html" title="class in org.bukkit.material"><code>DetectorRail</code></a> and <a href="PoweredRail.html" title="class in org.bukkit.material"><code>PoweredRail</code></a> classes
pub struct ExtendedRails<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ExtendedRails<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ExtendedRails<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ExtendedRails from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/ExtendedRails")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ExtendedRails object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::ExtendedRails<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/ExtendedRails");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::ExtendedRails::from_raw(&jni, res)
    }
    //

    pub fn set_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDirection",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_curve(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCurve", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::ExtendedRails<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/ExtendedRails;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::ExtendedRails::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn direction(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDirection", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_on_slope(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnSlope", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Rails<'mc>> for ExtendedRails<'mc> {
    fn into(self) -> crate::material::Rails<'mc> {
        crate::material::Rails::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ExtendedRails into crate::material::Rails")
    }
}
/// Represents a cauldron
pub struct Cauldron<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Cauldron<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Cauldron<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Cauldron from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Cauldron")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Cauldron object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Cauldron<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Cauldron");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Cauldron::from_raw(&jni, res)
    }
    //

    pub fn is_full(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFull", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Cauldron<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Cauldron into crate::material::MaterialData")
    }
}
/// Represents Ladder data
pub struct Ladder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Ladder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Ladder<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Ladder from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Ladder")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Ladder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Ladder<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Ladder");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Ladder::from_raw(&jni, res)
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn attached_face(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::SimpleAttachableMaterialData<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/SimpleAttachableMaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::SimpleAttachableMaterialData<'mc>> for Ladder<'mc> {
    fn into(self) -> crate::material::SimpleAttachableMaterialData<'mc> {
        crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Ladder into crate::material::SimpleAttachableMaterialData")
    }
}
/// Represents a command block
pub struct Command<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Command<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Command<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Command from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Command")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Command object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Command<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Command");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Command::from_raw(&jni, res)
    }
    //

    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Sets the current state of this Material
    pub fn set_powered(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::Command<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/Command;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::Command::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Redstone<'mc>> for Command<'mc> {
    fn into(self) -> crate::material::Redstone<'mc> {
        crate::material::Redstone::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Command into crate::material::Redstone")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Command<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Command into crate::material::MaterialData")
    }
}
/// Represents a detector rail
pub struct DetectorRail<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for DetectorRail<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DetectorRail<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DetectorRail from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/DetectorRail")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DetectorRail object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::DetectorRail<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/DetectorRail");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::DetectorRail::from_raw(&jni, res)
    }
    //

    pub fn is_pressed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPressed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    pub fn set_pressed(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPressed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::DetectorRail<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/DetectorRail;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::DetectorRail::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDirection",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_curve(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCurve", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn direction(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDirection", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_on_slope(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnSlope", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::PressureSensor<'mc>> for DetectorRail<'mc> {
    fn into(self) -> crate::material::PressureSensor<'mc> {
        crate::material::PressureSensor::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting DetectorRail into crate::material::PressureSensor")
    }
}
impl<'mc> Into<crate::material::ExtendedRails<'mc>> for DetectorRail<'mc> {
    fn into(self) -> crate::material::ExtendedRails<'mc> {
        crate::material::ExtendedRails::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting DetectorRail into crate::material::ExtendedRails")
    }
}
/// Represents a hopper in an active or deactivated state and facing in a specific direction.
pub struct Hopper<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Hopper<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Hopper<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Hopper from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Hopper")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Hopper object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<crate::material::Hopper<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Hopper");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Hopper::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_block_face(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Hopper<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Hopper");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Hopper::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// Sets whether the hopper is active or not.
    pub fn set_active(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setActive",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_active(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isActive", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Directional<'mc>> for Hopper<'mc> {
    fn into(self) -> crate::material::Directional<'mc> {
        crate::material::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Hopper into crate::material::Directional")
    }
}
impl<'mc> Into<crate::material::Redstone<'mc>> for Hopper<'mc> {
    fn into(self) -> crate::material::Redstone<'mc> {
        crate::material::Redstone::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Hopper into crate::material::Redstone")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Hopper<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Hopper into crate::material::MaterialData")
    }
}
/// Represents a skull.
pub struct Skull<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Skull<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Skull<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Skull from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Skull")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Skull object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<crate::material::Skull<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Skull");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Skull::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Skull<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Skull");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Skull::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Directional<'mc>> for Skull<'mc> {
    fn into(self) -> crate::material::Directional<'mc> {
        crate::material::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Skull into crate::material::Directional")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Skull<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Skull into crate::material::MaterialData")
    }
}
/// MaterialData for signs
pub struct Sign<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Sign<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Sign<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Sign from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Sign")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Sign object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Sign<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Material;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Sign");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Sign::from_raw(&jni, res)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn attached_face(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_wall_sign(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWallSign", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/MaterialData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Attachable<'mc>> for Sign<'mc> {
    fn into(self) -> crate::material::Attachable<'mc> {
        crate::material::Attachable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Sign into crate::material::Attachable")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Sign<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Sign into crate::material::MaterialData")
    }
}
/// Represents a Wool/Cloth block
pub struct Wool<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Wool<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Wool<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Wool from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Wool")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Wool object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::DyeColor<'mc>>>,
    ) -> Result<crate::material::Wool<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/DyeColor;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Wool");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Wool::from_raw(&jni, res)
    }
    //['since', '']

    //['forRemoval', 'false']

    #[deprecated]
    pub fn new_with_material(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::material::Wool<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "B";
            let val_2 = jni::objects::JValueGen::Byte(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/material/Wool");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::material::Wool::from_raw(&jni, res)
    }
    //

    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/DyeColor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn color(&mut self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/DyeColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::DyeColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::DyeColor::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn clone(&mut self) -> Result<crate::material::Wool<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/material/Wool;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::Wool::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@Deprecated

    #[deprecated]

    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/inventory/ItemStack;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toItemStack", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &mut self,
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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::material::Colorable<'mc>> for Wool<'mc> {
    fn into(self) -> crate::material::Colorable<'mc> {
        crate::material::Colorable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Wool into crate::material::Colorable")
    }
}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Wool<'mc> {
    fn into(self) -> crate::material::MaterialData<'mc> {
        crate::material::MaterialData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Wool into crate::material::MaterialData")
    }
}
/// Indicates that a block can be attached to another block
///
/// This is a representation of an abstract class.
pub struct Attachable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Attachable<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Attachable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Attachable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Attachable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn attached_face(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Attachable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::material::Directional<'mc>> for Attachable<'mc> {
    fn into(self) -> crate::material::Directional<'mc> {
        crate::material::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Attachable into crate::material::Directional")
    }
}
pub mod types;
