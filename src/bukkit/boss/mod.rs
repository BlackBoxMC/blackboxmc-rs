use crate::JNIRaw;
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
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub BarStyleEnum,
);
impl<'mc> std::ops::Deref for BarStyle<'mc> {
    type Target = BarStyleEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> crate::JNIRaw<'mc> for BarStyle<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BarStyle<'mc> {
    pub const SOLID: BarStyleEnum = BarStyleEnum::Solid;
    pub const SEGMENTED6: BarStyleEnum = BarStyleEnum::Segmented6;
    pub const SEGMENTED10: BarStyleEnum = BarStyleEnum::Segmented10;
    pub const SEGMENTED12: BarStyleEnum = BarStyleEnum::Segmented12;
    pub const SEGMENTED20: BarStyleEnum = BarStyleEnum::Segmented20;
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
    pub fn value_of(
        mut jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::boss::BarStyle<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/boss/BarStyle")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/boss/BarStyle;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::boss::BarStyle(
                jni,
                raw_obj,
                crate::bukkit::boss::BarStyle::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
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
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub BarColorEnum,
);
impl<'mc> std::ops::Deref for BarColor<'mc> {
    type Target = BarColorEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> crate::JNIRaw<'mc> for BarColor<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BarColor<'mc> {
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
    pub fn value_of(
        mut jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::boss::BarColor<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/boss/BarColor")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/boss/BarColor;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::boss::BarColor(
                jni,
                raw_obj,
                crate::bukkit::boss::BarColor::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
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
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub BarFlagEnum,
);
impl<'mc> std::ops::Deref for BarFlag<'mc> {
    type Target = BarFlagEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> crate::JNIRaw<'mc> for BarFlag<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BarFlag<'mc> {
    pub const DARKENSKY: BarFlagEnum = BarFlagEnum::DarkenSky;
    pub const PLAYBOSSMUSIC: BarFlagEnum = BarFlagEnum::PlayBossMusic;
    pub const CREATEFOG: BarFlagEnum = BarFlagEnum::CreateFog;
    pub fn from_string(str: String) -> std::option::Option<BarFlagEnum> {
        match str.as_str() {
            "DARKEN_SKY" => Some(BarFlagEnum::DarkenSky),
            "PLAY_BOSS_MUSIC" => Some(BarFlagEnum::PlayBossMusic),
            "CREATE_FOG" => Some(BarFlagEnum::CreateFog),
            _ => None,
        }
    }
    pub fn value_of(
        mut jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::boss::BarFlag<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/boss/BarFlag")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/boss/BarFlag;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::boss::BarFlag(
                jni,
                raw_obj,
                crate::bukkit::boss::BarFlag::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
}
/// An instantiatable struct that implements KeyedBossBar. Needed for returning it from Java.
pub struct KeyedBossBar<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> KeyedBossBar<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate KeyedBossBar from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("KeyedBossBar") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a KeyedBossBar object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_color(
        &mut self,
        arg0: crate::bukkit::boss::BarColor<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/boss/BarColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn add_flag(
        &mut self,
        arg0: crate::bukkit::boss::BarFlag<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "addFlag",
            "(Lorg/bukkit/boss/BarFlag;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn remove_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "removeAll", "()V", &[])?;
        Ok(())
    }
    pub fn is_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisible", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_visible(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVisible",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn color(
        &mut self,
    ) -> Result<crate::bukkit::boss::BarColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/boss/BarColor;",
            &[],
        )?;
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
            crate::bukkit::boss::BarColor(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::boss::BarColor::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn title(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTitle",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn add_player(
        &mut self,
        arg0: crate::bukkit::entity::Player<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "addPlayer",
            "(Lorg/bukkit/entity/Player;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn remove_player(
        &mut self,
        arg0: crate::bukkit::entity::Player<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "removePlayer",
            "(Lorg/bukkit/entity/Player;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_title(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn style(
        &mut self,
    ) -> Result<crate::bukkit::boss::BarStyle<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStyle",
            "()Lorg/bukkit/boss/BarStyle;",
            &[],
        )?;
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
            crate::bukkit::boss::BarStyle(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::boss::BarStyle::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_style(
        &mut self,
        arg0: crate::bukkit::boss::BarStyle<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setStyle",
            "(Lorg/bukkit/boss/BarStyle;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn remove_flag(
        &mut self,
        arg0: crate::bukkit::boss::BarFlag<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeFlag",
            "(Lorg/bukkit/boss/BarFlag;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_flag(
        &mut self,
        arg0: crate::bukkit::boss::BarFlag<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasFlag",
            "(Lorg/bukkit/boss/BarFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn set_progress(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setProgress",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn progress(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getProgress", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn show(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "show", "()V", &[])?;
        Ok(())
    }
    pub fn hide(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "hide", "()V", &[])?;
        Ok(())
    }
    pub fn key(&mut self) -> Result<crate::bukkit::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKey",
            "()Lorg/bukkit/NamespacedKey;",
            &[],
        )?;
        let ret = {
            crate::bukkit::NamespacedKey(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for KeyedBossBar<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements BossBar. Needed for returning it from Java.
pub struct BossBar<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BossBar<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BossBar from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("BossBar") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BossBar object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_color(
        &mut self,
        arg0: crate::bukkit::boss::BarColor<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/boss/BarColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn add_flag(
        &mut self,
        arg0: crate::bukkit::boss::BarFlag<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "addFlag",
            "(Lorg/bukkit/boss/BarFlag;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn remove_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "removeAll", "()V", &[])?;
        Ok(())
    }
    pub fn is_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisible", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_visible(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVisible",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn color(
        &mut self,
    ) -> Result<crate::bukkit::boss::BarColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/boss/BarColor;",
            &[],
        )?;
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
            crate::bukkit::boss::BarColor(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::boss::BarColor::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn title(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTitle",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn add_player(
        &mut self,
        arg0: crate::bukkit::entity::Player<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "addPlayer",
            "(Lorg/bukkit/entity/Player;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn remove_player(
        &mut self,
        arg0: crate::bukkit::entity::Player<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "removePlayer",
            "(Lorg/bukkit/entity/Player;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_title(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn style(
        &mut self,
    ) -> Result<crate::bukkit::boss::BarStyle<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStyle",
            "()Lorg/bukkit/boss/BarStyle;",
            &[],
        )?;
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
            crate::bukkit::boss::BarStyle(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::boss::BarStyle::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_style(
        &mut self,
        arg0: crate::bukkit::boss::BarStyle<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setStyle",
            "(Lorg/bukkit/boss/BarStyle;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn remove_flag(
        &mut self,
        arg0: crate::bukkit::boss::BarFlag<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeFlag",
            "(Lorg/bukkit/boss/BarFlag;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_flag(
        &mut self,
        arg0: crate::bukkit::boss::BarFlag<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasFlag",
            "(Lorg/bukkit/boss/BarFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn set_progress(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setProgress",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn progress(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getProgress", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn show(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "show", "()V", &[])?;
        Ok(())
    }
    pub fn hide(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "hide", "()V", &[])?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for BossBar<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements DragonBattle. Needed for returning it from Java.
pub struct DragonBattle<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> DragonBattle<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DragonBattle from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("DragonBattle") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DragonBattle object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn respawn_phase(
        &mut self,
    ) -> Result<crate::bukkit::boss::DragonBattleRespawnPhase<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRespawnPhase",
            "()Lorg/bukkit/boss/DragonBattle$RespawnPhase;",
            &[],
        )?;
        let ret = {
            crate::bukkit::boss::DragonBattleRespawnPhase(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn boss_bar(
        &mut self,
    ) -> Result<crate::bukkit::boss::BossBar<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBossBar",
            "()Lorg/bukkit/boss/BossBar;",
            &[],
        )?;
        let ret = {
            crate::bukkit::boss::BossBar(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn ender_dragon(
        &mut self,
    ) -> Result<crate::bukkit::entity::EnderDragon<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnderDragon",
            "()Lorg/bukkit/entity/EnderDragon;",
            &[],
        )?;
        let ret = {
            crate::bukkit::entity::EnderDragon(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn end_portal_location(
        &mut self,
    ) -> Result<crate::bukkit::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEndPortalLocation",
            "()Lorg/bukkit/Location;",
            &[],
        )?;
        let ret = {
            crate::bukkit::Location(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn generate_end_portal(&mut self, arg0: bool) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "generateEndPortal",
            "(Z)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_been_previously_killed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasBeenPreviouslyKilled",
            "()Z",
            &[],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn initiate_respawn(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "initiateRespawn", "()V", &[])?;
        Ok(())
    }
    pub fn set_respawn_phase(
        &mut self,
        arg0: crate::bukkit::boss::DragonBattleRespawnPhase<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRespawnPhase",
            "(Lorg/bukkit/boss/DragonBattle$RespawnPhase;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn reset_crystals(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "resetCrystals", "()V", &[])?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for DragonBattle<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct DragonBattleRespawnPhase<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for DragonBattleRespawnPhase<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DragonBattleRespawnPhase<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate DragonBattleRespawnPhase from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("DragonBattleRespawnPhase") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DragonBattleRespawnPhase object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "name", "()Ljava/lang/String;", &[])?;
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
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
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
        )?;
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
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
