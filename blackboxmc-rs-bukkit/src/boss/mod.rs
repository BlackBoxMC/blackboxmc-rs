#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct DragonBattle<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate DragonBattle from null object.")
                .into());
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
	pub fn ender_dragon(&self) 
-> Result<Option<crate::entity::EnderDragon<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::entity::EnderDragon;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEnderDragon",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::EnderDragon::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn boss_bar(&self) 
-> Result<crate::boss::BossBar<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::boss::BossBar;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBossBar",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::boss::BossBar::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn end_portal_location(&self) 
-> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEndPortalLocation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn generate_end_portal(&self,with_portals: bool) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Z)Lbool;");
let val_1 = jni::objects::JValueGen::Bool(with_portals.into());
let res = self.jni_ref().call_method(&self.jni_object(),"generateEndPortal",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn has_been_previously_killed(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"hasBeenPreviouslyKilled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_previously_killed(&self,previously_killed: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(previously_killed.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setPreviouslyKilled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn respawn_phase(&self) 
-> Result<crate::boss::DragonBattleRespawnPhase<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::boss::DragonBattleRespawnPhase;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRespawnPhase",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::boss::DragonBattleRespawnPhase::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_respawn_phase(&self,phase: impl Into<crate::boss::DragonBattleRespawnPhase<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/boss/DragonBattle/RespawnPhase;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(phase.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setRespawnPhase",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn reset_crystals(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"resetCrystals",sig.as_str(),vec![]);
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
pub struct KeyedBossBar<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate KeyedBossBar from null object.")
                .into());
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
	pub fn title(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTitle",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn set_title(&self,title: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(title.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setTitle",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn color(&self) 
-> Result<crate::boss::BarColor<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::boss::BarColor;");
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::boss::BarColor::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_color(&self,color: impl Into<crate::boss::BarColor<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/boss/BarColor;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(color.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn style(&self) 
-> Result<crate::boss::BarStyle<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::boss::BarStyle;");
let res = self.jni_ref().call_method(&self.jni_object(),"getStyle",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::boss::BarStyle::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_style(&self,style: impl Into<crate::boss::BarStyle<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/boss/BarStyle;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(style.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setStyle",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn remove_flag(&self,flag: impl Into<crate::boss::BarFlag<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/boss/BarFlag;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(flag.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removeFlag",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn add_flag(&self,flag: impl Into<crate::boss::BarFlag<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/boss/BarFlag;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(flag.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addFlag",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn has_flag(&self,flag: impl Into<crate::boss::BarFlag<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/boss/BarFlag;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(flag.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"hasFlag",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_progress(&self,progress: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(progress);
let res = self.jni_ref().call_method(&self.jni_object(),"setProgress",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn progress(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getProgress",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn add_player(&self,player: impl Into<crate::entity::Player<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addPlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn remove_player(&self,player: impl Into<crate::entity::Player<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removePlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn remove_all(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn players(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn set_visible(&self,visible: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(visible.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setVisible",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_visible(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isVisible",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[deprecated]

	pub fn show(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"show",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]

	pub fn hide(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"hide",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn key(&self) 
-> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::NamespacedKey;");
let res = self.jni_ref().call_method(&self.jni_object(),"getKey",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::NamespacedKey::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::boss::BossBar<'mc>> for KeyedBossBar<'mc>{

fn into(self) -> crate::boss::BossBar<'mc> {

crate::boss::BossBar::from_raw(&self.jni_ref(), self.1).expect("Error converting KeyedBossBar into crate::boss::BossBar")

   }
}
impl<'mc> Into<crate::Keyed<'mc>> for KeyedBossBar<'mc>{

fn into(self) -> crate::Keyed<'mc> {

crate::Keyed::from_raw(&self.jni_ref(), self.1).expect("Error converting KeyedBossBar into crate::Keyed")

   }
}
pub enum BarStyle<'mc> {
	Solid {inner: BarStyleStruct<'mc>},
	Segmented6 {inner: BarStyleStruct<'mc>},
	Segmented10 {inner: BarStyleStruct<'mc>},
	Segmented12 {inner: BarStyleStruct<'mc>},
	Segmented20 {inner: BarStyleStruct<'mc>},
}
impl<'mc> std::fmt::Display for BarStyle<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           BarStyle::Solid { .. } => f.write_str("SOLID"),
           BarStyle::Segmented6 { .. } => f.write_str("SEGMENTED_6"),
           BarStyle::Segmented10 { .. } => f.write_str("SEGMENTED_10"),
           BarStyle::Segmented12 { .. } => f.write_str("SEGMENTED_12"),
           BarStyle::Segmented20 { .. } => f.write_str("SEGMENTED_20"),
       }
   }
}

        impl<'mc> BarStyle<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<BarStyle<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/boss/BarStyle");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/boss/BarStyle;",
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
                    
"SOLID" => Ok(BarStyle::Solid { inner: BarStyleStruct::from_raw(env,obj)?}),
"SEGMENTED_6" => Ok(BarStyle::Segmented6 { inner: BarStyleStruct::from_raw(env,obj)?}),
"SEGMENTED_10" => Ok(BarStyle::Segmented10 { inner: BarStyleStruct::from_raw(env,obj)?}),
"SEGMENTED_12" => Ok(BarStyle::Segmented12 { inner: BarStyleStruct::from_raw(env,obj)?}),
"SEGMENTED_20" => Ok(BarStyle::Segmented20 { inner: BarStyleStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct BarStyleStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BarStyle<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Solid { inner } => inner.0.clone(),
Self::Segmented6 { inner } => inner.0.clone(),
Self::Segmented10 { inner } => inner.0.clone(),
Self::Segmented12 { inner } => inner.0.clone(),
Self::Segmented20 { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Solid { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Segmented6 { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Segmented10 { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Segmented12 { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Segmented20 { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for BarStyle<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BarStyle from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/BarStyle")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BarStyle object, got {}",
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
                    "SOLID" => Ok(BarStyle::Solid { inner: BarStyleStruct::from_raw(env,obj)?}),"SEGMENTED_6" => Ok(BarStyle::Segmented6 { inner: BarStyleStruct::from_raw(env,obj)?}),"SEGMENTED_10" => Ok(BarStyle::Segmented10 { inner: BarStyleStruct::from_raw(env,obj)?}),"SEGMENTED_12" => Ok(BarStyle::Segmented12 { inner: BarStyleStruct::from_raw(env,obj)?}),"SEGMENTED_20" => Ok(BarStyle::Segmented20 { inner: BarStyleStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for BarStyleStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BarStyleStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BarStyleStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/BarStyle")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BarStyleStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BarStyleStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::boss::BarStyle<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::boss::BarStyle;");
let cls = jni.find_class("org/bukkit/boss/BarStyle"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::boss::BarStyle::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub enum BarColor<'mc> {
	Pink {inner: BarColorStruct<'mc>},
	Blue {inner: BarColorStruct<'mc>},
	Red {inner: BarColorStruct<'mc>},
	Green {inner: BarColorStruct<'mc>},
	Yellow {inner: BarColorStruct<'mc>},
	Purple {inner: BarColorStruct<'mc>},
	White {inner: BarColorStruct<'mc>},
}
impl<'mc> std::fmt::Display for BarColor<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           BarColor::Pink { .. } => f.write_str("PINK"),
           BarColor::Blue { .. } => f.write_str("BLUE"),
           BarColor::Red { .. } => f.write_str("RED"),
           BarColor::Green { .. } => f.write_str("GREEN"),
           BarColor::Yellow { .. } => f.write_str("YELLOW"),
           BarColor::Purple { .. } => f.write_str("PURPLE"),
           BarColor::White { .. } => f.write_str("WHITE"),
       }
   }
}

        impl<'mc> BarColor<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<BarColor<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/boss/BarColor");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/boss/BarColor;",
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
                    
"PINK" => Ok(BarColor::Pink { inner: BarColorStruct::from_raw(env,obj)?}),
"BLUE" => Ok(BarColor::Blue { inner: BarColorStruct::from_raw(env,obj)?}),
"RED" => Ok(BarColor::Red { inner: BarColorStruct::from_raw(env,obj)?}),
"GREEN" => Ok(BarColor::Green { inner: BarColorStruct::from_raw(env,obj)?}),
"YELLOW" => Ok(BarColor::Yellow { inner: BarColorStruct::from_raw(env,obj)?}),
"PURPLE" => Ok(BarColor::Purple { inner: BarColorStruct::from_raw(env,obj)?}),
"WHITE" => Ok(BarColor::White { inner: BarColorStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct BarColorStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BarColor<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Pink { inner } => inner.0.clone(),
Self::Blue { inner } => inner.0.clone(),
Self::Red { inner } => inner.0.clone(),
Self::Green { inner } => inner.0.clone(),
Self::Yellow { inner } => inner.0.clone(),
Self::Purple { inner } => inner.0.clone(),
Self::White { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Pink { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Blue { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Red { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Green { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Yellow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Purple { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::White { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for BarColor<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BarColor from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/BarColor")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BarColor object, got {}",
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
                    "PINK" => Ok(BarColor::Pink { inner: BarColorStruct::from_raw(env,obj)?}),"BLUE" => Ok(BarColor::Blue { inner: BarColorStruct::from_raw(env,obj)?}),"RED" => Ok(BarColor::Red { inner: BarColorStruct::from_raw(env,obj)?}),"GREEN" => Ok(BarColor::Green { inner: BarColorStruct::from_raw(env,obj)?}),"YELLOW" => Ok(BarColor::Yellow { inner: BarColorStruct::from_raw(env,obj)?}),"PURPLE" => Ok(BarColor::Purple { inner: BarColorStruct::from_raw(env,obj)?}),"WHITE" => Ok(BarColor::White { inner: BarColorStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for BarColorStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BarColorStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BarColorStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/BarColor")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BarColorStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BarColorStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::boss::BarColor<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::boss::BarColor;");
let cls = jni.find_class("org/bukkit/boss/BarColor"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::boss::BarColor::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub enum BarFlag<'mc> {
	DarkenSky {inner: BarFlagStruct<'mc>},
	PlayBossMusic {inner: BarFlagStruct<'mc>},
	CreateFog {inner: BarFlagStruct<'mc>},
}
impl<'mc> std::fmt::Display for BarFlag<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           BarFlag::DarkenSky { .. } => f.write_str("DARKEN_SKY"),
           BarFlag::PlayBossMusic { .. } => f.write_str("PLAY_BOSS_MUSIC"),
           BarFlag::CreateFog { .. } => f.write_str("CREATE_FOG"),
       }
   }
}

        impl<'mc> BarFlag<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<BarFlag<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/boss/BarFlag");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/boss/BarFlag;",
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
                    
"DARKEN_SKY" => Ok(BarFlag::DarkenSky { inner: BarFlagStruct::from_raw(env,obj)?}),
"PLAY_BOSS_MUSIC" => Ok(BarFlag::PlayBossMusic { inner: BarFlagStruct::from_raw(env,obj)?}),
"CREATE_FOG" => Ok(BarFlag::CreateFog { inner: BarFlagStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct BarFlagStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BarFlag<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::DarkenSky { inner } => inner.0.clone(),
Self::PlayBossMusic { inner } => inner.0.clone(),
Self::CreateFog { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::DarkenSky { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::PlayBossMusic { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CreateFog { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for BarFlag<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BarFlag from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/BarFlag")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BarFlag object, got {}",
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
                    "DARKEN_SKY" => Ok(BarFlag::DarkenSky { inner: BarFlagStruct::from_raw(env,obj)?}),"PLAY_BOSS_MUSIC" => Ok(BarFlag::PlayBossMusic { inner: BarFlagStruct::from_raw(env,obj)?}),"CREATE_FOG" => Ok(BarFlag::CreateFog { inner: BarFlagStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for BarFlagStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BarFlagStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BarFlagStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/BarFlag")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BarFlagStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BarFlagStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::boss::BarFlag<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::boss::BarFlag;");
let cls = jni.find_class("org/bukkit/boss/BarFlag"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::boss::BarFlag::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub enum DragonBattleRespawnPhase<'mc> {
	Start {inner: DragonBattleRespawnPhaseStruct<'mc>},
	PreparingToSummonPillars {inner: DragonBattleRespawnPhaseStruct<'mc>},
	SummoningPillars {inner: DragonBattleRespawnPhaseStruct<'mc>},
	SummoningDragon {inner: DragonBattleRespawnPhaseStruct<'mc>},
	End {inner: DragonBattleRespawnPhaseStruct<'mc>},
	None {inner: DragonBattleRespawnPhaseStruct<'mc>},
}
impl<'mc> std::fmt::Display for DragonBattleRespawnPhase<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           DragonBattleRespawnPhase::Start { .. } => f.write_str("START"),
           DragonBattleRespawnPhase::PreparingToSummonPillars { .. } => f.write_str("PREPARING_TO_SUMMON_PILLARS"),
           DragonBattleRespawnPhase::SummoningPillars { .. } => f.write_str("SUMMONING_PILLARS"),
           DragonBattleRespawnPhase::SummoningDragon { .. } => f.write_str("SUMMONING_DRAGON"),
           DragonBattleRespawnPhase::End { .. } => f.write_str("END"),
           DragonBattleRespawnPhase::None { .. } => f.write_str("NONE"),
       }
   }
}

        impl<'mc> DragonBattleRespawnPhase<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<DragonBattleRespawnPhase<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/boss/DragonBattle/RespawnPhase");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/boss/DragonBattle/RespawnPhase;",
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
                    
"START" => Ok(DragonBattleRespawnPhase::Start { inner: DragonBattleRespawnPhaseStruct::from_raw(env,obj)?}),
"PREPARING_TO_SUMMON_PILLARS" => Ok(DragonBattleRespawnPhase::PreparingToSummonPillars { inner: DragonBattleRespawnPhaseStruct::from_raw(env,obj)?}),
"SUMMONING_PILLARS" => Ok(DragonBattleRespawnPhase::SummoningPillars { inner: DragonBattleRespawnPhaseStruct::from_raw(env,obj)?}),
"SUMMONING_DRAGON" => Ok(DragonBattleRespawnPhase::SummoningDragon { inner: DragonBattleRespawnPhaseStruct::from_raw(env,obj)?}),
"END" => Ok(DragonBattleRespawnPhase::End { inner: DragonBattleRespawnPhaseStruct::from_raw(env,obj)?}),
"NONE" => Ok(DragonBattleRespawnPhase::None { inner: DragonBattleRespawnPhaseStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct DragonBattleRespawnPhaseStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for DragonBattleRespawnPhase<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Start { inner } => inner.0.clone(),
Self::PreparingToSummonPillars { inner } => inner.0.clone(),
Self::SummoningPillars { inner } => inner.0.clone(),
Self::SummoningDragon { inner } => inner.0.clone(),
Self::End { inner } => inner.0.clone(),
Self::None { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Start { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::PreparingToSummonPillars { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SummoningPillars { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SummoningDragon { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::End { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::None { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for DragonBattleRespawnPhase<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate DragonBattleRespawnPhase from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/DragonBattle/RespawnPhase")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a DragonBattleRespawnPhase object, got {}",
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
                    "START" => Ok(DragonBattleRespawnPhase::Start { inner: DragonBattleRespawnPhaseStruct::from_raw(env,obj)?}),"PREPARING_TO_SUMMON_PILLARS" => Ok(DragonBattleRespawnPhase::PreparingToSummonPillars { inner: DragonBattleRespawnPhaseStruct::from_raw(env,obj)?}),"SUMMONING_PILLARS" => Ok(DragonBattleRespawnPhase::SummoningPillars { inner: DragonBattleRespawnPhaseStruct::from_raw(env,obj)?}),"SUMMONING_DRAGON" => Ok(DragonBattleRespawnPhase::SummoningDragon { inner: DragonBattleRespawnPhaseStruct::from_raw(env,obj)?}),"END" => Ok(DragonBattleRespawnPhase::End { inner: DragonBattleRespawnPhaseStruct::from_raw(env,obj)?}),"NONE" => Ok(DragonBattleRespawnPhase::None { inner: DragonBattleRespawnPhaseStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for DragonBattleRespawnPhaseStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for DragonBattleRespawnPhaseStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate DragonBattleRespawnPhaseStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/boss/DragonBattle/RespawnPhase")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a DragonBattleRespawnPhaseStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DragonBattleRespawnPhaseStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::boss::DragonBattleRespawnPhase<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::boss::DragonBattleRespawnPhase;");
let cls = jni.find_class("org/bukkit/boss/DragonBattle/RespawnPhase"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::boss::DragonBattleRespawnPhase::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BossBar<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate BossBar from null object.")
                .into());
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
	pub fn title(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTitle",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn set_title(&self,title: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(title.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setTitle",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn color(&self) 
-> Result<crate::boss::BarColor<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::boss::BarColor;");
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::boss::BarColor::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_color(&self,color: impl Into<crate::boss::BarColor<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/boss/BarColor;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(color.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn style(&self) 
-> Result<crate::boss::BarStyle<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::boss::BarStyle;");
let res = self.jni_ref().call_method(&self.jni_object(),"getStyle",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::boss::BarStyle::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_style(&self,style: impl Into<crate::boss::BarStyle<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/boss/BarStyle;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(style.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setStyle",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn remove_flag(&self,flag: impl Into<crate::boss::BarFlag<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/boss/BarFlag;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(flag.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removeFlag",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn add_flag(&self,flag: impl Into<crate::boss::BarFlag<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/boss/BarFlag;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(flag.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addFlag",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn has_flag(&self,flag: impl Into<crate::boss::BarFlag<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/boss/BarFlag;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(flag.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"hasFlag",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_progress(&self,progress: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(progress);
let res = self.jni_ref().call_method(&self.jni_object(),"setProgress",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn progress(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getProgress",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn add_player(&self,player: impl Into<crate::entity::Player<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addPlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn remove_player(&self,player: impl Into<crate::entity::Player<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removePlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn remove_all(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn players(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn set_visible(&self,visible: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(visible.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setVisible",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_visible(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isVisible",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[deprecated]

	pub fn show(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"show",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]

	pub fn hide(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"hide",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
