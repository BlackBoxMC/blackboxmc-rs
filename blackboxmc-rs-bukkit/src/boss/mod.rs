#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub enum BarStyleEnum {
    Solid,
    Segmented6,
    Segmented10,
    Segmented12,
    Segmented20,
}
impl std::fmt::Display for BarStyleEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            BarStyleEnum::Solid => f.write_str("SOLID"),
            BarStyleEnum::Segmented6 => f.write_str("SEGMENTED_6"),
            BarStyleEnum::Segmented10 => f.write_str("SEGMENTED_10"),
            BarStyleEnum::Segmented12 => f.write_str("SEGMENTED_12"),
            BarStyleEnum::Segmented20 => f.write_str("SEGMENTED_20"),
        }
    }
}
pub struct BarStyle<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub BarStyleEnum,
);
impl<'mc> std::ops::Deref for BarStyle<'mc> {
    type Target = BarStyleEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for BarStyle<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BarStyle<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: BarStyleEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BarStyle from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/BarStyle")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BarStyle object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const SOLID: BarStyleEnum = BarStyleEnum::Solid;
    pub const SEGMENTED_6: BarStyleEnum = BarStyleEnum::Segmented6;
    pub const SEGMENTED_10: BarStyleEnum = BarStyleEnum::Segmented10;
    pub const SEGMENTED_12: BarStyleEnum = BarStyleEnum::Segmented12;
    pub const SEGMENTED_20: BarStyleEnum = BarStyleEnum::Segmented20;
    pub fn from_string(str: String) -> std::option::Option<BarStyleEnum> {
        match str.as_str() {
            "SOLID" => Some(BarStyleEnum::Solid),
            "SEGMENTED_6" => Some(BarStyleEnum::Segmented6),
            "SEGMENTED_10" => Some(BarStyleEnum::Segmented10),
            "SEGMENTED_12" => Some(BarStyleEnum::Segmented12),
            "SEGMENTED_20" => Some(BarStyleEnum::Segmented20),
            _ => None,
        }
    }
}
pub enum BarColorEnum {
    Pink,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}
