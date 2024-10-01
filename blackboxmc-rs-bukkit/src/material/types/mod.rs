#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub enum MushroomBlockTexture<'mc> {}
impl<'mc> std::fmt::Display for MushroomBlockTexture<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {}
    }
}

impl<'mc> MushroomBlockTexture<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<MushroomBlockTexture<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/material/types/MushroomBlockTexture");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/material/types/MushroomBlockTexture;",
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
            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct MushroomBlockTextureStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MushroomBlockTexture<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {}
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {}
    }
}
impl<'mc> JNIInstantiatable<'mc> for MushroomBlockTexture<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MushroomBlockTexture from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/material/types/MushroomBlockTexture")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MushroomBlockTexture object, got {}",
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
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for MushroomBlockTextureStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MushroomBlockTextureStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate MushroomBlockTextureStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/material/types/MushroomBlockTexture")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MushroomBlockTextureStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MushroomBlockTextureStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::material::types::MushroomBlockTexture<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/material/types/MushroomBlockTexture;");
        let cls = jni.find_class("org/bukkit/material/types/MushroomBlockTexture");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::material::types::MushroomBlockTexture::from_raw(&jni, obj)
    }
    #[deprecated]
    /// Gets the associated data value representing this mushroom block face.
    pub fn data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    /// Gets the face that has cap texture.
    pub fn cap_face(
        &self,
    ) -> Result<Option<crate::block::BlockFace<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCapFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    #[deprecated]
    /// Gets the MushroomBlockType with the given data value.
    pub fn get_by_data(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        data: i8,
    ) -> Result<Option<crate::material::types::MushroomBlockTexture<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(B)Lorg/bukkit/material/types/MushroomBlockTexture;");
        let val_1 = jni::objects::JValueGen::Byte(data);
        let cls = jni.find_class("org/bukkit/material/types/MushroomBlockTexture");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        let obj = res.l()?;
        Ok(Some(
            crate::material::types::MushroomBlockTexture::from_raw(&jni, obj)?,
        ))
    }
    /// Gets the MushroomBlockType with cap texture on the given block face.
    pub fn get_cap_by_face(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        face: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<Option<crate::material::types::MushroomBlockTexture<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Lorg/bukkit/block/BlockFace;)Lorg/bukkit/material/types/MushroomBlockTexture;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/material/types/MushroomBlockTexture");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getCapByFace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        let obj = res.l()?;
        Ok(Some(
            crate::material::types::MushroomBlockTexture::from_raw(&jni, obj)?,
        ))
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
