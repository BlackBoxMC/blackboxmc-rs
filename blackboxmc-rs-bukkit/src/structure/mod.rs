#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// An instantiatable struct that implements Palette. Needed for returning it from Java.
pub struct Palette<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Palette<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Palette from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Palette")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Palette object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn blocks(
        &mut self,
    ) -> Result<blackboxmc_java::JavaList<'mc, orgBlockState>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlocks", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn block_count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockCount", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for Palette<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements StructureManager. Needed for returning it from Java.
pub struct StructureManager<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> StructureManager<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate StructureManager from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "StructureManager")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StructureManager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn copy(
        &mut self,
        arg0: impl Into<&'mc crate::structure::Structure<'mc>>,
    ) -> Result<crate::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copy",
            "(Lorg/bukkit/structure/Structure;)Lorg/bukkit/structure/Structure;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::structure::Structure::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_structure(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
    ) -> Result<crate::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStructure",
            "(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/structure/Structure;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::structure::Structure::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn structures(
        &mut self,
    ) -> Result<
        blackboxmc_java::JavaMap<'mc, orgNamespacedKey, orgStructure>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStructures",
            "()Ljava/util/Map;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn register_structure(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
        arg1: impl Into<&'mc crate::structure::Structure<'mc>>,
    ) -> Result<crate::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"registerStructure","(Lorg/bukkit/NamespacedKey;Lorg/bukkit/structure/Structure;)Lorg/bukkit/structure/Structure;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::structure::Structure::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn unregister_structure(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
    ) -> Result<crate::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "unregisterStructure",
            "(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/structure/Structure;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::structure::Structure::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn load_structure_with_namespaced_key(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::NamespacedKey<'mc>>>,
        arg1: std::option::Option<bool>,
    ) -> Result<crate::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "loadStructure",
            "(Lorg/bukkit/NamespacedKey;Z)Lorg/bukkit/structure/Structure;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::structure::Structure::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn save_structure_with_file(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::structure::Structure<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "saveStructure",
            "(Lorg/bukkit/NamespacedKey;Lorg/bukkit/structure/Structure;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn delete_structure_with_namespaced_key(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::NamespacedKey<'mc>>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "deleteStructure",
            "(Lorg/bukkit/NamespacedKey;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn create_structure(
        &mut self,
    ) -> Result<crate::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createStructure",
            "()Lorg/bukkit/structure/Structure;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::structure::Structure::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for StructureManager<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements Structure. Needed for returning it from Java.
pub struct Structure<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Structure<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Structure from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Structure")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Structure object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn fill_with_location(
        &mut self,
        arg0: impl Into<&'mc crate::Location<'mc>>,
        arg1: impl Into<&'mc crate::Location<'mc>>,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        // 2
        let val_3 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "fill",
            "(Lorg/bukkit/Location;Lorg/bukkit/Location;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn size(&mut self) -> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSize",
            "()Lorg/bukkit/util/BlockVector;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BlockVector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn entities(
        &mut self,
    ) -> Result<blackboxmc_java::JavaList<'mc, orgEntity>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEntities",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn entity_count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityCount", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn palettes(
        &mut self,
    ) -> Result<blackboxmc_java::JavaList<'mc, orgPalette>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPalettes",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn palette_count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPaletteCount", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn place_with_location(
        &mut self,
        arg0: impl Into<&'mc crate::RegionAccessor<'mc>>,
        arg1: impl Into<&'mc crate::util::BlockVector<'mc>>,
        arg2: bool,
        arg3: impl Into<&'mc crate::block::structure::StructureRotation<'mc>>,
        arg4: impl Into<&'mc crate::block::structure::Mirror<'mc>>,
        arg5: i32,
        arg6: std::option::Option<f32>,
        arg7: std::option::Option<impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        // 6
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
        let val_7 = jni::objects::JValueGen::Float(arg6.unwrap().into());
        let val_8 =
            unsafe { jni::objects::JObject::from_raw(arg7.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"place","(Lorg/bukkit/RegionAccessor;Lorg/bukkit/util/BlockVector;ZLorg/bukkit/block/structure/StructureRotation;Lorg/bukkit/block/structure/Mirror;IFLjava/util/Random;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6),jni::objects::JValueGen::from(&val_7),jni::objects::JValueGen::from(&val_8)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for Structure<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::persistence::PersistentDataHolder<'mc>> for Structure<'mc> {
    fn into(self) -> crate::persistence::PersistentDataHolder<'mc> {
        crate::persistence::PersistentDataHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