impl std::fmt::Display for BarColorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            BarColorEnum::Pink => f.write_str("PINK"),
            BarColorEnum::Blue => f.write_str("BLUE"),
            BarColorEnum::Red => f.write_str("RED"),
            BarColorEnum::Green => f.write_str("GREEN"),
            BarColorEnum::Yellow => f.write_str("YELLOW"),
            BarColorEnum::Purple => f.write_str("PURPLE"),
            BarColorEnum::White => f.write_str("WHITE"),
        }
    }
}
pub struct BarColor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub BarColorEnum,
);
impl<'mc> std::ops::Deref for BarColor<'mc> {
    type Target = BarColorEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for BarColor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BarColor<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: BarColorEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BarColor from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/BarColor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BarColor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const PINK: BarColorEnum = BarColorEnum::Pink;
    pub const BLUE: BarColorEnum = BarColorEnum::Blue;
    pub const RED: BarColorEnum = BarColorEnum::Red;
    pub const GREEN: BarColorEnum = BarColorEnum::Green;
    pub const YELLOW: BarColorEnum = BarColorEnum::Yellow;
    pub const PURPLE: BarColorEnum = BarColorEnum::Purple;
    pub const WHITE: BarColorEnum = BarColorEnum::White;
    pub fn from_string(str: String) -> std::option::Option<BarColorEnum> {
        match str.as_str() {
            "PINK" => Some(BarColorEnum::Pink),
            "BLUE" => Some(BarColorEnum::Blue),
            "RED" => Some(BarColorEnum::Red),
            "GREEN" => Some(BarColorEnum::Green),
            "YELLOW" => Some(BarColorEnum::Yellow),
            "PURPLE" => Some(BarColorEnum::Purple),
            "WHITE" => Some(BarColorEnum::White),
            _ => None,
        }
    }
}
pub enum BarFlagEnum {
    DarkenSky,
    PlayBossMusic,
    CreateFog,
}
impl std::fmt::Display for BarFlagEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            BarFlagEnum::DarkenSky => f.write_str("DARKEN_SKY"),
            BarFlagEnum::PlayBossMusic => f.write_str("PLAY_BOSS_MUSIC"),
            BarFlagEnum::CreateFog => f.write_str("CREATE_FOG"),
        }
    }
}
pub struct BarFlag<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub BarFlagEnum,
);
impl<'mc> std::ops::Deref for BarFlag<'mc> {
    type Target = BarFlagEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for BarFlag<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BarFlag<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: BarFlagEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BarFlag from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/BarFlag")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BarFlag object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const DARKEN_SKY: BarFlagEnum = BarFlagEnum::DarkenSky;
    pub const PLAY_BOSS_MUSIC: BarFlagEnum = BarFlagEnum::PlayBossMusic;
    pub const CREATE_FOG: BarFlagEnum = BarFlagEnum::CreateFog;
    pub fn from_string(str: String) -> std::option::Option<BarFlagEnum> {
        match str.as_str() {
            "DARKEN_SKY" => Some(BarFlagEnum::DarkenSky),
            "PLAY_BOSS_MUSIC" => Some(BarFlagEnum::PlayBossMusic),
            "CREATE_FOG" => Some(BarFlagEnum::CreateFog),
            _ => None,
        }
    }
}
/// Represents a custom <a title="interface in org.bukkit.boss" href="BossBar.html"><code>BossBar</code></a> that has a <a href="../NamespacedKey.html" title="class in org.bukkit"><code>NamespacedKey</code></a>
///
/// This is a representation of an abstract class.
pub struct KeyedBossBar<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> KeyedBossBar<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate KeyedBossBar from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/KeyedBossBar")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a KeyedBossBar object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }

    pub fn add_flag(
        &mut self,
        arg0: impl Into<&'mc crate::boss::BarFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addFlag",
            "(Lorg/bukkit/boss/BarFlag;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_color(
        &mut self,
        arg0: impl Into<&'mc crate::boss::BarColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/boss/BarColor;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisible", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_visible(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisible",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn color(&mut self) -> Result<crate::boss::BarColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/boss/BarColor;",
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
        crate::boss::BarColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::boss::BarColor::from_string(variant_str).unwrap(),
        )
    }

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

    pub fn players(
        &mut self,
    ) -> Result<Vec<crate::entity::Player<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPlayers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Player::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn add_player(
        &mut self,
        arg0: impl Into<&'mc crate::entity::Player<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPlayer",
            "(Lorg/bukkit/entity/Player;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_player(
        &mut self,
        arg0: impl Into<&'mc crate::entity::Player<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePlayer",
            "(Lorg/bukkit/entity/Player;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn style(&mut self) -> Result<crate::boss::BarStyle<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStyle",
            "()Lorg/bukkit/boss/BarStyle;",
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
        crate::boss::BarStyle::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::boss::BarStyle::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_style(
        &mut self,
        arg0: impl Into<&'mc crate::boss::BarStyle<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStyle",
            "(Lorg/bukkit/boss/BarStyle;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_flag(
        &mut self,
        arg0: impl Into<&'mc crate::boss::BarFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeFlag",
            "(Lorg/bukkit/boss/BarFlag;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_flag(
        &mut self,
        arg0: impl Into<&'mc crate::boss::BarFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasFlag",
            "(Lorg/bukkit/boss/BarFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_progress(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProgress",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn progress(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getProgress", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    #[deprecated]

    pub fn show(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "show", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn hide(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hide", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "removeAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
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
impl<'mc> JNIRaw<'mc> for KeyedBossBar<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::boss::BossBar<'mc>> for KeyedBossBar<'mc> {
    fn into(self) -> crate::boss::BossBar<'mc> {
        crate::boss::BossBar::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for KeyedBossBar<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}

///
/// This is a representation of an abstract class.
pub struct BossBar<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BossBar<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BossBar from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/BossBar")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BossBar object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Add an optional flag to this boss bar
    pub fn add_flag(
        &mut self,
        arg0: impl Into<&'mc crate::boss::BarFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addFlag",
            "(Lorg/bukkit/boss/BarFlag;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the color of this boss bar.
    pub fn set_color(
        &mut self,
        arg0: impl Into<&'mc crate::boss::BarColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/boss/BarColor;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Return if the boss bar is displayed to attached players.
    pub fn is_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisible", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Set if the boss bar is displayed to attached players.
    pub fn set_visible(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisible",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns the color of this boss bar
    pub fn color(&mut self) -> Result<crate::boss::BarColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/boss/BarColor;",
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
        crate::boss::BarColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::boss::BarColor::from_string(variant_str).unwrap(),
        )
    }
    /// Returns the title of this boss bar
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
    /// Returns all players viewing this boss bar
    pub fn players(
        &mut self,
    ) -> Result<Vec<crate::entity::Player<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPlayers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Player::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Adds the player to this boss bar causing it to display on their screen.
    pub fn add_player(
        &mut self,
        arg0: impl Into<&'mc crate::entity::Player<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPlayer",
            "(Lorg/bukkit/entity/Player;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Removes the player from this boss bar causing it to be removed from their screen.
    pub fn remove_player(
        &mut self,
        arg0: impl Into<&'mc crate::entity::Player<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePlayer",
            "(Lorg/bukkit/entity/Player;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the title of this boss bar
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
    /// Returns the style of this boss bar
    pub fn style(&mut self) -> Result<crate::boss::BarStyle<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStyle",
            "()Lorg/bukkit/boss/BarStyle;",
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
        crate::boss::BarStyle::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::boss::BarStyle::from_string(variant_str).unwrap(),
        )
    }
    /// Sets the bar style of this boss bar
    pub fn set_style(
        &mut self,
        arg0: impl Into<&'mc crate::boss::BarStyle<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStyle",
            "(Lorg/bukkit/boss/BarStyle;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Remove an existing flag on this boss bar
    pub fn remove_flag(
        &mut self,
        arg0: impl Into<&'mc crate::boss::BarFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeFlag",
            "(Lorg/bukkit/boss/BarFlag;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether this boss bar as the passed flag set
    pub fn has_flag(
        &mut self,
        arg0: impl Into<&'mc crate::boss::BarFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasFlag",
            "(Lorg/bukkit/boss/BarFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Sets the progress of the bar. Values should be between 0.0 (empty) and 1.0 (full)
    pub fn set_progress(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProgress",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns the progress of the bar between 0.0 and 1.0
    pub fn progress(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getProgress", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// <a href="#setVisible(boolean)"><code>setVisible(boolean)</code></a>
    /// </div>
    /// <a href="#setVisible(boolean)"><code>setVisible(boolean)</code></a>
    ///
    /// Shows the previously hidden boss bar to all attached players
    pub fn show(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "show", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// <a href="#setVisible(boolean)"><code>setVisible(boolean)</code></a>
    /// </div>
    /// <a href="#setVisible(boolean)"><code>setVisible(boolean)</code></a>
    ///
    /// Hides this boss bar from all attached players
    pub fn hide(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hide", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Removes all players from this boss bar
    pub fn remove_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "removeAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for BossBar<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents a dragon battle state for a world with an end environment.
///
/// This is a representation of an abstract class.
pub struct DragonBattle<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> DragonBattle<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DragonBattle from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/DragonBattle")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DragonBattle object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Get the boss bar to be displayed for this dragon battle.
    pub fn boss_bar(&mut self) -> Result<crate::boss::BossBar<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBossBar",
            "()Lorg/bukkit/boss/BossBar;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::boss::BossBar::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the <a title="interface in org.bukkit.entity" href="../entity/EnderDragon.html"><code>EnderDragon</code></a> active in this battle. Will return null if the dragon has been slain.
    pub fn ender_dragon(
        &mut self,
    ) -> Result<crate::entity::EnderDragon<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnderDragon",
            "()Lorg/bukkit/entity/EnderDragon;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EnderDragon::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the location of the end portal. This location will be at the center of the base (bottom) of the portal.
    pub fn end_portal_location(
        &mut self,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEndPortalLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Generate the end portal.
    pub fn generate_end_portal(&mut self, arg0: bool) -> Result<bool, Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "generateEndPortal",
            "(Z)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Check whether or not the first dragon has been killed already.
    pub fn has_been_previously_killed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasBeenPreviouslyKilled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Get this battle's current respawn phase.
    pub fn respawn_phase(
        &mut self,
    ) -> Result<crate::boss::DragonBattleRespawnPhase<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRespawnPhase",
            "()Lorg/bukkit/boss/DragonBattle$RespawnPhase;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::boss::DragonBattleRespawnPhase::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the dragon's respawn phase. This method will is unsuccessful if a dragon respawn is not in progress.
    pub fn set_respawn_phase(
        &mut self,
        arg0: impl Into<&'mc crate::boss::DragonBattleRespawnPhase<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRespawnPhase",
            "(Lorg/bukkit/boss/DragonBattle$RespawnPhase;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Reset the crystals located on the obsidian pillars (remove their beam targets and invulnerability).
    pub fn reset_crystals(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "resetCrystals", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for DragonBattle<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents a phase in the dragon respawn process.
pub struct DragonBattleRespawnPhase<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for DragonBattleRespawnPhase<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DragonBattleRespawnPhase<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate DragonBattleRespawnPhase from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/DragonBattleRespawnPhase")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DragonBattleRespawnPhase object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Returns the enum constant of this type with the specified name. The string must match <i>exactly</i> an identifier used to declare an enum constant in this type. (Extraneous whitespace characters are not permitted.)
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
