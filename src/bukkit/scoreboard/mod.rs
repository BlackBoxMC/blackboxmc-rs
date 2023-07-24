#![allow(deprecated)]
use crate::JNIRaw;
use color_eyre::eyre::Result;
pub struct Criterias<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Criterias<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Criterias<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Criterias from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Criterias") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Criterias object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
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
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
pub struct TeamOptionStatus<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for TeamOptionStatus<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TeamOptionStatus<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate TeamOptionStatus from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("TeamOptionStatus") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TeamOptionStatus object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn value_of_with_string(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let cls = &jni.find_class("java/lang/Enum")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "name", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn compare_to_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compareTo",
            "(Ljava/lang/Enum;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
/// An instantiatable struct that implements Criteria. Needed for returning it from Java.
pub struct Criteria<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Criteria<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Criteria from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Criteria") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Criteria object, got {}",
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
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn create(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::bukkit::scoreboard::Criteria<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("org/bukkit/scoreboard/Criteria")?;
        let res = jni.call_static_method(
            cls,
            "create",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Criteria;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let mut obj = res.l()?;
            crate::bukkit::scoreboard::Criteria(jni, obj)
        };
        Ok(ret)
    }
    pub fn is_read_only(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isReadOnly", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn default_render_type(
        &mut self,
    ) -> Result<crate::bukkit::scoreboard::RenderType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultRenderType",
            "()Lorg/bukkit/scoreboard/RenderType;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
            crate::bukkit::scoreboard::RenderType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::scoreboard::RenderType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn statistic_with_statistic(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::Statistic<'mc>>>,
        arg1: std::option::Option<impl Into<&'mc crate::bukkit::entity::EntityType<'mc>>>,
    ) -> Result<crate::bukkit::scoreboard::Criteria<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/scoreboard/Criteria")?;
        let res = jni.call_static_method(cls,"statistic",
"(Lorg/bukkit/Statistic;Lorg/bukkit/entity/EntityType;)Lorg/bukkit/scoreboard/Criteria;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
        let ret = {
            let mut obj = res.l()?;
            crate::bukkit::scoreboard::Criteria(jni, obj)
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for Criteria<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
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
        match &self {
            NameTagVisibilityEnum::Always => f.write_str("ALWAYS"),
            NameTagVisibilityEnum::Never => f.write_str("NEVER"),
            NameTagVisibilityEnum::HideForOtherTeams => f.write_str("HIDE_FOR_OTHER_TEAMS"),
            NameTagVisibilityEnum::HideForOwnTeam => f.write_str("HIDE_FOR_OWN_TEAM"),
        }
    }
}
pub struct NameTagVisibility<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
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
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> NameTagVisibility<'mc> {
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
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::bukkit::scoreboard::NameTagVisibility<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("org/bukkit/scoreboard/NameTagVisibility")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/NameTagVisibility;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let mut obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::scoreboard::NameTagVisibility(
                jni,
                raw_obj,
                crate::bukkit::scoreboard::NameTagVisibility::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
}
pub enum RenderTypeEnum {
    Integer,
    Hearts,
}
impl std::fmt::Display for RenderTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RenderTypeEnum::Integer => f.write_str("INTEGER"),
            RenderTypeEnum::Hearts => f.write_str("HEARTS"),
        }
    }
}
pub struct RenderType<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
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
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RenderType<'mc> {
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
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::bukkit::scoreboard::RenderType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("org/bukkit/scoreboard/RenderType")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/RenderType;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let mut obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::scoreboard::RenderType(
                jni,
                raw_obj,
                crate::bukkit::scoreboard::RenderType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
}
/// An instantiatable struct that implements Objective. Needed for returning it from Java.
pub struct Objective<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Objective<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Objective from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Objective") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Objective object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn unregister(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "unregister", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    #[deprecated]
    pub fn criteria(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCriteria",
            "()Ljava/lang/String;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn scoreboard(
        &mut self,
    ) -> Result<crate::bukkit::scoreboard::Scoreboard<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboard",
            "()Lorg/bukkit/scoreboard/Scoreboard;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scoreboard::Scoreboard(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_score_with_offline_player(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<crate::bukkit::scoreboard::Score<'mc>, Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScore",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Score;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scoreboard::Score(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn tracked_criteria(
        &mut self,
    ) -> Result<crate::bukkit::scoreboard::Criteria<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTrackedCriteria",
            "()Lorg/bukkit/scoreboard/Criteria;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scoreboard::Criteria(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_modifiable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isModifiable", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_slot(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::scoreboard::DisplaySlot<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplaySlot",
            "(Lorg/bukkit/scoreboard/DisplaySlot;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_slot(
        &mut self,
    ) -> Result<crate::bukkit::scoreboard::DisplaySlot<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplaySlot",
            "()Lorg/bukkit/scoreboard/DisplaySlot;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
            crate::bukkit::scoreboard::DisplaySlot(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::scoreboard::DisplaySlot::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_render_type(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::scoreboard::RenderType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRenderType",
            "(Lorg/bukkit/scoreboard/RenderType;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn render_type(
        &mut self,
    ) -> Result<crate::bukkit::scoreboard::RenderType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRenderType",
            "()Lorg/bukkit/scoreboard/RenderType;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
            crate::bukkit::scoreboard::RenderType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::scoreboard::RenderType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for Objective<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements Score. Needed for returning it from Java.
pub struct Score<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Score<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Score from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Score") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Score object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn entry(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntry", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    #[deprecated]
    pub fn player(
        &mut self,
    ) -> Result<crate::bukkit::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlayer",
            "()Lorg/bukkit/OfflinePlayer;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::OfflinePlayer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn scoreboard(
        &mut self,
    ) -> Result<crate::bukkit::scoreboard::Scoreboard<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboard",
            "()Lorg/bukkit/scoreboard/Scoreboard;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scoreboard::Scoreboard(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn objective(
        &mut self,
    ) -> Result<crate::bukkit::scoreboard::Objective<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getObjective",
            "()Lorg/bukkit/scoreboard/Objective;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scoreboard::Objective(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn score(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getScore", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_score(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setScore",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_score_set(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isScoreSet", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> crate::JNIRaw<'mc> for Score<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements Scoreboard. Needed for returning it from Java.
pub struct Scoreboard<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Scoreboard<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Scoreboard from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Scoreboard") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Scoreboard object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn register_new_objective_with_string(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc String>,
        arg2: impl Into<&'mc String>,
        arg3: std::option::Option<impl Into<&'mc crate::bukkit::scoreboard::RenderType<'mc>>>,
    ) -> Result<crate::bukkit::scoreboard::Objective<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg2.into()).unwrap());
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"registerNewObjective","(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lorg/bukkit/scoreboard/RenderType;)Lorg/bukkit/scoreboard/Objective;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scoreboard::Objective(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_objective_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::scoreboard::DisplaySlot<'mc>>>,
    ) -> Result<crate::bukkit::scoreboard::Objective<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getObjective",
            "(Lorg/bukkit/scoreboard/DisplaySlot;)Lorg/bukkit/scoreboard/Objective;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scoreboard::Objective(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn reset_scores_with_offline_player(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "resetScores",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    #[deprecated]
    pub fn get_player_team(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::OfflinePlayer<'mc>>,
    ) -> Result<crate::bukkit::scoreboard::Team<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlayerTeam",
            "(Lorg/bukkit/OfflinePlayer;)Lorg/bukkit/scoreboard/Team;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scoreboard::Team(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_entry_team(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::bukkit::scoreboard::Team<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEntryTeam",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Team;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scoreboard::Team(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_team(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::bukkit::scoreboard::Team<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTeam",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Team;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scoreboard::Team(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn register_new_team(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::bukkit::scoreboard::Team<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "registerNewTeam",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Team;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scoreboard::Team(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn clear_slot(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::scoreboard::DisplaySlot<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clearSlot",
            "(Lorg/bukkit/scoreboard/DisplaySlot;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for Scoreboard<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
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
        match &self {
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
    pub(crate) crate::SharedJNIEnv<'mc>,
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
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DisplaySlot<'mc> {
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
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::bukkit::scoreboard::DisplaySlot<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("org/bukkit/scoreboard/DisplaySlot")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/scoreboard/DisplaySlot;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let mut obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::scoreboard::DisplaySlot(
                jni,
                raw_obj,
                crate::bukkit::scoreboard::DisplaySlot::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
}
/// An instantiatable struct that implements ScoreboardManager. Needed for returning it from Java.
pub struct ScoreboardManager<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ScoreboardManager<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ScoreboardManager from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ScoreboardManager") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ScoreboardManager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn main_scoreboard(
        &mut self,
    ) -> Result<crate::bukkit::scoreboard::Scoreboard<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMainScoreboard",
            "()Lorg/bukkit/scoreboard/Scoreboard;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scoreboard::Scoreboard(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn new_scoreboard(
        &mut self,
    ) -> Result<crate::bukkit::scoreboard::Scoreboard<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNewScoreboard",
            "()Lorg/bukkit/scoreboard/Scoreboard;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scoreboard::Scoreboard(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for ScoreboardManager<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements Team. Needed for returning it from Java.
pub struct Team<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Team<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Team from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Team") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Team object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn remove_entry(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEntry",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn add_entry(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEntry",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn unregister(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "unregister", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_color(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/ChatColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn prefix(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrefix",
            "()Ljava/lang/String;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn color(&mut self) -> Result<crate::bukkit::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/ChatColor;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::ChatColor(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_prefix(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPrefix",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn suffix(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSuffix",
            "()Ljava/lang/String;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_suffix(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSuffix",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn allow_friendly_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "allowFriendlyFire", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_allow_friendly_fire(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAllowFriendlyFire",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn can_see_friendly_invisibles(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "canSeeFriendlyInvisibles", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_can_see_friendly_invisibles(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCanSeeFriendlyInvisibles",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    #[deprecated]
    pub fn name_tag_visibility(
        &mut self,
    ) -> Result<crate::bukkit::scoreboard::NameTagVisibility<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNameTagVisibility",
            "()Lorg/bukkit/scoreboard/NameTagVisibility;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
            crate::bukkit::scoreboard::NameTagVisibility(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::scoreboard::NameTagVisibility::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_name_tag_visibility(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::scoreboard::NameTagVisibility<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNameTagVisibility",
            "(Lorg/bukkit/scoreboard/NameTagVisibility;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn scoreboard(
        &mut self,
    ) -> Result<crate::bukkit::scoreboard::Scoreboard<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboard",
            "()Lorg/bukkit/scoreboard/Scoreboard;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scoreboard::Scoreboard(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn add_player(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::OfflinePlayer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPlayer",
            "(Lorg/bukkit/OfflinePlayer;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    #[deprecated]
    pub fn remove_player(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::OfflinePlayer<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePlayer",
            "(Lorg/bukkit/OfflinePlayer;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn has_player(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::OfflinePlayer<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasPlayer",
            "(Lorg/bukkit/OfflinePlayer;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn has_entry(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEntry",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn get_option(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::scoreboard::TeamOption<'mc>>,
    ) -> Result<crate::bukkit::scoreboard::TeamOptionStatus<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOption",
            "(Lorg/bukkit/scoreboard/Team$Option;)Lorg/bukkit/scoreboard/Team$OptionStatus;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scoreboard::TeamOptionStatus(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_option(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::scoreboard::TeamOption<'mc>>,
        arg1: impl Into<&'mc crate::bukkit::scoreboard::TeamOptionStatus<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOption",
            "(Lorg/bukkit/scoreboard/Team$Option;Lorg/bukkit/scoreboard/Team$OptionStatus;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for Team<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct TeamOption<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for TeamOption<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TeamOption<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TeamOption from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("TeamOption") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TeamOption object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn value_of_with_string(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let cls = &jni.find_class("java/lang/Enum")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "name", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn compare_to_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compareTo",
            "(Ljava/lang/Enum;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
