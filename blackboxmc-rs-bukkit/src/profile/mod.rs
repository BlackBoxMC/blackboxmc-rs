#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIInstantiatableEnum;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Provides access to the textures stored inside a <a href="PlayerProfile.html" title="interface in org.bukkit.profile"><code>PlayerProfile</code></a>.
/// <p>Modifying the textures immediately invalidates and clears any previously present attributes that are specific to official player profiles, such as the <a href="#getTimestamp()"><code>timestamp</code></a> and <a href="#isSigned()"><code>signature</code></a>.</p>
///
/// This is a representation of an abstract class.
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
    //

    pub fn is_signed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSigned", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn timestamp(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTimestamp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn skin(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/net/URL;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSkin", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

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
    //

    pub fn skin_model(
        &self,
    ) -> Result<crate::profile::PlayerTexturesSkinModel<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/profile/PlayerTextures$SkinModel;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSkinModel", sig.as_str(), vec![]);
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
        crate::profile::PlayerTexturesSkinModel::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::profile::PlayerTexturesSkinModel::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn cape(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/net/URL;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCape", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

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
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
/// The different Minecraft skin models.
#[derive(PartialEq, Eq)]
pub enum PlayerTexturesSkinModelEnum {
    Classic,
    Slim,
}
impl std::fmt::Display for PlayerTexturesSkinModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerTexturesSkinModelEnum::Classic => f.write_str("CLASSIC"),
            PlayerTexturesSkinModelEnum::Slim => f.write_str("SLIM"),
        }
    }
}
impl<'mc> std::fmt::Display for PlayerTexturesSkinModel<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
pub struct PlayerTexturesSkinModel<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub PlayerTexturesSkinModelEnum,
);
impl<'mc> std::ops::Deref for PlayerTexturesSkinModel<'mc> {
    type Target = PlayerTexturesSkinModelEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}

impl<'mc> JNIRaw<'mc> for PlayerTexturesSkinModel<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatableEnum<'mc> for PlayerTexturesSkinModel<'mc> {
    type Enum = PlayerTexturesSkinModelEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
}

impl<'mc> PlayerTexturesSkinModel<'mc> {
    pub const CLASSIC: PlayerTexturesSkinModelEnum = PlayerTexturesSkinModelEnum::Classic;
    pub const SLIM: PlayerTexturesSkinModelEnum = PlayerTexturesSkinModelEnum::Slim;
    pub fn from_string(str: String) -> std::option::Option<PlayerTexturesSkinModelEnum> {
        match str.as_str() {
            "CLASSIC" => Some(PlayerTexturesSkinModelEnum::Classic),
            "SLIM" => Some(PlayerTexturesSkinModelEnum::Slim),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<PlayerTexturesSkinModel<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/profile/PlayerTextures$SkinModel");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/profile/PlayerTextures$SkinModel;",
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
        PlayerTexturesSkinModel::from_raw(
            &jni,
            raw_obj,
            PlayerTexturesSkinModel::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    //
}
/// A player profile.
/// <p>A player profile always provides a unique id, a non-empty name, or both. Its unique id and name are immutable, but other properties (such as its textures) can be altered.</p>
/// <p>New profiles can be created via <a href="../Server.html#createPlayerProfile(java.util.UUID,java.lang.String)"><code>Server.createPlayerProfile(UUID, String)</code></a>.</p>
///
/// This is a representation of an abstract class.
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
    //

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
    //

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
    //

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
    //

    pub fn is_complete(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isComplete", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn clone(&self) -> Result<crate::profile::PlayerProfile<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
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
    //

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
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for PlayerProfile<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting PlayerProfile into crate::configuration::serialization::ConfigurationSerializable")
    }
}
#[derive(PartialEq, Eq)]
pub enum SkinModelEnum {
    Classic,
    Slim,
}
impl std::fmt::Display for SkinModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkinModelEnum::Classic => f.write_str("CLASSIC"),
            SkinModelEnum::Slim => f.write_str("SLIM"),
        }
    }
}
impl<'mc> std::fmt::Display for SkinModel<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
pub struct SkinModel<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub SkinModelEnum,
);
impl<'mc> std::ops::Deref for SkinModel<'mc> {
    type Target = SkinModelEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}

impl<'mc> JNIRaw<'mc> for SkinModel<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatableEnum<'mc> for SkinModel<'mc> {
    type Enum = SkinModelEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
}

impl<'mc> SkinModel<'mc> {
    pub const CLASSIC: SkinModelEnum = SkinModelEnum::Classic;
    pub const SLIM: SkinModelEnum = SkinModelEnum::Slim;
    pub fn from_string(str: String) -> std::option::Option<SkinModelEnum> {
        match str.as_str() {
            "CLASSIC" => Some(SkinModelEnum::Classic),
            "SLIM" => Some(SkinModelEnum::Slim),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<SkinModel<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/profile/SkinModel");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/profile/SkinModel;",
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
        SkinModel::from_raw(
            &jni,
            raw_obj,
            SkinModel::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
