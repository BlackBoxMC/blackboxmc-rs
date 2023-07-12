use crate::JNIRaw;
/// An instantiatable struct that implements Palette. Needed for returning it from Java.
pub struct Palette<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Palette<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn block_count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockCount", "()I", &[])?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> crate::JNIRaw<'mc> for Palette<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements StructureManager. Needed for returning it from Java.
pub struct StructureManager<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> StructureManager<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn get_structure(
        &mut self,
        arg0: crate::bukkit::NamespacedKey<'mc>,
    ) -> Result<crate::bukkit::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStructure",
            "(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/structure/Structure;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::structure::Structure(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn register_structure(
        &mut self,
        arg0: crate::bukkit::NamespacedKey<'mc>,
        arg1: crate::bukkit::structure::Structure<'mc>,
    ) -> Result<crate::bukkit::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.1.clone()) };
        let res =
self.jni_ref().call_method(&self.jni_object(),"registerStructure","(Lorg/bukkit/NamespacedKey;Lorg/bukkit/structure/Structure;)Lorg/bukkit/structure/Structure;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
        let ret = {
            crate::bukkit::structure::Structure(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn unregister_structure(
        &mut self,
        arg0: crate::bukkit::NamespacedKey<'mc>,
    ) -> Result<crate::bukkit::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "unregisterStructure",
            "(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/structure/Structure;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::structure::Structure(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn delete_structure_with_namespaced_key(
        &mut self,
        arg0: std::option::Option<crate::bukkit::NamespacedKey<'mc>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().1.clone()) };
        let val_1 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "deleteStructure",
            "(Lorg/bukkit/NamespacedKey;Z)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn create_structure(
        &mut self,
    ) -> Result<crate::bukkit::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createStructure",
            "()Lorg/bukkit/structure/Structure;",
            &[],
        )?;
        let ret = {
            crate::bukkit::structure::Structure(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn copy(
        &mut self,
        arg0: crate::bukkit::structure::Structure<'mc>,
    ) -> Result<crate::bukkit::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copy",
            "(Lorg/bukkit/structure/Structure;)Lorg/bukkit/structure/Structure;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::structure::Structure(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for StructureManager<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements Structure. Needed for returning it from Java.
pub struct Structure<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Structure<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn entity_count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityCount", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn palette_count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPaletteCount", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn size(
        &mut self,
    ) -> Result<crate::bukkit::util::BlockVector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSize",
            "()Lorg/bukkit/util/BlockVector;",
            &[],
        )?;
        let ret = {
            crate::bukkit::util::BlockVector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for Structure<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
