#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIInstantiatableEnum;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[derive(PartialEq, Eq)]
pub enum BarStyleEnum {
    Solid,
    Segmented6,
    Segmented10,
    Segmented12,
    Segmented20,
}
impl std::fmt::Display for BarStyleEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BarStyleEnum::Solid => f.write_str("SOLID"),
            BarStyleEnum::Segmented6 => f.write_str("SEGMENTED_6"),
            BarStyleEnum::Segmented10 => f.write_str("SEGMENTED_10"),
            BarStyleEnum::Segmented12 => f.write_str("SEGMENTED_12"),
            BarStyleEnum::Segmented20 => f.write_str("SEGMENTED_20"),
        }
    }
}
impl<'mc> std::fmt::Display for BarStyle<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
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

impl<'mc> JNIInstantiatableEnum<'mc> for BarStyle<'mc> {
    type Enum = BarStyleEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
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
}

impl<'mc> BarStyle<'mc> {
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

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BarStyle<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/boss/BarStyle");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/boss/BarStyle;",
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
        BarStyle::from_raw(
            &jni,
            raw_obj,
            BarStyle::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
#[derive(PartialEq, Eq)]
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
        match self {
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
impl<'mc> std::fmt::Display for BarColor<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
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

impl<'mc> JNIInstantiatableEnum<'mc> for BarColor<'mc> {
    type Enum = BarColorEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
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
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BarColor<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/boss/BarColor");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/boss/BarColor;",
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
        BarColor::from_raw(
            &jni,
            raw_obj,
            BarColor::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
#[derive(PartialEq, Eq)]
pub enum BarFlagEnum {
    DarkenSky,
    PlayBossMusic,
    CreateFog,
}
impl std::fmt::Display for BarFlagEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BarFlagEnum::DarkenSky => f.write_str("DARKEN_SKY"),
            BarFlagEnum::PlayBossMusic => f.write_str("PLAY_BOSS_MUSIC"),
            BarFlagEnum::CreateFog => f.write_str("CREATE_FOG"),
        }
    }
}
impl<'mc> std::fmt::Display for BarFlag<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
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

impl<'mc> JNIInstantiatableEnum<'mc> for BarFlag<'mc> {
    type Enum = BarFlagEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
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
}

impl<'mc> BarFlag<'mc> {
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

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BarFlag<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/boss/BarFlag");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/boss/BarFlag;",
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
        BarFlag::from_raw(
            &jni,
            raw_obj,
            BarFlag::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
#[derive(PartialEq, Eq)]
pub enum RespawnPhaseEnum {
    Start,
    PreparingToSummonPillars,
    SummoningPillars,
    SummoningDragon,
    End,
    None,
}
impl std::fmt::Display for RespawnPhaseEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RespawnPhaseEnum::Start => f.write_str("START"),
            RespawnPhaseEnum::PreparingToSummonPillars => {
                f.write_str("PREPARING_TO_SUMMON_PILLARS")
            }
            RespawnPhaseEnum::SummoningPillars => f.write_str("SUMMONING_PILLARS"),
            RespawnPhaseEnum::SummoningDragon => f.write_str("SUMMONING_DRAGON"),
            RespawnPhaseEnum::End => f.write_str("END"),
            RespawnPhaseEnum::None => f.write_str("NONE"),
        }
    }
}
impl<'mc> std::fmt::Display for RespawnPhase<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
pub struct RespawnPhase<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub RespawnPhaseEnum,
);
impl<'mc> std::ops::Deref for RespawnPhase<'mc> {
    type Target = RespawnPhaseEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}

impl<'mc> JNIRaw<'mc> for RespawnPhase<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatableEnum<'mc> for RespawnPhase<'mc> {
    type Enum = RespawnPhaseEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate RespawnPhase from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/RespawnPhase")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RespawnPhase object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
}

