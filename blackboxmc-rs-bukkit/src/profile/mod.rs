#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
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
        let cls = env.find_class("org/bukkit/profile/PlayerTextures/SkinModel");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/profile/PlayerTextures/SkinModel;",
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
            env.validate_name(&obj, "org/bukkit/profile/PlayerTextures/SkinModel")?;
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
            env.validate_name(&obj, "org/bukkit/profile/PlayerTextures/SkinModel")?;
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
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::profile::PlayerTexturesSkinModel<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/profile/PlayerTextures/SkinModel;");
        let cls = jni.find_class("org/bukkit/profile/PlayerTextures/SkinModel");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::profile::PlayerTexturesSkinModel::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
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
    /// Checks if the profile stores no textures.
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Clears the textures.
    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the URL that points to the player's skin.
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
    /// Sets the player's skin and {@link SkinModel}.
    ///
    /// The URL <b>must</b> point to the Minecraft texture server. Example URL:
    /// <pre>
    /// http://textures.minecraft.net/texture/b3fbd454b599df593f57101bfca34e67d292a8861213d2202bb575da7fd091ac
    /// </pre>
    ///
    /// A skin model of <code>null</code> results in {@link SkinModel#CLASSIC} to
    /// be used.
    pub fn set_skin(
        &self,
        skin_url: jni::objects::JObject<'mc>,
        skin_model: std::option::Option<impl Into<crate::profile::PlayerTexturesSkinModel<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/net/URL;";
        let val_1 = jni::objects::JValueGen::Object(skin_url);
        args.push(val_1);
        if let Some(a) = skin_model {
            sig += "Lorg/bukkit/profile/PlayerTextures/SkinModel;";
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
    /// Gets the model of the player's skin.
    ///
    /// This returns {@link SkinModel#CLASSIC} if no skin is set.
    pub fn skin_model(
        &self,
    ) -> Result<crate::profile::PlayerTexturesSkinModel<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/profile/PlayerTextures/SkinModel;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSkinModel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::profile::PlayerTexturesSkinModel::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the URL that points to the player's cape.
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
    /// Sets the URL that points to the player's cape.
    ///
    /// The URL <b>must</b> point to the Minecraft texture server. Example URL:
    /// <pre>
    /// http://textures.minecraft.net/texture/2340c0e03dd24a11b15a8b33c2a7e9e32abb2051b2481d0ba7defd635ca7a933
    /// </pre>
    pub fn set_cape(
        &self,
        cape_url: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/net/URL;)V");
        let val_1 = jni::objects::JValueGen::Object(cape_url);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCape",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the timestamp at which the profile was last updated.
    pub fn timestamp(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTimestamp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    /// Checks if the textures are signed and the signature is valid.
    pub fn is_signed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSigned", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
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
    /// Gets the player's unique id.
    pub fn unique_id(
        &self,
    ) -> Result<Option<blackboxmc_java::util::JavaUUID<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/UUID;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(blackboxmc_java::util::JavaUUID::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the player name.
    pub fn name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
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
    /// Gets the {@link PlayerTextures} of this profile.
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
    /// Copies the given textures.
    pub fn set_textures(
        &self,
        textures: impl Into<crate::profile::PlayerTextures<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/profile/PlayerTextures;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(textures.into().jni_object().clone())
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
    /// Checks whether this profile is complete.
    ///
    /// A profile is currently considered complete if it has a name, a unique id,
    /// and textures.
    pub fn is_complete(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isComplete", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clone(&self) -> Result<crate::profile::PlayerProfile<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/profile/PlayerProfile;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::profile::PlayerProfile::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    for PlayerProfile<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting PlayerProfile into crate::configuration::serialization::ConfigurationSerializable")
    }
}
