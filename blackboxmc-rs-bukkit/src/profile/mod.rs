#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Provides access to the textures stored inside a <a title="interface in org.bukkit.profile" href="PlayerProfile.html"><code>PlayerProfile</code></a>.
/// <p>Modifying the textures immediately invalidates and clears any previously present attributes that are specific to official player profiles, such as the <a href="#getTimestamp()"><code>timestamp</code></a> and <a href="#isSigned()"><code>signature</code></a>.</p>
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct PlayerTextures<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PlayerTextures<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PlayerTextures<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PlayerTextures from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/profile/PlayerTextures")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PlayerTextures object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PlayerTextures<'mc> {
    pub fn timestamp(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTimestamp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn is_signed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSigned", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn skin(&self) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/net/URL;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSkin", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }

    pub fn set_skin_with_url(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<impl Into<crate::profile::PlayerTexturesSkinModel<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/net/URL;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/profile/PlayerTextures$SkinModel;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setSkin", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn skin_model(
        &self,
    ) -> Result<crate::profile::PlayerTexturesSkinModel<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/profile/PlayerTextures$SkinModel;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSkinModel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::profile::PlayerTexturesSkinModel::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn cape(&self) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/net/URL;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCape", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }

    pub fn set_cape(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/net/URL;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCape",
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

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// The different Minecraft skin models.
pub enum PlayerTexturesSkinModel<'mc> {
    Classic {
        inner: PlayerTexturesSkinModelStruct<'mc>,
    },
    Slim {
        inner: PlayerTexturesSkinModelStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for PlayerTexturesSkinModel<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerTexturesSkinModel::Classic { .. } => f.write_str("CLASSIC"),
            PlayerTexturesSkinModel::Slim { .. } => f.write_str("SLIM"),
        }
    }
}

impl<'mc> PlayerTexturesSkinModel<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<PlayerTexturesSkinModel<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/profile/PlayerTextures$SkinModel");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/profile/PlayerTextures$SkinModel;",
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
            "CLASSIC" => Ok(PlayerTexturesSkinModel::Classic {
                inner: PlayerTexturesSkinModelStruct::from_raw(env, obj)?,
            }),
            "SLIM" => Ok(PlayerTexturesSkinModel::Slim {
                inner: PlayerTexturesSkinModelStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct PlayerTexturesSkinModelStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PlayerTexturesSkinModel<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Classic { inner } => inner.0.clone(),
            Self::Slim { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Classic { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Slim { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PlayerTexturesSkinModel<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PlayerTexturesSkinModel from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/profile/PlayerTextures$SkinModel")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PlayerTexturesSkinModel object, got {}",
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
                "CLASSIC" => Ok(PlayerTexturesSkinModel::Classic {
                    inner: PlayerTexturesSkinModelStruct::from_raw(env, obj)?,
                }),
                "SLIM" => Ok(PlayerTexturesSkinModel::Slim {
                    inner: PlayerTexturesSkinModelStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for PlayerTexturesSkinModelStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PlayerTexturesSkinModelStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PlayerTexturesSkinModelStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/profile/PlayerTextures$SkinModel")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PlayerTexturesSkinModelStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PlayerTexturesSkinModelStruct<'mc> {
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// A player profile.
/// <p>A player profile always provides a unique id, a non-empty name, or both. Its unique id and name are immutable, but other properties (such as its textures) can be altered.</p>
/// <p>New profiles can be created via <a href="../Server.html#createPlayerProfile(java.util.UUID,java.lang.String)"><code>Server.createPlayerProfile(UUID, String)</code></a>.</p>
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct PlayerProfile<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PlayerProfile<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PlayerProfile<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PlayerProfile from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/profile/PlayerProfile")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PlayerProfile object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PlayerProfile<'mc> {
    pub fn unique_id(
        &self,
    ) -> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/UUID;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn textures(
        &self,
    ) -> Result<crate::profile::PlayerTextures<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/profile/PlayerTextures;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTextures", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::profile::PlayerTextures::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_textures(
        &self,
        arg0: impl Into<crate::profile::PlayerTextures<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/profile/PlayerTextures;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTextures",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_complete(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isComplete", sig.as_str(), vec![]);
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

    pub fn clone(&self) -> Result<crate::profile::PlayerProfile<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/profile/PlayerProfile;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::profile::PlayerProfile::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = PlayerProfile::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for PlayerProfile<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting PlayerProfile into crate::configuration::serialization::ConfigurationSerializable")
    }
}
pub enum SkinModel<'mc> {
    Classic { inner: SkinModelStruct<'mc> },
    Slim { inner: SkinModelStruct<'mc> },
}
impl<'mc> std::fmt::Display for SkinModel<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkinModel::Classic { .. } => f.write_str("CLASSIC"),
            SkinModel::Slim { .. } => f.write_str("SLIM"),
        }
    }
}

impl<'mc> SkinModel<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<SkinModel<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/profile/SkinModel");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/profile/SkinModel;",
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
            "CLASSIC" => Ok(SkinModel::Classic {
                inner: SkinModelStruct::from_raw(env, obj)?,
            }),
            "SLIM" => Ok(SkinModel::Slim {
                inner: SkinModelStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct SkinModelStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SkinModel<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Classic { inner } => inner.0.clone(),
            Self::Slim { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Classic { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Slim { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SkinModel<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SkinModel from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/profile/SkinModel")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SkinModel object, got {}",
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
                "CLASSIC" => Ok(SkinModel::Classic {
                    inner: SkinModelStruct::from_raw(env, obj)?,
                }),
                "SLIM" => Ok(SkinModel::Slim {
                    inner: SkinModelStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for SkinModelStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SkinModelStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SkinModelStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/profile/SkinModel")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SkinModelStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SkinModelStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
