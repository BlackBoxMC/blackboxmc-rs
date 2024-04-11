#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct Score<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Score<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Score<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Score from null object.")
                .into());
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
    }
    
impl<'mc> Score<'mc> {
#[deprecated]

	pub fn player(&self) 
-> Result<crate::OfflinePlayer<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::OfflinePlayer;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::OfflinePlayer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entry(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntry",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn objective(&self) 
-> Result<crate::scoreboard::Objective<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::Objective;");
let res = self.jni_ref().call_method(&self.jni_object(),"getObjective",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::Objective::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn score(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getScore",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn set_score(&self,score: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(score);
let res = self.jni_ref().call_method(&self.jni_object(),"setScore",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_score_set(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isScoreSet",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn scoreboard(&self) 
-> Result<Option<crate::scoreboard::Scoreboard<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::Scoreboard;");
let res = self.jni_ref().call_method(&self.jni_object(),"getScoreboard",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::scoreboard::Scoreboard::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct ScoreboardManager<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ScoreboardManager<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ScoreboardManager<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ScoreboardManager from null object.")
                .into());
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
    }
    
impl<'mc> ScoreboardManager<'mc> {
	pub fn main_scoreboard(&self) 
-> Result<crate::scoreboard::Scoreboard<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::Scoreboard;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMainScoreboard",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::Scoreboard::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn new_scoreboard(&self) 
-> Result<crate::scoreboard::Scoreboard<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::Scoreboard;");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewScoreboard",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::Scoreboard::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct Team<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Team<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Team<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Team from null object.")
                .into());
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
    }
    
impl<'mc> Team<'mc> {
	pub fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn display_name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn set_display_name(&self,display_name: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(display_name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayName",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn prefix(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPrefix",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn set_prefix(&self,prefix: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(prefix.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setPrefix",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn suffix(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSuffix",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn set_suffix(&self,suffix: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(suffix.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setSuffix",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn color(&self) 
-> Result<crate::ChatColor<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::ChatColor;");
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::ChatColor::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_color(&self,color: impl Into<crate::ChatColor<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/ChatColor;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(color.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn allow_friendly_fire(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"allowFriendlyFire",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_allow_friendly_fire(&self,enabled: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(enabled.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setAllowFriendlyFire",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn can_see_friendly_invisibles(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"canSeeFriendlyInvisibles",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_can_see_friendly_invisibles(&self,enabled: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(enabled.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCanSeeFriendlyInvisibles",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]

	pub fn name_tag_visibility(&self) 
-> Result<crate::scoreboard::NameTagVisibility<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::NameTagVisibility;");
let res = self.jni_ref().call_method(&self.jni_object(),"getNameTagVisibility",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::NameTagVisibility::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]

	pub fn set_name_tag_visibility(&self,visibility: impl Into<crate::scoreboard::NameTagVisibility<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/scoreboard/NameTagVisibility;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(visibility.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setNameTagVisibility",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]

	pub fn players(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entries(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntries",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn size(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn scoreboard(&self) 
-> Result<Option<crate::scoreboard::Scoreboard<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::Scoreboard;");
let res = self.jni_ref().call_method(&self.jni_object(),"getScoreboard",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::scoreboard::Scoreboard::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
#[deprecated]

	pub fn add_player(&self,player: impl Into<crate::OfflinePlayer<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/OfflinePlayer;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addPlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn add_entry(&self,entry: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(entry.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"addEntry",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]

	pub fn remove_player(&self,player: impl Into<crate::OfflinePlayer<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/OfflinePlayer;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removePlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn remove_entry(&self,entry: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(entry.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"removeEntry",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn unregister(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"unregister",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]

	pub fn has_player(&self,player: impl Into<crate::OfflinePlayer<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/OfflinePlayer;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"hasPlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn has_entry(&self,entry: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(entry.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"hasEntry",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_option(&self,option: impl Into<crate::scoreboard::TeamOption<'mc>>) 
-> Result<crate::scoreboard::TeamOptionStatus<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/scoreboard/Team/Option;)Lcrate::scoreboard::TeamOptionStatus;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(option.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getOption",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::TeamOptionStatus::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_option(&self,option: impl Into<crate::scoreboard::TeamOption<'mc>>,status: impl Into<crate::scoreboard::TeamOptionStatus<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/scoreboard/Team/Option;Lorg/bukkit/scoreboard/Team/OptionStatus;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(option.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(status.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setOption",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub enum TeamOptionStatus<'mc> {
	Always {inner: TeamOptionStatusStruct<'mc>},
	Never {inner: TeamOptionStatusStruct<'mc>},
	ForOtherTeams {inner: TeamOptionStatusStruct<'mc>},
	ForOwnTeam {inner: TeamOptionStatusStruct<'mc>},
}
impl<'mc> std::fmt::Display for TeamOptionStatus<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           TeamOptionStatus::Always { .. } => f.write_str("ALWAYS"),
           TeamOptionStatus::Never { .. } => f.write_str("NEVER"),
           TeamOptionStatus::ForOtherTeams { .. } => f.write_str("FOR_OTHER_TEAMS"),
           TeamOptionStatus::ForOwnTeam { .. } => f.write_str("FOR_OWN_TEAM"),
       }
   }
}

        impl<'mc> TeamOptionStatus<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<TeamOptionStatus<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/scoreboard/Team/OptionStatus");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Team/OptionStatus;",
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
                    
"ALWAYS" => Ok(TeamOptionStatus::Always { inner: TeamOptionStatusStruct::from_raw(env,obj)?}),
"NEVER" => Ok(TeamOptionStatus::Never { inner: TeamOptionStatusStruct::from_raw(env,obj)?}),
"FOR_OTHER_TEAMS" => Ok(TeamOptionStatus::ForOtherTeams { inner: TeamOptionStatusStruct::from_raw(env,obj)?}),
"FOR_OWN_TEAM" => Ok(TeamOptionStatus::ForOwnTeam { inner: TeamOptionStatusStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct TeamOptionStatusStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for TeamOptionStatus<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Always { inner } => inner.0.clone(),
Self::Never { inner } => inner.0.clone(),
Self::ForOtherTeams { inner } => inner.0.clone(),
Self::ForOwnTeam { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Always { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Never { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ForOtherTeams { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ForOwnTeam { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for TeamOptionStatus<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TeamOptionStatus from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/Team/OptionStatus")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TeamOptionStatus object, got {}",
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
                    "ALWAYS" => Ok(TeamOptionStatus::Always { inner: TeamOptionStatusStruct::from_raw(env,obj)?}),"NEVER" => Ok(TeamOptionStatus::Never { inner: TeamOptionStatusStruct::from_raw(env,obj)?}),"FOR_OTHER_TEAMS" => Ok(TeamOptionStatus::ForOtherTeams { inner: TeamOptionStatusStruct::from_raw(env,obj)?}),"FOR_OWN_TEAM" => Ok(TeamOptionStatus::ForOwnTeam { inner: TeamOptionStatusStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for TeamOptionStatusStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for TeamOptionStatusStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TeamOptionStatusStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/Team/OptionStatus")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TeamOptionStatusStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TeamOptionStatusStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::scoreboard::TeamOptionStatus<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::TeamOptionStatus;");
let cls = jni.find_class("org/bukkit/scoreboard/Team/OptionStatus"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::scoreboard::TeamOptionStatus::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub enum NameTagVisibility<'mc> {
	Always {inner: NameTagVisibilityStruct<'mc>},
	Never {inner: NameTagVisibilityStruct<'mc>},
	HideForOtherTeams {inner: NameTagVisibilityStruct<'mc>},
	HideForOwnTeam {inner: NameTagVisibilityStruct<'mc>},
}
impl<'mc> std::fmt::Display for NameTagVisibility<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           NameTagVisibility::Always { .. } => f.write_str("ALWAYS"),
           NameTagVisibility::Never { .. } => f.write_str("NEVER"),
           NameTagVisibility::HideForOtherTeams { .. } => f.write_str("HIDE_FOR_OTHER_TEAMS"),
           NameTagVisibility::HideForOwnTeam { .. } => f.write_str("HIDE_FOR_OWN_TEAM"),
       }
   }
}

        impl<'mc> NameTagVisibility<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<NameTagVisibility<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/scoreboard/NameTagVisibility");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/scoreboard/NameTagVisibility;",
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
                    
"ALWAYS" => Ok(NameTagVisibility::Always { inner: NameTagVisibilityStruct::from_raw(env,obj)?}),
"NEVER" => Ok(NameTagVisibility::Never { inner: NameTagVisibilityStruct::from_raw(env,obj)?}),
"HIDE_FOR_OTHER_TEAMS" => Ok(NameTagVisibility::HideForOtherTeams { inner: NameTagVisibilityStruct::from_raw(env,obj)?}),
"HIDE_FOR_OWN_TEAM" => Ok(NameTagVisibility::HideForOwnTeam { inner: NameTagVisibilityStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct NameTagVisibilityStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for NameTagVisibility<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Always { inner } => inner.0.clone(),
Self::Never { inner } => inner.0.clone(),
Self::HideForOtherTeams { inner } => inner.0.clone(),
Self::HideForOwnTeam { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Always { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Never { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::HideForOtherTeams { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::HideForOwnTeam { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for NameTagVisibility<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate NameTagVisibility from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/NameTagVisibility")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a NameTagVisibility object, got {}",
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
                    "ALWAYS" => Ok(NameTagVisibility::Always { inner: NameTagVisibilityStruct::from_raw(env,obj)?}),"NEVER" => Ok(NameTagVisibility::Never { inner: NameTagVisibilityStruct::from_raw(env,obj)?}),"HIDE_FOR_OTHER_TEAMS" => Ok(NameTagVisibility::HideForOtherTeams { inner: NameTagVisibilityStruct::from_raw(env,obj)?}),"HIDE_FOR_OWN_TEAM" => Ok(NameTagVisibility::HideForOwnTeam { inner: NameTagVisibilityStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for NameTagVisibilityStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for NameTagVisibilityStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate NameTagVisibilityStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/NameTagVisibility")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a NameTagVisibilityStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> NameTagVisibilityStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::scoreboard::NameTagVisibility<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::NameTagVisibility;");
let cls = jni.find_class("org/bukkit/scoreboard/NameTagVisibility"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::scoreboard::NameTagVisibility::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub enum DisplaySlot<'mc> {
	BelowName {inner: DisplaySlotStruct<'mc>},
	PlayerList {inner: DisplaySlotStruct<'mc>},
	Sidebar {inner: DisplaySlotStruct<'mc>},
	SidebarBlack {inner: DisplaySlotStruct<'mc>},
	SidebarDarkBlue {inner: DisplaySlotStruct<'mc>},
	SidebarDarkGreen {inner: DisplaySlotStruct<'mc>},
	SidebarDarkAqua {inner: DisplaySlotStruct<'mc>},
	SidebarDarkRed {inner: DisplaySlotStruct<'mc>},
	SidebarDarkPurple {inner: DisplaySlotStruct<'mc>},
	SidebarGold {inner: DisplaySlotStruct<'mc>},
	SidebarGray {inner: DisplaySlotStruct<'mc>},
	SidebarDarkGray {inner: DisplaySlotStruct<'mc>},
	SidebarBlue {inner: DisplaySlotStruct<'mc>},
	SidebarGreen {inner: DisplaySlotStruct<'mc>},
	SidebarAqua {inner: DisplaySlotStruct<'mc>},
	SidebarRed {inner: DisplaySlotStruct<'mc>},
	SidebarLightPurple {inner: DisplaySlotStruct<'mc>},
	SidebarYellow {inner: DisplaySlotStruct<'mc>},
	SidebarWhite {inner: DisplaySlotStruct<'mc>},
}
impl<'mc> std::fmt::Display for DisplaySlot<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           DisplaySlot::BelowName { .. } => f.write_str("BELOW_NAME"),
           DisplaySlot::PlayerList { .. } => f.write_str("PLAYER_LIST"),
           DisplaySlot::Sidebar { .. } => f.write_str("SIDEBAR"),
           DisplaySlot::SidebarBlack { .. } => f.write_str("SIDEBAR_BLACK"),
           DisplaySlot::SidebarDarkBlue { .. } => f.write_str("SIDEBAR_DARK_BLUE"),
           DisplaySlot::SidebarDarkGreen { .. } => f.write_str("SIDEBAR_DARK_GREEN"),
           DisplaySlot::SidebarDarkAqua { .. } => f.write_str("SIDEBAR_DARK_AQUA"),
           DisplaySlot::SidebarDarkRed { .. } => f.write_str("SIDEBAR_DARK_RED"),
           DisplaySlot::SidebarDarkPurple { .. } => f.write_str("SIDEBAR_DARK_PURPLE"),
           DisplaySlot::SidebarGold { .. } => f.write_str("SIDEBAR_GOLD"),
           DisplaySlot::SidebarGray { .. } => f.write_str("SIDEBAR_GRAY"),
           DisplaySlot::SidebarDarkGray { .. } => f.write_str("SIDEBAR_DARK_GRAY"),
           DisplaySlot::SidebarBlue { .. } => f.write_str("SIDEBAR_BLUE"),
           DisplaySlot::SidebarGreen { .. } => f.write_str("SIDEBAR_GREEN"),
           DisplaySlot::SidebarAqua { .. } => f.write_str("SIDEBAR_AQUA"),
           DisplaySlot::SidebarRed { .. } => f.write_str("SIDEBAR_RED"),
           DisplaySlot::SidebarLightPurple { .. } => f.write_str("SIDEBAR_LIGHT_PURPLE"),
           DisplaySlot::SidebarYellow { .. } => f.write_str("SIDEBAR_YELLOW"),
           DisplaySlot::SidebarWhite { .. } => f.write_str("SIDEBAR_WHITE"),
       }
   }
}

        impl<'mc> DisplaySlot<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<DisplaySlot<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/scoreboard/DisplaySlot");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/scoreboard/DisplaySlot;",
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
                    
"BELOW_NAME" => Ok(DisplaySlot::BelowName { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"PLAYER_LIST" => Ok(DisplaySlot::PlayerList { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR" => Ok(DisplaySlot::Sidebar { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_BLACK" => Ok(DisplaySlot::SidebarBlack { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_DARK_BLUE" => Ok(DisplaySlot::SidebarDarkBlue { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_DARK_GREEN" => Ok(DisplaySlot::SidebarDarkGreen { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_DARK_AQUA" => Ok(DisplaySlot::SidebarDarkAqua { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_DARK_RED" => Ok(DisplaySlot::SidebarDarkRed { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_DARK_PURPLE" => Ok(DisplaySlot::SidebarDarkPurple { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_GOLD" => Ok(DisplaySlot::SidebarGold { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_GRAY" => Ok(DisplaySlot::SidebarGray { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_DARK_GRAY" => Ok(DisplaySlot::SidebarDarkGray { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_BLUE" => Ok(DisplaySlot::SidebarBlue { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_GREEN" => Ok(DisplaySlot::SidebarGreen { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_AQUA" => Ok(DisplaySlot::SidebarAqua { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_RED" => Ok(DisplaySlot::SidebarRed { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_LIGHT_PURPLE" => Ok(DisplaySlot::SidebarLightPurple { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_YELLOW" => Ok(DisplaySlot::SidebarYellow { inner: DisplaySlotStruct::from_raw(env,obj)?}),
"SIDEBAR_WHITE" => Ok(DisplaySlot::SidebarWhite { inner: DisplaySlotStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct DisplaySlotStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for DisplaySlot<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::BelowName { inner } => inner.0.clone(),
Self::PlayerList { inner } => inner.0.clone(),
Self::Sidebar { inner } => inner.0.clone(),
Self::SidebarBlack { inner } => inner.0.clone(),
Self::SidebarDarkBlue { inner } => inner.0.clone(),
Self::SidebarDarkGreen { inner } => inner.0.clone(),
Self::SidebarDarkAqua { inner } => inner.0.clone(),
Self::SidebarDarkRed { inner } => inner.0.clone(),
Self::SidebarDarkPurple { inner } => inner.0.clone(),
Self::SidebarGold { inner } => inner.0.clone(),
Self::SidebarGray { inner } => inner.0.clone(),
Self::SidebarDarkGray { inner } => inner.0.clone(),
Self::SidebarBlue { inner } => inner.0.clone(),
Self::SidebarGreen { inner } => inner.0.clone(),
Self::SidebarAqua { inner } => inner.0.clone(),
Self::SidebarRed { inner } => inner.0.clone(),
Self::SidebarLightPurple { inner } => inner.0.clone(),
Self::SidebarYellow { inner } => inner.0.clone(),
Self::SidebarWhite { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::BelowName { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::PlayerList { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Sidebar { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarBlack { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarDarkBlue { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarDarkGreen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarDarkAqua { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarDarkRed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarDarkPurple { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarGold { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarGray { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarDarkGray { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarBlue { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarGreen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarAqua { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarRed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarLightPurple { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarYellow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SidebarWhite { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for DisplaySlot<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate DisplaySlot from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/DisplaySlot")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a DisplaySlot object, got {}",
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
                    "BELOW_NAME" => Ok(DisplaySlot::BelowName { inner: DisplaySlotStruct::from_raw(env,obj)?}),"PLAYER_LIST" => Ok(DisplaySlot::PlayerList { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR" => Ok(DisplaySlot::Sidebar { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_BLACK" => Ok(DisplaySlot::SidebarBlack { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_DARK_BLUE" => Ok(DisplaySlot::SidebarDarkBlue { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_DARK_GREEN" => Ok(DisplaySlot::SidebarDarkGreen { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_DARK_AQUA" => Ok(DisplaySlot::SidebarDarkAqua { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_DARK_RED" => Ok(DisplaySlot::SidebarDarkRed { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_DARK_PURPLE" => Ok(DisplaySlot::SidebarDarkPurple { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_GOLD" => Ok(DisplaySlot::SidebarGold { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_GRAY" => Ok(DisplaySlot::SidebarGray { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_DARK_GRAY" => Ok(DisplaySlot::SidebarDarkGray { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_BLUE" => Ok(DisplaySlot::SidebarBlue { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_GREEN" => Ok(DisplaySlot::SidebarGreen { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_AQUA" => Ok(DisplaySlot::SidebarAqua { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_RED" => Ok(DisplaySlot::SidebarRed { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_LIGHT_PURPLE" => Ok(DisplaySlot::SidebarLightPurple { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_YELLOW" => Ok(DisplaySlot::SidebarYellow { inner: DisplaySlotStruct::from_raw(env,obj)?}),"SIDEBAR_WHITE" => Ok(DisplaySlot::SidebarWhite { inner: DisplaySlotStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for DisplaySlotStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for DisplaySlotStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate DisplaySlotStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/DisplaySlot")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a DisplaySlotStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DisplaySlotStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::scoreboard::DisplaySlot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::DisplaySlot;");
let cls = jni.find_class("org/bukkit/scoreboard/DisplaySlot"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::scoreboard::DisplaySlot::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct Criterias<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Criterias<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Criterias<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Criterias from null object.")
                .into());
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
    }
    
impl<'mc> Criterias<'mc> {
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub enum RenderType<'mc> {
	Integer {inner: RenderTypeStruct<'mc>},
	Hearts {inner: RenderTypeStruct<'mc>},
}
impl<'mc> std::fmt::Display for RenderType<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           RenderType::Integer { .. } => f.write_str("INTEGER"),
           RenderType::Hearts { .. } => f.write_str("HEARTS"),
       }
   }
}

        impl<'mc> RenderType<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<RenderType<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/scoreboard/RenderType");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/scoreboard/RenderType;",
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
                    
"INTEGER" => Ok(RenderType::Integer { inner: RenderTypeStruct::from_raw(env,obj)?}),
"HEARTS" => Ok(RenderType::Hearts { inner: RenderTypeStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct RenderTypeStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RenderType<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Integer { inner } => inner.0.clone(),
Self::Hearts { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Integer { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Hearts { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for RenderType<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RenderType from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/RenderType")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RenderType object, got {}",
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
                    "INTEGER" => Ok(RenderType::Integer { inner: RenderTypeStruct::from_raw(env,obj)?}),"HEARTS" => Ok(RenderType::Hearts { inner: RenderTypeStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for RenderTypeStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RenderTypeStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RenderTypeStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/RenderType")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RenderTypeStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RenderTypeStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::scoreboard::RenderType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::RenderType;");
let cls = jni.find_class("org/bukkit/scoreboard/RenderType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::scoreboard::RenderType::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub enum TeamOption<'mc> {
	NameTagVisibility {inner: TeamOptionStruct<'mc>},
	DeathMessageVisibility {inner: TeamOptionStruct<'mc>},
	CollisionRule {inner: TeamOptionStruct<'mc>},
}
impl<'mc> std::fmt::Display for TeamOption<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           TeamOption::NameTagVisibility { .. } => f.write_str("NAME_TAG_VISIBILITY"),
           TeamOption::DeathMessageVisibility { .. } => f.write_str("DEATH_MESSAGE_VISIBILITY"),
           TeamOption::CollisionRule { .. } => f.write_str("COLLISION_RULE"),
       }
   }
}

        impl<'mc> TeamOption<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<TeamOption<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/scoreboard/Team/Option");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/scoreboard/Team/Option;",
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
                    
"NAME_TAG_VISIBILITY" => Ok(TeamOption::NameTagVisibility { inner: TeamOptionStruct::from_raw(env,obj)?}),
"DEATH_MESSAGE_VISIBILITY" => Ok(TeamOption::DeathMessageVisibility { inner: TeamOptionStruct::from_raw(env,obj)?}),
"COLLISION_RULE" => Ok(TeamOption::CollisionRule { inner: TeamOptionStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct TeamOptionStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for TeamOption<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::NameTagVisibility { inner } => inner.0.clone(),
Self::DeathMessageVisibility { inner } => inner.0.clone(),
Self::CollisionRule { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::NameTagVisibility { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::DeathMessageVisibility { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CollisionRule { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for TeamOption<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TeamOption from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/Team/Option")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TeamOption object, got {}",
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
                    "NAME_TAG_VISIBILITY" => Ok(TeamOption::NameTagVisibility { inner: TeamOptionStruct::from_raw(env,obj)?}),"DEATH_MESSAGE_VISIBILITY" => Ok(TeamOption::DeathMessageVisibility { inner: TeamOptionStruct::from_raw(env,obj)?}),"COLLISION_RULE" => Ok(TeamOption::CollisionRule { inner: TeamOptionStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for TeamOptionStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for TeamOptionStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TeamOptionStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/scoreboard/Team/Option")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TeamOptionStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TeamOptionStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::scoreboard::TeamOption<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::TeamOption;");
let cls = jni.find_class("org/bukkit/scoreboard/Team/Option"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::scoreboard::TeamOption::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct Objective<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Objective<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Objective<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Objective from null object.")
                .into());
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
    }
    
impl<'mc> Objective<'mc> {
	pub fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn display_name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn set_display_name(&self,display_name: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(display_name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayName",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]

	pub fn criteria(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCriteria",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn tracked_criteria(&self) 
-> Result<crate::scoreboard::Criteria<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::Criteria;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTrackedCriteria",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::Criteria::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_modifiable(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isModifiable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn scoreboard(&self) 
-> Result<Option<crate::scoreboard::Scoreboard<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::Scoreboard;");
let res = self.jni_ref().call_method(&self.jni_object(),"getScoreboard",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::scoreboard::Scoreboard::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn unregister(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"unregister",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_display_slot(&self,slot: impl Into<crate::scoreboard::DisplaySlot<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/scoreboard/DisplaySlot;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(slot.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplaySlot",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_slot(&self) 
-> Result<Option<crate::scoreboard::DisplaySlot<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::DisplaySlot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplaySlot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::scoreboard::DisplaySlot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn set_render_type(&self,render_type: impl Into<crate::scoreboard::RenderType<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/scoreboard/RenderType;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(render_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setRenderType",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn render_type(&self) 
-> Result<crate::scoreboard::RenderType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::RenderType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRenderType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::RenderType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_score_with_entry(&self,entry: impl Into<String>) 
-> Result<crate::scoreboard::Score<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(entry.into())?));
args.push(val_1);
sig += ")Lorg/bukkit/scoreboard/Score;";
let res = self.jni_ref().call_method(&self.jni_object(),"getScore",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::Score::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct Scoreboard<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Scoreboard<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Scoreboard<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Scoreboard from null object.")
                .into());
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
    }
    
impl<'mc> Scoreboard<'mc> {
	pub fn register_new_objective_with_name(&self,name: impl Into<String>,criteria: impl Into<crate::scoreboard::Criteria<'mc>>,display_name: std::option::Option<impl Into<String>>,render_type: std::option::Option<impl Into<crate::scoreboard::RenderType<'mc>>>) 
-> Result<crate::scoreboard::Objective<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
args.push(val_1);
sig += "Lorg/bukkit/scoreboard/Criteria;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(criteria.into().jni_object().clone())});
args.push(val_2);
if let Some(a) = display_name {
sig += "Ljava/lang/String;";
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(a.into())?));
args.push(val_3);
}
if let Some(a) = render_type {
sig += "Lorg/bukkit/scoreboard/RenderType;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_4);
}
sig += ")Lorg/bukkit/scoreboard/Objective;";
let res = self.jni_ref().call_method(&self.jni_object(),"registerNewObjective",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::Objective::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_objective_with_slot(&self,slot: impl Into<crate::scoreboard::DisplaySlot<'mc>>) 
-> Result<Option<crate::scoreboard::Objective<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/scoreboard/DisplaySlot;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(slot.into().jni_object().clone())});
args.push(val_1);
sig += ")Lorg/bukkit/scoreboard/Objective;";
let res = self.jni_ref().call_method(&self.jni_object(),"getObjective",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::scoreboard::Objective::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn get_objectives_by_criteria_with_criteria(&self,criteria: impl Into<crate::scoreboard::Criteria<'mc>>) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/scoreboard/Criteria;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(criteria.into().jni_object().clone())});
args.push(val_1);
sig += ")Ljava/util/Set;";
let res = self.jni_ref().call_method(&self.jni_object(),"getObjectivesByCriteria",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn objectives(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getObjectives",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_scores_with_entry(&self,entry: impl Into<String>) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(entry.into())?));
args.push(val_1);
sig += ")Ljava/util/Set;";
let res = self.jni_ref().call_method(&self.jni_object(),"getScores",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn reset_scores_with_entry(&self,entry: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(entry.into())?));
args.push(val_1);
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"resetScores",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]

	pub fn get_player_team(&self,player: impl Into<crate::OfflinePlayer<'mc>>) 
-> Result<Option<crate::scoreboard::Team<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/OfflinePlayer;)Lcrate::scoreboard::Team;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayerTeam",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::scoreboard::Team::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn get_entry_team(&self,entry: impl Into<String>) 
-> Result<Option<crate::scoreboard::Team<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lcrate::scoreboard::Team;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(entry.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getEntryTeam",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::scoreboard::Team::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn get_team(&self,team_name: impl Into<String>) 
-> Result<Option<crate::scoreboard::Team<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lcrate::scoreboard::Team;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(team_name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getTeam",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::scoreboard::Team::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn teams(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTeams",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn register_new_team(&self,name: impl Into<String>) 
-> Result<crate::scoreboard::Team<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lcrate::scoreboard::Team;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"registerNewTeam",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::Team::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]

	pub fn players(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entries(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntries",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn clear_slot(&self,slot: impl Into<crate::scoreboard::DisplaySlot<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/scoreboard/DisplaySlot;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(slot.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"clearSlot",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct Criteria<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Criteria<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Criteria<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Criteria from null object.")
                .into());
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
    }
    
impl<'mc> Criteria<'mc> {
	pub fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn is_read_only(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isReadOnly",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn default_render_type(&self) 
-> Result<crate::scoreboard::RenderType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::scoreboard::RenderType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultRenderType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::RenderType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn statistic_with_statistic(jni: &blackboxmc_general::SharedJNIEnv<'mc>,statistic: impl Into<crate::Statistic<'mc>>,entity_type: std::option::Option<impl Into<crate::entity::EntityType<'mc>>>) 
-> Result<crate::scoreboard::Criteria<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Statistic;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(statistic.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = entity_type {
sig += "Lorg/bukkit/entity/EntityType;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/scoreboard/Criteria;";
let cls = jni.find_class("org/bukkit/scoreboard/Criteria"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"statistic",
sig.as_str(),args);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::scoreboard::Criteria::from_raw(&jni,obj
)}
	pub fn create(jni: &blackboxmc_general::SharedJNIEnv<'mc>,name: impl Into<String>) 
-> Result<crate::scoreboard::Criteria<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lcrate::scoreboard::Criteria;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(name.into())?));
let cls = jni.find_class("org/bukkit/scoreboard/Criteria"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"create",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::scoreboard::Criteria::from_raw(&jni,obj
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
