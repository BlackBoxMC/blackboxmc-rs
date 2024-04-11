#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
pub enum EnchantmentTarget<'mc> {
	All {inner: EnchantmentTargetStruct<'mc>},
	Armor {inner: EnchantmentTargetStruct<'mc>},
	ArmorFeet {inner: EnchantmentTargetStruct<'mc>},
	ArmorLegs {inner: EnchantmentTargetStruct<'mc>},
	ArmorTorso {inner: EnchantmentTargetStruct<'mc>},
	ArmorHead {inner: EnchantmentTargetStruct<'mc>},
	Weapon {inner: EnchantmentTargetStruct<'mc>},
	Tool {inner: EnchantmentTargetStruct<'mc>},
	Bow {inner: EnchantmentTargetStruct<'mc>},
	FishingRod {inner: EnchantmentTargetStruct<'mc>},
	Breakable {inner: EnchantmentTargetStruct<'mc>},
	Wearable {inner: EnchantmentTargetStruct<'mc>},
	Trident {inner: EnchantmentTargetStruct<'mc>},
	Crossbow {inner: EnchantmentTargetStruct<'mc>},
	Vanishable {inner: EnchantmentTargetStruct<'mc>},
}
impl<'mc> std::fmt::Display for EnchantmentTarget<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           EnchantmentTarget::All { .. } => f.write_str("ALL"),
           EnchantmentTarget::Armor { .. } => f.write_str("ARMOR"),
           EnchantmentTarget::ArmorFeet { .. } => f.write_str("ARMOR_FEET"),
           EnchantmentTarget::ArmorLegs { .. } => f.write_str("ARMOR_LEGS"),
           EnchantmentTarget::ArmorTorso { .. } => f.write_str("ARMOR_TORSO"),
           EnchantmentTarget::ArmorHead { .. } => f.write_str("ARMOR_HEAD"),
           EnchantmentTarget::Weapon { .. } => f.write_str("WEAPON"),
           EnchantmentTarget::Tool { .. } => f.write_str("TOOL"),
           EnchantmentTarget::Bow { .. } => f.write_str("BOW"),
           EnchantmentTarget::FishingRod { .. } => f.write_str("FISHING_ROD"),
           EnchantmentTarget::Breakable { .. } => f.write_str("BREAKABLE"),
           EnchantmentTarget::Wearable { .. } => f.write_str("WEARABLE"),
           EnchantmentTarget::Trident { .. } => f.write_str("TRIDENT"),
           EnchantmentTarget::Crossbow { .. } => f.write_str("CROSSBOW"),
           EnchantmentTarget::Vanishable { .. } => f.write_str("VANISHABLE"),
       }
   }
}

        impl<'mc> EnchantmentTarget<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<EnchantmentTarget<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/enchantments/EnchantmentTarget");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/enchantments/EnchantmentTarget;",
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
                    
"ALL" => Ok(EnchantmentTarget::All { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),
"ARMOR" => Ok(EnchantmentTarget::Armor { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),
"ARMOR_FEET" => Ok(EnchantmentTarget::ArmorFeet { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),
"ARMOR_LEGS" => Ok(EnchantmentTarget::ArmorLegs { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),
"ARMOR_TORSO" => Ok(EnchantmentTarget::ArmorTorso { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),
"ARMOR_HEAD" => Ok(EnchantmentTarget::ArmorHead { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),
"WEAPON" => Ok(EnchantmentTarget::Weapon { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),
"TOOL" => Ok(EnchantmentTarget::Tool { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),
"BOW" => Ok(EnchantmentTarget::Bow { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),
"FISHING_ROD" => Ok(EnchantmentTarget::FishingRod { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),
"BREAKABLE" => Ok(EnchantmentTarget::Breakable { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),
"WEARABLE" => Ok(EnchantmentTarget::Wearable { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),
"TRIDENT" => Ok(EnchantmentTarget::Trident { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),
"CROSSBOW" => Ok(EnchantmentTarget::Crossbow { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),
"VANISHABLE" => Ok(EnchantmentTarget::Vanishable { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct EnchantmentTargetStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for EnchantmentTarget<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::All { inner } => inner.0.clone(),
Self::Armor { inner } => inner.0.clone(),
Self::ArmorFeet { inner } => inner.0.clone(),
Self::ArmorLegs { inner } => inner.0.clone(),
Self::ArmorTorso { inner } => inner.0.clone(),
Self::ArmorHead { inner } => inner.0.clone(),
Self::Weapon { inner } => inner.0.clone(),
Self::Tool { inner } => inner.0.clone(),
Self::Bow { inner } => inner.0.clone(),
Self::FishingRod { inner } => inner.0.clone(),
Self::Breakable { inner } => inner.0.clone(),
Self::Wearable { inner } => inner.0.clone(),
Self::Trident { inner } => inner.0.clone(),
Self::Crossbow { inner } => inner.0.clone(),
Self::Vanishable { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::All { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Armor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ArmorFeet { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ArmorLegs { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ArmorTorso { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ArmorHead { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Weapon { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Tool { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Bow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::FishingRod { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Breakable { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Wearable { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Trident { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Crossbow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Vanishable { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentTarget<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EnchantmentTarget from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/enchantments/EnchantmentTarget")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EnchantmentTarget object, got {}",
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
                    "ALL" => Ok(EnchantmentTarget::All { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),"ARMOR" => Ok(EnchantmentTarget::Armor { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),"ARMOR_FEET" => Ok(EnchantmentTarget::ArmorFeet { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),"ARMOR_LEGS" => Ok(EnchantmentTarget::ArmorLegs { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),"ARMOR_TORSO" => Ok(EnchantmentTarget::ArmorTorso { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),"ARMOR_HEAD" => Ok(EnchantmentTarget::ArmorHead { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),"WEAPON" => Ok(EnchantmentTarget::Weapon { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),"TOOL" => Ok(EnchantmentTarget::Tool { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),"BOW" => Ok(EnchantmentTarget::Bow { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),"FISHING_ROD" => Ok(EnchantmentTarget::FishingRod { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),"BREAKABLE" => Ok(EnchantmentTarget::Breakable { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),"WEARABLE" => Ok(EnchantmentTarget::Wearable { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),"TRIDENT" => Ok(EnchantmentTarget::Trident { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),"CROSSBOW" => Ok(EnchantmentTarget::Crossbow { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),"VANISHABLE" => Ok(EnchantmentTarget::Vanishable { inner: EnchantmentTargetStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for EnchantmentTargetStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentTargetStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EnchantmentTargetStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/enchantments/EnchantmentTarget")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EnchantmentTargetStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> EnchantmentTargetStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::enchantments::EnchantmentTarget<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::enchantments::EnchantmentTarget;");
let cls = jni.find_class("org/bukkit/enchantments/EnchantmentTarget"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::enchantments::EnchantmentTarget::from_raw(&jni,obj
)}
	pub fn includes(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"includes",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct EnchantmentOffer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for EnchantmentOffer<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentOffer<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EnchantmentOffer from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/enchantments/EnchantmentOffer")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EnchantmentOffer object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> EnchantmentOffer<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,enchantment: impl Into<crate::enchantments::Enchantment<'mc>>,enchantment_level: i32,cost: i32) 
-> Result<crate::enchantments::EnchantmentOffer<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;II)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(enchantment.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(enchantment_level);
let val_3 = jni::objects::JValueGen::Int(cost);
let cls = jni.find_class("org/bukkit/enchantments/EnchantmentOffer"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::enchantments::EnchantmentOffer::from_raw(&jni,res
)}
	pub fn enchantment(&self) 
-> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::enchantments::Enchantment;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEnchantment",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::enchantments::Enchantment::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_enchantment(&self,enchantment: impl Into<crate::enchantments::Enchantment<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(enchantment.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setEnchantment",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn enchantment_level(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEnchantmentLevel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn set_enchantment_level(&self,enchantment_level: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(enchantment_level);
let res = self.jni_ref().call_method(&self.jni_object(),"setEnchantmentLevel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn cost(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCost",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn set_cost(&self,cost: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(cost);
let res = self.jni_ref().call_method(&self.jni_object(),"setCost",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct EnchantmentWrapper<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for EnchantmentWrapper<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentWrapper<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EnchantmentWrapper from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/enchantments/EnchantmentWrapper")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EnchantmentWrapper object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> EnchantmentWrapper<'mc> {
	pub fn enchantment(&self) 
-> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::enchantments::Enchantment;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEnchantment",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::enchantments::Enchantment::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
// SUPER CLASS: Enchantment

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::enchantments::Enchantment<'mc>> for EnchantmentWrapper<'mc>{

fn into(self) -> crate::enchantments::Enchantment<'mc> {

crate::enchantments::Enchantment::from_raw(&self.jni_ref(), self.1).expect("Error converting EnchantmentWrapper into crate::enchantments::Enchantment")

   }
}
#[repr(C)]
pub struct Enchantment<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Enchantment<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Enchantment<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Enchantment from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/enchantments/Enchantment")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Enchantment object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> Enchantment<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/enchantments/Enchantment"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::enchantments::Enchantment::from_raw(&jni,res
)}
#[deprecated]

	pub fn get_by_key(jni: &blackboxmc_general::SharedJNIEnv<'mc>,key: impl Into<crate::NamespacedKey<'mc>>) 
-> Result<Option<crate::enchantments::Enchantment<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;)Lcrate::enchantments::Enchantment;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/enchantments/Enchantment"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getByKey",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let obj = res.l()?;
Ok(
Some(
crate::enchantments::Enchantment::from_raw(&jni,obj
)?
)
)}
#[deprecated]

	pub fn get_by_name(jni: &blackboxmc_general::SharedJNIEnv<'mc>,name: impl Into<String>) 
-> Result<Option<crate::enchantments::Enchantment<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lcrate::enchantments::Enchantment;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(name.into())?));
let cls = jni.find_class("org/bukkit/enchantments/Enchantment"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getByName",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let obj = res.l()?;
Ok(
Some(
crate::enchantments::Enchantment::from_raw(&jni,obj
)?
)
)}
#[deprecated]

	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::enchantments::Enchantment;");
let cls = jni.find_class("org/bukkit/enchantments/Enchantment"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::enchantments::Enchantment::from_raw(&jni,obj
)}
	pub fn key(&self) 
-> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::NamespacedKey;");
let res = self.jni_ref().call_method(&self.jni_object(),"getKey",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::NamespacedKey::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn translation_key(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTranslationKey",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::Keyed<'mc>> for Enchantment<'mc>{

fn into(self) -> crate::Keyed<'mc> {

crate::Keyed::from_raw(&self.jni_ref(), self.1).expect("Error converting Enchantment into crate::Keyed")

   }
}
impl<'mc> Into<crate::Translatable<'mc>> for Enchantment<'mc>{

fn into(self) -> crate::Translatable<'mc> {

crate::Translatable::from_raw(&self.jni_ref(), self.1).expect("Error converting Enchantment into crate::Translatable")

   }
}
