#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Criteria names which trigger an objective to be modified by actions in-game
pub struct Criterias<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Criterias<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Criterias<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Criterias from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/Criterias")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Criterias object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
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
    //

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
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// How an option may be applied to members of this team.
pub enum TeamOptionStatusEnum {
    Always,
    Never,
    ForOtherTeams,
    ForOwnTeam,
}
impl std::fmt::Display for TeamOptionStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamOptionStatusEnum::Always => f.write_str("ALWAYS"),
            TeamOptionStatusEnum::Never => f.write_str("NEVER"),
            TeamOptionStatusEnum::ForOtherTeams => f.write_str("FOR_OTHER_TEAMS"),
            TeamOptionStatusEnum::ForOwnTeam => f.write_str("FOR_OWN_TEAM"),
        }
    }
}
pub struct TeamOptionStatus<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub TeamOptionStatusEnum,
);
impl<'mc> std::ops::Deref for TeamOptionStatus<'mc> {
    type Target = TeamOptionStatusEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for TeamOptionStatus<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TeamOptionStatus<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: TeamOptionStatusEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate TeamOptionStatus from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/Team$OptionStatus")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TeamOptionStatus object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const ALWAYS: TeamOptionStatusEnum = TeamOptionStatusEnum::Always;
    pub const NEVER: TeamOptionStatusEnum = TeamOptionStatusEnum::Never;
    pub const FOR_OTHER_TEAMS: TeamOptionStatusEnum = TeamOptionStatusEnum::ForOtherTeams;
    pub const FOR_OWN_TEAM: TeamOptionStatusEnum = TeamOptionStatusEnum::ForOwnTeam;
    pub fn from_string(str: String) -> std::option::Option<TeamOptionStatusEnum> {
        match str.as_str() {
            "ALWAYS" => Some(TeamOptionStatusEnum::Always),
            "NEVER" => Some(TeamOptionStatusEnum::Never),
            "FOR_OTHER_TEAMS" => Some(TeamOptionStatusEnum::ForOtherTeams),
            "FOR_OWN_TEAM" => Some(TeamOptionStatusEnum::ForOwnTeam),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<TeamOptionStatus<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/scoreboard/Team$OptionStatus");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Team$OptionStatus;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        TeamOptionStatus::from_raw(
            &jni,
            raw_obj,
            TeamOptionStatus::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    //
}
/// Represents a scoreboard criteria, either custom or built-in to the Minecraft server, used to keep track of and manually or automatically change scores on a scoreboard.
/// <p>While this class outlines constants for standard criteria, see <a href="#statistic(org.bukkit.Statistic)"><code>statistic(Statistic)</code></a> (and its overloads) to create instances for statistically-backed criteria.</p>
///
/// This is a representation of an abstract class.
pub struct Criteria<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Criteria<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Criteria from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/Criteria")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Criteria object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn default_render_type(
        &mut self,
    ) -> Result<crate::scoreboard::RenderType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultRenderType",
            "()Lorg/bukkit/scoreboard/RenderType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .jni_ref()
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::scoreboard::RenderType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::scoreboard::RenderType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn statistic_with_statistic(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Statistic<'mc>>>,
        arg1: std::option::Option<impl Into<crate::entity::EntityType<'mc>>>,
    ) -> Result<crate::scoreboard::Criteria<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe {
            jni::objects::JObject::from_raw(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let val_2 = unsafe {
            jni::objects::JObject::from_raw(
                arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let cls = jni.find_class("org/bukkit/scoreboard/Criteria");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls,"statistic",
"(Lorg/bukkit/Statistic;Lorg/bukkit/entity/EntityType;)Lorg/bukkit/scoreboard/Criteria;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::scoreboard::Criteria::from_raw(&jni, obj)
    }
    //

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn create(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::scoreboard::Criteria<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/scoreboard/Criteria");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "create",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Criteria;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::scoreboard::Criteria::from_raw(&jni, obj)
    }
    //

    pub fn is_read_only(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isReadOnly", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
impl<'mc> JNIRaw<'mc> for Criteria<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub enum NameTagVisibilityEnum {
    Always,
    Never,
    HideForOtherTeams,
    HideForOwnTeam,
}
impl std::fmt::Display for NameTagVisibilityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NameTagVisibilityEnum::Always => f.write_str("ALWAYS"),
            NameTagVisibilityEnum::Never => f.write_str("NEVER"),
            NameTagVisibilityEnum::HideForOtherTeams => f.write_str("HIDE_FOR_OTHER_TEAMS"),
            NameTagVisibilityEnum::HideForOwnTeam => f.write_str("HIDE_FOR_OWN_TEAM"),
        }
    }
}
pub struct NameTagVisibility<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub NameTagVisibilityEnum,
);
impl<'mc> std::ops::Deref for NameTagVisibility<'mc> {
    type Target = NameTagVisibilityEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for NameTagVisibility<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> NameTagVisibility<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: NameTagVisibilityEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate NameTagVisibility from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/NameTagVisibility")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a NameTagVisibility object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const ALWAYS: NameTagVisibilityEnum = NameTagVisibilityEnum::Always;
    pub const NEVER: NameTagVisibilityEnum = NameTagVisibilityEnum::Never;
    pub const HIDE_FOR_OTHER_TEAMS: NameTagVisibilityEnum =
        NameTagVisibilityEnum::HideForOtherTeams;
    pub const HIDE_FOR_OWN_TEAM: NameTagVisibilityEnum = NameTagVisibilityEnum::HideForOwnTeam;
    pub fn from_string(str: String) -> std::option::Option<NameTagVisibilityEnum> {
        match str.as_str() {
            "ALWAYS" => Some(NameTagVisibilityEnum::Always),
            "NEVER" => Some(NameTagVisibilityEnum::Never),
            "HIDE_FOR_OTHER_TEAMS" => Some(NameTagVisibilityEnum::HideForOtherTeams),
            "HIDE_FOR_OWN_TEAM" => Some(NameTagVisibilityEnum::HideForOwnTeam),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<NameTagVisibility<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/scoreboard/NameTagVisibility");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/NameTagVisibility;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        NameTagVisibility::from_raw(
            &jni,
            raw_obj,
            NameTagVisibility::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
pub enum RenderTypeEnum {
    Integer,
    Hearts,
}
impl std::fmt::Display for RenderTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderTypeEnum::Integer => f.write_str("INTEGER"),
            RenderTypeEnum::Hearts => f.write_str("HEARTS"),
        }
    }
}
pub struct RenderType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub RenderTypeEnum,
);
impl<'mc> std::ops::Deref for RenderType<'mc> {
    type Target = RenderTypeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for RenderType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RenderType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: RenderTypeEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate RenderType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/RenderType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RenderType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const INTEGER: RenderTypeEnum = RenderTypeEnum::Integer;
    pub const HEARTS: RenderTypeEnum = RenderTypeEnum::Hearts;
    pub fn from_string(str: String) -> std::option::Option<RenderTypeEnum> {
        match str.as_str() {
            "INTEGER" => Some(RenderTypeEnum::Integer),
            "HEARTS" => Some(RenderTypeEnum::Hearts),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<RenderType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/scoreboard/RenderType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/RenderType;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        RenderType::from_raw(
            &jni,
            raw_obj,
            RenderType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
/// An objective on a scoreboard that can show scores specific to entries. This objective is only relevant to the display of the associated <a href="#getScoreboard()"><code>scoreboard</code></a>.
///
/// This is a representation of an abstract class.
pub struct Objective<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Objective<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Objective from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/Objective")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Objective object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn unregister(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "unregister", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn criteria(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCriteria",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn scoreboard(
        &mut self,
    ) -> Result<crate::scoreboard::Scoreboard<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboard",
            "()Lorg/bukkit/scoreboard/Scoreboard;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scoreboard::Scoreboard::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_score_with_offline_player(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<crate::scoreboard::Score<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(
            self.jni_ref().new_string(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into(),
            )?,
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScore",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Score;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scoreboard::Score::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn tracked_criteria(
        &mut self,
    ) -> Result<crate::scoreboard::Criteria<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTrackedCriteria",
            "()Lorg/bukkit/scoreboard/Criteria;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scoreboard::Criteria::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_modifiable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isModifiable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_display_slot(
        &mut self,
        arg0: impl Into<crate::scoreboard::DisplaySlot<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplaySlot",
            "(Lorg/bukkit/scoreboard/DisplaySlot;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_slot(
        &mut self,
    ) -> Result<crate::scoreboard::DisplaySlot<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplaySlot",
            "()Lorg/bukkit/scoreboard/DisplaySlot;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .jni_ref()
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::scoreboard::DisplaySlot::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::scoreboard::DisplaySlot::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_render_type(
        &mut self,
        arg0: impl Into<crate::scoreboard::RenderType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRenderType",
            "(Lorg/bukkit/scoreboard/RenderType;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn render_type(
        &mut self,
    ) -> Result<crate::scoreboard::RenderType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRenderType",
            "()Lorg/bukkit/scoreboard/RenderType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .jni_ref()
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::scoreboard::RenderType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::scoreboard::RenderType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
}
impl<'mc> JNIRaw<'mc> for Objective<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// A score entry for an <a href="#getEntry()"><code>entry</code></a> on an <a href="#getObjective()"><code>objective</code></a>. Changing this will not affect any other objective or scoreboard.
///
/// This is a representation of an abstract class.
pub struct Score<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Score<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Score from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/Score")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Score object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn entry(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntry", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn player(&mut self) -> Result<crate::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlayer",
            "()Lorg/bukkit/OfflinePlayer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::OfflinePlayer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn scoreboard(
        &mut self,
    ) -> Result<crate::scoreboard::Scoreboard<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboard",
            "()Lorg/bukkit/scoreboard/Scoreboard;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scoreboard::Scoreboard::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn objective(
        &mut self,
    ) -> Result<crate::scoreboard::Objective<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getObjective",
            "()Lorg/bukkit/scoreboard/Objective;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scoreboard::Objective::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn score(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getScore", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    /// Sets the current score.
    pub fn set_score(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setScore",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_score_set(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isScoreSet", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
impl<'mc> JNIRaw<'mc> for Score<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// A scoreboard
///
/// This is a representation of an abstract class.
pub struct Scoreboard<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Scoreboard<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Scoreboard from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/Scoreboard")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Scoreboard object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn players(&mut self) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPlayers", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn register_new_objective_with_string(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
        arg2: impl Into<String>,
        arg3: std::option::Option<impl Into<crate::scoreboard::RenderType<'mc>>>,
    ) -> Result<crate::scoreboard::Objective<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into())?);
        let val_3 = jni::objects::JObject::from(self.jni_ref().new_string(arg2.into())?);
        let val_4 = unsafe {
            jni::objects::JObject::from_raw(
                arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let res = self.jni_ref().call_method(&self.jni_object(),"registerNewObjective","(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lorg/bukkit/scoreboard/RenderType;)Lorg/bukkit/scoreboard/Objective;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::scoreboard::Objective::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_objective_with_display_slot(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<crate::scoreboard::Objective<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(
            self.jni_ref().new_string(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into(),
            )?,
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getObjective",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Objective;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scoreboard::Objective::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_objectives_by_criteria_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<crate::scoreboard::Criteria<'mc>>>,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe {
            jni::objects::JObject::from_raw(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getObjectivesByCriteria",
            "(Lorg/bukkit/scoreboard/Criteria;)Ljava/util/Set;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn objectives(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getObjectives",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_scores_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<crate::OfflinePlayer<'mc>>>,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe {
            jni::objects::JObject::from_raw(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScores",
            "(Lorg/bukkit/OfflinePlayer;)Ljava/util/Set;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn reset_scores_with_offline_player(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(
            self.jni_ref().new_string(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into(),
            )?,
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "resetScores",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get_player_team(
        &mut self,
        arg0: impl Into<crate::OfflinePlayer<'mc>>,
    ) -> Result<crate::scoreboard::Team<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlayerTeam",
            "(Lorg/bukkit/OfflinePlayer;)Lorg/bukkit/scoreboard/Team;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scoreboard::Team::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_entry_team(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<crate::scoreboard::Team<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEntryTeam",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Team;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scoreboard::Team::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_team(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<crate::scoreboard::Team<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTeam",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Team;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scoreboard::Team::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn teams(&mut self) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTeams", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn register_new_team(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<crate::scoreboard::Team<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "registerNewTeam",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Team;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scoreboard::Team::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn clear_slot(
        &mut self,
        arg0: impl Into<crate::scoreboard::DisplaySlot<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clearSlot",
            "(Lorg/bukkit/scoreboard/DisplaySlot;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn entries(&mut self) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntries", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for Scoreboard<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub enum DisplaySlotEnum {
    BelowName,
    PlayerList,
    Sidebar,
    SidebarBlack,
    SidebarDarkBlue,
    SidebarDarkGreen,
    SidebarDarkAqua,
    SidebarDarkRed,
    SidebarDarkPurple,
    SidebarGold,
    SidebarGray,
    SidebarDarkGray,
    SidebarBlue,
    SidebarGreen,
    SidebarAqua,
    SidebarRed,
    SidebarLightPurple,
    SidebarYellow,
    SidebarWhite,
}
impl std::fmt::Display for DisplaySlotEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplaySlotEnum::BelowName => f.write_str("BELOW_NAME"),
            DisplaySlotEnum::PlayerList => f.write_str("PLAYER_LIST"),
            DisplaySlotEnum::Sidebar => f.write_str("SIDEBAR"),
            DisplaySlotEnum::SidebarBlack => f.write_str("SIDEBAR_BLACK"),
            DisplaySlotEnum::SidebarDarkBlue => f.write_str("SIDEBAR_DARK_BLUE"),
            DisplaySlotEnum::SidebarDarkGreen => f.write_str("SIDEBAR_DARK_GREEN"),
            DisplaySlotEnum::SidebarDarkAqua => f.write_str("SIDEBAR_DARK_AQUA"),
            DisplaySlotEnum::SidebarDarkRed => f.write_str("SIDEBAR_DARK_RED"),
            DisplaySlotEnum::SidebarDarkPurple => f.write_str("SIDEBAR_DARK_PURPLE"),
            DisplaySlotEnum::SidebarGold => f.write_str("SIDEBAR_GOLD"),
            DisplaySlotEnum::SidebarGray => f.write_str("SIDEBAR_GRAY"),
            DisplaySlotEnum::SidebarDarkGray => f.write_str("SIDEBAR_DARK_GRAY"),
            DisplaySlotEnum::SidebarBlue => f.write_str("SIDEBAR_BLUE"),
            DisplaySlotEnum::SidebarGreen => f.write_str("SIDEBAR_GREEN"),
            DisplaySlotEnum::SidebarAqua => f.write_str("SIDEBAR_AQUA"),
            DisplaySlotEnum::SidebarRed => f.write_str("SIDEBAR_RED"),
            DisplaySlotEnum::SidebarLightPurple => f.write_str("SIDEBAR_LIGHT_PURPLE"),
            DisplaySlotEnum::SidebarYellow => f.write_str("SIDEBAR_YELLOW"),
            DisplaySlotEnum::SidebarWhite => f.write_str("SIDEBAR_WHITE"),
        }
    }
}
pub struct DisplaySlot<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub DisplaySlotEnum,
);
impl<'mc> std::ops::Deref for DisplaySlot<'mc> {
    type Target = DisplaySlotEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for DisplaySlot<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DisplaySlot<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: DisplaySlotEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DisplaySlot from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/DisplaySlot")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DisplaySlot object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const BELOW_NAME: DisplaySlotEnum = DisplaySlotEnum::BelowName;
    pub const PLAYER_LIST: DisplaySlotEnum = DisplaySlotEnum::PlayerList;
    pub const SIDEBAR: DisplaySlotEnum = DisplaySlotEnum::Sidebar;
    pub const SIDEBAR_BLACK: DisplaySlotEnum = DisplaySlotEnum::SidebarBlack;
    pub const SIDEBAR_DARK_BLUE: DisplaySlotEnum = DisplaySlotEnum::SidebarDarkBlue;
    pub const SIDEBAR_DARK_GREEN: DisplaySlotEnum = DisplaySlotEnum::SidebarDarkGreen;
    pub const SIDEBAR_DARK_AQUA: DisplaySlotEnum = DisplaySlotEnum::SidebarDarkAqua;
    pub const SIDEBAR_DARK_RED: DisplaySlotEnum = DisplaySlotEnum::SidebarDarkRed;
    pub const SIDEBAR_DARK_PURPLE: DisplaySlotEnum = DisplaySlotEnum::SidebarDarkPurple;
    pub const SIDEBAR_GOLD: DisplaySlotEnum = DisplaySlotEnum::SidebarGold;
    pub const SIDEBAR_GRAY: DisplaySlotEnum = DisplaySlotEnum::SidebarGray;
    pub const SIDEBAR_DARK_GRAY: DisplaySlotEnum = DisplaySlotEnum::SidebarDarkGray;
    pub const SIDEBAR_BLUE: DisplaySlotEnum = DisplaySlotEnum::SidebarBlue;
    pub const SIDEBAR_GREEN: DisplaySlotEnum = DisplaySlotEnum::SidebarGreen;
    pub const SIDEBAR_AQUA: DisplaySlotEnum = DisplaySlotEnum::SidebarAqua;
    pub const SIDEBAR_RED: DisplaySlotEnum = DisplaySlotEnum::SidebarRed;
    pub const SIDEBAR_LIGHT_PURPLE: DisplaySlotEnum = DisplaySlotEnum::SidebarLightPurple;
    pub const SIDEBAR_YELLOW: DisplaySlotEnum = DisplaySlotEnum::SidebarYellow;
    pub const SIDEBAR_WHITE: DisplaySlotEnum = DisplaySlotEnum::SidebarWhite;
    pub fn from_string(str: String) -> std::option::Option<DisplaySlotEnum> {
        match str.as_str() {
            "BELOW_NAME" => Some(DisplaySlotEnum::BelowName),
            "PLAYER_LIST" => Some(DisplaySlotEnum::PlayerList),
            "SIDEBAR" => Some(DisplaySlotEnum::Sidebar),
            "SIDEBAR_BLACK" => Some(DisplaySlotEnum::SidebarBlack),
            "SIDEBAR_DARK_BLUE" => Some(DisplaySlotEnum::SidebarDarkBlue),
            "SIDEBAR_DARK_GREEN" => Some(DisplaySlotEnum::SidebarDarkGreen),
            "SIDEBAR_DARK_AQUA" => Some(DisplaySlotEnum::SidebarDarkAqua),
            "SIDEBAR_DARK_RED" => Some(DisplaySlotEnum::SidebarDarkRed),
            "SIDEBAR_DARK_PURPLE" => Some(DisplaySlotEnum::SidebarDarkPurple),
            "SIDEBAR_GOLD" => Some(DisplaySlotEnum::SidebarGold),
            "SIDEBAR_GRAY" => Some(DisplaySlotEnum::SidebarGray),
            "SIDEBAR_DARK_GRAY" => Some(DisplaySlotEnum::SidebarDarkGray),
            "SIDEBAR_BLUE" => Some(DisplaySlotEnum::SidebarBlue),
            "SIDEBAR_GREEN" => Some(DisplaySlotEnum::SidebarGreen),
            "SIDEBAR_AQUA" => Some(DisplaySlotEnum::SidebarAqua),
            "SIDEBAR_RED" => Some(DisplaySlotEnum::SidebarRed),
            "SIDEBAR_LIGHT_PURPLE" => Some(DisplaySlotEnum::SidebarLightPurple),
            "SIDEBAR_YELLOW" => Some(DisplaySlotEnum::SidebarYellow),
            "SIDEBAR_WHITE" => Some(DisplaySlotEnum::SidebarWhite),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<DisplaySlot<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/scoreboard/DisplaySlot");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/DisplaySlot;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        DisplaySlot::from_raw(
            &jni,
            raw_obj,
            DisplaySlot::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
/// A team on a scoreboard that has a common display theme and other properties. This team is only relevant to the display of the associated <a href="#getScoreboard()"><code>scoreboard</code></a>.
///
/// This is a representation of an abstract class.
pub struct Team<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Team<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Team from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/Team")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Team object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn remove_entry(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEntry",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/ChatColor;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn unregister(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "unregister", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn prefix(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrefix",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn color(&mut self) -> Result<crate::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/ChatColor;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::ChatColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_prefix(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPrefix",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn suffix(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSuffix",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn set_suffix(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSuffix",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn allow_friendly_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "allowFriendlyFire", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// Sets the team friendly fire state
    pub fn set_allow_friendly_fire(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAllowFriendlyFire",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn can_see_friendly_invisibles(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "canSeeFriendlyInvisibles", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// Sets the team's ability to see <a href="../potion/PotionEffectType.html#INVISIBILITY"><code>invisible</code></a> teammates.
    pub fn set_can_see_friendly_invisibles(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCanSeeFriendlyInvisibles",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn name_tag_visibility(
        &mut self,
    ) -> Result<crate::scoreboard::NameTagVisibility<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNameTagVisibility",
            "()Lorg/bukkit/scoreboard/NameTagVisibility;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .jni_ref()
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::scoreboard::NameTagVisibility::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::scoreboard::NameTagVisibility::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_name_tag_visibility(
        &mut self,
        arg0: impl Into<crate::scoreboard::NameTagVisibility<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNameTagVisibility",
            "(Lorg/bukkit/scoreboard/NameTagVisibility;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn players(&mut self) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPlayers", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn scoreboard(
        &mut self,
    ) -> Result<crate::scoreboard::Scoreboard<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboard",
            "()Lorg/bukkit/scoreboard/Scoreboard;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scoreboard::Scoreboard::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_player(
        &mut self,
        arg0: impl Into<crate::OfflinePlayer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPlayer",
            "(Lorg/bukkit/OfflinePlayer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_player(
        &mut self,
        arg0: impl Into<crate::OfflinePlayer<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePlayer",
            "(Lorg/bukkit/OfflinePlayer;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn has_player(
        &mut self,
        arg0: impl Into<crate::OfflinePlayer<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasPlayer",
            "(Lorg/bukkit/OfflinePlayer;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn has_entry(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEntry",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get_option(
        &mut self,
        arg0: impl Into<crate::scoreboard::TeamOption<'mc>>,
    ) -> Result<crate::scoreboard::TeamOptionStatus<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOption",
            "(Lorg/bukkit/scoreboard/Team$Option;)Lorg/bukkit/scoreboard/Team$OptionStatus;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .jni_ref()
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::scoreboard::TeamOptionStatus::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::scoreboard::TeamOptionStatus::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_option(
        &mut self,
        arg0: impl Into<crate::scoreboard::TeamOption<'mc>>,
        arg1: impl Into<crate::scoreboard::TeamOptionStatus<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOption",
            "(Lorg/bukkit/scoreboard/Team$Option;Lorg/bukkit/scoreboard/Team$OptionStatus;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn add_entry(&mut self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEntry",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn entries(&mut self) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntries", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for Team<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Manager of Scoreboards
///
/// This is a representation of an abstract class.
pub struct ScoreboardManager<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ScoreboardManager<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ScoreboardManager from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/ScoreboardManager")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ScoreboardManager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn main_scoreboard(
        &mut self,
    ) -> Result<crate::scoreboard::Scoreboard<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMainScoreboard",
            "()Lorg/bukkit/scoreboard/Scoreboard;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scoreboard::Scoreboard::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn new_scoreboard(
        &mut self,
    ) -> Result<crate::scoreboard::Scoreboard<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNewScoreboard",
            "()Lorg/bukkit/scoreboard/Scoreboard;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scoreboard::Scoreboard::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for ScoreboardManager<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub enum OptionEnum {
    NameTagVisibility,
    DeathMessageVisibility,
    CollisionRule,
}
impl std::fmt::Display for OptionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionEnum::NameTagVisibility => f.write_str("NAME_TAG_VISIBILITY"),
            OptionEnum::DeathMessageVisibility => f.write_str("DEATH_MESSAGE_VISIBILITY"),
            OptionEnum::CollisionRule => f.write_str("COLLISION_RULE"),
        }
    }
}
pub struct Option<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub OptionEnum,
);
impl<'mc> std::ops::Deref for Option<'mc> {
    type Target = OptionEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for Option<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Option<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: OptionEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Option from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/Option")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Option object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const NAME_TAG_VISIBILITY: OptionEnum = OptionEnum::NameTagVisibility;
    pub const DEATH_MESSAGE_VISIBILITY: OptionEnum = OptionEnum::DeathMessageVisibility;
    pub const COLLISION_RULE: OptionEnum = OptionEnum::CollisionRule;
    pub fn from_string(str: String) -> std::option::Option<OptionEnum> {
        match str.as_str() {
            "NAME_TAG_VISIBILITY" => Some(OptionEnum::NameTagVisibility),
            "DEATH_MESSAGE_VISIBILITY" => Some(OptionEnum::DeathMessageVisibility),
            "COLLISION_RULE" => Some(OptionEnum::CollisionRule),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Option<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/scoreboard/Option");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Option;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        Option::from_raw(
            &jni,
            raw_obj,
            Option::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
pub enum OptionStatusEnum {
    Always,
    Never,
    ForOtherTeams,
    ForOwnTeam,
}
impl std::fmt::Display for OptionStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionStatusEnum::Always => f.write_str("ALWAYS"),
            OptionStatusEnum::Never => f.write_str("NEVER"),
            OptionStatusEnum::ForOtherTeams => f.write_str("FOR_OTHER_TEAMS"),
            OptionStatusEnum::ForOwnTeam => f.write_str("FOR_OWN_TEAM"),
        }
    }
}
pub struct OptionStatus<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub OptionStatusEnum,
);
impl<'mc> std::ops::Deref for OptionStatus<'mc> {
    type Target = OptionStatusEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for OptionStatus<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> OptionStatus<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: OptionStatusEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate OptionStatus from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/OptionStatus")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a OptionStatus object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const ALWAYS: OptionStatusEnum = OptionStatusEnum::Always;
    pub const NEVER: OptionStatusEnum = OptionStatusEnum::Never;
    pub const FOR_OTHER_TEAMS: OptionStatusEnum = OptionStatusEnum::ForOtherTeams;
    pub const FOR_OWN_TEAM: OptionStatusEnum = OptionStatusEnum::ForOwnTeam;
    pub fn from_string(str: String) -> std::option::Option<OptionStatusEnum> {
        match str.as_str() {
            "ALWAYS" => Some(OptionStatusEnum::Always),
            "NEVER" => Some(OptionStatusEnum::Never),
            "FOR_OTHER_TEAMS" => Some(OptionStatusEnum::ForOtherTeams),
            "FOR_OWN_TEAM" => Some(OptionStatusEnum::ForOwnTeam),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<OptionStatus<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/scoreboard/OptionStatus");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/OptionStatus;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        OptionStatus::from_raw(
            &jni,
            raw_obj,
            OptionStatus::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
/// Represents an option which may be applied to this team.
pub enum TeamOptionEnum {
    NameTagVisibility,
    DeathMessageVisibility,
    CollisionRule,
}
impl std::fmt::Display for TeamOptionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamOptionEnum::NameTagVisibility => f.write_str("NAME_TAG_VISIBILITY"),
            TeamOptionEnum::DeathMessageVisibility => f.write_str("DEATH_MESSAGE_VISIBILITY"),
            TeamOptionEnum::CollisionRule => f.write_str("COLLISION_RULE"),
        }
    }
}
pub struct TeamOption<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub TeamOptionEnum,
);
impl<'mc> std::ops::Deref for TeamOption<'mc> {
    type Target = TeamOptionEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for TeamOption<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TeamOption<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: TeamOptionEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TeamOption from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/Team$Option")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TeamOption object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const NAME_TAG_VISIBILITY: TeamOptionEnum = TeamOptionEnum::NameTagVisibility;
    pub const DEATH_MESSAGE_VISIBILITY: TeamOptionEnum = TeamOptionEnum::DeathMessageVisibility;
    pub const COLLISION_RULE: TeamOptionEnum = TeamOptionEnum::CollisionRule;
    pub fn from_string(str: String) -> std::option::Option<TeamOptionEnum> {
        match str.as_str() {
            "NAME_TAG_VISIBILITY" => Some(TeamOptionEnum::NameTagVisibility),
            "DEATH_MESSAGE_VISIBILITY" => Some(TeamOptionEnum::DeathMessageVisibility),
            "COLLISION_RULE" => Some(TeamOptionEnum::CollisionRule),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<TeamOption<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/scoreboard/Team$Option");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Team$Option;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        TeamOption::from_raw(
            &jni,
            raw_obj,
            TeamOption::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    //
}
