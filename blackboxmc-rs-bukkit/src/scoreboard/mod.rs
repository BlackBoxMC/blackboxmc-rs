#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/scoreboard/mod.rs*/

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
    
impl<'mc> ScoreTrait<'mc> for Score<'mc> {}
pub trait ScoreTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]
/// Gets the OfflinePlayer being tracked by this Score
	fn player(&self) 
-> Result<crate::OfflinePlayer<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/OfflinePlayer;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::OfflinePlayer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the entry being tracked by this Score
	fn entry(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntry",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gets the Objective being tracked by this Score
	fn objective(&self) 
-> Result<crate::scoreboard::Objective<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/Objective;");
let res = self.jni_ref().call_method(&self.jni_object(),"getObjective",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::Objective::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the current score
	fn score(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getScore",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the current score.
	fn set_score(&self,score: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(score);
let res = self.jni_ref().call_method(&self.jni_object(),"setScore",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Shows if this score has been set at any point in time.
	fn is_score_set(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isScoreSet",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the scoreboard for the associated objective.
	fn scoreboard(&self) 
-> Result<Option<crate::scoreboard::Scoreboard<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/Scoreboard;");
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

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> ScoreboardManagerTrait<'mc> for ScoreboardManager<'mc> {}
pub trait ScoreboardManagerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the primary Scoreboard controlled by the server.
/// 
/// This Scoreboard is saved by the server, is affected by the /scoreboard
/// command, and is the scoreboard shown by default to players.
	fn main_scoreboard(&self) 
-> Result<crate::scoreboard::Scoreboard<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/Scoreboard;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMainScoreboard",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::Scoreboard::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets a new Scoreboard to be tracked by the server. This scoreboard will
/// be tracked as long as a reference is kept, either by a player or by a
/// plugin.
	fn new_scoreboard(&self) 
-> Result<crate::scoreboard::Scoreboard<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/Scoreboard;");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewScoreboard",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::Scoreboard::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> TeamTrait<'mc> for Team<'mc> {}
pub trait TeamTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the name of this Team
	fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gets the name displayed to entries for this team
	fn display_name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the name displayed to entries for this team
	fn set_display_name(&self,display_name: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(display_name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayName",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the prefix prepended to the display of entries on this team.
	fn prefix(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPrefix",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the prefix prepended to the display of entries on this team.
	fn set_prefix(&self,prefix: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(prefix.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setPrefix",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the suffix appended to the display of entries on this team.
	fn suffix(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSuffix",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the suffix appended to the display of entries on this team.
	fn set_suffix(&self,suffix: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(suffix.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setSuffix",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the color of the team.
/// 
/// This only sets the team outline, other occurrences of colors such as in
/// names are handled by prefixes / suffixes.
	fn color(&self) 
-> Result<crate::ChatColor<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/ChatColor;");
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::ChatColor::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the color of the team.
/// 
/// This only sets the team outline, other occurrences of colors such as in
/// names are handled by prefixes / suffixes.
	fn set_color(&self,color: impl Into<crate::ChatColor<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/ChatColor;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(color.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the team friendly fire state
	fn allow_friendly_fire(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"allowFriendlyFire",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets the team friendly fire state
	fn set_allow_friendly_fire(&self,enabled: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(enabled.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setAllowFriendlyFire",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the team's ability to see {@link PotionEffectType#INVISIBILITY
/// invisible} teammates.
	fn can_see_friendly_invisibles(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"canSeeFriendlyInvisibles",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets the team's ability to see {@link PotionEffectType#INVISIBILITY
/// invisible} teammates.
	fn set_can_see_friendly_invisibles(&self,enabled: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(enabled.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCanSeeFriendlyInvisibles",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Gets the team's ability to see name tags
	fn name_tag_visibility(&self) 
-> Result<crate::scoreboard::NameTagVisibility<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/NameTagVisibility;");
let res = self.jni_ref().call_method(&self.jni_object(),"getNameTagVisibility",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::NameTagVisibility::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Set's the team's ability to see name tags
	fn set_name_tag_visibility(&self,visibility: impl Into<crate::scoreboard::NameTagVisibility<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/scoreboard/NameTagVisibility;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(visibility.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setNameTagVisibility",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Gets the Set of players on the team
	fn players(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the Set of entries on the team
	fn entries(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntries",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the size of the team
	fn size(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getSize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the Scoreboard to which this team is attached
	fn scoreboard(&self) 
-> Result<Option<crate::scoreboard::Scoreboard<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/Scoreboard;");
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
/// This puts the specified player onto this team for the scoreboard.This will remove the player from any other team on the scoreboard.
	fn add_player(&self,player: impl Into<crate::OfflinePlayer<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/OfflinePlayer;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addPlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// This puts the specified entry onto this team for the scoreboard.
/// 
/// This will remove the entry from any other team on the scoreboard.
	fn add_entry(&self,entry: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(entry.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"addEntry",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Removes the player from this team.
	fn remove_player(&self,player: impl Into<crate::OfflinePlayer<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/OfflinePlayer;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removePlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Removes the entry from this team.
	fn remove_entry(&self,entry: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(entry.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"removeEntry",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Unregisters this team from the Scoreboard
	fn unregister(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"unregister",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Checks to see if the specified player is a member of this team.
	fn has_player(&self,player: impl Into<crate::OfflinePlayer<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/OfflinePlayer;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"hasPlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks to see if the specified entry is a member of this team.
	fn has_entry(&self,entry: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(entry.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"hasEntry",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Get an option for this team
	fn get_option(&self,option: impl Into<crate::scoreboard::TeamOption<'mc>>) 
-> Result<crate::scoreboard::TeamOptionStatus<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/scoreboard/Team/Option;)Lorg/bukkit/scoreboard/Team/OptionStatus;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(option.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getOption",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::TeamOptionStatus::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Set an option for this team
	fn set_option(&self,option: impl Into<crate::scoreboard::TeamOption<'mc>>,status: impl Into<crate::scoreboard::TeamOptionStatus<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/scoreboard/Team/Option;Lorg/bukkit/scoreboard/Team/OptionStatus;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(option.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(status.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setOption",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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

        impl<'mc> TeamOptionStatusTrait<'mc> for TeamOptionStatus<'mc> {}
        
        pub trait TeamOptionStatusTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
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

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::scoreboard::TeamOptionStatus<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/Team/OptionStatus;");
let cls = jni.find_class("org/bukkit/scoreboard/Team/OptionStatus"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::scoreboard::TeamOptionStatus::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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

        impl<'mc> NameTagVisibilityTrait<'mc> for NameTagVisibility<'mc> {}
        
        pub trait NameTagVisibilityTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
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

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::scoreboard::NameTagVisibility<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/NameTagVisibility;");
let cls = jni.find_class("org/bukkit/scoreboard/NameTagVisibility"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::scoreboard::NameTagVisibility::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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

        impl<'mc> DisplaySlotTrait<'mc> for DisplaySlot<'mc> {}
        
        pub trait DisplaySlotTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
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

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::scoreboard::DisplaySlot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/DisplaySlot;");
let cls = jni.find_class("org/bukkit/scoreboard/DisplaySlot"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::scoreboard::DisplaySlot::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> CriteriasTrait<'mc> for Criterias<'mc> {}
pub trait CriteriasTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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

        impl<'mc> RenderTypeTrait<'mc> for RenderType<'mc> {}
        
        pub trait RenderTypeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
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

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::scoreboard::RenderType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/RenderType;");
let cls = jni.find_class("org/bukkit/scoreboard/RenderType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::scoreboard::RenderType::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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

        impl<'mc> TeamOptionTrait<'mc> for TeamOption<'mc> {}
        
        pub trait TeamOptionTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
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

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::scoreboard::TeamOption<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/Team/Option;");
let cls = jni.find_class("org/bukkit/scoreboard/Team/Option"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::scoreboard::TeamOption::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> ObjectiveTrait<'mc> for Objective<'mc> {}
pub trait ObjectiveTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the name of this Objective
	fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gets the name displayed to players for this objective
	fn display_name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the name displayed to players for this objective.
	fn set_display_name(&self,display_name: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(display_name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayName",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Gets the criteria this objective tracks.
	fn criteria(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCriteria",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gets the criteria this objective tracks.
	fn tracked_criteria(&self) 
-> Result<crate::scoreboard::Criteria<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/Criteria;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTrackedCriteria",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::Criteria::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets if the objective's scores can be modified directly by a plugin.
	fn is_modifiable(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isModifiable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the scoreboard to which this objective is attached.
	fn scoreboard(&self) 
-> Result<Option<crate::scoreboard::Scoreboard<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/Scoreboard;");
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
/// Unregisters this objective from the {@link Scoreboard scoreboard.}
	fn unregister(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"unregister",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets this objective to display on the specified slot for the
/// scoreboard, removing it from any other display slot.
	fn set_display_slot(&self,slot: impl Into<crate::scoreboard::DisplaySlot<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/scoreboard/DisplaySlot;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(slot.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplaySlot",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the display slot this objective is displayed at.
	fn display_slot(&self) 
-> Result<Option<crate::scoreboard::DisplaySlot<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/DisplaySlot;");
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
/// Sets manner in which this objective will be rendered.
	fn set_render_type(&self,render_type: impl Into<crate::scoreboard::RenderType<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/scoreboard/RenderType;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(render_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setRenderType",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets manner in which this objective will be rendered.
	fn render_type(&self) 
-> Result<crate::scoreboard::RenderType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/RenderType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRenderType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::RenderType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets an entry's Score for an Objective on this Scoreboard.
	fn get_score(&self,entry: impl Into<String>) 
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

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> ScoreboardTrait<'mc> for Scoreboard<'mc> {}
pub trait ScoreboardTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Registers an Objective on this Scoreboard
	fn register_new_objective(&self,name: impl Into<String>,criteria: impl Into<crate::scoreboard::Criteria<'mc>>,display_name: std::option::Option<impl Into<String>>,render_type: std::option::Option<impl Into<crate::scoreboard::RenderType<'mc>>>) 
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
/// Gets the Objective currently displayed in a DisplaySlot on this
/// Scoreboard
	fn get_objective(&self,slot: impl Into<crate::scoreboard::DisplaySlot<'mc>>) 
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
/// Gets all Objectives of a Criteria on the Scoreboard
	fn get_objectives_by_criteria(&self,criteria: impl Into<crate::scoreboard::Criteria<'mc>>) 
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
/// Gets all Objectives on this Scoreboard
	fn objectives(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getObjectives",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets all scores for an entry on this Scoreboard
	fn get_scores(&self,entry: impl Into<String>) 
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
/// Removes all scores for an entry on this Scoreboard
	fn reset_scores(&self,entry: impl Into<String>) 
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
/// Gets a player's Team on this Scoreboard
	fn get_player_team(&self,player: impl Into<crate::OfflinePlayer<'mc>>) 
-> Result<Option<crate::scoreboard::Team<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/OfflinePlayer;)Lorg/bukkit/scoreboard/Team;");
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
/// Gets a entries Team on this Scoreboard
	fn get_entry_team(&self,entry: impl Into<String>) 
-> Result<Option<crate::scoreboard::Team<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/scoreboard/Team;");
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
/// Gets a Team by name on this Scoreboard
	fn get_team(&self,team_name: impl Into<String>) 
-> Result<Option<crate::scoreboard::Team<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/scoreboard/Team;");
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
/// Gets all teams on this Scoreboard
	fn teams(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTeams",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Registers a Team on this Scoreboard
	fn register_new_team(&self,name: impl Into<String>) 
-> Result<crate::scoreboard::Team<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/scoreboard/Team;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"registerNewTeam",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::Team::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Gets all players tracked by this Scoreboard
	fn players(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets all entries tracked by this Scoreboard
	fn entries(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntries",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Clears any objective in the specified slot.
	fn clear_slot(&self,slot: impl Into<crate::scoreboard::DisplaySlot<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/scoreboard/DisplaySlot;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(slot.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"clearSlot",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> CriteriaTrait<'mc> for Criteria<'mc> {}
pub trait CriteriaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the name of this criteria (its unique id).
	fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Get whether or not this criteria is read only. If read only, scoreboards with this criteria
/// cannot have their scores changed.
	fn is_read_only(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isReadOnly",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Get the {@link RenderType} used by default for this criteria.
	fn default_render_type(&self) 
-> Result<crate::scoreboard::RenderType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/scoreboard/RenderType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultRenderType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scoreboard::RenderType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get a {@link Criteria} for the specified statistic pertaining to an entity type.
/// 
/// This method expects a {@link Statistic} of {@link Type#ENTITY}. This acts as a convenience
/// to create more complex compound criteria such as being killed by a specific entity type.
/// An example would be {@code Criteria.statistic(Statistic.KILL_ENTITY, EntityType.CREEPER)},
/// returning a Criteria representing "minecraft.killed:minecraft.creeper" which will increment
/// when the player kills a creepers.
/// 
/// If the provided statistic does not require additional data, {@link #statistic(Statistic)}
/// is called and returned instead.
/// 
/// This method provides no guarantee that any given criteria exists on the vanilla server.
	fn statistic(jni: &blackboxmc_general::SharedJNIEnv<'mc>,statistic: impl Into<crate::Statistic<'mc>>,entity_type: std::option::Option<impl Into<crate::entity::EntityType<'mc>>>) 
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
/// Get (or create) a new {@link Criteria} by its name.
	fn create(jni: &blackboxmc_general::SharedJNIEnv<'mc>,name: impl Into<String>) 
-> Result<crate::scoreboard::Criteria<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/scoreboard/Criteria;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(name.into())?));
let cls = jni.find_class("org/bukkit/scoreboard/Criteria"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"create",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::scoreboard::Criteria::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
