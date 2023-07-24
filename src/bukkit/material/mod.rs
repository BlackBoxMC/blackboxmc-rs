#![allow(deprecated)]
use crate::JNIRaw;
use color_eyre::eyre::Result;
pub struct Chest<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Chest<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Chest<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>>,
    ) -> Result<crate::bukkit::material::Chest<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Chest")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Chest(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Chest<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Chest")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Chest(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Chest from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Chest") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Chest object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::Chest<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/Chest;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::Chest(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::DirectionalContainer<'mc>> for Chest<'mc> {
    fn into(self) -> crate::bukkit::material::DirectionalContainer<'mc> {
        crate::bukkit::material::DirectionalContainer::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Diode<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Diode<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Diode<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>>,
    ) -> Result<crate::bukkit::material::Diode<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Diode")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Diode(jni, res) };
        Ok(ret)
    }
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::bukkit::material::Diode<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Diode")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/BlockFace;I)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Diode(jni, res) };
        Ok(ret)
    }
    pub fn new_with_block_face(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
        arg1: i32,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::bukkit::material::Diode<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // 2
        let val_2 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Diode")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/BlockFace;IZ)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = { crate::bukkit::material::Diode(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Diode from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Diode") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Diode object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn delay(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDelay", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_delay(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDelay",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Directional<'mc>> for Diode<'mc> {
    fn into(self) -> crate::bukkit::material::Directional<'mc> {
        crate::bukkit::material::Directional::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::Redstone<'mc>> for Diode<'mc> {
    fn into(self) -> crate::bukkit::material::Redstone<'mc> {
        crate::bukkit::material::Redstone::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Diode<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Pumpkin<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Pumpkin<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Pumpkin<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>>,
    ) -> Result<crate::bukkit::material::Pumpkin<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Pumpkin")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Pumpkin(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Pumpkin<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Pumpkin")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Pumpkin(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Pumpkin from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Pumpkin") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Pumpkin object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_lit(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLit", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Directional<'mc>> for Pumpkin<'mc> {
    fn into(self) -> crate::bukkit::material::Directional<'mc> {
        crate::bukkit::material::Directional::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Pumpkin<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct FlowerPot<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for FlowerPot<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FlowerPot<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::FlowerPot<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/FlowerPot")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::FlowerPot(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate FlowerPot from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("FlowerPot") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FlowerPot object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn contents(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getContents",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_contents(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setContents",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for FlowerPot<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct PoweredRail<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for PoweredRail<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PoweredRail<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::PoweredRail<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/PoweredRail")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::PoweredRail(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PoweredRail from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PoweredRail") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PoweredRail object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::PoweredRail<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/PoweredRail;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::PoweredRail(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_powered(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDirection",
            "(Lorg/bukkit/block/BlockFace;Z)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_curve(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCurve", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn direction(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDirection",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_on_slope(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnSlope", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Redstone<'mc>> for PoweredRail<'mc> {
    fn into(self) -> crate::bukkit::material::Redstone<'mc> {
        crate::bukkit::material::Redstone::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::ExtendedRails<'mc>> for PoweredRail<'mc> {
    fn into(self) -> crate::bukkit::material::ExtendedRails<'mc> {
        crate::bukkit::material::ExtendedRails::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct TripwireHook<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for TripwireHook<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TripwireHook<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::TripwireHook<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/TripwireHook")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::TripwireHook(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TripwireHook from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("TripwireHook") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TripwireHook object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn attached_face(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttachedFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn is_connected(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isConnected", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_connected(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setConnected",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_activated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isActivated", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_activated(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setActivated",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Redstone<'mc>> for TripwireHook<'mc> {
    fn into(self) -> crate::bukkit::material::Redstone<'mc> {
        crate::bukkit::material::Redstone::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::SimpleAttachableMaterialData<'mc>> for TripwireHook<'mc> {
    fn into(self) -> crate::bukkit::material::SimpleAttachableMaterialData<'mc> {
        crate::bukkit::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
pub struct SimpleAttachableMaterialData<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for SimpleAttachableMaterialData<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SimpleAttachableMaterialData<'mc> {
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>>,
    ) -> Result<
        crate::bukkit::material::SimpleAttachableMaterialData<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/SimpleAttachableMaterialData")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;Lorg/bukkit/block/BlockFace;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::SimpleAttachableMaterialData(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate SimpleAttachableMaterialData from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("SimpleAttachableMaterialData") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SimpleAttachableMaterialData object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn attached_face(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttachedFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Attachable<'mc>> for SimpleAttachableMaterialData<'mc> {
    fn into(self) -> crate::bukkit::material::Attachable<'mc> {
        crate::bukkit::material::Attachable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for SimpleAttachableMaterialData<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Openable. Needed for returning it from Java.
pub struct Openable<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Openable<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Openable from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Openable") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Openable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn is_open(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOpen", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_open(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOpen",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for Openable<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct Torch<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Torch<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Torch<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Torch<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Torch")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Torch(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Torch from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Torch") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Torch object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::Torch<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/Torch;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::Torch(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn attached_face(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttachedFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::SimpleAttachableMaterialData<'mc>> for Torch<'mc> {
    fn into(self) -> crate::bukkit::material::SimpleAttachableMaterialData<'mc> {
        crate::bukkit::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
pub struct Comparator<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Comparator<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Comparator<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>>,
    ) -> Result<crate::bukkit::material::Comparator<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Comparator")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Comparator(jni, res) };
        Ok(ret)
    }
    pub fn new_with_block_face(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
        arg1: bool,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::bukkit::material::Comparator<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        // 2
        let val_1 = jni::objects::JValueGen::Bool(arg1.into());
        // 2
        let val_2 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Comparator")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/BlockFace;ZZ)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = { crate::bukkit::material::Comparator(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Comparator from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Comparator") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Comparator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_subtraction_mode(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSubtractionMode",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_subtraction_mode(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSubtractionMode", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn is_being_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBeingPowered", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Directional<'mc>> for Comparator<'mc> {
    fn into(self) -> crate::bukkit::material::Directional<'mc> {
        crate::bukkit::material::Directional::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::Redstone<'mc>> for Comparator<'mc> {
    fn into(self) -> crate::bukkit::material::Redstone<'mc> {
        crate::bukkit::material::Redstone::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Comparator<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Button<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Button<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Button<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Button<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Button")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Button(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Button from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Button") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Button object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn attached_face(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttachedFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_powered(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Redstone<'mc>> for Button<'mc> {
    fn into(self) -> crate::bukkit::material::Redstone<'mc> {
        crate::bukkit::material::Redstone::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::SimpleAttachableMaterialData<'mc>> for Button<'mc> {
    fn into(self) -> crate::bukkit::material::SimpleAttachableMaterialData<'mc> {
        crate::bukkit::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
pub struct Coal<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Coal<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Coal<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::CoalType<'mc>>>,
    ) -> Result<crate::bukkit::material::Coal<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Coal")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/CoalType;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Coal(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Coal<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Coal")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Coal(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Coal from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Coal") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Coal object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn get_type(&mut self) -> Result<crate::bukkit::CoalType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/CoalType;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::CoalType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::CoalType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::CoalType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/CoalType;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Coal<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Colorable. Needed for returning it from Java.
pub struct Colorable<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Colorable<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Colorable from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Colorable") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Colorable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_color(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/DyeColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn color(&mut self) -> Result<crate::bukkit::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::DyeColor(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::DyeColor::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for Colorable<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct Tree<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Tree<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Tree<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>>,
    ) -> Result<crate::bukkit::material::Tree<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Tree")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/TreeSpecies;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Tree(jni, res) };
        Ok(ret)
    }
    pub fn new_with_tree_species(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>>,
    ) -> Result<crate::bukkit::material::Tree<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Tree")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;Lorg/bukkit/TreeSpecies;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Tree(jni, res) };
        Ok(ret)
    }
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>>,
        arg2: std::option::Option<impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>>,
    ) -> Result<crate::bukkit::material::Tree<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Tree")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;Lorg/bukkit/TreeSpecies;Lorg/bukkit/block/BlockFace;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = { crate::bukkit::material::Tree(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Tree from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Tree") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Tree object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn direction(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDirection",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_species(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpecies",
            "(Lorg/bukkit/TreeSpecies;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn species(
        &mut self,
    ) -> Result<crate::bukkit::TreeSpecies<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpecies",
            "()Lorg/bukkit/TreeSpecies;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::TreeSpecies(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::TreeSpecies::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Wood<'mc>> for Tree<'mc> {
    fn into(self) -> crate::bukkit::material::Wood<'mc> {
        crate::bukkit::material::Wood::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct PistonBaseMaterial<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for PistonBaseMaterial<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PistonBaseMaterial<'mc> {
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::PistonBaseMaterial<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/PistonBaseMaterial")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::PistonBaseMaterial(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PistonBaseMaterial from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PistonBaseMaterial") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PistonBaseMaterial object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_powered(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_sticky(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSticky", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Directional<'mc>> for PistonBaseMaterial<'mc> {
    fn into(self) -> crate::bukkit::material::Directional<'mc> {
        crate::bukkit::material::Directional::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::Redstone<'mc>> for PistonBaseMaterial<'mc> {
    fn into(self) -> crate::bukkit::material::Redstone<'mc> {
        crate::bukkit::material::Redstone::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for PistonBaseMaterial<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct TrapDoor<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for TrapDoor<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TrapDoor<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::TrapDoor<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/TrapDoor")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::TrapDoor(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TrapDoor from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("TrapDoor") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TrapDoor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_open(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOpen", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn attached_face(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttachedFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_open(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOpen",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_inverted(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInverted", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_inverted(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInverted",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Openable<'mc>> for TrapDoor<'mc> {
    fn into(self) -> crate::bukkit::material::Openable<'mc> {
        crate::bukkit::material::Openable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::SimpleAttachableMaterialData<'mc>> for TrapDoor<'mc> {
    fn into(self) -> crate::bukkit::material::SimpleAttachableMaterialData<'mc> {
        crate::bukkit::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
pub struct Crops<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Crops<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Crops<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::CropState<'mc>>>,
    ) -> Result<crate::bukkit::material::Crops<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Crops")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/CropState;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Crops(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Crops<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Crops")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Crops(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Crops from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Crops") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Crops object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn state(&mut self) -> Result<crate::bukkit::CropState<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getState",
            "()Lorg/bukkit/CropState;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::CropState(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::CropState::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_state(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::CropState<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setState",
            "(Lorg/bukkit/CropState;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Crops<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct SpawnEgg<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for SpawnEgg<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SpawnEgg<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::SpawnEgg<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/SpawnEgg")?;
        let res = jni.new_object(cls, "(B)V", &[jni::objects::JValueGen::from(&val_0)])?;
        let ret = { crate::bukkit::material::SpawnEgg(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::SpawnEgg<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/SpawnEgg")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::SpawnEgg(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SpawnEgg from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("SpawnEgg") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpawnEgg object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    #[deprecated]
    pub fn spawned_type(
        &mut self,
    ) -> Result<crate::bukkit::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnedType",
            "()Lorg/bukkit/entity/EntityType;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::EntityType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::EntityType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_spawned_type(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::EntityType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnedType",
            "(Lorg/bukkit/entity/EntityType;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for SpawnEgg<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct MonsterEggs<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for MonsterEggs<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MonsterEggs<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::MonsterEggs<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/MonsterEggs")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::MonsterEggs(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MonsterEggs from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MonsterEggs") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MonsterEggs object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn material(&mut self) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaterial",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_material(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaterial",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::TexturedMaterial<'mc>> for MonsterEggs<'mc> {
    fn into(self) -> crate::bukkit::material::TexturedMaterial<'mc> {
        crate::bukkit::material::TexturedMaterial::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Furnace<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Furnace<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Furnace<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>>,
    ) -> Result<crate::bukkit::material::Furnace<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Furnace")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Furnace(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Furnace<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Furnace")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Furnace(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Furnace from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Furnace") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Furnace object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::Furnace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/Furnace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::Furnace(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::FurnaceAndDispenser<'mc>> for Furnace<'mc> {
    fn into(self) -> crate::bukkit::material::FurnaceAndDispenser<'mc> {
        crate::bukkit::material::FurnaceAndDispenser::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Rails<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Rails<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Rails<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Rails<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Rails")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Rails(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Rails from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Rails") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Rails object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn direction(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDirection",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDirection",
            "(Lorg/bukkit/block/BlockFace;Z)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_curve(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCurve", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn is_on_slope(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnSlope", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Rails<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Sapling<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Sapling<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Sapling<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>>,
    ) -> Result<crate::bukkit::material::Sapling<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Sapling")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/TreeSpecies;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Sapling(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Sapling<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Sapling")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Sapling(jni, res) };
        Ok(ret)
    }
    pub fn new_with_tree_species(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>>,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::bukkit::material::Sapling<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        // 1
        let val_2 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Sapling")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;Lorg/bukkit/TreeSpecies;Z)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = { crate::bukkit::material::Sapling(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Sapling from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Sapling") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Sapling object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_instant_growable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInstantGrowable", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_is_instant_growable(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setIsInstantGrowable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_species(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpecies",
            "(Lorg/bukkit/TreeSpecies;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn species(
        &mut self,
    ) -> Result<crate::bukkit::TreeSpecies<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpecies",
            "()Lorg/bukkit/TreeSpecies;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::TreeSpecies(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::TreeSpecies::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Wood<'mc>> for Sapling<'mc> {
    fn into(self) -> crate::bukkit::material::Wood<'mc> {
        crate::bukkit::material::Wood::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Redstone. Needed for returning it from Java.
pub struct Redstone<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Redstone<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Redstone from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Redstone") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Redstone object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> crate::JNIRaw<'mc> for Redstone<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct PistonExtensionMaterial<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for PistonExtensionMaterial<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PistonExtensionMaterial<'mc> {
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::PistonExtensionMaterial<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/PistonExtensionMaterial")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::PistonExtensionMaterial(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PistonExtensionMaterial from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PistonExtensionMaterial") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PistonExtensionMaterial object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::PistonExtensionMaterial<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/PistonExtensionMaterial;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::PistonExtensionMaterial(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn attached_face(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttachedFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_sticky(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSticky", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_sticky(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSticky",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Attachable<'mc>> for PistonExtensionMaterial<'mc> {
    fn into(self) -> crate::bukkit::material::Attachable<'mc> {
        crate::bukkit::material::Attachable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for PistonExtensionMaterial<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Lever<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Lever<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Lever<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Lever<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Lever")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Lever(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Lever from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Lever") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Lever object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn attached_face(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttachedFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_powered(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Redstone<'mc>> for Lever<'mc> {
    fn into(self) -> crate::bukkit::material::Redstone<'mc> {
        crate::bukkit::material::Redstone::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::SimpleAttachableMaterialData<'mc>> for Lever<'mc> {
    fn into(self) -> crate::bukkit::material::SimpleAttachableMaterialData<'mc> {
        crate::bukkit::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
pub struct DirectionalContainer<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for DirectionalContainer<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DirectionalContainer<'mc> {
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::DirectionalContainer<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/DirectionalContainer")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::DirectionalContainer(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DirectionalContainer from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("DirectionalContainer") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DirectionalContainer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Directional<'mc>> for DirectionalContainer<'mc> {
    fn into(self) -> crate::bukkit::material::Directional<'mc> {
        crate::bukkit::material::Directional::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for DirectionalContainer<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct NetherWarts<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for NetherWarts<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> NetherWarts<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::NetherWartsState<'mc>>>,
    ) -> Result<crate::bukkit::material::NetherWarts<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/NetherWarts")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/NetherWartsState;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::NetherWarts(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::NetherWarts<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/NetherWarts")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::NetherWarts(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate NetherWarts from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("NetherWarts") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a NetherWarts object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn state(
        &mut self,
    ) -> Result<crate::bukkit::NetherWartsState<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getState",
            "()Lorg/bukkit/NetherWartsState;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::NetherWartsState(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::NetherWartsState::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_state(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::NetherWartsState<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setState",
            "(Lorg/bukkit/NetherWartsState;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for NetherWarts<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct MaterialData<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for MaterialData<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MaterialData<'mc> {
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/MaterialData")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::MaterialData(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MaterialData from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MaterialData") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MaterialData object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
pub struct LongGrass<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for LongGrass<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> LongGrass<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::GrassSpecies<'mc>>>,
    ) -> Result<crate::bukkit::material::LongGrass<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/LongGrass")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/GrassSpecies;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::LongGrass(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::LongGrass<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/LongGrass")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::LongGrass(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LongGrass from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("LongGrass") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LongGrass object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn set_species(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::GrassSpecies<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpecies",
            "(Lorg/bukkit/GrassSpecies;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn species(
        &mut self,
    ) -> Result<crate::bukkit::GrassSpecies<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpecies",
            "()Lorg/bukkit/GrassSpecies;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::GrassSpecies(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::GrassSpecies::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for LongGrass<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Tripwire<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Tripwire<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Tripwire<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Tripwire<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Tripwire")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Tripwire(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Tripwire from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Tripwire") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Tripwire object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn is_activated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isActivated", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_activated(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setActivated",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_object_triggering(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isObjectTriggering", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_object_triggering(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setObjectTriggering",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Tripwire<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct TexturedMaterial<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for TexturedMaterial<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TexturedMaterial<'mc> {
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::TexturedMaterial<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/TexturedMaterial")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::TexturedMaterial(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate TexturedMaterial from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("TexturedMaterial") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TexturedMaterial object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn material(&mut self) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaterial",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_material(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaterial",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for TexturedMaterial<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements PressureSensor. Needed for returning it from Java.
pub struct PressureSensor<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PressureSensor<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PressureSensor from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PressureSensor") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PressureSensor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn is_pressed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPressed", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> crate::JNIRaw<'mc> for PressureSensor<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct Dispenser<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Dispenser<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Dispenser<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>>,
    ) -> Result<crate::bukkit::material::Dispenser<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Dispenser")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Dispenser(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Dispenser<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Dispenser")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Dispenser(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Dispenser from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Dispenser") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Dispenser object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::Dispenser<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/Dispenser;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::Dispenser(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::FurnaceAndDispenser<'mc>> for Dispenser<'mc> {
    fn into(self) -> crate::bukkit::material::FurnaceAndDispenser<'mc> {
        crate::bukkit::material::FurnaceAndDispenser::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Cake<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Cake<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Cake<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Cake<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Cake")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Cake(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Cake from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Cake") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Cake object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn slices_eaten(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSlicesEaten", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn slices_remaining(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSlicesRemaining", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_slices_eaten(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlicesEaten",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_slices_remaining(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlicesRemaining",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Cake<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Gate<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Gate<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Gate<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Gate<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Gate")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Gate(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Gate from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Gate") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Gate object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_open(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOpen", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_open(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOpen",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Directional<'mc>> for Gate<'mc> {
    fn into(self) -> crate::bukkit::material::Directional<'mc> {
        crate::bukkit::material::Directional::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::Openable<'mc>> for Gate<'mc> {
    fn into(self) -> crate::bukkit::material::Openable<'mc> {
        crate::bukkit::material::Openable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Gate<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Step<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Step<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Step<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Step<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Step")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Step(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Step from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Step") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Step object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_inverted(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInverted", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_inverted(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInverted",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn material(&mut self) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaterial",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_material(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaterial",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::TexturedMaterial<'mc>> for Step<'mc> {
    fn into(self) -> crate::bukkit::material::TexturedMaterial<'mc> {
        crate::bukkit::material::TexturedMaterial::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct SmoothBrick<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for SmoothBrick<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SmoothBrick<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::SmoothBrick<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/SmoothBrick")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::SmoothBrick(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SmoothBrick from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("SmoothBrick") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SmoothBrick object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn material(&mut self) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaterial",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_material(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaterial",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::TexturedMaterial<'mc>> for SmoothBrick<'mc> {
    fn into(self) -> crate::bukkit::material::TexturedMaterial<'mc> {
        crate::bukkit::material::TexturedMaterial::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Leaves<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Leaves<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Leaves<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>>,
    ) -> Result<crate::bukkit::material::Leaves<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Leaves")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/TreeSpecies;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Leaves(jni, res) };
        Ok(ret)
    }
    pub fn new_with_tree_species(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>>,
    ) -> Result<crate::bukkit::material::Leaves<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Leaves")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;Lorg/bukkit/TreeSpecies;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Leaves(jni, res) };
        Ok(ret)
    }
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>>,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::bukkit::material::Leaves<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        // 1
        let val_2 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Leaves")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;Lorg/bukkit/TreeSpecies;Z)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = { crate::bukkit::material::Leaves(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Leaves from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Leaves") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Leaves object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_decayable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDecayable", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_decayable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDecayable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_decaying(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDecaying", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_decaying(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDecaying",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_species(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpecies",
            "(Lorg/bukkit/TreeSpecies;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn species(
        &mut self,
    ) -> Result<crate::bukkit::TreeSpecies<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpecies",
            "()Lorg/bukkit/TreeSpecies;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::TreeSpecies(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::TreeSpecies::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Wood<'mc>> for Leaves<'mc> {
    fn into(self) -> crate::bukkit::material::Wood<'mc> {
        crate::bukkit::material::Wood::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Dye<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Dye<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Dye<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
    ) -> Result<crate::bukkit::material::Dye<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Dye")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Dye(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Dye<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Dye")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Dye(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Dye from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Dye") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Dye object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn set_color(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/DyeColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn color(&mut self) -> Result<crate::bukkit::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::DyeColor(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::DyeColor::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Colorable<'mc>> for Dye<'mc> {
    fn into(self) -> crate::bukkit::material::Colorable<'mc> {
        crate::bukkit::material::Colorable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Dye<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct PressurePlate<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for PressurePlate<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PressurePlate<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::PressurePlate<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/PressurePlate")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::PressurePlate(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PressurePlate from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PressurePlate") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PressurePlate object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_pressed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPressed", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::PressureSensor<'mc>> for PressurePlate<'mc> {
    fn into(self) -> crate::bukkit::material::PressureSensor<'mc> {
        crate::bukkit::material::PressureSensor::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for PressurePlate<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Observer<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Observer<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Observer<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>>,
    ) -> Result<crate::bukkit::material::Observer<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Observer")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Observer(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Observer<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Observer")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Observer(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Observer from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Observer") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Observer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Directional<'mc>> for Observer<'mc> {
    fn into(self) -> crate::bukkit::material::Directional<'mc> {
        crate::bukkit::material::Directional::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::Redstone<'mc>> for Observer<'mc> {
    fn into(self) -> crate::bukkit::material::Redstone<'mc> {
        crate::bukkit::material::Redstone::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Observer<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Stairs<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Stairs<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Stairs<'mc> {
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Stairs<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Stairs")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Stairs(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Stairs from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Stairs") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Stairs object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::Stairs<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/Stairs;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::Stairs(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_inverted(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInverted", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_inverted(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInverted",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn ascending_direction(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAscendingDirection",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn descending_direction(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDescendingDirection",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Directional<'mc>> for Stairs<'mc> {
    fn into(self) -> crate::bukkit::material::Directional<'mc> {
        crate::bukkit::material::Directional::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Stairs<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Sandstone<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Sandstone<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Sandstone<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::SandstoneType<'mc>>>,
    ) -> Result<crate::bukkit::material::Sandstone<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Sandstone")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/SandstoneType;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Sandstone(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Sandstone<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Sandstone")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Sandstone(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Sandstone from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Sandstone") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Sandstone object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn get_type(
        &mut self,
    ) -> Result<crate::bukkit::SandstoneType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/SandstoneType;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::SandstoneType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::SandstoneType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::SandstoneType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/SandstoneType;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Sandstone<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct EnderChest<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for EnderChest<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EnderChest<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>>,
    ) -> Result<crate::bukkit::material::EnderChest<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/EnderChest")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::EnderChest(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::EnderChest<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/EnderChest")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::EnderChest(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EnderChest from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("EnderChest") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnderChest object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::EnderChest<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/EnderChest;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::EnderChest(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::DirectionalContainer<'mc>> for EnderChest<'mc> {
    fn into(self) -> crate::bukkit::material::DirectionalContainer<'mc> {
        crate::bukkit::material::DirectionalContainer::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct FurnaceAndDispenser<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for FurnaceAndDispenser<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FurnaceAndDispenser<'mc> {
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::FurnaceAndDispenser<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/FurnaceAndDispenser")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::FurnaceAndDispenser(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FurnaceAndDispenser from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("FurnaceAndDispenser") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FurnaceAndDispenser object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::FurnaceAndDispenser<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/FurnaceAndDispenser;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::FurnaceAndDispenser(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::DirectionalContainer<'mc>> for FurnaceAndDispenser<'mc> {
    fn into(self) -> crate::bukkit::material::DirectionalContainer<'mc> {
        crate::bukkit::material::DirectionalContainer::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Bed<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Bed<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Bed<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>>,
    ) -> Result<crate::bukkit::material::Bed<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Bed")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Bed(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Bed<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Bed")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Bed(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Bed from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Bed") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Bed object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_head_of_bed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isHeadOfBed", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_head_of_bed(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHeadOfBed",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Directional<'mc>> for Bed<'mc> {
    fn into(self) -> crate::bukkit::material::Directional<'mc> {
        crate::bukkit::material::Directional::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Bed<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct RedstoneWire<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for RedstoneWire<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RedstoneWire<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::RedstoneWire<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/RedstoneWire")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::RedstoneWire(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate RedstoneWire from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("RedstoneWire") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RedstoneWire object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Redstone<'mc>> for RedstoneWire<'mc> {
    fn into(self) -> crate::bukkit::material::Redstone<'mc> {
        crate::bukkit::material::Redstone::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for RedstoneWire<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct WoodenStep<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for WoodenStep<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> WoodenStep<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::WoodenStep<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/WoodenStep")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::WoodenStep(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate WoodenStep from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("WoodenStep") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WoodenStep object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_inverted(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInverted", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_inverted(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInverted",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_species(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpecies",
            "(Lorg/bukkit/TreeSpecies;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn species(
        &mut self,
    ) -> Result<crate::bukkit::TreeSpecies<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpecies",
            "()Lorg/bukkit/TreeSpecies;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::TreeSpecies(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::TreeSpecies::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Wood<'mc>> for WoodenStep<'mc> {
    fn into(self) -> crate::bukkit::material::Wood<'mc> {
        crate::bukkit::material::Wood::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Vine<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Vine<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Vine<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Vine<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Vine")?;
        let res = jni.new_object(cls, "(B)V", &[jni::objects::JValueGen::from(&val_0)])?;
        let ret = { crate::bukkit::material::Vine(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_enum_set(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Vine<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Vine")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Vine(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Vine from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Vine") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Vine object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_on_face(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isOnFace",
            "(Lorg/bukkit/block/BlockFace;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn put_on_face(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putOnFace",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn remove_from_face(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeFromFace",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Vine<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Wood<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Wood<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Wood<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>>,
    ) -> Result<crate::bukkit::material::Wood<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Wood")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/TreeSpecies;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Wood(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Wood<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Wood")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Wood(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Wood from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Wood") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Wood object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_species(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpecies",
            "(Lorg/bukkit/TreeSpecies;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn species(
        &mut self,
    ) -> Result<crate::bukkit::TreeSpecies<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpecies",
            "()Lorg/bukkit/TreeSpecies;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::TreeSpecies(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::TreeSpecies::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Wood<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Banner<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Banner<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Banner<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Banner<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Banner")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Banner(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Banner from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Banner") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Banner object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn attached_face(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttachedFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_wall_banner(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isWallBanner", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Attachable<'mc>> for Banner<'mc> {
    fn into(self) -> crate::bukkit::material::Attachable<'mc> {
        crate::bukkit::material::Attachable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Banner<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Mushroom<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Mushroom<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Mushroom<'mc> {
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>>,
    ) -> Result<crate::bukkit::material::Mushroom<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Mushroom")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;Lorg/bukkit/block/BlockFace;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Mushroom(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Mushroom from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Mushroom") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Mushroom object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn is_stem(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isStem", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn set_stem(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setStem", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn block_texture(
        &mut self,
    ) -> Result<crate::bukkit::material::types::MushroomBlockTexture<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockTexture",
            "()Lorg/bukkit/material/types/MushroomBlockTexture;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::material::types::MushroomBlockTexture(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::material::types::MushroomBlockTexture::from_string(variant_str)
                    .unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_block_texture(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::material::types::MushroomBlockTexture<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockTexture",
            "(Lorg/bukkit/material/types/MushroomBlockTexture;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_face_painted(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFacePainted",
            "(Lorg/bukkit/block/BlockFace;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn set_face_painted(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacePainted",
            "(Lorg/bukkit/block/BlockFace;Z)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Mushroom<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct RedstoneTorch<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for RedstoneTorch<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RedstoneTorch<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::RedstoneTorch<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/RedstoneTorch")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::RedstoneTorch(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate RedstoneTorch from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("RedstoneTorch") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RedstoneTorch object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::RedstoneTorch<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/RedstoneTorch;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::RedstoneTorch(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn attached_face(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttachedFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Redstone<'mc>> for RedstoneTorch<'mc> {
    fn into(self) -> crate::bukkit::material::Redstone<'mc> {
        crate::bukkit::material::Redstone::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::Torch<'mc>> for RedstoneTorch<'mc> {
    fn into(self) -> crate::bukkit::material::Torch<'mc> {
        crate::bukkit::material::Torch::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct CocoaPlant<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub struct CocoaPlantCocoaPlantSize<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for CocoaPlantCocoaPlantSize<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CocoaPlantCocoaPlantSize<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CocoaPlantCocoaPlantSize from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("CocoaPlantCocoaPlantSize") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CocoaPlantCocoaPlantSize object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn value_of_with_string(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let cls = &jni.find_class("java/lang/Enum")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "name", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn compare_to_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compareTo",
            "(Ljava/lang/Enum;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn describe_constable(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            "()Ljava/util/Optional;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
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
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for CocoaPlant<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CocoaPlant<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::CocoaPlant<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/CocoaPlant")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::CocoaPlant(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CocoaPlant from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("CocoaPlant") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CocoaPlant object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn size(
        &mut self,
    ) -> Result<crate::bukkit::material::CocoaPlantCocoaPlantSize<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSize",
            "()Lorg/bukkit/material/CocoaPlant$CocoaPlantSize;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::CocoaPlantCocoaPlantSize(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_size(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::material::CocoaPlantCocoaPlantSize<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSize",
            "(Lorg/bukkit/material/CocoaPlant$CocoaPlantSize;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn attached_face(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttachedFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Directional<'mc>> for CocoaPlant<'mc> {
    fn into(self) -> crate::bukkit::material::Directional<'mc> {
        crate::bukkit::material::Directional::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::Attachable<'mc>> for CocoaPlant<'mc> {
    fn into(self) -> crate::bukkit::material::Attachable<'mc> {
        crate::bukkit::material::Attachable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for CocoaPlant<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Door<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Door<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Door<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>>,
        arg1: std::option::Option<impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>>,
    ) -> Result<crate::bukkit::material::Door<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Door")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/TreeSpecies;Lorg/bukkit/block/BlockFace;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Door(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_tree_species(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Door<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Door")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Door(jni, res) };
        Ok(ret)
    }
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::bukkit::material::Door<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        // 2
        let val_2 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Door")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;Lorg/bukkit/block/BlockFace;Z)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = { crate::bukkit::material::Door(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Door from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Door") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Door object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn is_open(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOpen", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_open(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOpen",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn hinge(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHinge", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_hinge(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHinge",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_top_half(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTopHalf",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn get_wood_door_of_species(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::TreeSpecies<'mc>>,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/Material")?;
        let res = jni.call_static_method(
            cls,
            "getWoodDoorOfSpecies",
            "(Lorg/bukkit/TreeSpecies;)Lorg/bukkit/Material;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let mut obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                jni,
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_top_half(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isTopHalf", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn hinge_corner(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHingeCorner",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Directional<'mc>> for Door<'mc> {
    fn into(self) -> crate::bukkit::material::Directional<'mc> {
        crate::bukkit::material::Directional::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::Openable<'mc>> for Door<'mc> {
    fn into(self) -> crate::bukkit::material::Openable<'mc> {
        crate::bukkit::material::Openable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Door<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Directional. Needed for returning it from Java.
pub struct Directional<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Directional<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Directional from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Directional") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Directional object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for Directional<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct ExtendedRails<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for ExtendedRails<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ExtendedRails<'mc> {
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::ExtendedRails<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/ExtendedRails")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::ExtendedRails(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ExtendedRails from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ExtendedRails") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ExtendedRails object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDirection",
            "(Lorg/bukkit/block/BlockFace;Z)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_curve(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCurve", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn direction(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDirection",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_on_slope(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnSlope", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Rails<'mc>> for ExtendedRails<'mc> {
    fn into(self) -> crate::bukkit::material::Rails<'mc> {
        crate::bukkit::material::Rails::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Cauldron<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Cauldron<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Cauldron<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Cauldron<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Cauldron")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Cauldron(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Cauldron from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Cauldron") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Cauldron object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn is_full(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFull", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Cauldron<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Ladder<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Ladder<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Ladder<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Ladder<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Ladder")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Ladder(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Ladder from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Ladder") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Ladder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::Ladder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/Ladder;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::Ladder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn attached_face(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttachedFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::SimpleAttachableMaterialData<'mc>> for Ladder<'mc> {
    fn into(self) -> crate::bukkit::material::SimpleAttachableMaterialData<'mc> {
        crate::bukkit::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
pub struct Command<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Command<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Command<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Command<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Command")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Command(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Command from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Command") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Command object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_powered(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Redstone<'mc>> for Command<'mc> {
    fn into(self) -> crate::bukkit::material::Redstone<'mc> {
        crate::bukkit::material::Redstone::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Command<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct DetectorRail<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for DetectorRail<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DetectorRail<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::DetectorRail<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/DetectorRail")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::DetectorRail(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DetectorRail from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("DetectorRail") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DetectorRail object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::material::DetectorRail<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/material/DetectorRail;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::DetectorRail(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_pressed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPressed", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_pressed(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPressed",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDirection",
            "(Lorg/bukkit/block/BlockFace;Z)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_curve(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCurve", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn direction(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDirection",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_on_slope(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnSlope", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::PressureSensor<'mc>> for DetectorRail<'mc> {
    fn into(self) -> crate::bukkit::material::PressureSensor<'mc> {
        crate::bukkit::material::PressureSensor::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::ExtendedRails<'mc>> for DetectorRail<'mc> {
    fn into(self) -> crate::bukkit::material::ExtendedRails<'mc> {
        crate::bukkit::material::ExtendedRails::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Hopper<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Hopper<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Hopper<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
    ) -> Result<crate::bukkit::material::Hopper<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Hopper")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Hopper(jni, res) };
        Ok(ret)
    }
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
        arg1: std::option::Option<bool>,
    ) -> Result<crate::bukkit::material::Hopper<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        // 1
        let val_1 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Hopper")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/BlockFace;Z)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Hopper(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Hopper from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Hopper") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Hopper object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn is_active(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isActive", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_active(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setActive",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Directional<'mc>> for Hopper<'mc> {
    fn into(self) -> crate::bukkit::material::Directional<'mc> {
        crate::bukkit::material::Directional::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::Redstone<'mc>> for Hopper<'mc> {
    fn into(self) -> crate::bukkit::material::Redstone<'mc> {
        crate::bukkit::material::Redstone::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Hopper<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Skull<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Skull<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Skull<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>>,
    ) -> Result<crate::bukkit::material::Skull<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Skull")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Skull(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Skull<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Skull")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Skull(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Skull from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Skull") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Skull object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Directional<'mc>> for Skull<'mc> {
    fn into(self) -> crate::bukkit::material::Directional<'mc> {
        crate::bukkit::material::Directional::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Skull<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Sign<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Sign<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Sign<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Material<'mc>>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Sign<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Sign")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Sign(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Sign from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Sign") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Sign object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn attached_face(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttachedFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_wall_sign(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isWallSign", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Attachable<'mc>> for Sign<'mc> {
    fn into(self) -> crate::bukkit::material::Attachable<'mc> {
        crate::bukkit::material::Attachable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Sign<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct Wool<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Wool<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Wool<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::DyeColor<'mc>>>,
    ) -> Result<crate::bukkit::material::Wool<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/Wool")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/DyeColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::material::Wool(jni, res) };
        Ok(ret)
    }
    #[deprecated]
    pub fn new_with_material(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::bukkit::Material<'mc>>,
        arg1: std::option::Option<i8>,
    ) -> Result<crate::bukkit::material::Wool<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Byte(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/material/Wool")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Material;B)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::material::Wool(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Wool from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Wool") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Wool object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn set_color(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/DyeColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn color(&mut self) -> Result<crate::bukkit::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::DyeColor(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::DyeColor::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn set_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn item_type(
        &mut self,
    ) -> Result<crate::bukkit::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Material(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Material::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn to_item_stack(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toItemStack",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::bukkit::material::Colorable<'mc>> for Wool<'mc> {
    fn into(self) -> crate::bukkit::material::Colorable<'mc> {
        crate::bukkit::material::Colorable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::material::MaterialData<'mc>> for Wool<'mc> {
    fn into(self) -> crate::bukkit::material::MaterialData<'mc> {
        crate::bukkit::material::MaterialData::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Attachable. Needed for returning it from Java.
pub struct Attachable<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Attachable<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Attachable from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Attachable") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Attachable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn attached_face(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttachedFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_facing_direction(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacingDirection",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for Attachable<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::material::Directional<'mc>> for Attachable<'mc> {
    fn into(self) -> crate::bukkit::material::Directional<'mc> {
        crate::bukkit::material::Directional::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub mod types;
