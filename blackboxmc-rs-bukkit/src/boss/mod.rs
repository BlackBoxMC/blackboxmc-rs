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
        let (valid, name) = env.validate_name(&obj, "BarStyle")?;
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
        let (valid, name) = env.validate_name(&obj, "BarColor")?;
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
        let (valid, name) = env.validate_name(&obj, "BarFlag")?;
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
/// An instantiatable struct that implements KeyedBossBar. Needed for returning it from Java.
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
        let (valid, name) = env.validate_name(&obj, "KeyedBossBar")?;
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
/// An instantiatable struct that implements BossBar. Needed for returning it from Java.
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
        let (valid, name) = env.validate_name(&obj, "BossBar")?;
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
}
impl<'mc> JNIRaw<'mc> for BossBar<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements DragonBattle. Needed for returning it from Java.
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
        let (valid, name) = env.validate_name(&obj, "DragonBattle")?;
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
}
impl<'mc> JNIRaw<'mc> for DragonBattle<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
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
        let (valid, name) = env.validate_name(&obj, "DragonBattleRespawnPhase")?;
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
}
