#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/potion/mod.rs*/

#[repr(C)]
pub struct Potion<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Potion<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Potion<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Potion from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/Potion")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Potion object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PotionTrait<'mc> for Potion<'mc> {}
pub trait PotionTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]
/// Create a new potion of the given type and level.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: impl Into<crate::potion::PotionType<'mc>>,level: std::option::Option<i32>,splash: std::option::Option<bool>,extended: std::option::Option<bool>) 
-> Result<crate::potion::Potion<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/potion/PotionType;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = level {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a);
args.push(val_2);
}
if let Some(a) = splash {
sig += "Z";
let val_3 = jni::objects::JValueGen::Bool(a.into());
args.push(val_3);
}
if let Some(a) = extended {
sig += "Z";
let val_4 = jni::objects::JValueGen::Bool(a.into());
args.push(val_4);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/potion/Potion"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::potion::Potion::from_raw(&jni,res
)}
/// Chain this to the constructor to make the potion a splash potion.
	fn splash(&self) 
-> Result<crate::potion::Potion<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/potion/Potion;");
let res = self.jni_ref().call_method(&self.jni_object(),"splash",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::potion::Potion::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Chain this to the constructor to extend the potion's duration.
	fn extend(&self) 
-> Result<crate::potion::Potion<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/potion/Potion;");
let res = self.jni_ref().call_method(&self.jni_object(),"extend",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::potion::Potion::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Applies the effects that would be applied by this potion to the given
/// {@link LivingEntity}.
	fn apply(&self,to: impl Into<crate::entity::LivingEntity<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/LivingEntity;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(to.into().jni_object().clone())});
args.push(val_1);
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"apply",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns a collection of {@link PotionEffect}s that this {@link Potion}
/// would confer upon a {@link LivingEntity}.
	fn effects(&self) 
-> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Collection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEffects",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Returns the level of this potion.
	fn level(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getLevel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Returns the {@link PotionType} of this potion.
	fn get_type(&self) 
-> Result<crate::potion::PotionType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/potion/PotionType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::potion::PotionType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns whether this potion has an extended duration.
	fn has_extended_duration(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasExtendedDuration",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Returns whether this potion is a splash potion.
	fn is_splash(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isSplash",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set whether this potion has extended duration. This will cause the
/// potion to have roughly 8/3 more duration than a regular potion.
	fn set_has_extended_duration(&self,is_extended: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_extended.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setHasExtendedDuration",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets whether this potion is a splash potion. Splash potions can be
/// thrown for a radius effect.
	fn set_splash(&self,is_splash: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_splash.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSplash",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets the {@link PotionType} of this potion.
	fn set_type(&self,val_type: impl Into<crate::potion::PotionType<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionType;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setType",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets the level of this potion.
	fn set_level(&self,level: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(level);
let res = self.jni_ref().call_method(&self.jni_object(),"setLevel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Converts this potion to a valid potion damage short, usable for potion item stacks.
	fn to_damage_value(&self) 
-> Result<i16, Box<dyn std::error::Error>>

{let sig = String::from("()S");
let res = self.jni_ref().call_method(&self.jni_object(),"toDamageValue",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.s()?
)}
/// Converts this potion to an {@link ItemStack} with the specified amount
/// and a correct damage value.
	fn to_item_stack(&self,amount: i32) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
let val_1 = jni::objects::JValueGen::Int(amount);
let res = self.jni_ref().call_method(&self.jni_object(),"toItemStack",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the potion from its damage value.
	fn from_damage(jni: &blackboxmc_general::SharedJNIEnv<'mc>,damage: i32) 
-> Result<crate::potion::Potion<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(I)Lorg/bukkit/potion/Potion;");
let val_1 = jni::objects::JValueGen::Int(damage);
let cls = jni.find_class("org/bukkit/potion/Potion"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"fromDamage",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::potion::Potion::from_raw(&jni,obj
)}

	fn from_item_stack(jni: &blackboxmc_general::SharedJNIEnv<'mc>,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<crate::potion::Potion<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Lorg/bukkit/potion/Potion;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/potion/Potion"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"fromItemStack",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::potion::Potion::from_raw(&jni,obj
)}
/// Returns an instance of {@link PotionBrewer}.
	fn brewer(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::potion::PotionBrewer<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/potion/PotionBrewer;");
let cls = jni.find_class("org/bukkit/potion/Potion"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getBrewer",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::potion::PotionBrewer::from_raw(&jni,obj
)}
/// Sets the current instance of {@link PotionBrewer}. Generally not to be
/// used from within a plugin.
	fn set_potion_brewer(jni: &blackboxmc_general::SharedJNIEnv<'mc>,other: impl Into<crate::potion::PotionBrewer<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionBrewer;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/potion/Potion"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"setPotionBrewer",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
jni.translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Gets the potion from its name id.
	fn name_id(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getNameId",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PotionEffectTypeWrapper<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PotionEffectTypeWrapper<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PotionEffectTypeWrapper<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PotionEffectTypeWrapper from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionEffectTypeWrapper")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PotionEffectTypeWrapper object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PotionEffectTypeWrapperTrait<'mc> for PotionEffectTypeWrapper<'mc> {}
pub trait PotionEffectTypeWrapperTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the potion type bound to this wrapper.
	fn get_type(&self) 
-> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::potion::PotionEffectType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::potion::PotionEffectType<'mc>> for PotionEffectTypeWrapper<'mc>{

fn into(self) -> crate::potion::PotionEffectType<'mc> {

crate::potion::PotionEffectType::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PotionEffectTypeWrapper into crate::potion::PotionEffectType")

   }
}
impl<'mc> crate::potion::PotionEffectTypeTrait<'mc> for PotionEffectTypeWrapper<'mc> {}
#[repr(C)]
pub struct PotionBrewer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PotionBrewer<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PotionBrewer<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PotionBrewer from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionBrewer")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PotionBrewer object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PotionBrewerTrait<'mc> for PotionBrewer<'mc> {}
pub trait PotionBrewerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Creates a {@link PotionEffect} from the given {@link PotionEffectType},
/// applying duration modifiers and checks.
	fn create_effect(&self,potion: impl Into<crate::potion::PotionEffectType<'mc>>,duration: i32,amplifier: i32) 
-> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;II)Lorg/bukkit/potion/PotionEffect;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(potion.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(duration);
let val_3 = jni::objects::JValueGen::Int(amplifier);
let res = self.jni_ref().call_method(&self.jni_object(),"createEffect",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::potion::PotionEffect::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Returns a collection of {@link PotionEffect} that would be applied from a potion with the given data value.
	fn get_effects_from_damage(&self,damage: i32) 
-> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(I)Ljava/util/Collection;");
let val_1 = jni::objects::JValueGen::Int(damage);
let res = self.jni_ref().call_method(&self.jni_object(),"getEffectsFromDamage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
#[deprecated]
/// Returns a collection of {@link PotionEffect} that would be applied from a potion with the given type.
	fn get_effects(&self,val_type: impl Into<crate::potion::PotionType<'mc>>,upgraded: bool,extended: bool) 
-> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionType;ZZ)Ljava/util/Collection;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Bool(upgraded.into());
let val_3 = jni::objects::JValueGen::Bool(extended.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getEffects",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PotionData<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PotionData<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PotionData<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PotionData from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionData")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PotionData object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PotionDataTrait<'mc> for PotionData<'mc> {}
pub trait PotionDataTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Instantiates a final PotionData object to contain information about a
/// Potion
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: impl Into<crate::potion::PotionType<'mc>>,extended: std::option::Option<bool>,upgraded: std::option::Option<bool>) 
-> Result<crate::potion::PotionData<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/potion/PotionType;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = extended {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
if let Some(a) = upgraded {
sig += "Z";
let val_3 = jni::objects::JValueGen::Bool(a.into());
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/potion/PotionData"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::potion::PotionData::from_raw(&jni,res
)}
/// Gets the type of the potion, Type matches up with each kind of craftable
/// potion
	fn get_type(&self) 
-> Result<crate::potion::PotionType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/potion/PotionType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::potion::PotionType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Checks if the potion is in an upgraded state. This refers to whether or
/// not the potion is Tier 2, such as Potion of Fire Resistance II.
	fn is_upgraded(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isUpgraded",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks if the potion is in an extended state. This refers to the extended
/// duration potions
	fn is_extended(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isExtended",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub enum PotionType<'mc> {
	Uncraftable {inner: PotionTypeStruct<'mc>},
	Water {inner: PotionTypeStruct<'mc>},
	Mundane {inner: PotionTypeStruct<'mc>},
	Thick {inner: PotionTypeStruct<'mc>},
	Awkward {inner: PotionTypeStruct<'mc>},
	NightVision {inner: PotionTypeStruct<'mc>},
	LongNightVision {inner: PotionTypeStruct<'mc>},
	Invisibility {inner: PotionTypeStruct<'mc>},
	LongInvisibility {inner: PotionTypeStruct<'mc>},
	Jump {inner: PotionTypeStruct<'mc>},
	LongLeaping {inner: PotionTypeStruct<'mc>},
	StrongLeaping {inner: PotionTypeStruct<'mc>},
	FireResistance {inner: PotionTypeStruct<'mc>},
	LongFireResistance {inner: PotionTypeStruct<'mc>},
	Speed {inner: PotionTypeStruct<'mc>},
	LongSwiftness {inner: PotionTypeStruct<'mc>},
	StrongSwiftness {inner: PotionTypeStruct<'mc>},
	Slowness {inner: PotionTypeStruct<'mc>},
	LongSlowness {inner: PotionTypeStruct<'mc>},
	StrongSlowness {inner: PotionTypeStruct<'mc>},
	WaterBreathing {inner: PotionTypeStruct<'mc>},
	LongWaterBreathing {inner: PotionTypeStruct<'mc>},
	InstantHeal {inner: PotionTypeStruct<'mc>},
	StrongHealing {inner: PotionTypeStruct<'mc>},
	InstantDamage {inner: PotionTypeStruct<'mc>},
	StrongHarming {inner: PotionTypeStruct<'mc>},
	Poison {inner: PotionTypeStruct<'mc>},
	LongPoison {inner: PotionTypeStruct<'mc>},
	StrongPoison {inner: PotionTypeStruct<'mc>},
	Regen {inner: PotionTypeStruct<'mc>},
	LongRegeneration {inner: PotionTypeStruct<'mc>},
	StrongRegeneration {inner: PotionTypeStruct<'mc>},
	Strength {inner: PotionTypeStruct<'mc>},
	LongStrength {inner: PotionTypeStruct<'mc>},
	StrongStrength {inner: PotionTypeStruct<'mc>},
	Weakness {inner: PotionTypeStruct<'mc>},
	LongWeakness {inner: PotionTypeStruct<'mc>},
	Luck {inner: PotionTypeStruct<'mc>},
	TurtleMaster {inner: PotionTypeStruct<'mc>},
	LongTurtleMaster {inner: PotionTypeStruct<'mc>},
	StrongTurtleMaster {inner: PotionTypeStruct<'mc>},
	SlowFalling {inner: PotionTypeStruct<'mc>},
	LongSlowFalling {inner: PotionTypeStruct<'mc>},
}
impl<'mc> std::fmt::Display for PotionType<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PotionType::Uncraftable { .. } => f.write_str("UNCRAFTABLE"),
           PotionType::Water { .. } => f.write_str("WATER"),
           PotionType::Mundane { .. } => f.write_str("MUNDANE"),
           PotionType::Thick { .. } => f.write_str("THICK"),
           PotionType::Awkward { .. } => f.write_str("AWKWARD"),
           PotionType::NightVision { .. } => f.write_str("NIGHT_VISION"),
           PotionType::LongNightVision { .. } => f.write_str("LONG_NIGHT_VISION"),
           PotionType::Invisibility { .. } => f.write_str("INVISIBILITY"),
           PotionType::LongInvisibility { .. } => f.write_str("LONG_INVISIBILITY"),
           PotionType::Jump { .. } => f.write_str("JUMP"),
           PotionType::LongLeaping { .. } => f.write_str("LONG_LEAPING"),
           PotionType::StrongLeaping { .. } => f.write_str("STRONG_LEAPING"),
           PotionType::FireResistance { .. } => f.write_str("FIRE_RESISTANCE"),
           PotionType::LongFireResistance { .. } => f.write_str("LONG_FIRE_RESISTANCE"),
           PotionType::Speed { .. } => f.write_str("SPEED"),
           PotionType::LongSwiftness { .. } => f.write_str("LONG_SWIFTNESS"),
           PotionType::StrongSwiftness { .. } => f.write_str("STRONG_SWIFTNESS"),
           PotionType::Slowness { .. } => f.write_str("SLOWNESS"),
           PotionType::LongSlowness { .. } => f.write_str("LONG_SLOWNESS"),
           PotionType::StrongSlowness { .. } => f.write_str("STRONG_SLOWNESS"),
           PotionType::WaterBreathing { .. } => f.write_str("WATER_BREATHING"),
           PotionType::LongWaterBreathing { .. } => f.write_str("LONG_WATER_BREATHING"),
           PotionType::InstantHeal { .. } => f.write_str("INSTANT_HEAL"),
           PotionType::StrongHealing { .. } => f.write_str("STRONG_HEALING"),
           PotionType::InstantDamage { .. } => f.write_str("INSTANT_DAMAGE"),
           PotionType::StrongHarming { .. } => f.write_str("STRONG_HARMING"),
           PotionType::Poison { .. } => f.write_str("POISON"),
           PotionType::LongPoison { .. } => f.write_str("LONG_POISON"),
           PotionType::StrongPoison { .. } => f.write_str("STRONG_POISON"),
           PotionType::Regen { .. } => f.write_str("REGEN"),
           PotionType::LongRegeneration { .. } => f.write_str("LONG_REGENERATION"),
           PotionType::StrongRegeneration { .. } => f.write_str("STRONG_REGENERATION"),
           PotionType::Strength { .. } => f.write_str("STRENGTH"),
           PotionType::LongStrength { .. } => f.write_str("LONG_STRENGTH"),
           PotionType::StrongStrength { .. } => f.write_str("STRONG_STRENGTH"),
           PotionType::Weakness { .. } => f.write_str("WEAKNESS"),
           PotionType::LongWeakness { .. } => f.write_str("LONG_WEAKNESS"),
           PotionType::Luck { .. } => f.write_str("LUCK"),
           PotionType::TurtleMaster { .. } => f.write_str("TURTLE_MASTER"),
           PotionType::LongTurtleMaster { .. } => f.write_str("LONG_TURTLE_MASTER"),
           PotionType::StrongTurtleMaster { .. } => f.write_str("STRONG_TURTLE_MASTER"),
           PotionType::SlowFalling { .. } => f.write_str("SLOW_FALLING"),
           PotionType::LongSlowFalling { .. } => f.write_str("LONG_SLOW_FALLING"),
       }
   }
}

        impl<'mc> PotionTypeTrait<'mc> for PotionType<'mc> {}
        
        pub trait PotionTypeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PotionType<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/potion/PotionType");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/potion/PotionType;",
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
                    
"UNCRAFTABLE" => Ok(PotionType::Uncraftable { inner: PotionTypeStruct::from_raw(env,obj)?}),
"WATER" => Ok(PotionType::Water { inner: PotionTypeStruct::from_raw(env,obj)?}),
"MUNDANE" => Ok(PotionType::Mundane { inner: PotionTypeStruct::from_raw(env,obj)?}),
"THICK" => Ok(PotionType::Thick { inner: PotionTypeStruct::from_raw(env,obj)?}),
"AWKWARD" => Ok(PotionType::Awkward { inner: PotionTypeStruct::from_raw(env,obj)?}),
"NIGHT_VISION" => Ok(PotionType::NightVision { inner: PotionTypeStruct::from_raw(env,obj)?}),
"LONG_NIGHT_VISION" => Ok(PotionType::LongNightVision { inner: PotionTypeStruct::from_raw(env,obj)?}),
"INVISIBILITY" => Ok(PotionType::Invisibility { inner: PotionTypeStruct::from_raw(env,obj)?}),
"LONG_INVISIBILITY" => Ok(PotionType::LongInvisibility { inner: PotionTypeStruct::from_raw(env,obj)?}),
"JUMP" => Ok(PotionType::Jump { inner: PotionTypeStruct::from_raw(env,obj)?}),
"LONG_LEAPING" => Ok(PotionType::LongLeaping { inner: PotionTypeStruct::from_raw(env,obj)?}),
"STRONG_LEAPING" => Ok(PotionType::StrongLeaping { inner: PotionTypeStruct::from_raw(env,obj)?}),
"FIRE_RESISTANCE" => Ok(PotionType::FireResistance { inner: PotionTypeStruct::from_raw(env,obj)?}),
"LONG_FIRE_RESISTANCE" => Ok(PotionType::LongFireResistance { inner: PotionTypeStruct::from_raw(env,obj)?}),
"SPEED" => Ok(PotionType::Speed { inner: PotionTypeStruct::from_raw(env,obj)?}),
"LONG_SWIFTNESS" => Ok(PotionType::LongSwiftness { inner: PotionTypeStruct::from_raw(env,obj)?}),
"STRONG_SWIFTNESS" => Ok(PotionType::StrongSwiftness { inner: PotionTypeStruct::from_raw(env,obj)?}),
"SLOWNESS" => Ok(PotionType::Slowness { inner: PotionTypeStruct::from_raw(env,obj)?}),
"LONG_SLOWNESS" => Ok(PotionType::LongSlowness { inner: PotionTypeStruct::from_raw(env,obj)?}),
"STRONG_SLOWNESS" => Ok(PotionType::StrongSlowness { inner: PotionTypeStruct::from_raw(env,obj)?}),
"WATER_BREATHING" => Ok(PotionType::WaterBreathing { inner: PotionTypeStruct::from_raw(env,obj)?}),
"LONG_WATER_BREATHING" => Ok(PotionType::LongWaterBreathing { inner: PotionTypeStruct::from_raw(env,obj)?}),
"INSTANT_HEAL" => Ok(PotionType::InstantHeal { inner: PotionTypeStruct::from_raw(env,obj)?}),
"STRONG_HEALING" => Ok(PotionType::StrongHealing { inner: PotionTypeStruct::from_raw(env,obj)?}),
"INSTANT_DAMAGE" => Ok(PotionType::InstantDamage { inner: PotionTypeStruct::from_raw(env,obj)?}),
"STRONG_HARMING" => Ok(PotionType::StrongHarming { inner: PotionTypeStruct::from_raw(env,obj)?}),
"POISON" => Ok(PotionType::Poison { inner: PotionTypeStruct::from_raw(env,obj)?}),
"LONG_POISON" => Ok(PotionType::LongPoison { inner: PotionTypeStruct::from_raw(env,obj)?}),
"STRONG_POISON" => Ok(PotionType::StrongPoison { inner: PotionTypeStruct::from_raw(env,obj)?}),
"REGEN" => Ok(PotionType::Regen { inner: PotionTypeStruct::from_raw(env,obj)?}),
"LONG_REGENERATION" => Ok(PotionType::LongRegeneration { inner: PotionTypeStruct::from_raw(env,obj)?}),
"STRONG_REGENERATION" => Ok(PotionType::StrongRegeneration { inner: PotionTypeStruct::from_raw(env,obj)?}),
"STRENGTH" => Ok(PotionType::Strength { inner: PotionTypeStruct::from_raw(env,obj)?}),
"LONG_STRENGTH" => Ok(PotionType::LongStrength { inner: PotionTypeStruct::from_raw(env,obj)?}),
"STRONG_STRENGTH" => Ok(PotionType::StrongStrength { inner: PotionTypeStruct::from_raw(env,obj)?}),
"WEAKNESS" => Ok(PotionType::Weakness { inner: PotionTypeStruct::from_raw(env,obj)?}),
"LONG_WEAKNESS" => Ok(PotionType::LongWeakness { inner: PotionTypeStruct::from_raw(env,obj)?}),
"LUCK" => Ok(PotionType::Luck { inner: PotionTypeStruct::from_raw(env,obj)?}),
"TURTLE_MASTER" => Ok(PotionType::TurtleMaster { inner: PotionTypeStruct::from_raw(env,obj)?}),
"LONG_TURTLE_MASTER" => Ok(PotionType::LongTurtleMaster { inner: PotionTypeStruct::from_raw(env,obj)?}),
"STRONG_TURTLE_MASTER" => Ok(PotionType::StrongTurtleMaster { inner: PotionTypeStruct::from_raw(env,obj)?}),
"SLOW_FALLING" => Ok(PotionType::SlowFalling { inner: PotionTypeStruct::from_raw(env,obj)?}),
"LONG_SLOW_FALLING" => Ok(PotionType::LongSlowFalling { inner: PotionTypeStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PotionTypeStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PotionType<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Uncraftable { inner } => inner.0.clone(),
Self::Water { inner } => inner.0.clone(),
Self::Mundane { inner } => inner.0.clone(),
Self::Thick { inner } => inner.0.clone(),
Self::Awkward { inner } => inner.0.clone(),
Self::NightVision { inner } => inner.0.clone(),
Self::LongNightVision { inner } => inner.0.clone(),
Self::Invisibility { inner } => inner.0.clone(),
Self::LongInvisibility { inner } => inner.0.clone(),
Self::Jump { inner } => inner.0.clone(),
Self::LongLeaping { inner } => inner.0.clone(),
Self::StrongLeaping { inner } => inner.0.clone(),
Self::FireResistance { inner } => inner.0.clone(),
Self::LongFireResistance { inner } => inner.0.clone(),
Self::Speed { inner } => inner.0.clone(),
Self::LongSwiftness { inner } => inner.0.clone(),
Self::StrongSwiftness { inner } => inner.0.clone(),
Self::Slowness { inner } => inner.0.clone(),
Self::LongSlowness { inner } => inner.0.clone(),
Self::StrongSlowness { inner } => inner.0.clone(),
Self::WaterBreathing { inner } => inner.0.clone(),
Self::LongWaterBreathing { inner } => inner.0.clone(),
Self::InstantHeal { inner } => inner.0.clone(),
Self::StrongHealing { inner } => inner.0.clone(),
Self::InstantDamage { inner } => inner.0.clone(),
Self::StrongHarming { inner } => inner.0.clone(),
Self::Poison { inner } => inner.0.clone(),
Self::LongPoison { inner } => inner.0.clone(),
Self::StrongPoison { inner } => inner.0.clone(),
Self::Regen { inner } => inner.0.clone(),
Self::LongRegeneration { inner } => inner.0.clone(),
Self::StrongRegeneration { inner } => inner.0.clone(),
Self::Strength { inner } => inner.0.clone(),
Self::LongStrength { inner } => inner.0.clone(),
Self::StrongStrength { inner } => inner.0.clone(),
Self::Weakness { inner } => inner.0.clone(),
Self::LongWeakness { inner } => inner.0.clone(),
Self::Luck { inner } => inner.0.clone(),
Self::TurtleMaster { inner } => inner.0.clone(),
Self::LongTurtleMaster { inner } => inner.0.clone(),
Self::StrongTurtleMaster { inner } => inner.0.clone(),
Self::SlowFalling { inner } => inner.0.clone(),
Self::LongSlowFalling { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Uncraftable { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Water { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Mundane { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Thick { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Awkward { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::NightVision { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LongNightVision { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Invisibility { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LongInvisibility { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Jump { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LongLeaping { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::StrongLeaping { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::FireResistance { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LongFireResistance { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Speed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LongSwiftness { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::StrongSwiftness { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Slowness { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LongSlowness { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::StrongSlowness { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::WaterBreathing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LongWaterBreathing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::InstantHeal { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::StrongHealing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::InstantDamage { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::StrongHarming { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Poison { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LongPoison { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::StrongPoison { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Regen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LongRegeneration { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::StrongRegeneration { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Strength { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LongStrength { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::StrongStrength { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Weakness { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LongWeakness { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Luck { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::TurtleMaster { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LongTurtleMaster { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::StrongTurtleMaster { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SlowFalling { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LongSlowFalling { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PotionType<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PotionType from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionType")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PotionType object, got {}",
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
                    "UNCRAFTABLE" => Ok(PotionType::Uncraftable { inner: PotionTypeStruct::from_raw(env,obj)?}),"WATER" => Ok(PotionType::Water { inner: PotionTypeStruct::from_raw(env,obj)?}),"MUNDANE" => Ok(PotionType::Mundane { inner: PotionTypeStruct::from_raw(env,obj)?}),"THICK" => Ok(PotionType::Thick { inner: PotionTypeStruct::from_raw(env,obj)?}),"AWKWARD" => Ok(PotionType::Awkward { inner: PotionTypeStruct::from_raw(env,obj)?}),"NIGHT_VISION" => Ok(PotionType::NightVision { inner: PotionTypeStruct::from_raw(env,obj)?}),"LONG_NIGHT_VISION" => Ok(PotionType::LongNightVision { inner: PotionTypeStruct::from_raw(env,obj)?}),"INVISIBILITY" => Ok(PotionType::Invisibility { inner: PotionTypeStruct::from_raw(env,obj)?}),"LONG_INVISIBILITY" => Ok(PotionType::LongInvisibility { inner: PotionTypeStruct::from_raw(env,obj)?}),"JUMP" => Ok(PotionType::Jump { inner: PotionTypeStruct::from_raw(env,obj)?}),"LONG_LEAPING" => Ok(PotionType::LongLeaping { inner: PotionTypeStruct::from_raw(env,obj)?}),"STRONG_LEAPING" => Ok(PotionType::StrongLeaping { inner: PotionTypeStruct::from_raw(env,obj)?}),"FIRE_RESISTANCE" => Ok(PotionType::FireResistance { inner: PotionTypeStruct::from_raw(env,obj)?}),"LONG_FIRE_RESISTANCE" => Ok(PotionType::LongFireResistance { inner: PotionTypeStruct::from_raw(env,obj)?}),"SPEED" => Ok(PotionType::Speed { inner: PotionTypeStruct::from_raw(env,obj)?}),"LONG_SWIFTNESS" => Ok(PotionType::LongSwiftness { inner: PotionTypeStruct::from_raw(env,obj)?}),"STRONG_SWIFTNESS" => Ok(PotionType::StrongSwiftness { inner: PotionTypeStruct::from_raw(env,obj)?}),"SLOWNESS" => Ok(PotionType::Slowness { inner: PotionTypeStruct::from_raw(env,obj)?}),"LONG_SLOWNESS" => Ok(PotionType::LongSlowness { inner: PotionTypeStruct::from_raw(env,obj)?}),"STRONG_SLOWNESS" => Ok(PotionType::StrongSlowness { inner: PotionTypeStruct::from_raw(env,obj)?}),"WATER_BREATHING" => Ok(PotionType::WaterBreathing { inner: PotionTypeStruct::from_raw(env,obj)?}),"LONG_WATER_BREATHING" => Ok(PotionType::LongWaterBreathing { inner: PotionTypeStruct::from_raw(env,obj)?}),"INSTANT_HEAL" => Ok(PotionType::InstantHeal { inner: PotionTypeStruct::from_raw(env,obj)?}),"STRONG_HEALING" => Ok(PotionType::StrongHealing { inner: PotionTypeStruct::from_raw(env,obj)?}),"INSTANT_DAMAGE" => Ok(PotionType::InstantDamage { inner: PotionTypeStruct::from_raw(env,obj)?}),"STRONG_HARMING" => Ok(PotionType::StrongHarming { inner: PotionTypeStruct::from_raw(env,obj)?}),"POISON" => Ok(PotionType::Poison { inner: PotionTypeStruct::from_raw(env,obj)?}),"LONG_POISON" => Ok(PotionType::LongPoison { inner: PotionTypeStruct::from_raw(env,obj)?}),"STRONG_POISON" => Ok(PotionType::StrongPoison { inner: PotionTypeStruct::from_raw(env,obj)?}),"REGEN" => Ok(PotionType::Regen { inner: PotionTypeStruct::from_raw(env,obj)?}),"LONG_REGENERATION" => Ok(PotionType::LongRegeneration { inner: PotionTypeStruct::from_raw(env,obj)?}),"STRONG_REGENERATION" => Ok(PotionType::StrongRegeneration { inner: PotionTypeStruct::from_raw(env,obj)?}),"STRENGTH" => Ok(PotionType::Strength { inner: PotionTypeStruct::from_raw(env,obj)?}),"LONG_STRENGTH" => Ok(PotionType::LongStrength { inner: PotionTypeStruct::from_raw(env,obj)?}),"STRONG_STRENGTH" => Ok(PotionType::StrongStrength { inner: PotionTypeStruct::from_raw(env,obj)?}),"WEAKNESS" => Ok(PotionType::Weakness { inner: PotionTypeStruct::from_raw(env,obj)?}),"LONG_WEAKNESS" => Ok(PotionType::LongWeakness { inner: PotionTypeStruct::from_raw(env,obj)?}),"LUCK" => Ok(PotionType::Luck { inner: PotionTypeStruct::from_raw(env,obj)?}),"TURTLE_MASTER" => Ok(PotionType::TurtleMaster { inner: PotionTypeStruct::from_raw(env,obj)?}),"LONG_TURTLE_MASTER" => Ok(PotionType::LongTurtleMaster { inner: PotionTypeStruct::from_raw(env,obj)?}),"STRONG_TURTLE_MASTER" => Ok(PotionType::StrongTurtleMaster { inner: PotionTypeStruct::from_raw(env,obj)?}),"SLOW_FALLING" => Ok(PotionType::SlowFalling { inner: PotionTypeStruct::from_raw(env,obj)?}),"LONG_SLOW_FALLING" => Ok(PotionType::LongSlowFalling { inner: PotionTypeStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PotionTypeStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PotionTypeStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PotionTypeStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionType")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PotionTypeStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PotionTypeStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::potion::PotionType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/potion/PotionType;");
let cls = jni.find_class("org/bukkit/potion/PotionType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::potion::PotionType::from_raw(&jni,obj
)}
#[deprecated]

	fn effect_type(&self) 
-> Result<Option<crate::potion::PotionEffectType<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEffectType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::potion::PotionEffectType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn potion_effects(&self) 
-> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPotionEffects",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
#[deprecated]

	fn is_instant(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isInstant",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks if the potion type has an upgraded state.
/// This refers to whether or not the potion type can be Tier 2,
/// such as Potion of Fire Resistance II.
	fn is_upgradeable(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isUpgradeable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks if the potion type has an extended state.
/// This refers to the extended duration potions
	fn is_extendable(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isExtendable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn max_level(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxLevel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
#[deprecated]

	fn get_by_effect(jni: &blackboxmc_general::SharedJNIEnv<'mc>,effect_type: impl Into<crate::potion::PotionEffectType<'mc>>) 
-> Result<Option<crate::potion::PotionType<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)Lorg/bukkit/potion/PotionType;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(effect_type.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/potion/PotionType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getByEffect",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let obj = res.l()?;
Ok(
Some(
crate::potion::PotionType::from_raw(&jni,obj
)?
)
)}

	fn key(&self) 
-> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/NamespacedKey;");
let res = self.jni_ref().call_method(&self.jni_object(),"getKey",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::NamespacedKey::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PotionEffect<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PotionEffect<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PotionEffect<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PotionEffect from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionEffect")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PotionEffect object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PotionEffectTrait<'mc> for PotionEffect<'mc> {}
pub trait PotionEffectTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Creates a potion effect.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: impl Into<crate::potion::PotionEffectType<'mc>>,duration: std::option::Option<i32>,amplifier: std::option::Option<i32>,ambient: std::option::Option<bool>,particles: std::option::Option<bool>,icon: std::option::Option<bool>) 
-> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/potion/PotionEffectType;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = duration {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a);
args.push(val_2);
}
if let Some(a) = amplifier {
sig += "I";
let val_3 = jni::objects::JValueGen::Int(a);
args.push(val_3);
}
if let Some(a) = ambient {
sig += "Z";
let val_4 = jni::objects::JValueGen::Bool(a.into());
args.push(val_4);
}
if let Some(a) = particles {
sig += "Z";
let val_5 = jni::objects::JValueGen::Bool(a.into());
args.push(val_5);
}
if let Some(a) = icon {
sig += "Z";
let val_6 = jni::objects::JValueGen::Bool(a.into());
args.push(val_6);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/potion/PotionEffect"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::potion::PotionEffect::from_raw(&jni,res
)}

	fn serialize(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"serialize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Attempts to add the effect represented by this object to the given
/// {@link LivingEntity}.
	fn apply(&self,entity: impl Into<crate::entity::LivingEntity<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/LivingEntity;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"apply",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns the amplifier of this effect. A higher amplifier means the
/// potion effect happens more often over its duration and in some cases
/// has more effect on its target.
	fn amplifier(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getAmplifier",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Returns the duration (in ticks) that this effect will run for when
/// applied to a {@link LivingEntity}.
	fn duration(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getDuration",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Returns whether or not this potion effect has an infinite duration. Potion
/// effects with infinite durations will display an infinite symbol and never
/// expire unless manually removed.
	fn is_infinite(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isInfinite",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns whether or not this potion effect has a shorter duration than the
/// provided potion effect.
/// 
/// An infinite duration is considered longer than non-infinite durations. If
/// both potion effects have infinite durations, then neither is shorter than
/// the other and this method will return false.
	fn is_shorter_than(&self,other: impl Into<crate::potion::PotionEffect<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionEffect;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"isShorterThan",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns the {@link PotionEffectType} of this effect.
	fn get_type(&self) 
-> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::potion::PotionEffectType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Makes potion effect produce more, translucent, particles.
	fn is_ambient(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isAmbient",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn has_particles(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasParticles",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[deprecated]

	fn color(&self) 
-> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Color;");
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Color::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn has_icon(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasIcon",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for PotionEffect<'mc> {
            fn to_string(&self) -> String {
                match PotionEffectTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling PotionEffect.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>> for PotionEffect<'mc>{

fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {

crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PotionEffect into crate::configuration::serialization::ConfigurationSerializable")

   }
}
impl<'mc> crate::configuration::serialization::ConfigurationSerializableTrait<'mc> for PotionEffect<'mc> {}
#[repr(C)]
pub struct PotionEffectType<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PotionEffectType<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PotionEffectType<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PotionEffectType from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionEffectType")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PotionEffectType object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PotionEffectTypeTrait<'mc> for PotionEffectType<'mc> {}
pub trait PotionEffectTypeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/potion/PotionEffectType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::potion::PotionEffectType::from_raw(&jni,res
)}
#[deprecated]
/// Gets the PotionEffectType at the specified key
	fn get_by_key(jni: &blackboxmc_general::SharedJNIEnv<'mc>,key: impl Into<crate::NamespacedKey<'mc>>) 
-> Result<Option<crate::potion::PotionEffectType<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/potion/PotionEffectType;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/potion/PotionEffectType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getByKey",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let obj = res.l()?;
Ok(
Some(
crate::potion::PotionEffectType::from_raw(&jni,obj
)?
)
)}
#[deprecated]
/// Gets the effect type specified by the unique id.
	fn get_by_id(jni: &blackboxmc_general::SharedJNIEnv<'mc>,id: i32) 
-> Result<Option<crate::potion::PotionEffectType<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(I)Lorg/bukkit/potion/PotionEffectType;");
let val_1 = jni::objects::JValueGen::Int(id);
let cls = jni.find_class("org/bukkit/potion/PotionEffectType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getById",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let obj = res.l()?;
Ok(
Some(
crate::potion::PotionEffectType::from_raw(&jni,obj
)?
)
)}
#[deprecated]
/// Gets the effect type specified by the given name.
	fn get_by_name(jni: &blackboxmc_general::SharedJNIEnv<'mc>,name: impl Into<String>) 
-> Result<Option<crate::potion::PotionEffectType<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/potion/PotionEffectType;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(name.into())?));
let cls = jni.find_class("org/bukkit/potion/PotionEffectType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getByName",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let obj = res.l()?;
Ok(
Some(
crate::potion::PotionEffectType::from_raw(&jni,obj
)?
)
)}
#[deprecated]

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
let cls = jni.find_class("org/bukkit/potion/PotionEffectType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::potion::PotionEffectType::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::Keyed<'mc>> for PotionEffectType<'mc>{

fn into(self) -> crate::Keyed<'mc> {

crate::Keyed::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PotionEffectType into crate::Keyed")

   }
}
impl<'mc> crate::KeyedTrait<'mc> for PotionEffectType<'mc> {}
impl<'mc> Into<crate::Translatable<'mc>> for PotionEffectType<'mc>{

fn into(self) -> crate::Translatable<'mc> {

crate::Translatable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PotionEffectType into crate::Translatable")

   }
}
impl<'mc> crate::TranslatableTrait<'mc> for PotionEffectType<'mc> {}
#[repr(C)]
pub struct PotionTypeInternalPotionData<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PotionTypeInternalPotionData<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PotionTypeInternalPotionData<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PotionTypeInternalPotionData from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/potion/PotionType/InternalPotionData")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PotionTypeInternalPotionData object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PotionTypeInternalPotionDataTrait<'mc> for PotionTypeInternalPotionData<'mc> {}
pub trait PotionTypeInternalPotionDataTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn effect_type(&self) 
-> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEffectType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::potion::PotionEffectType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn potion_effects(&self) 
-> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPotionEffects",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}

	fn is_instant(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isInstant",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn is_upgradeable(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isUpgradeable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn is_extendable(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isExtendable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn max_level(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxLevel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