impl<'mc> RespawnPhase<'mc> {
    pub const START: RespawnPhaseEnum = RespawnPhaseEnum::Start;
    pub const PREPARING_TO_SUMMON_PILLARS: RespawnPhaseEnum =
        RespawnPhaseEnum::PreparingToSummonPillars;
    pub const SUMMONING_PILLARS: RespawnPhaseEnum = RespawnPhaseEnum::SummoningPillars;
    pub const SUMMONING_DRAGON: RespawnPhaseEnum = RespawnPhaseEnum::SummoningDragon;
    pub const END: RespawnPhaseEnum = RespawnPhaseEnum::End;
    pub const NONE: RespawnPhaseEnum = RespawnPhaseEnum::None;
    pub fn from_string(str: String) -> std::option::Option<RespawnPhaseEnum> {
        match str.as_str() {
            "START" => Some(RespawnPhaseEnum::Start),
            "PREPARING_TO_SUMMON_PILLARS" => Some(RespawnPhaseEnum::PreparingToSummonPillars),
            "SUMMONING_PILLARS" => Some(RespawnPhaseEnum::SummoningPillars),
            "SUMMONING_DRAGON" => Some(RespawnPhaseEnum::SummoningDragon),
            "END" => Some(RespawnPhaseEnum::End),
            "NONE" => Some(RespawnPhaseEnum::None),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<RespawnPhase<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/boss/RespawnPhase");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/boss/RespawnPhase;",
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
        RespawnPhase::from_raw(
            &jni,
            raw_obj,
            RespawnPhase::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
/// Represents a custom <a title="interface in org.bukkit.boss" href="BossBar.html"><code>BossBar</code></a> that has a <a href="../NamespacedKey.html" title="class in org.bukkit"><code>NamespacedKey</code></a>
///
/// This is a representation of an abstract class.
pub struct KeyedBossBar<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for KeyedBossBar<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for KeyedBossBar<'mc> {
    fn from_raw(
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
}

impl<'mc> KeyedBossBar<'mc> {
    pub fn set_color(
        &self,
        arg0: impl Into<crate::boss::BarColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/boss/BarColor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_flag(
        &self,
        arg0: impl Into<crate::boss::BarFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/boss/BarFlag;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addFlag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "removeAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_visible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisible", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_visible(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisible",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn color(&self) -> Result<Option<crate::boss::BarColor<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/boss/BarColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
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
        Ok(Some(crate::boss::BarColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::boss::BarColor::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )?))
    }

    pub fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    #[deprecated]

    pub fn players(&self) -> Result<Vec<crate::entity::Player<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPlayers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Player::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn add_player(
        &self,
        arg0: impl Into<crate::entity::Player<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Player;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPlayer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_player(
        &self,
        arg0: impl Into<crate::entity::Player<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Player;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePlayer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_title(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn style(&self) -> Result<crate::boss::BarStyle<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/boss/BarStyle;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getStyle", sig.as_str(), vec![]);
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
        crate::boss::BarStyle::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::boss::BarStyle::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    pub fn set_style(
        &self,
        arg0: impl Into<crate::boss::BarStyle<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/boss/BarStyle;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStyle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_flag(
        &self,
        arg0: impl Into<crate::boss::BarFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/boss/BarFlag;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeFlag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_flag(
        &self,
        arg0: impl Into<crate::boss::BarFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/boss/BarFlag;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasFlag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_progress(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProgress",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn progress(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getProgress", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    #[deprecated]

    pub fn show(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "show", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn hide(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hide", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/NamespacedKey;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKey", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::NamespacedKey::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> Into<crate::boss::BossBar<'mc>> for KeyedBossBar<'mc> {
    fn into(self) -> crate::boss::BossBar<'mc> {
        crate::boss::BossBar::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting KeyedBossBar into crate::boss::BossBar")
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for KeyedBossBar<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting KeyedBossBar into crate::Keyed")
    }
}

///
/// This is a representation of an abstract class.
pub struct BossBar<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BossBar<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for BossBar<'mc> {
    fn from_raw(
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
}

impl<'mc> BossBar<'mc> {
    pub fn set_color(
        &self,
        arg0: impl Into<crate::boss::BarColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/boss/BarColor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_flag(
        &self,
        arg0: impl Into<crate::boss::BarFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/boss/BarFlag;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addFlag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "removeAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_visible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisible", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Set if the boss bar is displayed to attached players.
    pub fn set_visible(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisible",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn color(&self) -> Result<Option<crate::boss::BarColor<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/boss/BarColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
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
        Ok(Some(crate::boss::BarColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::boss::BarColor::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )?))
    }

    pub fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    #[deprecated]

    pub fn players(&self) -> Result<Vec<crate::entity::Player<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPlayers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Player::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn add_player(
        &self,
        arg0: impl Into<crate::entity::Player<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Player;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPlayer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_player(
        &self,
        arg0: impl Into<crate::entity::Player<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Player;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePlayer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_title(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn style(&self) -> Result<crate::boss::BarStyle<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/boss/BarStyle;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getStyle", sig.as_str(), vec![]);
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
        crate::boss::BarStyle::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::boss::BarStyle::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    pub fn set_style(
        &self,
        arg0: impl Into<crate::boss::BarStyle<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/boss/BarStyle;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStyle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_flag(
        &self,
        arg0: impl Into<crate::boss::BarFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/boss/BarFlag;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeFlag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_flag(
        &self,
        arg0: impl Into<crate::boss::BarFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/boss/BarFlag;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasFlag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the progress of the bar. Values should be between 0.0 (empty) and 1.0 (full)
    pub fn set_progress(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProgress",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn progress(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getProgress", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    #[deprecated]

    pub fn show(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "show", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn hide(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hide", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents a dragon battle state for a world with an end environment.
///
/// This is a representation of an abstract class.
pub struct DragonBattle<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DragonBattle<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for DragonBattle<'mc> {
    fn from_raw(
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
}

impl<'mc> DragonBattle<'mc> {
    pub fn boss_bar(
        &self,
    ) -> Result<Option<crate::boss::BossBar<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/boss/BossBar;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBossBar", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::boss::BossBar::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn ender_dragon(
        &self,
    ) -> Result<Option<crate::entity::EnderDragon<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EnderDragon;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnderDragon", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EnderDragon::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn end_portal_location(
        &self,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEndPortalLocation",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Generate the end portal.
    pub fn generate_end_portal(&self, arg0: bool) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Z");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "generateEndPortal",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn has_been_previously_killed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasBeenPreviouslyKilled",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn respawn_phase(
        &self,
    ) -> Result<crate::boss::DragonBattleRespawnPhase<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/boss/DragonBattle$RespawnPhase;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRespawnPhase", sig.as_str(), vec![]);
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
        crate::boss::DragonBattleRespawnPhase::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::boss::DragonBattleRespawnPhase::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    pub fn set_respawn_phase(
        &self,
        arg0: impl Into<crate::boss::DragonBattleRespawnPhase<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/boss/DragonBattle$RespawnPhase;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRespawnPhase",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn reset_crystals(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "resetCrystals", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents a phase in the dragon respawn process.
#[derive(PartialEq, Eq)]
pub enum DragonBattleRespawnPhaseEnum {
    Start,
    PreparingToSummonPillars,
    SummoningPillars,
    SummoningDragon,
    End,
    None,
}
impl std::fmt::Display for DragonBattleRespawnPhaseEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DragonBattleRespawnPhaseEnum::Start => f.write_str("START"),
            DragonBattleRespawnPhaseEnum::PreparingToSummonPillars => {
                f.write_str("PREPARING_TO_SUMMON_PILLARS")
            }
            DragonBattleRespawnPhaseEnum::SummoningPillars => f.write_str("SUMMONING_PILLARS"),
            DragonBattleRespawnPhaseEnum::SummoningDragon => f.write_str("SUMMONING_DRAGON"),
            DragonBattleRespawnPhaseEnum::End => f.write_str("END"),
            DragonBattleRespawnPhaseEnum::None => f.write_str("NONE"),
        }
    }
}
impl<'mc> std::fmt::Display for DragonBattleRespawnPhase<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
pub struct DragonBattleRespawnPhase<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub DragonBattleRespawnPhaseEnum,
);
impl<'mc> std::ops::Deref for DragonBattleRespawnPhase<'mc> {
    type Target = DragonBattleRespawnPhaseEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}

impl<'mc> JNIRaw<'mc> for DragonBattleRespawnPhase<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatableEnum<'mc> for DragonBattleRespawnPhase<'mc> {
    type Enum = DragonBattleRespawnPhaseEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate DragonBattleRespawnPhase from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/DragonBattle$RespawnPhase")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DragonBattleRespawnPhase object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
}

impl<'mc> DragonBattleRespawnPhase<'mc> {
    pub const START: DragonBattleRespawnPhaseEnum = DragonBattleRespawnPhaseEnum::Start;
    pub const PREPARING_TO_SUMMON_PILLARS: DragonBattleRespawnPhaseEnum =
        DragonBattleRespawnPhaseEnum::PreparingToSummonPillars;
    pub const SUMMONING_PILLARS: DragonBattleRespawnPhaseEnum =
        DragonBattleRespawnPhaseEnum::SummoningPillars;
    pub const SUMMONING_DRAGON: DragonBattleRespawnPhaseEnum =
        DragonBattleRespawnPhaseEnum::SummoningDragon;
    pub const END: DragonBattleRespawnPhaseEnum = DragonBattleRespawnPhaseEnum::End;
    pub const NONE: DragonBattleRespawnPhaseEnum = DragonBattleRespawnPhaseEnum::None;
    pub fn from_string(str: String) -> std::option::Option<DragonBattleRespawnPhaseEnum> {
        match str.as_str() {
            "START" => Some(DragonBattleRespawnPhaseEnum::Start),
            "PREPARING_TO_SUMMON_PILLARS" => {
                Some(DragonBattleRespawnPhaseEnum::PreparingToSummonPillars)
            }
            "SUMMONING_PILLARS" => Some(DragonBattleRespawnPhaseEnum::SummoningPillars),
            "SUMMONING_DRAGON" => Some(DragonBattleRespawnPhaseEnum::SummoningDragon),
            "END" => Some(DragonBattleRespawnPhaseEnum::End),
            "NONE" => Some(DragonBattleRespawnPhaseEnum::None),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<DragonBattleRespawnPhase<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/boss/DragonBattle$RespawnPhase");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/boss/DragonBattle$RespawnPhase;",
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
        DragonBattleRespawnPhase::from_raw(
            &jni,
            raw_obj,
            DragonBattleRespawnPhase::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
