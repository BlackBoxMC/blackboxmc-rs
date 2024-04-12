#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/inventory/mod.rs*/

#[repr(C)]
pub struct AnvilInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for AnvilInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for AnvilInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate AnvilInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/AnvilInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a AnvilInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> AnvilInventoryTrait<'mc> for AnvilInventory<'mc> {}
pub trait AnvilInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the name to be applied to the repaired item. An empty string denotes
/// the default item name.
	fn rename_text(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRenameText",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Get the item cost (in amount) to complete the current repair.
	fn repair_cost_amount(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getRepairCostAmount",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the item cost (in amount) to complete the current repair.
	fn set_repair_cost_amount(&self,amount: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(amount);
let res = self.jni_ref().call_method(&self.jni_object(),"setRepairCostAmount",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the experience cost (in levels) to complete the current repair.
	fn repair_cost(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getRepairCost",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the experience cost (in levels) to complete the current repair.
	fn set_repair_cost(&self,levels: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(levels);
let res = self.jni_ref().call_method(&self.jni_object(),"setRepairCost",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the maximum experience cost (in levels) to be allowed by the current
/// repair. If the result of {@link #getRepairCost()} exceeds the returned
/// value, the repair result will be air to due being "too expensive".
/// 
/// By default, this level is set to 40. Players in creative mode ignore the
/// maximum repair cost.
	fn maximum_repair_cost(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaximumRepairCost",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the maximum experience cost (in levels) to be allowed by the current
/// repair. The default value set by vanilla Minecraft is 40.
	fn set_maximum_repair_cost(&self,levels: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(levels);
let res = self.jni_ref().call_method(&self.jni_object(),"setMaximumRepairCost",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for AnvilInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting AnvilInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for AnvilInventory<'mc> {}
pub enum InventoryViewProperty<'mc> {
	BrewTime {inner: InventoryViewPropertyStruct<'mc>},
	FuelTime {inner: InventoryViewPropertyStruct<'mc>},
	BurnTime {inner: InventoryViewPropertyStruct<'mc>},
	TicksForCurrentFuel {inner: InventoryViewPropertyStruct<'mc>},
	CookTime {inner: InventoryViewPropertyStruct<'mc>},
	TicksForCurrentSmelting {inner: InventoryViewPropertyStruct<'mc>},
	EnchantButton1 {inner: InventoryViewPropertyStruct<'mc>},
	EnchantButton2 {inner: InventoryViewPropertyStruct<'mc>},
	EnchantButton3 {inner: InventoryViewPropertyStruct<'mc>},
	EnchantXpSeed {inner: InventoryViewPropertyStruct<'mc>},
	EnchantId1 {inner: InventoryViewPropertyStruct<'mc>},
	EnchantId2 {inner: InventoryViewPropertyStruct<'mc>},
	EnchantId3 {inner: InventoryViewPropertyStruct<'mc>},
	EnchantLevel1 {inner: InventoryViewPropertyStruct<'mc>},
	EnchantLevel2 {inner: InventoryViewPropertyStruct<'mc>},
	EnchantLevel3 {inner: InventoryViewPropertyStruct<'mc>},
	Levels {inner: InventoryViewPropertyStruct<'mc>},
	PrimaryEffect {inner: InventoryViewPropertyStruct<'mc>},
	SecondaryEffect {inner: InventoryViewPropertyStruct<'mc>},
	RepairCost {inner: InventoryViewPropertyStruct<'mc>},
	BookPage {inner: InventoryViewPropertyStruct<'mc>},
}
impl<'mc> std::fmt::Display for InventoryViewProperty<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           InventoryViewProperty::BrewTime { .. } => f.write_str("BREW_TIME"),
           InventoryViewProperty::FuelTime { .. } => f.write_str("FUEL_TIME"),
           InventoryViewProperty::BurnTime { .. } => f.write_str("BURN_TIME"),
           InventoryViewProperty::TicksForCurrentFuel { .. } => f.write_str("TICKS_FOR_CURRENT_FUEL"),
           InventoryViewProperty::CookTime { .. } => f.write_str("COOK_TIME"),
           InventoryViewProperty::TicksForCurrentSmelting { .. } => f.write_str("TICKS_FOR_CURRENT_SMELTING"),
           InventoryViewProperty::EnchantButton1 { .. } => f.write_str("ENCHANT_BUTTON1"),
           InventoryViewProperty::EnchantButton2 { .. } => f.write_str("ENCHANT_BUTTON2"),
           InventoryViewProperty::EnchantButton3 { .. } => f.write_str("ENCHANT_BUTTON3"),
           InventoryViewProperty::EnchantXpSeed { .. } => f.write_str("ENCHANT_XP_SEED"),
           InventoryViewProperty::EnchantId1 { .. } => f.write_str("ENCHANT_ID1"),
           InventoryViewProperty::EnchantId2 { .. } => f.write_str("ENCHANT_ID2"),
           InventoryViewProperty::EnchantId3 { .. } => f.write_str("ENCHANT_ID3"),
           InventoryViewProperty::EnchantLevel1 { .. } => f.write_str("ENCHANT_LEVEL1"),
           InventoryViewProperty::EnchantLevel2 { .. } => f.write_str("ENCHANT_LEVEL2"),
           InventoryViewProperty::EnchantLevel3 { .. } => f.write_str("ENCHANT_LEVEL3"),
           InventoryViewProperty::Levels { .. } => f.write_str("LEVELS"),
           InventoryViewProperty::PrimaryEffect { .. } => f.write_str("PRIMARY_EFFECT"),
           InventoryViewProperty::SecondaryEffect { .. } => f.write_str("SECONDARY_EFFECT"),
           InventoryViewProperty::RepairCost { .. } => f.write_str("REPAIR_COST"),
           InventoryViewProperty::BookPage { .. } => f.write_str("BOOK_PAGE"),
       }
   }
}

        impl<'mc> InventoryViewPropertyTrait<'mc> for InventoryViewProperty<'mc> {}
        
        pub trait InventoryViewPropertyTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<InventoryViewProperty<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/inventory/InventoryView/Property");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/inventory/InventoryView/Property;",
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
                    
"BREW_TIME" => Ok(InventoryViewProperty::BrewTime { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"FUEL_TIME" => Ok(InventoryViewProperty::FuelTime { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"BURN_TIME" => Ok(InventoryViewProperty::BurnTime { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"TICKS_FOR_CURRENT_FUEL" => Ok(InventoryViewProperty::TicksForCurrentFuel { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"COOK_TIME" => Ok(InventoryViewProperty::CookTime { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"TICKS_FOR_CURRENT_SMELTING" => Ok(InventoryViewProperty::TicksForCurrentSmelting { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"ENCHANT_BUTTON1" => Ok(InventoryViewProperty::EnchantButton1 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"ENCHANT_BUTTON2" => Ok(InventoryViewProperty::EnchantButton2 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"ENCHANT_BUTTON3" => Ok(InventoryViewProperty::EnchantButton3 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"ENCHANT_XP_SEED" => Ok(InventoryViewProperty::EnchantXpSeed { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"ENCHANT_ID1" => Ok(InventoryViewProperty::EnchantId1 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"ENCHANT_ID2" => Ok(InventoryViewProperty::EnchantId2 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"ENCHANT_ID3" => Ok(InventoryViewProperty::EnchantId3 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"ENCHANT_LEVEL1" => Ok(InventoryViewProperty::EnchantLevel1 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"ENCHANT_LEVEL2" => Ok(InventoryViewProperty::EnchantLevel2 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"ENCHANT_LEVEL3" => Ok(InventoryViewProperty::EnchantLevel3 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"LEVELS" => Ok(InventoryViewProperty::Levels { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"PRIMARY_EFFECT" => Ok(InventoryViewProperty::PrimaryEffect { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"SECONDARY_EFFECT" => Ok(InventoryViewProperty::SecondaryEffect { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"REPAIR_COST" => Ok(InventoryViewProperty::RepairCost { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),
"BOOK_PAGE" => Ok(InventoryViewProperty::BookPage { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct InventoryViewPropertyStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for InventoryViewProperty<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::BrewTime { inner } => inner.0.clone(),
Self::FuelTime { inner } => inner.0.clone(),
Self::BurnTime { inner } => inner.0.clone(),
Self::TicksForCurrentFuel { inner } => inner.0.clone(),
Self::CookTime { inner } => inner.0.clone(),
Self::TicksForCurrentSmelting { inner } => inner.0.clone(),
Self::EnchantButton1 { inner } => inner.0.clone(),
Self::EnchantButton2 { inner } => inner.0.clone(),
Self::EnchantButton3 { inner } => inner.0.clone(),
Self::EnchantXpSeed { inner } => inner.0.clone(),
Self::EnchantId1 { inner } => inner.0.clone(),
Self::EnchantId2 { inner } => inner.0.clone(),
Self::EnchantId3 { inner } => inner.0.clone(),
Self::EnchantLevel1 { inner } => inner.0.clone(),
Self::EnchantLevel2 { inner } => inner.0.clone(),
Self::EnchantLevel3 { inner } => inner.0.clone(),
Self::Levels { inner } => inner.0.clone(),
Self::PrimaryEffect { inner } => inner.0.clone(),
Self::SecondaryEffect { inner } => inner.0.clone(),
Self::RepairCost { inner } => inner.0.clone(),
Self::BookPage { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::BrewTime { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::FuelTime { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::BurnTime { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::TicksForCurrentFuel { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CookTime { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::TicksForCurrentSmelting { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EnchantButton1 { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EnchantButton2 { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EnchantButton3 { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EnchantXpSeed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EnchantId1 { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EnchantId2 { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EnchantId3 { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EnchantLevel1 { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EnchantLevel2 { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EnchantLevel3 { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Levels { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::PrimaryEffect { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SecondaryEffect { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::RepairCost { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::BookPage { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for InventoryViewProperty<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate InventoryViewProperty from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/InventoryView/Property")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a InventoryViewProperty object, got {}",
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
                    "BREW_TIME" => Ok(InventoryViewProperty::BrewTime { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"FUEL_TIME" => Ok(InventoryViewProperty::FuelTime { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"BURN_TIME" => Ok(InventoryViewProperty::BurnTime { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"TICKS_FOR_CURRENT_FUEL" => Ok(InventoryViewProperty::TicksForCurrentFuel { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"COOK_TIME" => Ok(InventoryViewProperty::CookTime { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"TICKS_FOR_CURRENT_SMELTING" => Ok(InventoryViewProperty::TicksForCurrentSmelting { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"ENCHANT_BUTTON1" => Ok(InventoryViewProperty::EnchantButton1 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"ENCHANT_BUTTON2" => Ok(InventoryViewProperty::EnchantButton2 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"ENCHANT_BUTTON3" => Ok(InventoryViewProperty::EnchantButton3 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"ENCHANT_XP_SEED" => Ok(InventoryViewProperty::EnchantXpSeed { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"ENCHANT_ID1" => Ok(InventoryViewProperty::EnchantId1 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"ENCHANT_ID2" => Ok(InventoryViewProperty::EnchantId2 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"ENCHANT_ID3" => Ok(InventoryViewProperty::EnchantId3 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"ENCHANT_LEVEL1" => Ok(InventoryViewProperty::EnchantLevel1 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"ENCHANT_LEVEL2" => Ok(InventoryViewProperty::EnchantLevel2 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"ENCHANT_LEVEL3" => Ok(InventoryViewProperty::EnchantLevel3 { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"LEVELS" => Ok(InventoryViewProperty::Levels { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"PRIMARY_EFFECT" => Ok(InventoryViewProperty::PrimaryEffect { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"SECONDARY_EFFECT" => Ok(InventoryViewProperty::SecondaryEffect { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"REPAIR_COST" => Ok(InventoryViewProperty::RepairCost { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),"BOOK_PAGE" => Ok(InventoryViewProperty::BookPage { inner: InventoryViewPropertyStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for InventoryViewPropertyStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for InventoryViewPropertyStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate InventoryViewPropertyStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/InventoryView/Property")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a InventoryViewPropertyStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> InventoryViewPropertyStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::inventory::InventoryViewProperty<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/InventoryView/Property;");
let cls = jni.find_class("org/bukkit/inventory/InventoryView/Property"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::inventory::InventoryViewProperty::from_raw(&jni,obj
)}

	fn get_type(&self) 
-> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::inventory::InventoryType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Gets the id of this view.
	fn id(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getId",sig.as_str(),vec![]);
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
pub struct Merchant<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Merchant<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Merchant<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Merchant from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/Merchant")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Merchant object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MerchantTrait<'mc> for Merchant<'mc> {}
pub trait MerchantTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get a list of trades currently available from this merchant.
	fn recipes(&self) 
-> Result<Vec<crate::inventory::MerchantRecipe<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipes",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::inventory::MerchantRecipe::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Set the list of trades currently available from this merchant.
/// 
/// This will not change the selected trades of players currently trading
/// with this merchant.
	fn set_recipes(&self,recipes: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/List;)V");
let raw_val_1 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in recipes{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_1,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_1 = jni::objects::JValueGen::Object(raw_val_1);
let res = self.jni_ref().call_method(&self.jni_object(),"setRecipes",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the recipe at a certain index of this merchant's trade list.
	fn get_recipe(&self,i: i32) 
-> Result<crate::inventory::MerchantRecipe<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(I)Lorg/bukkit/inventory/MerchantRecipe;");
let val_1 = jni::objects::JValueGen::Int(i);
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipe",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::MerchantRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Set the recipe at a certain index of this merchant's trade list.
	fn set_recipe(&self,i: i32,recipe: impl Into<crate::inventory::MerchantRecipe<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(ILorg/bukkit/inventory/MerchantRecipe;)V");
let val_1 = jni::objects::JValueGen::Int(i);
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(recipe.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setRecipe",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the number of trades this merchant currently has available.
	fn recipe_count(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipeCount",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets whether this merchant is currently trading.
	fn is_trading(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isTrading",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the player this merchant is trading with, or null if it is not
/// currently trading.
	fn trader(&self) 
-> Result<Option<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTrader",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct JukeboxInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for JukeboxInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for JukeboxInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate JukeboxInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/JukeboxInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a JukeboxInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> JukeboxInventoryTrait<'mc> for JukeboxInventory<'mc> {}
pub trait JukeboxInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Set the record in the jukebox.
/// 
/// This will immediately start playing the inserted item or stop playing if the
/// item provided is null. If the provided item is not a record (according to
/// {@link Tag#ITEMS_MUSIC_DISCS}), this method will do nothing and not set the
/// item in the inventory.
	fn set_record(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setRecord",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the record in the jukebox.
	fn record(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecord",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn holder(&self) 
-> Result<Option<crate::block::Jukebox<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Jukebox;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHolder",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::Jukebox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for JukeboxInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting JukeboxInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for JukeboxInventory<'mc> {}
#[repr(C)]
pub struct CrafterInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CrafterInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CrafterInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CrafterInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/CrafterInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CrafterInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CrafterInventoryTrait<'mc> for CrafterInventory<'mc> {}
pub trait CrafterInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for CrafterInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CrafterInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for CrafterInventory<'mc> {}
#[repr(C)]
pub struct SmithingTrimRecipe<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SmithingTrimRecipe<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SmithingTrimRecipe<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SmithingTrimRecipe from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/SmithingTrimRecipe")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SmithingTrimRecipe object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SmithingTrimRecipeTrait<'mc> for SmithingTrimRecipe<'mc> {}
pub trait SmithingTrimRecipeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Create a smithing recipe to produce the specified result ItemStack.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,key: impl Into<crate::NamespacedKey<'mc>>,template: impl Into<crate::inventory::RecipeChoice<'mc>>,base: impl Into<crate::inventory::RecipeChoice<'mc>>,addition: impl Into<crate::inventory::RecipeChoice<'mc>>) 
-> Result<crate::inventory::SmithingTrimRecipe<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/RecipeChoice;Lorg/bukkit/inventory/RecipeChoice;Lorg/bukkit/inventory/RecipeChoice;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(template.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(base.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(addition.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/inventory/SmithingTrimRecipe"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::SmithingTrimRecipe::from_raw(&jni,res
)}
/// Get the template recipe item.
	fn template(&self) 
-> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/RecipeChoice;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTemplate",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::RecipeChoice::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::ComplexRecipe<'mc>> for SmithingTrimRecipe<'mc>{

fn into(self) -> crate::inventory::ComplexRecipe<'mc> {

crate::inventory::ComplexRecipe::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SmithingTrimRecipe into crate::inventory::ComplexRecipe")

   }
}
impl<'mc> crate::inventory::ComplexRecipeTrait<'mc> for SmithingTrimRecipe<'mc> {}
impl<'mc> Into<crate::inventory::SmithingRecipe<'mc>> for SmithingTrimRecipe<'mc>{

fn into(self) -> crate::inventory::SmithingRecipe<'mc> {

crate::inventory::SmithingRecipe::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SmithingTrimRecipe into crate::inventory::SmithingRecipe")

   }
}
impl<'mc> crate::inventory::SmithingRecipeTrait<'mc> for SmithingTrimRecipe<'mc> {}
#[repr(C)]
pub struct LecternInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for LecternInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for LecternInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate LecternInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/LecternInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a LecternInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> LecternInventoryTrait<'mc> for LecternInventory<'mc> {}
pub trait LecternInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn holder(&self) 
-> Result<Option<crate::block::Lectern<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Lectern;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHolder",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::Lectern::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for LecternInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting LecternInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for LecternInventory<'mc> {}
#[repr(C)]
pub struct ShapedRecipe<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ShapedRecipe<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ShapedRecipe<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ShapedRecipe from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ShapedRecipe")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ShapedRecipe object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ShapedRecipeTrait<'mc> for ShapedRecipe<'mc> {}
pub trait ShapedRecipeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Create a shaped recipe to craft the specified ItemStack. The
/// constructor merely determines the result and type; to set the actual
/// recipe, you'll need to call the appropriate methods.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,key: impl Into<crate::NamespacedKey<'mc>>,result: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>) 
-> Result<crate::inventory::ShapedRecipe<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/NamespacedKey;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = result {
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/inventory/ShapedRecipe"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::ShapedRecipe::from_raw(&jni,res
)}
/// Set the shape of this recipe to the specified rows. Each character
/// represents a different ingredient; exactly what each character
/// represents is set separately. The first row supplied corresponds with
/// the upper most part of the recipe on the workbench e.g. if all three
/// rows are supplies the first string represents the top row on the
/// workbench.
	fn shape(&self,shape: std::option::Option<impl Into<String>>) 
-> Result<crate::inventory::ShapedRecipe<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = shape {
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(a.into())?));
args.push(val_1);
}
sig += ")Lorg/bukkit/inventory/ShapedRecipe;";
let res = self.jni_ref().call_method(&self.jni_object(),"shape",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ShapedRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Sets the material that a character in the recipe shape refers to.Note that before an ingredient can be set, the recipe's shape must be defined with {@link #shape(String...)}.
	fn set_ingredient(&self,key: u16,ingredient: impl Into<crate::Material<'mc>>,raw: std::option::Option<i32>) 
-> Result<crate::inventory::ShapedRecipe<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "C";
let val_1 = jni::objects::JValueGen::Char(key);
args.push(val_1);
sig += "Lorg/bukkit/Material;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ingredient.into().jni_object().clone())});
args.push(val_2);
if let Some(a) = raw {
sig += "I";
let val_3 = jni::objects::JValueGen::Int(a);
args.push(val_3);
}
sig += ")Lorg/bukkit/inventory/ShapedRecipe;";
let res = self.jni_ref().call_method(&self.jni_object(),"setIngredient",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ShapedRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get a copy of the ingredients map.
	fn ingredient_map(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"getIngredientMap",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get a copy of the choice map.
	fn choice_map(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChoiceMap",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Set the shape of this recipe to the specified rows. Each character
/// represents a different ingredient; exactly what each character
/// represents is set separately. The first row supplied corresponds with
/// the upper most part of the recipe on the workbench e.g. if all three
/// rows are supplies the first string represents the top row on the
/// workbench.
	fn shape_with_shape(&self,shape: std::option::Option<impl Into<String>>) 
-> Result<crate::inventory::ShapedRecipe<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = shape {
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(a.into())?));
args.push(val_1);
}
sig += ")Lorg/bukkit/inventory/ShapedRecipe;";
let res = self.jni_ref().call_method(&self.jni_object(),"getShape",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ShapedRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::CraftingRecipe<'mc>> for ShapedRecipe<'mc>{

fn into(self) -> crate::inventory::CraftingRecipe<'mc> {

crate::inventory::CraftingRecipe::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ShapedRecipe into crate::inventory::CraftingRecipe")

   }
}
impl<'mc> crate::inventory::CraftingRecipeTrait<'mc> for ShapedRecipe<'mc> {}
#[repr(C)]
pub struct SmithingRecipe<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SmithingRecipe<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SmithingRecipe<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SmithingRecipe from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/SmithingRecipe")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SmithingRecipe object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SmithingRecipeTrait<'mc> for SmithingRecipe<'mc> {}
pub trait SmithingRecipeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]
/// Create a smithing recipe to produce the specified result ItemStack.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,key: impl Into<crate::NamespacedKey<'mc>>,result: impl Into<crate::inventory::ItemStack<'mc>>,base: impl Into<crate::inventory::RecipeChoice<'mc>>,addition: impl Into<crate::inventory::RecipeChoice<'mc>>) 
-> Result<crate::inventory::SmithingRecipe<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/RecipeChoice;Lorg/bukkit/inventory/RecipeChoice;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(base.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(addition.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/inventory/SmithingRecipe"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::SmithingRecipe::from_raw(&jni,res
)}
/// Get the base recipe item.
	fn base(&self) 
-> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/RecipeChoice;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBase",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::RecipeChoice::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the addition recipe item.
	fn addition(&self) 
-> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/RecipeChoice;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAddition",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::RecipeChoice::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn result(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::inventory::Recipe<'mc>> for SmithingRecipe<'mc>{

fn into(self) -> crate::inventory::Recipe<'mc> {

crate::inventory::Recipe::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SmithingRecipe into crate::inventory::Recipe")

   }
}
impl<'mc> crate::inventory::RecipeTrait<'mc> for SmithingRecipe<'mc> {}
impl<'mc> Into<crate::Keyed<'mc>> for SmithingRecipe<'mc>{

fn into(self) -> crate::Keyed<'mc> {

crate::Keyed::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SmithingRecipe into crate::Keyed")

   }
}
impl<'mc> crate::KeyedTrait<'mc> for SmithingRecipe<'mc> {}
#[repr(C)]
pub struct StonecuttingRecipe<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for StonecuttingRecipe<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for StonecuttingRecipe<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate StonecuttingRecipe from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/StonecuttingRecipe")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a StonecuttingRecipe object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> StonecuttingRecipeTrait<'mc> for StonecuttingRecipe<'mc> {}
pub trait StonecuttingRecipeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Create a cooking recipe to craft the specified ItemStack.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,key: impl Into<crate::NamespacedKey<'mc>>,result: impl Into<crate::inventory::ItemStack<'mc>>,input: impl Into<crate::inventory::RecipeChoice<'mc>>) 
-> Result<crate::inventory::StonecuttingRecipe<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/NamespacedKey;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/inventory/RecipeChoice;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(input.into().jni_object().clone())});
args.push(val_3);
sig += ")V";
let cls = jni.find_class("org/bukkit/inventory/StonecuttingRecipe"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::StonecuttingRecipe::from_raw(&jni,res
)}
/// Sets the input of this cooking recipe.
	fn set_input(&self,input: impl Into<crate::Material<'mc>>) 
-> Result<crate::inventory::StonecuttingRecipe<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Material;)Lorg/bukkit/inventory/StonecuttingRecipe;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(input.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setInput",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::StonecuttingRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the input material.
	fn input(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInput",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the input of this cooking recipe.
	fn set_input_choice(&self,input: impl Into<crate::inventory::RecipeChoice<'mc>>) 
-> Result<crate::inventory::StonecuttingRecipe<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/RecipeChoice;)Lorg/bukkit/inventory/StonecuttingRecipe;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(input.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setInputChoice",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::StonecuttingRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the input choice.
	fn input_choice(&self) 
-> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/RecipeChoice;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInputChoice",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::RecipeChoice::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the result of this recipe.
	fn result(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn key(&self) 
-> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/NamespacedKey;");
let res = self.jni_ref().call_method(&self.jni_object(),"getKey",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::NamespacedKey::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the group of this recipe. Recipes with the same group may be grouped
/// together when displayed in the client.
	fn group(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getGroup",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Set the group of this recipe. Recipes with the same group may be grouped
/// together when displayed in the client.
	fn set_group(&self,group: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(group.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setGroup",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Recipe<'mc>> for StonecuttingRecipe<'mc>{

fn into(self) -> crate::inventory::Recipe<'mc> {

crate::inventory::Recipe::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting StonecuttingRecipe into crate::inventory::Recipe")

   }
}
impl<'mc> crate::inventory::RecipeTrait<'mc> for StonecuttingRecipe<'mc> {}
impl<'mc> Into<crate::Keyed<'mc>> for StonecuttingRecipe<'mc>{

fn into(self) -> crate::Keyed<'mc> {

crate::Keyed::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting StonecuttingRecipe into crate::Keyed")

   }
}
impl<'mc> crate::KeyedTrait<'mc> for StonecuttingRecipe<'mc> {}
#[repr(C)]
pub struct CraftingRecipe<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CraftingRecipe<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CraftingRecipe<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CraftingRecipe from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/CraftingRecipe")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CraftingRecipe object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CraftingRecipeTrait<'mc> for CraftingRecipe<'mc> {}
pub trait CraftingRecipeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn key(&self) 
-> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/NamespacedKey;");
let res = self.jni_ref().call_method(&self.jni_object(),"getKey",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::NamespacedKey::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the result of this recipe.
	fn result(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the group of this recipe. Recipes with the same group may be grouped
/// together when displayed in the client.
	fn group(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getGroup",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Set the group of this recipe. Recipes with the same group may be grouped
/// together when displayed in the client.
	fn set_group(&self,group: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(group.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setGroup",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the category which this recipe will appear in the recipe book under.
/// Defaults to {@link CraftingBookCategory#MISC} if not set.
	fn category(&self) 
-> Result<crate::inventory::recipe::CraftingBookCategory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/recipe/CraftingBookCategory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCategory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::recipe::CraftingBookCategory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the category which this recipe will appear in the recipe book under.
/// Defaults to {@link CraftingBookCategory#MISC} if not set.
	fn set_category(&self,category: impl Into<crate::inventory::recipe::CraftingBookCategory<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/recipe/CraftingBookCategory;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(category.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setCategory",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Recipe<'mc>> for CraftingRecipe<'mc>{

fn into(self) -> crate::inventory::Recipe<'mc> {

crate::inventory::Recipe::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CraftingRecipe into crate::inventory::Recipe")

   }
}
impl<'mc> crate::inventory::RecipeTrait<'mc> for CraftingRecipe<'mc> {}
impl<'mc> Into<crate::Keyed<'mc>> for CraftingRecipe<'mc>{

fn into(self) -> crate::Keyed<'mc> {

crate::Keyed::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CraftingRecipe into crate::Keyed")

   }
}
impl<'mc> crate::KeyedTrait<'mc> for CraftingRecipe<'mc> {}
#[repr(C)]
pub struct InventoryHolder<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for InventoryHolder<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for InventoryHolder<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate InventoryHolder from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/InventoryHolder")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a InventoryHolder object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> InventoryHolderTrait<'mc> for InventoryHolder<'mc> {}
pub trait InventoryHolderTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the object's inventory.
	fn inventory(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BeaconInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BeaconInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BeaconInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BeaconInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/BeaconInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BeaconInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BeaconInventoryTrait<'mc> for BeaconInventory<'mc> {}
pub trait BeaconInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Set the item powering the beacon.
	fn set_item(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the item powering the beacon.
	fn item(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for BeaconInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BeaconInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for BeaconInventory<'mc> {}
#[repr(C)]
pub struct SmokingRecipe<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SmokingRecipe<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SmokingRecipe<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SmokingRecipe from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/SmokingRecipe")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SmokingRecipe object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SmokingRecipeTrait<'mc> for SmokingRecipe<'mc> {}
pub trait SmokingRecipeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,key: impl Into<crate::NamespacedKey<'mc>>,result: impl Into<crate::inventory::ItemStack<'mc>>,input: impl Into<crate::inventory::RecipeChoice<'mc>>,experience: f32,cooking_time: i32) 
-> Result<crate::inventory::SmokingRecipe<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/NamespacedKey;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/inventory/RecipeChoice;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(input.into().jni_object().clone())});
args.push(val_3);
sig += "F";
let val_4 = jni::objects::JValueGen::Float(experience);
args.push(val_4);
sig += "I";
let val_5 = jni::objects::JValueGen::Int(cooking_time);
args.push(val_5);
sig += ")V";
let cls = jni.find_class("org/bukkit/inventory/SmokingRecipe"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::SmokingRecipe::from_raw(&jni,res
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::CookingRecipe<'mc>> for SmokingRecipe<'mc>{

fn into(self) -> crate::inventory::CookingRecipe<'mc> {

crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SmokingRecipe into crate::inventory::CookingRecipe")

   }
}
impl<'mc> crate::inventory::CookingRecipeTrait<'mc> for SmokingRecipe<'mc> {}
#[repr(C)]
pub struct ItemFactory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ItemFactory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ItemFactory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ItemFactory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ItemFactory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ItemFactory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ItemFactoryTrait<'mc> for ItemFactory<'mc> {}
pub trait ItemFactoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// This creates a new item meta for the material.
	fn get_item_meta(&self,material: impl Into<crate::Material<'mc>>) 
-> Result<Option<crate::inventory::meta::ItemMeta<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Material;)Lorg/bukkit/inventory/meta/ItemMeta;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(material.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getItemMeta",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// This method checks the item meta to confirm that it is applicable (no
/// data lost if applied) to the specified Material.
/// 
/// A {@link SkullMeta} would not be valid for a sword, but a normal {@link
/// ItemMeta} from an enchanted dirt block would.
	fn is_applicable(&self,meta: impl Into<crate::inventory::meta::ItemMeta<'mc>>,material: impl Into<crate::Material<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/meta/ItemMeta;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(meta.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/Material;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(material.into().jni_object().clone())});
args.push(val_2);
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"isApplicable",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// This method is used to compare two item meta data objects.
	fn equals(&self,meta1: impl Into<crate::inventory::meta::ItemMeta<'mc>>,meta2: impl Into<crate::inventory::meta::ItemMeta<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/meta/ItemMeta;Lorg/bukkit/inventory/meta/ItemMeta;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(meta1.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(meta2.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns an appropriate item meta for the specified material.
/// 
/// The item meta returned will always be a valid meta for a given
/// ItemStack of the specified material. It may be a more or less specific
/// meta, and could also be the same meta or meta type as the parameter.
/// The item meta returned will also always be the most appropriate meta.
/// 
/// Example, if a {@link SkullMeta} is being applied to a book, this method
/// would return a {@link BookMeta} containing all information in the
/// specified meta that is applicable to an {@link ItemMeta}, the highest
/// common interface.
	fn as_meta_for(&self,meta: impl Into<crate::inventory::meta::ItemMeta<'mc>>,material: impl Into<crate::Material<'mc>>) 
-> Result<Option<crate::inventory::meta::ItemMeta<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/meta/ItemMeta;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(meta.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/Material;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(material.into().jni_object().clone())});
args.push(val_2);
sig += ")Lorg/bukkit/inventory/meta/ItemMeta;";
let res = self.jni_ref().call_method(&self.jni_object(),"asMetaFor",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Returns the default color for all leather armor.
	fn default_leather_color(&self) 
-> Result<crate::Color<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Color;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultLeatherColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Color::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Create a new {@link ItemStack} given the supplied input.
/// 
/// The input should match the same input as expected by Minecraft's {@code /give}
/// command. For example, "minecraft:diamond_sword{Enchantments:[{id:"minecraft:sharpness", lvl:3}]}"
/// would yield an ItemStack of {@link Material#DIAMOND_SWORD} with an {@link ItemMeta}
/// containing a level 3 {@link Enchantment#DAMAGE_ALL}
/// enchantment.
	fn create_item_stack(&self,input: impl Into<String>) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/inventory/ItemStack;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(input.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"createItemStack",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Apply a material change for an item meta. Do not use under any
/// circumstances.
	fn update_material(&self,meta: impl Into<crate::inventory::meta::ItemMeta<'mc>>,material: impl Into<crate::Material<'mc>>) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/meta/ItemMeta;Lorg/bukkit/Material;)Lorg/bukkit/Material;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(meta.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(material.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"updateMaterial",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Material::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets a {@link Material} representing the spawn egg for the provided
/// {@link EntityType}.
/// 
/// Will return null for EntityTypes that do not have a corresponding spawn egg.
	fn get_spawn_egg(&self,val_type: impl Into<crate::entity::EntityType<'mc>>) 
-> Result<Option<crate::Material<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/EntityType;)Lorg/bukkit/Material;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getSpawnEgg",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Material::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Enchants the given item at the provided level.
/// 
/// If an item that is air is passed through an error is thrown.
	fn enchant_item(&self,world: impl Into<crate::World<'mc>>,item: impl Into<crate::inventory::ItemStack<'mc>>,level: i32,allow_treasures: std::option::Option<bool>) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/World;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
args.push(val_2);
sig += "I";
let val_3 = jni::objects::JValueGen::Int(level);
args.push(val_3);
if let Some(a) = allow_treasures {
sig += "Z";
let val_4 = jni::objects::JValueGen::Bool(a.into());
args.push(val_4);
}
sig += ")Lorg/bukkit/inventory/ItemStack;";
let res = self.jni_ref().call_method(&self.jni_object(),"enchantItem",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct StonecutterInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for StonecutterInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for StonecutterInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate StonecutterInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/StonecutterInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a StonecutterInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> StonecutterInventoryTrait<'mc> for StonecutterInventory<'mc> {}
pub trait StonecutterInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for StonecutterInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting StonecutterInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for StonecutterInventory<'mc> {}
#[repr(C)]
pub struct CampfireRecipe<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CampfireRecipe<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CampfireRecipe<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CampfireRecipe from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/CampfireRecipe")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CampfireRecipe object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CampfireRecipeTrait<'mc> for CampfireRecipe<'mc> {}
pub trait CampfireRecipeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,key: impl Into<crate::NamespacedKey<'mc>>,result: impl Into<crate::inventory::ItemStack<'mc>>,input: impl Into<crate::inventory::RecipeChoice<'mc>>,experience: f32,cooking_time: i32) 
-> Result<crate::inventory::CampfireRecipe<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/NamespacedKey;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/inventory/RecipeChoice;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(input.into().jni_object().clone())});
args.push(val_3);
sig += "F";
let val_4 = jni::objects::JValueGen::Float(experience);
args.push(val_4);
sig += "I";
let val_5 = jni::objects::JValueGen::Int(cooking_time);
args.push(val_5);
sig += ")V";
let cls = jni.find_class("org/bukkit/inventory/CampfireRecipe"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::CampfireRecipe::from_raw(&jni,res
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::CookingRecipe<'mc>> for CampfireRecipe<'mc>{

fn into(self) -> crate::inventory::CookingRecipe<'mc> {

crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CampfireRecipe into crate::inventory::CookingRecipe")

   }
}
impl<'mc> crate::inventory::CookingRecipeTrait<'mc> for CampfireRecipe<'mc> {}
#[repr(C)]
pub struct HorseInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for HorseInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for HorseInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate HorseInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/HorseInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a HorseInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> HorseInventoryTrait<'mc> for HorseInventory<'mc> {}
pub trait HorseInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the item in the horse's armor slot.
	fn armor(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getArmor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the item in the horse's armor slot.
	fn set_armor(&self,stack: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(stack.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setArmor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::AbstractHorseInventory<'mc>> for HorseInventory<'mc>{

fn into(self) -> crate::inventory::AbstractHorseInventory<'mc> {

crate::inventory::AbstractHorseInventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting HorseInventory into crate::inventory::AbstractHorseInventory")

   }
}
impl<'mc> crate::inventory::AbstractHorseInventoryTrait<'mc> for HorseInventory<'mc> {}
#[repr(C)]
pub struct PlayerInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/PlayerInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerInventoryTrait<'mc> for PlayerInventory<'mc> {}
pub trait PlayerInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets all ItemStacks from the armor slots.
	fn armor_contents(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getArmorContents",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get all additional ItemStacks stored in this inventory.
/// 
/// NB: What defines an extra slot is up to the implementation, however it
/// will not be contained within {@link #getStorageContents()} or
/// {@link #getArmorContents()}
	fn extra_contents(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getExtraContents",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Return the ItemStack from the helmet slot
	fn helmet(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHelmet",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Return the ItemStack from the chestplate slot
	fn chestplate(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChestplate",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Return the ItemStack from the leg slot
	fn leggings(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLeggings",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Return the ItemStack from the boots slot
	fn boots(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBoots",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Stores the ItemStack at the given index of the inventory.
/// 
/// Indexes 0 through 8 refer to the hotbar. 9 through 35 refer to the main inventory, counting up from 9 at the top
/// left corner of the inventory, moving to the right, and moving to the row below it back on the left side when it
/// reaches the end of the row. It follows the same path in the inventory like you would read a book.
/// 
/// Indexes 36 through 39 refer to the armor slots. Though you can set armor with this method using these indexes,
/// you are encouraged to use the provided methods for those slots.
/// 
/// Index 40 refers to the off hand (shield) item slot. Though you can set off hand with this method using this index,
/// you are encouraged to use the provided method for this slot.
/// 
/// If you attempt to use this method with an index less than 0 or greater than 40, an ArrayIndexOutOfBounds
/// exception will be thrown.
	fn set_item(&self,index: i32,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Int(index);
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the ItemStack at the given equipment slot in the inventory.
	fn get_item(&self,slot: impl Into<crate::inventory::EquipmentSlot<'mc>>) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/EquipmentSlot;)Lorg/bukkit/inventory/ItemStack;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(slot.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Put the given ItemStacks into the armor slots
	fn set_armor_contents(&self,items: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(items.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setArmorContents",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Put the given ItemStacks into the extra slots
/// 
/// See {@link #getExtraContents()} for an explanation of extra slots.
	fn set_extra_contents(&self,items: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(items.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setExtraContents",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Put the given ItemStack into the helmet slot. This does not check if
/// the ItemStack is a helmet
	fn set_helmet(&self,helmet: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(helmet.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setHelmet",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Put the given ItemStack into the chestplate slot. This does not check
/// if the ItemStack is a chestplate
	fn set_chestplate(&self,chestplate: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(chestplate.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setChestplate",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Put the given ItemStack into the leg slot. This does not check if the
/// ItemStack is a pair of leggings
	fn set_leggings(&self,leggings: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(leggings.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setLeggings",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Put the given ItemStack into the boots slot. This does not check if the
/// ItemStack is a boots
	fn set_boots(&self,boots: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(boots.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setBoots",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets a copy of the item the player is currently holding
/// in their main hand.
	fn item_in_main_hand(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemInMainHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the item the player is holding in their main hand.
	fn set_item_in_main_hand(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItemInMainHand",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets a copy of the item the player is currently holding
/// in their off hand.
	fn item_in_off_hand(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemInOffHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the item the player is holding in their off hand.
	fn set_item_in_off_hand(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItemInOffHand",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Gets a copy of the item the player is currently holding
	fn item_in_hand(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemInHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Sets the item the player is holding
	fn set_item_in_hand(&self,stack: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(stack.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItemInHand",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the slot number of the currently held item
	fn held_item_slot(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getHeldItemSlot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the slot number of the currently held item.
/// 
/// This validates whether the slot is between 0 and 8 inclusive.
	fn set_held_item_slot(&self,slot: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(slot);
let res = self.jni_ref().call_method(&self.jni_object(),"setHeldItemSlot",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn holder(&self) 
-> Result<Option<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHolder",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for PlayerInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for PlayerInventory<'mc> {}
#[repr(C)]
pub struct BlastingRecipe<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlastingRecipe<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlastingRecipe<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlastingRecipe from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/BlastingRecipe")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlastingRecipe object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlastingRecipeTrait<'mc> for BlastingRecipe<'mc> {}
pub trait BlastingRecipeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,key: impl Into<crate::NamespacedKey<'mc>>,result: impl Into<crate::inventory::ItemStack<'mc>>,input: impl Into<crate::inventory::RecipeChoice<'mc>>,experience: f32,cooking_time: i32) 
-> Result<crate::inventory::BlastingRecipe<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/NamespacedKey;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/inventory/RecipeChoice;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(input.into().jni_object().clone())});
args.push(val_3);
sig += "F";
let val_4 = jni::objects::JValueGen::Float(experience);
args.push(val_4);
sig += "I";
let val_5 = jni::objects::JValueGen::Int(cooking_time);
args.push(val_5);
sig += ")V";
let cls = jni.find_class("org/bukkit/inventory/BlastingRecipe"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::BlastingRecipe::from_raw(&jni,res
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::CookingRecipe<'mc>> for BlastingRecipe<'mc>{

fn into(self) -> crate::inventory::CookingRecipe<'mc> {

crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlastingRecipe into crate::inventory::CookingRecipe")

   }
}
impl<'mc> crate::inventory::CookingRecipeTrait<'mc> for BlastingRecipe<'mc> {}
#[repr(C)]
pub struct FurnaceRecipe<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for FurnaceRecipe<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for FurnaceRecipe<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate FurnaceRecipe from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/FurnaceRecipe")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a FurnaceRecipe object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> FurnaceRecipeTrait<'mc> for FurnaceRecipe<'mc> {}
pub trait FurnaceRecipeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,key: impl Into<crate::NamespacedKey<'mc>>,result: impl Into<crate::inventory::ItemStack<'mc>>,source: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i32>,experience: std::option::Option<f32>,cooking_time: std::option::Option<i32>) 
-> Result<crate::inventory::FurnaceRecipe<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/NamespacedKey;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
args.push(val_2);
if let Some(a) = source {
sig += "Lorg/bukkit/Material;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
if let Some(a) = data {
sig += "I";
let val_4 = jni::objects::JValueGen::Int(a);
args.push(val_4);
}
if let Some(a) = experience {
sig += "F";
let val_5 = jni::objects::JValueGen::Float(a);
args.push(val_5);
}
if let Some(a) = cooking_time {
sig += "I";
let val_6 = jni::objects::JValueGen::Int(a);
args.push(val_6);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/inventory/FurnaceRecipe"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::FurnaceRecipe::from_raw(&jni,res
)}

	fn set_input(&self,input: impl Into<crate::Material<'mc>>) 
-> Result<crate::inventory::FurnaceRecipe<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Material;)Lorg/bukkit/inventory/FurnaceRecipe;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(input.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setInput",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::FurnaceRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn set_input_choice(&self,input: impl Into<crate::inventory::RecipeChoice<'mc>>) 
-> Result<crate::inventory::FurnaceRecipe<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/RecipeChoice;)Lorg/bukkit/inventory/FurnaceRecipe;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(input.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setInputChoice",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::FurnaceRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::CookingRecipe<'mc>> for FurnaceRecipe<'mc>{

fn into(self) -> crate::inventory::CookingRecipe<'mc> {

crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting FurnaceRecipe into crate::inventory::CookingRecipe")

   }
}
impl<'mc> crate::inventory::CookingRecipeTrait<'mc> for FurnaceRecipe<'mc> {}
pub enum MainHand<'mc> {
	Left {inner: MainHandStruct<'mc>},
	Right {inner: MainHandStruct<'mc>},
}
impl<'mc> std::fmt::Display for MainHand<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           MainHand::Left { .. } => f.write_str("LEFT"),
           MainHand::Right { .. } => f.write_str("RIGHT"),
       }
   }
}

        impl<'mc> MainHandTrait<'mc> for MainHand<'mc> {}
        
        pub trait MainHandTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<MainHand<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/inventory/MainHand");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/inventory/MainHand;",
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
                    
"LEFT" => Ok(MainHand::Left { inner: MainHandStruct::from_raw(env,obj)?}),
"RIGHT" => Ok(MainHand::Right { inner: MainHandStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct MainHandStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for MainHand<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Left { inner } => inner.0.clone(),
Self::Right { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Left { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Right { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for MainHand<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MainHand from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/MainHand")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MainHand object, got {}",
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
                    "LEFT" => Ok(MainHand::Left { inner: MainHandStruct::from_raw(env,obj)?}),"RIGHT" => Ok(MainHand::Right { inner: MainHandStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for MainHandStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for MainHandStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MainHandStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/MainHand")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MainHandStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MainHandStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::inventory::MainHand<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/MainHand;");
let cls = jni.find_class("org/bukkit/inventory/MainHand"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::inventory::MainHand::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct RecipeChoiceExactChoice<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RecipeChoiceExactChoice<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RecipeChoiceExactChoice<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RecipeChoiceExactChoice from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/RecipeChoice/ExactChoice")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RecipeChoiceExactChoice object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RecipeChoiceExactChoiceTrait<'mc> for RecipeChoiceExactChoice<'mc> {}
pub trait RecipeChoiceExactChoiceTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,choices: Vec<jni::objects::JObject<'mc>>) 
-> Result<crate::inventory::RecipeChoiceExactChoice<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/util/List;";
let raw_val_1 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in choices{
sig += "Ljava/lang/java/lang/Object;";
		let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_1,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_1 = jni::objects::JValueGen::Object(raw_val_1);
args.push(val_1);
sig += ")V";
let cls = jni.find_class("org/bukkit/inventory/RecipeChoice/ExactChoice"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::RecipeChoiceExactChoice::from_raw(&jni,res
)}

	fn item_stack(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemStack",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn choices(&self) 
-> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChoices",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}

	fn clone(&self) 
-> Result<crate::inventory::RecipeChoiceExactChoice<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/RecipeChoice/ExactChoice;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::RecipeChoiceExactChoice::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn test(&self,t: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(t.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"test",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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

        impl<'mc> std::string::ToString for RecipeChoiceExactChoice<'mc> {
            fn to_string(&self) -> String {
                match RecipeChoiceExactChoiceTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling RecipeChoiceExactChoice.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::inventory::RecipeChoice<'mc>> for RecipeChoiceExactChoice<'mc>{

fn into(self) -> crate::inventory::RecipeChoice<'mc> {

crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting RecipeChoiceExactChoice into crate::inventory::RecipeChoice")

   }
}
impl<'mc> crate::inventory::RecipeChoiceTrait<'mc> for RecipeChoiceExactChoice<'mc> {}
#[repr(C)]
pub struct Recipe<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Recipe<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Recipe<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Recipe from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/Recipe")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Recipe object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RecipeTrait<'mc> for Recipe<'mc> {}
pub trait RecipeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the result of this recipe.
	fn result(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct CraftingInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CraftingInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CraftingInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CraftingInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/CraftingInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CraftingInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CraftingInventoryTrait<'mc> for CraftingInventory<'mc> {}
pub trait CraftingInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Check what item is in the result slot of this crafting inventory.
	fn result(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Get the contents of the crafting matrix.
	fn matrix(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMatrix",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Set the item in the result slot of the crafting inventory.
	fn set_result(&self,new_result: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_result.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setResult",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Replace the contents of the crafting matrix
	fn set_matrix(&self,contents: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(contents.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setMatrix",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the current recipe formed on the crafting inventory, if any.
	fn recipe(&self) 
-> Result<Option<crate::inventory::Recipe<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/Recipe;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipe",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::Recipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for CraftingInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CraftingInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for CraftingInventory<'mc> {}
#[repr(C)]
pub struct DoubleChestInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for DoubleChestInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for DoubleChestInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate DoubleChestInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/DoubleChestInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a DoubleChestInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DoubleChestInventoryTrait<'mc> for DoubleChestInventory<'mc> {}
pub trait DoubleChestInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the left half of this double chest.
	fn left_side(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLeftSide",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the right side of this double chest.
	fn right_side(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRightSide",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn holder(&self) 
-> Result<Option<crate::block::DoubleChest<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/DoubleChest;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHolder",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::DoubleChest::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for DoubleChestInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting DoubleChestInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for DoubleChestInventory<'mc> {}
#[repr(C)]
pub struct MerchantInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for MerchantInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for MerchantInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MerchantInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/MerchantInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MerchantInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MerchantInventoryTrait<'mc> for MerchantInventory<'mc> {}
pub trait MerchantInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the index of the currently selected recipe.
	fn selected_recipe_index(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getSelectedRecipeIndex",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Get the currently active recipe.
/// 
/// This will be <code>null</code> if the items provided by the player do
/// not match the ingredients of the selected recipe. This does not
/// necessarily match the recipe selected by the player: If the player has
/// selected the first recipe, the merchant will search all of its offers
/// for a matching recipe to activate.
	fn selected_recipe(&self) 
-> Result<Option<crate::inventory::MerchantRecipe<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/MerchantRecipe;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSelectedRecipe",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::MerchantRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Gets the Merchant associated with this inventory.
	fn merchant(&self) 
-> Result<crate::inventory::Merchant<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/Merchant;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMerchant",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Merchant::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for MerchantInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting MerchantInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for MerchantInventory<'mc> {}
#[repr(C)]
pub struct InventoryView<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for InventoryView<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for InventoryView<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate InventoryView from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/InventoryView")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a InventoryView object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> InventoryViewTrait<'mc> for InventoryView<'mc> {}
pub trait InventoryViewTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/inventory/InventoryView"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::InventoryView::from_raw(&jni,res
)}
/// Sets one item in this inventory view by its raw slot ID.
/// 
/// Note: If slot ID -999 is chosen, it may be expected that the item is
/// dropped on the ground. This is not required behaviour, however.
	fn set_item(&self,slot: i32,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Int(slot);
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets one item in this inventory view by its raw slot ID.
	fn get_item(&self,slot: i32) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
let val_1 = jni::objects::JValueGen::Int(slot);
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the item on the cursor of one of the viewing players.
	fn set_cursor(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setCursor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the item on the cursor of one of the viewing players.
	fn cursor(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCursor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Gets the inventory corresponding to the given raw slot ID.
/// If the slot ID is {@link #OUTSIDE} null will be returned, otherwise
/// behaviour for illegal and negative slot IDs is undefined.
/// May be used with {@link #convertSlot(int)} to directly index an
/// underlying inventory.
	fn get_inventory(&self,raw_slot: i32) 
-> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(I)Lorg/bukkit/inventory/Inventory;");
let val_1 = jni::objects::JValueGen::Int(raw_slot);
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Converts a raw slot ID into its local slot ID into whichever of the two
/// inventories the slot points to.
/// 
/// If the raw slot refers to the upper inventory, it will be returned
/// unchanged and thus be suitable for getTopInventory().getItem(); if it
/// refers to the lower inventory, the output will differ from the input
/// and be suitable for getBottomInventory().getItem().
	fn convert_slot(&self,raw_slot: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(I)I");
let val_1 = jni::objects::JValueGen::Int(raw_slot);
let res = self.jni_ref().call_method(&self.jni_object(),"convertSlot",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Determine the type of the slot by its raw slot ID.
/// 
/// If the type of the slot is unknown, then
/// {@link InventoryType.SlotType#CONTAINER} will be returned.
	fn get_slot_type(&self,slot: i32) 
-> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(I)Lorg/bukkit/event/inventory/InventoryType/SlotType;");
let val_1 = jni::objects::JValueGen::Int(slot);
let res = self.jni_ref().call_method(&self.jni_object(),"getSlotType",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Closes the inventory view.
	fn close(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"close",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Check the total number of slots in this view, combining the upper and
/// lower inventories.
/// 
/// Note though that it's possible for this to be greater than the sum of
/// the two inventories if for example some slots are not being used.
	fn count_slots(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"countSlots",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets an extra property of this inventory if supported by that
/// inventory, for example the state of a progress bar.
	fn set_property(&self,prop: impl Into<crate::inventory::InventoryViewProperty<'mc>>,value: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/InventoryView/Property;I)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(prop.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(value);
let res = self.jni_ref().call_method(&self.jni_object(),"setProperty",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
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
#[repr(C)]
pub struct ItemStack<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ItemStack<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ItemStack<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ItemStack from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ItemStack")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ItemStack object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ItemStackTrait<'mc> for ItemStack<'mc> {}
pub trait ItemStackTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: impl Into<crate::Material<'mc>>,amount: std::option::Option<i32>,damage: std::option::Option<i16>,data: std::option::Option<i8>) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = amount {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a);
args.push(val_2);
}
if let Some(a) = damage {
sig += "S";
let val_3 = jni::objects::JValueGen::Short(a);
args.push(val_3);
}
if let Some(a) = data {
sig += "Ljava/lang/Byte;";
let val_4 = jni::objects::JValueGen::Object(jni.new_object("java/lang/Byte", "(Ljava/Lang/Object;)V", vec![a.into()])?);
args.push(val_4);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/inventory/ItemStack"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::ItemStack::from_raw(&jni,res
)}
/// Gets the type of this item
	fn get_type(&self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Material;");
let res = self.jni_ref().call_method(&self.jni_object(),"getType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Material::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the type of this item
/// 
/// Note that in doing so you will reset the MaterialData for this stack.
/// 
/// <b>IMPORTANT: An <i>Item</i>Stack is only designed to contain
/// <i>items</i>. Do not use this class to encapsulate Materials for which
/// {@link Material#isItem()} returns false.</b>
	fn set_type(&self,val_type: impl Into<crate::Material<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Material;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setType",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the amount of items in this stack
	fn amount(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getAmount",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the amount of items in this stack
	fn set_amount(&self,amount: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(amount);
let res = self.jni_ref().call_method(&self.jni_object(),"setAmount",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the MaterialData for this stack of items
	fn data(&self) 
-> Result<Option<crate::material::MaterialData<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/MaterialData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::material::MaterialData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the MaterialData for this stack of items
	fn set_data(&self,data: impl Into<crate::material::MaterialData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(data.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Sets the durability of this item
	fn set_durability(&self,durability: i16) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(S)V");
let val_1 = jni::objects::JValueGen::Short(durability);
let res = self.jni_ref().call_method(&self.jni_object(),"setDurability",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Gets the durability of this item
	fn durability(&self) 
-> Result<i16, Box<dyn std::error::Error>>

{let sig = String::from("()S");
let res = self.jni_ref().call_method(&self.jni_object(),"getDurability",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.s()?
)}
/// Get the maximum stacksize for the material hold in this ItemStack.
/// (Returns -1 if it has no idea)
	fn max_stack_size(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxStackSize",sig.as_str(),vec![]);
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
/// This method is the same as equals, but does not consider stack size
/// (amount).
	fn is_similar(&self,stack: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(stack.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"isSimilar",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn clone(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
/// Checks if this ItemStack contains the given {@link Enchantment}
	fn contains_enchantment(&self,ench: impl Into<crate::enchantments::Enchantment<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ench.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"containsEnchantment",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the level of the specified enchantment on this item stack
	fn get_enchantment_level(&self,ench: impl Into<crate::enchantments::Enchantment<'mc>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)I");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ench.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getEnchantmentLevel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets a map containing all enchantments and their levels on this item.
	fn enchantments(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEnchantments",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Adds the specified enchantments to this item stack.
/// 
/// This method is the same as calling {@link
/// #addEnchantment(org.bukkit.enchantments.Enchantment, int)} for each
/// element of the map.
	fn add_enchantments(&self,enchantments: impl Into<blackboxmc_java::util::JavaMap<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/Map;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(enchantments.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addEnchantments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Adds the specified {@link Enchantment} to this item stack.
/// 
/// If this item stack already contained the given enchantment (at any
/// level), it will be replaced.
	fn add_enchantment(&self,ench: impl Into<crate::enchantments::Enchantment<'mc>>,level: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;I)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ench.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(level);
let res = self.jni_ref().call_method(&self.jni_object(),"addEnchantment",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Adds the specified enchantments to this item stack in an unsafe manner.
/// 
/// This method is the same as calling {@link
/// #addUnsafeEnchantment(org.bukkit.enchantments.Enchantment, int)} for
/// each element of the map.
	fn add_unsafe_enchantments(&self,enchantments: impl Into<blackboxmc_java::util::JavaMap<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/Map;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(enchantments.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addUnsafeEnchantments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Adds the specified {@link Enchantment} to this item stack.
/// 
/// If this item stack already contained the given enchantment (at any
/// level), it will be replaced.
/// 
/// This method is unsafe and will ignore level restrictions or item type.
/// Use at your own discretion.
	fn add_unsafe_enchantment(&self,ench: impl Into<crate::enchantments::Enchantment<'mc>>,level: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;I)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ench.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(level);
let res = self.jni_ref().call_method(&self.jni_object(),"addUnsafeEnchantment",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Removes the specified {@link Enchantment} if it exists on this
/// ItemStack
	fn remove_enchantment(&self,ench: impl Into<crate::enchantments::Enchantment<'mc>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)I");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ench.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removeEnchantment",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Removes all enchantments on this ItemStack.
	fn remove_enchantments(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"removeEnchantments",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn serialize(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"serialize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Required method for configuration serialization
	fn deserialize(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_args: impl Into<blackboxmc_java::util::JavaMap<'mc>>) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/Map;)Lorg/bukkit/inventory/ItemStack;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_args.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/inventory/ItemStack"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"deserialize",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::inventory::ItemStack::from_raw(&jni,obj
)}
/// Get a copy of this ItemStack's {@link ItemMeta}.
	fn item_meta(&self) 
-> Result<Option<crate::inventory::meta::ItemMeta<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/ItemMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemMeta",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Checks to see if any meta data has been defined.
	fn has_item_meta(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasItemMeta",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set the ItemMeta of this ItemStack.
	fn set_item_meta(&self,item_meta: impl Into<crate::inventory::meta::ItemMeta<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/meta/ItemMeta;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item_meta.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItemMeta",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn translation_key(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTranslationKey",sig.as_str(),vec![]);
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

        impl<'mc> std::string::ToString for ItemStack<'mc> {
            fn to_string(&self) -> String {
                match ItemStackTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling ItemStack.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>> for ItemStack<'mc>{

fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {

crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ItemStack into crate::configuration::serialization::ConfigurationSerializable")

   }
}
impl<'mc> crate::configuration::serialization::ConfigurationSerializableTrait<'mc> for ItemStack<'mc> {}
impl<'mc> Into<crate::Translatable<'mc>> for ItemStack<'mc>{

fn into(self) -> crate::Translatable<'mc> {

crate::Translatable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ItemStack into crate::Translatable")

   }
}
impl<'mc> crate::TranslatableTrait<'mc> for ItemStack<'mc> {}
#[repr(C)]
pub struct ComplexRecipe<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ComplexRecipe<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ComplexRecipe<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ComplexRecipe from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ComplexRecipe")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ComplexRecipe object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ComplexRecipeTrait<'mc> for ComplexRecipe<'mc> {}
pub trait ComplexRecipeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Recipe<'mc>> for ComplexRecipe<'mc>{

fn into(self) -> crate::inventory::Recipe<'mc> {

crate::inventory::Recipe::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ComplexRecipe into crate::inventory::Recipe")

   }
}
impl<'mc> crate::inventory::RecipeTrait<'mc> for ComplexRecipe<'mc> {}
impl<'mc> Into<crate::Keyed<'mc>> for ComplexRecipe<'mc>{

fn into(self) -> crate::Keyed<'mc> {

crate::Keyed::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ComplexRecipe into crate::Keyed")

   }
}
impl<'mc> crate::KeyedTrait<'mc> for ComplexRecipe<'mc> {}
#[repr(C)]
pub struct GrindstoneInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for GrindstoneInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for GrindstoneInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate GrindstoneInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/GrindstoneInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a GrindstoneInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> GrindstoneInventoryTrait<'mc> for GrindstoneInventory<'mc> {}
pub trait GrindstoneInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for GrindstoneInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting GrindstoneInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for GrindstoneInventory<'mc> {}
#[repr(C)]
pub struct ChiseledBookshelfInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ChiseledBookshelfInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ChiseledBookshelfInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ChiseledBookshelfInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ChiseledBookshelfInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ChiseledBookshelfInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ChiseledBookshelfInventoryTrait<'mc> for ChiseledBookshelfInventory<'mc> {}
pub trait ChiseledBookshelfInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn holder(&self) 
-> Result<Option<crate::block::ChiseledBookshelf<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/ChiseledBookshelf;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHolder",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::ChiseledBookshelf::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for ChiseledBookshelfInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ChiseledBookshelfInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for ChiseledBookshelfInventory<'mc> {}
pub enum EquipmentSlot<'mc> {
	Hand {inner: EquipmentSlotStruct<'mc>},
	OffHand {inner: EquipmentSlotStruct<'mc>},
	Feet {inner: EquipmentSlotStruct<'mc>},
	Legs {inner: EquipmentSlotStruct<'mc>},
	Chest {inner: EquipmentSlotStruct<'mc>},
	Head {inner: EquipmentSlotStruct<'mc>},
}
impl<'mc> std::fmt::Display for EquipmentSlot<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           EquipmentSlot::Hand { .. } => f.write_str("HAND"),
           EquipmentSlot::OffHand { .. } => f.write_str("OFF_HAND"),
           EquipmentSlot::Feet { .. } => f.write_str("FEET"),
           EquipmentSlot::Legs { .. } => f.write_str("LEGS"),
           EquipmentSlot::Chest { .. } => f.write_str("CHEST"),
           EquipmentSlot::Head { .. } => f.write_str("HEAD"),
       }
   }
}

        impl<'mc> EquipmentSlotTrait<'mc> for EquipmentSlot<'mc> {}
        
        pub trait EquipmentSlotTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<EquipmentSlot<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/inventory/EquipmentSlot");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/inventory/EquipmentSlot;",
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
                    
"HAND" => Ok(EquipmentSlot::Hand { inner: EquipmentSlotStruct::from_raw(env,obj)?}),
"OFF_HAND" => Ok(EquipmentSlot::OffHand { inner: EquipmentSlotStruct::from_raw(env,obj)?}),
"FEET" => Ok(EquipmentSlot::Feet { inner: EquipmentSlotStruct::from_raw(env,obj)?}),
"LEGS" => Ok(EquipmentSlot::Legs { inner: EquipmentSlotStruct::from_raw(env,obj)?}),
"CHEST" => Ok(EquipmentSlot::Chest { inner: EquipmentSlotStruct::from_raw(env,obj)?}),
"HEAD" => Ok(EquipmentSlot::Head { inner: EquipmentSlotStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct EquipmentSlotStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for EquipmentSlot<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Hand { inner } => inner.0.clone(),
Self::OffHand { inner } => inner.0.clone(),
Self::Feet { inner } => inner.0.clone(),
Self::Legs { inner } => inner.0.clone(),
Self::Chest { inner } => inner.0.clone(),
Self::Head { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Hand { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::OffHand { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Feet { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Legs { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Chest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Head { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for EquipmentSlot<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EquipmentSlot from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/EquipmentSlot")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EquipmentSlot object, got {}",
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
                    "HAND" => Ok(EquipmentSlot::Hand { inner: EquipmentSlotStruct::from_raw(env,obj)?}),"OFF_HAND" => Ok(EquipmentSlot::OffHand { inner: EquipmentSlotStruct::from_raw(env,obj)?}),"FEET" => Ok(EquipmentSlot::Feet { inner: EquipmentSlotStruct::from_raw(env,obj)?}),"LEGS" => Ok(EquipmentSlot::Legs { inner: EquipmentSlotStruct::from_raw(env,obj)?}),"CHEST" => Ok(EquipmentSlot::Chest { inner: EquipmentSlotStruct::from_raw(env,obj)?}),"HEAD" => Ok(EquipmentSlot::Head { inner: EquipmentSlotStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for EquipmentSlotStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for EquipmentSlotStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EquipmentSlotStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/EquipmentSlot")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EquipmentSlotStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> EquipmentSlotStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
let cls = jni.find_class("org/bukkit/inventory/EquipmentSlot"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::inventory::EquipmentSlot::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct FurnaceInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for FurnaceInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for FurnaceInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate FurnaceInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/FurnaceInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a FurnaceInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> FurnaceInventoryTrait<'mc> for FurnaceInventory<'mc> {}
pub trait FurnaceInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the current item in the result slot.
	fn result(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Get the current fuel.
	fn fuel(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFuel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Get the item currently smelting.
	fn smelting(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSmelting",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Set the current fuel.
	fn set_fuel(&self,stack: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(stack.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFuel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Set the current item in the result slot.
	fn set_result(&self,stack: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(stack.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setResult",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Set the item currently smelting.
	fn set_smelting(&self,stack: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(stack.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setSmelting",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn holder(&self) 
-> Result<Option<crate::block::Furnace<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Furnace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHolder",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::Furnace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for FurnaceInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting FurnaceInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for FurnaceInventory<'mc> {}
#[repr(C)]
pub struct SmithingInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SmithingInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SmithingInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SmithingInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/SmithingInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SmithingInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SmithingInventoryTrait<'mc> for SmithingInventory<'mc> {}
pub trait SmithingInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Check what item is in the result slot of this smithing table.
	fn result(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Set the item in the result slot of the smithing table
	fn set_result(&self,new_result: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_result.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setResult",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the current recipe formed on the smithing table, if any.
	fn recipe(&self) 
-> Result<Option<crate::inventory::Recipe<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/Recipe;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipe",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::Recipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for SmithingInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SmithingInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for SmithingInventory<'mc> {}
pub enum ItemFlag<'mc> {
	HideEnchants {inner: ItemFlagStruct<'mc>},
	HideAttributes {inner: ItemFlagStruct<'mc>},
	HideUnbreakable {inner: ItemFlagStruct<'mc>},
	HideDestroys {inner: ItemFlagStruct<'mc>},
	HidePlacedOn {inner: ItemFlagStruct<'mc>},
	HidePotionEffects {inner: ItemFlagStruct<'mc>},
	HideDye {inner: ItemFlagStruct<'mc>},
	HideArmorTrim {inner: ItemFlagStruct<'mc>},
}
impl<'mc> std::fmt::Display for ItemFlag<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           ItemFlag::HideEnchants { .. } => f.write_str("HIDE_ENCHANTS"),
           ItemFlag::HideAttributes { .. } => f.write_str("HIDE_ATTRIBUTES"),
           ItemFlag::HideUnbreakable { .. } => f.write_str("HIDE_UNBREAKABLE"),
           ItemFlag::HideDestroys { .. } => f.write_str("HIDE_DESTROYS"),
           ItemFlag::HidePlacedOn { .. } => f.write_str("HIDE_PLACED_ON"),
           ItemFlag::HidePotionEffects { .. } => f.write_str("HIDE_POTION_EFFECTS"),
           ItemFlag::HideDye { .. } => f.write_str("HIDE_DYE"),
           ItemFlag::HideArmorTrim { .. } => f.write_str("HIDE_ARMOR_TRIM"),
       }
   }
}

        impl<'mc> ItemFlagTrait<'mc> for ItemFlag<'mc> {}
        
        pub trait ItemFlagTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<ItemFlag<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/inventory/ItemFlag");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/inventory/ItemFlag;",
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
                    
"HIDE_ENCHANTS" => Ok(ItemFlag::HideEnchants { inner: ItemFlagStruct::from_raw(env,obj)?}),
"HIDE_ATTRIBUTES" => Ok(ItemFlag::HideAttributes { inner: ItemFlagStruct::from_raw(env,obj)?}),
"HIDE_UNBREAKABLE" => Ok(ItemFlag::HideUnbreakable { inner: ItemFlagStruct::from_raw(env,obj)?}),
"HIDE_DESTROYS" => Ok(ItemFlag::HideDestroys { inner: ItemFlagStruct::from_raw(env,obj)?}),
"HIDE_PLACED_ON" => Ok(ItemFlag::HidePlacedOn { inner: ItemFlagStruct::from_raw(env,obj)?}),
"HIDE_POTION_EFFECTS" => Ok(ItemFlag::HidePotionEffects { inner: ItemFlagStruct::from_raw(env,obj)?}),
"HIDE_DYE" => Ok(ItemFlag::HideDye { inner: ItemFlagStruct::from_raw(env,obj)?}),
"HIDE_ARMOR_TRIM" => Ok(ItemFlag::HideArmorTrim { inner: ItemFlagStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct ItemFlagStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ItemFlag<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::HideEnchants { inner } => inner.0.clone(),
Self::HideAttributes { inner } => inner.0.clone(),
Self::HideUnbreakable { inner } => inner.0.clone(),
Self::HideDestroys { inner } => inner.0.clone(),
Self::HidePlacedOn { inner } => inner.0.clone(),
Self::HidePotionEffects { inner } => inner.0.clone(),
Self::HideDye { inner } => inner.0.clone(),
Self::HideArmorTrim { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::HideEnchants { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::HideAttributes { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::HideUnbreakable { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::HideDestroys { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::HidePlacedOn { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::HidePotionEffects { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::HideDye { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::HideArmorTrim { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for ItemFlag<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ItemFlag from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ItemFlag")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ItemFlag object, got {}",
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
                    "HIDE_ENCHANTS" => Ok(ItemFlag::HideEnchants { inner: ItemFlagStruct::from_raw(env,obj)?}),"HIDE_ATTRIBUTES" => Ok(ItemFlag::HideAttributes { inner: ItemFlagStruct::from_raw(env,obj)?}),"HIDE_UNBREAKABLE" => Ok(ItemFlag::HideUnbreakable { inner: ItemFlagStruct::from_raw(env,obj)?}),"HIDE_DESTROYS" => Ok(ItemFlag::HideDestroys { inner: ItemFlagStruct::from_raw(env,obj)?}),"HIDE_PLACED_ON" => Ok(ItemFlag::HidePlacedOn { inner: ItemFlagStruct::from_raw(env,obj)?}),"HIDE_POTION_EFFECTS" => Ok(ItemFlag::HidePotionEffects { inner: ItemFlagStruct::from_raw(env,obj)?}),"HIDE_DYE" => Ok(ItemFlag::HideDye { inner: ItemFlagStruct::from_raw(env,obj)?}),"HIDE_ARMOR_TRIM" => Ok(ItemFlag::HideArmorTrim { inner: ItemFlagStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for ItemFlagStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ItemFlagStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ItemFlagStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ItemFlag")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ItemFlagStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ItemFlagStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::inventory::ItemFlag<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemFlag;");
let cls = jni.find_class("org/bukkit/inventory/ItemFlag"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::inventory::ItemFlag::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct MerchantRecipe<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for MerchantRecipe<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for MerchantRecipe<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MerchantRecipe from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/MerchantRecipe")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MerchantRecipe object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MerchantRecipeTrait<'mc> for MerchantRecipe<'mc> {}
pub trait MerchantRecipeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,result: impl Into<crate::inventory::ItemStack<'mc>>,uses: i32,max_uses: std::option::Option<i32>,experience_reward: std::option::Option<bool>,villager_experience: std::option::Option<i32>,price_multiplier: std::option::Option<f32>,demand: std::option::Option<i32>,special_price: std::option::Option<i32>) 
-> Result<crate::inventory::MerchantRecipe<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
args.push(val_1);
sig += "I";
let val_2 = jni::objects::JValueGen::Int(uses);
args.push(val_2);
if let Some(a) = max_uses {
sig += "I";
let val_3 = jni::objects::JValueGen::Int(a);
args.push(val_3);
}
if let Some(a) = experience_reward {
sig += "Z";
let val_4 = jni::objects::JValueGen::Bool(a.into());
args.push(val_4);
}
if let Some(a) = villager_experience {
sig += "I";
let val_5 = jni::objects::JValueGen::Int(a);
args.push(val_5);
}
if let Some(a) = price_multiplier {
sig += "F";
let val_6 = jni::objects::JValueGen::Float(a);
args.push(val_6);
}
if let Some(a) = demand {
sig += "I";
let val_7 = jni::objects::JValueGen::Int(a);
args.push(val_7);
}
if let Some(a) = special_price {
sig += "I";
let val_8 = jni::objects::JValueGen::Int(a);
args.push(val_8);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/inventory/MerchantRecipe"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::MerchantRecipe::from_raw(&jni,res
)}

	fn result(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn add_ingredient(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addIngredient",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn remove_ingredient(&self,index: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(index);
let res = self.jni_ref().call_method(&self.jni_object(),"removeIngredient",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn set_ingredients(&self,ingredients: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/List;)V");
let raw_val_1 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in ingredients{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_1,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_1 = jni::objects::JValueGen::Object(raw_val_1);
let res = self.jni_ref().call_method(&self.jni_object(),"setIngredients",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn ingredients(&self) 
-> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getIngredients",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Gets the {@link #adjust(ItemStack) adjusted} first ingredient.
	fn adjusted_ingredient1(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAdjustedIngredient1",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Modifies the amount of the given {@link ItemStack} in the same way as
/// MerchantRecipe dynamically adjusts the amount of the first ingredient
/// during trading.
/// 
/// This is calculated by adding up the original amount of the item, the
/// demand scaled by the recipe's
/// {@link #getPriceMultiplier price multiplier} and truncated to the next
/// lowest integer value greater than or equal to 0, and the special price,
/// and then constraining the resulting value between <code>1</code> and the
/// {@link ItemStack}'s {@link ItemStack#getMaxStackSize()
/// maximum stack size}.
	fn adjust(&self,item_stack: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item_stack.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"adjust",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the demand for this trade.
	fn demand(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getDemand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the demand for this trade.
	fn set_demand(&self,demand: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(demand);
let res = self.jni_ref().call_method(&self.jni_object(),"setDemand",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the special price for this trade.
	fn special_price(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getSpecialPrice",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the special price for this trade.
	fn set_special_price(&self,special_price: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(special_price);
let res = self.jni_ref().call_method(&self.jni_object(),"setSpecialPrice",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the number of times this trade has been used.
	fn uses(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getUses",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the number of times this trade has been used.
	fn set_uses(&self,uses: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(uses);
let res = self.jni_ref().call_method(&self.jni_object(),"setUses",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the maximum number of uses this trade has.
	fn max_uses(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxUses",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the maximum number of uses this trade has.
	fn set_max_uses(&self,max_uses: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(max_uses);
let res = self.jni_ref().call_method(&self.jni_object(),"setMaxUses",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Whether to reward experience to the player for the trade.
	fn has_experience_reward(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasExperienceReward",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set whether to reward experience to the player for the trade.
	fn set_experience_reward(&self,flag: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(flag.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setExperienceReward",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the amount of experience the villager earns from this trade.
	fn villager_experience(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getVillagerExperience",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the amount of experience the villager earns from this trade.
	fn set_villager_experience(&self,villager_experience: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(villager_experience);
let res = self.jni_ref().call_method(&self.jni_object(),"setVillagerExperience",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the price multiplier for the cost of this trade.
	fn price_multiplier(&self) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("()F");
let res = self.jni_ref().call_method(&self.jni_object(),"getPriceMultiplier",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.f()?
)}
/// Sets the price multiplier for the cost of this trade.
	fn set_price_multiplier(&self,price_multiplier: f32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(F)V");
let val_1 = jni::objects::JValueGen::Float(price_multiplier);
let res = self.jni_ref().call_method(&self.jni_object(),"setPriceMultiplier",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Recipe<'mc>> for MerchantRecipe<'mc>{

fn into(self) -> crate::inventory::Recipe<'mc> {

crate::inventory::Recipe::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting MerchantRecipe into crate::inventory::Recipe")

   }
}
impl<'mc> crate::inventory::RecipeTrait<'mc> for MerchantRecipe<'mc> {}
#[repr(C)]
pub struct DecoratedPotInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for DecoratedPotInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for DecoratedPotInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate DecoratedPotInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/DecoratedPotInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a DecoratedPotInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DecoratedPotInventoryTrait<'mc> for DecoratedPotInventory<'mc> {}
pub trait DecoratedPotInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Set the item stack in the decorated pot.
	fn set_item(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the item stack in the decorated pot.
	fn item(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn holder(&self) 
-> Result<Option<crate::block::DecoratedPot<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/DecoratedPot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHolder",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::DecoratedPot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for DecoratedPotInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting DecoratedPotInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for DecoratedPotInventory<'mc> {}
#[repr(C)]
pub struct BlockInventoryHolder<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockInventoryHolder<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockInventoryHolder<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockInventoryHolder from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/BlockInventoryHolder")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockInventoryHolder object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockInventoryHolderTrait<'mc> for BlockInventoryHolder<'mc> {}
pub trait BlockInventoryHolderTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the block associated with this holder.
	fn block(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for BlockInventoryHolder<'mc>{

fn into(self) -> crate::inventory::InventoryHolder<'mc> {

crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockInventoryHolder into crate::inventory::InventoryHolder")

   }
}
impl<'mc> crate::inventory::InventoryHolderTrait<'mc> for BlockInventoryHolder<'mc> {}
#[repr(C)]
pub struct ItemCraftResult<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ItemCraftResult<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ItemCraftResult<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ItemCraftResult from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ItemCraftResult")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ItemCraftResult object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ItemCraftResultTrait<'mc> for ItemCraftResult<'mc> {}
pub trait ItemCraftResultTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// The resulting {@link ItemStack} that was crafted.
	fn result(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the resulting matrix from the crafting operation.
	fn resulting_matrix(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResultingMatrix",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the overflowed items for items that don't fit back into the crafting
/// matrix.
	fn overflow_items(&self) 
-> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getOverflowItems",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(),obj,)?);
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
pub struct LlamaInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for LlamaInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for LlamaInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate LlamaInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/LlamaInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a LlamaInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> LlamaInventoryTrait<'mc> for LlamaInventory<'mc> {}
pub trait LlamaInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the item in the llama's decor slot.
	fn decor(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDecor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the item in the llama's decor slot.
	fn set_decor(&self,stack: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(stack.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDecor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::AbstractHorseInventory<'mc>> for LlamaInventory<'mc>{

fn into(self) -> crate::inventory::AbstractHorseInventory<'mc> {

crate::inventory::AbstractHorseInventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting LlamaInventory into crate::inventory::AbstractHorseInventory")

   }
}
impl<'mc> crate::inventory::AbstractHorseInventoryTrait<'mc> for LlamaInventory<'mc> {}
#[repr(C)]
pub struct EntityEquipment<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for EntityEquipment<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for EntityEquipment<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EntityEquipment from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/EntityEquipment")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityEquipment object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> EntityEquipmentTrait<'mc> for EntityEquipment<'mc> {}
pub trait EntityEquipmentTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Stores the ItemStack at the given equipment slot in the inventory.
	fn set_item(&self,slot: impl Into<crate::inventory::EquipmentSlot<'mc>>,item: impl Into<crate::inventory::ItemStack<'mc>>,silent: std::option::Option<bool>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(slot.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
args.push(val_2);
if let Some(a) = silent {
sig += "Z";
let val_3 = jni::objects::JValueGen::Bool(a.into());
args.push(val_3);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"setItem",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the ItemStack at the given equipment slot in the inventory.
	fn get_item(&self,slot: impl Into<crate::inventory::EquipmentSlot<'mc>>) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/EquipmentSlot;)Lorg/bukkit/inventory/ItemStack;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(slot.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets a copy of the item the entity is currently holding
/// in their main hand.
	fn item_in_main_hand(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemInMainHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the item the entity is holding in their main hand.
	fn set_item_in_main_hand(&self,item: impl Into<crate::inventory::ItemStack<'mc>>,silent: std::option::Option<bool>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = silent {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"setItemInMainHand",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets a copy of the item the entity is currently holding
/// in their off hand.
	fn item_in_off_hand(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemInOffHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the item the entity is holding in their off hand.
	fn set_item_in_off_hand(&self,item: impl Into<crate::inventory::ItemStack<'mc>>,silent: std::option::Option<bool>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = silent {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"setItemInOffHand",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Gets a copy of the item the entity is currently holding
	fn item_in_hand(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemInHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Sets the item the entity is holding
	fn set_item_in_hand(&self,stack: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(stack.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItemInHand",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets a copy of the helmet currently being worn by the entity
	fn helmet(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHelmet",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the helmet worn by the entity
	fn set_helmet(&self,helmet: impl Into<crate::inventory::ItemStack<'mc>>,silent: std::option::Option<bool>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(helmet.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = silent {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"setHelmet",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets a copy of the chest plate currently being worn by the entity
	fn chestplate(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChestplate",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the chest plate worn by the entity
	fn set_chestplate(&self,chestplate: impl Into<crate::inventory::ItemStack<'mc>>,silent: std::option::Option<bool>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(chestplate.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = silent {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"setChestplate",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets a copy of the leggings currently being worn by the entity
	fn leggings(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLeggings",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the leggings worn by the entity
	fn set_leggings(&self,leggings: impl Into<crate::inventory::ItemStack<'mc>>,silent: std::option::Option<bool>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(leggings.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = silent {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"setLeggings",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets a copy of the boots currently being worn by the entity
	fn boots(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBoots",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the boots worn by the entity
	fn set_boots(&self,boots: impl Into<crate::inventory::ItemStack<'mc>>,silent: std::option::Option<bool>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(boots.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = silent {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"setBoots",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets all ItemStacks from the armor slots.
	fn armor_contents(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getArmorContents",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the entities armor to the provided array of ItemStacks
	fn set_armor_contents(&self,items: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(items.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setArmorContents",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Clears the entity of all armor and held items
	fn clear(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"clear",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]

	fn item_in_hand_drop_chance(&self) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("()F");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemInHandDropChance",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.f()?
)}
#[deprecated]

	fn set_item_in_hand_drop_chance(&self,chance: f32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(F)V");
let val_1 = jni::objects::JValueGen::Float(chance);
let res = self.jni_ref().call_method(&self.jni_object(),"setItemInHandDropChance",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the chance of the main hand item being dropped upon this creature's
/// death.
/// <ul>
/// <li>A drop chance of 0.0F will never drop
/// <li>A drop chance of 1.0F will always drop
/// </ul>
	fn item_in_main_hand_drop_chance(&self) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("()F");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemInMainHandDropChance",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.f()?
)}
/// Sets the chance of the item this creature is currently holding in their
/// main hand being dropped upon this creature's death.
/// <ul>
/// <li>A drop chance of 0.0F will never drop
/// <li>A drop chance of 1.0F will always drop
/// </ul>
	fn set_item_in_main_hand_drop_chance(&self,chance: f32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(F)V");
let val_1 = jni::objects::JValueGen::Float(chance);
let res = self.jni_ref().call_method(&self.jni_object(),"setItemInMainHandDropChance",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the chance of the off hand item being dropped upon this creature's
/// death.
/// <ul>
/// <li>A drop chance of 0.0F will never drop
/// <li>A drop chance of 1.0F will always drop
/// </ul>
	fn item_in_off_hand_drop_chance(&self) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("()F");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemInOffHandDropChance",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.f()?
)}
/// Sets the chance of the off hand item being dropped upon this creature's
/// death.
/// <ul>
/// <li>A drop chance of 0.0F will never drop
/// <li>A drop chance of 1.0F will always drop
/// </ul>
	fn set_item_in_off_hand_drop_chance(&self,chance: f32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(F)V");
let val_1 = jni::objects::JValueGen::Float(chance);
let res = self.jni_ref().call_method(&self.jni_object(),"setItemInOffHandDropChance",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the chance of the helmet being dropped upon this creature's death.
/// <ul>
/// <li>A drop chance of 0.0F will never drop
/// <li>A drop chance of 1.0F will always drop
/// </ul>
	fn helmet_drop_chance(&self) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("()F");
let res = self.jni_ref().call_method(&self.jni_object(),"getHelmetDropChance",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.f()?
)}
/// Sets the chance of the helmet being dropped upon this creature's death.
/// <ul>
/// <li>A drop chance of 0.0F will never drop
/// <li>A drop chance of 1.0F will always drop
/// </ul>
	fn set_helmet_drop_chance(&self,chance: f32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(F)V");
let val_1 = jni::objects::JValueGen::Float(chance);
let res = self.jni_ref().call_method(&self.jni_object(),"setHelmetDropChance",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the chance of the chest plate being dropped upon this creature's
/// death.
/// <ul>
/// <li>A drop chance of 0.0F will never drop
/// <li>A drop chance of 1.0F will always drop
/// </ul>
	fn chestplate_drop_chance(&self) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("()F");
let res = self.jni_ref().call_method(&self.jni_object(),"getChestplateDropChance",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.f()?
)}
/// Sets the chance of the chest plate being dropped upon this creature's
/// death.
/// <ul>
/// <li>A drop chance of 0.0F will never drop
/// <li>A drop chance of 1.0F will always drop
/// </ul>
	fn set_chestplate_drop_chance(&self,chance: f32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(F)V");
let val_1 = jni::objects::JValueGen::Float(chance);
let res = self.jni_ref().call_method(&self.jni_object(),"setChestplateDropChance",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the chance of the leggings being dropped upon this creature's
/// death.
/// <ul>
/// <li>A drop chance of 0.0F will never drop
/// <li>A drop chance of 1.0F will always drop
/// </ul>
	fn leggings_drop_chance(&self) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("()F");
let res = self.jni_ref().call_method(&self.jni_object(),"getLeggingsDropChance",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.f()?
)}
/// Sets the chance of the leggings being dropped upon this creature's
/// death.
/// <ul>
/// <li>A drop chance of 0.0F will never drop
/// <li>A drop chance of 1.0F will always drop
/// </ul>
	fn set_leggings_drop_chance(&self,chance: f32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(F)V");
let val_1 = jni::objects::JValueGen::Float(chance);
let res = self.jni_ref().call_method(&self.jni_object(),"setLeggingsDropChance",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the chance of the boots being dropped upon this creature's death.
/// <ul>
/// <li>A drop chance of 0.0F will never drop
/// <li>A drop chance of 1.0F will always drop
/// </ul>
	fn boots_drop_chance(&self) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("()F");
let res = self.jni_ref().call_method(&self.jni_object(),"getBootsDropChance",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.f()?
)}
/// Sets the chance of the boots being dropped upon this creature's death.
/// <ul>
/// <li>A drop chance of 0.0F will never drop
/// <li>A drop chance of 1.0F will always drop
/// </ul>
	fn set_boots_drop_chance(&self,chance: f32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(F)V");
let val_1 = jni::objects::JValueGen::Float(chance);
let res = self.jni_ref().call_method(&self.jni_object(),"setBootsDropChance",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the entity this EntityEquipment belongs to
	fn holder(&self) 
-> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Entity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHolder",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BrewerInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BrewerInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BrewerInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BrewerInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/BrewerInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BrewerInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BrewerInventoryTrait<'mc> for BrewerInventory<'mc> {}
pub trait BrewerInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the current ingredient for brewing.
	fn ingredient(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getIngredient",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Set the current ingredient for brewing.
	fn set_ingredient(&self,ingredient: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ingredient.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setIngredient",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the current fuel for brewing.
	fn fuel(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFuel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Set the current fuel for brewing. Generally only
/// {@link Material#BLAZE_POWDER} will be of use.
	fn set_fuel(&self,fuel: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(fuel.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFuel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn holder(&self) 
-> Result<Option<crate::block::BrewingStand<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BrewingStand;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHolder",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::BrewingStand::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for BrewerInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BrewerInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for BrewerInventory<'mc> {}
#[repr(C)]
pub struct CookingRecipe<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CookingRecipe<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CookingRecipe<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CookingRecipe from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/CookingRecipe")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CookingRecipe object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CookingRecipeTrait<'mc> for CookingRecipe<'mc> {}
pub trait CookingRecipeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Create a cooking recipe to craft the specified ItemStack.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,key: impl Into<crate::NamespacedKey<'mc>>,result: impl Into<crate::inventory::ItemStack<'mc>>,input: impl Into<crate::inventory::RecipeChoice<'mc>>,experience: f32,cooking_time: i32) 
-> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/NamespacedKey;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/inventory/RecipeChoice;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(input.into().jni_object().clone())});
args.push(val_3);
sig += "F";
let val_4 = jni::objects::JValueGen::Float(experience);
args.push(val_4);
sig += "I";
let val_5 = jni::objects::JValueGen::Int(cooking_time);
args.push(val_5);
sig += ")V";
let cls = jni.find_class("org/bukkit/inventory/CookingRecipe"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::CookingRecipe::from_raw(&jni,res
)}
/// Sets the input of this cooking recipe.
	fn set_input(&self,input: impl Into<crate::Material<'mc>>) 
-> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Material;)Lorg/bukkit/inventory/CookingRecipe;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(input.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setInput",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::CookingRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the input material.
	fn input(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInput",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the input of this cooking recipe.
	fn set_input_choice(&self,input: impl Into<crate::inventory::RecipeChoice<'mc>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/RecipeChoice;)LT;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(input.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setInputChoice",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
/// Get the input choice.
	fn input_choice(&self) 
-> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/RecipeChoice;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInputChoice",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::RecipeChoice::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the result of this recipe.
	fn result(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the experience given by this recipe.
	fn set_experience(&self,experience: f32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(F)V");
let val_1 = jni::objects::JValueGen::Float(experience);
let res = self.jni_ref().call_method(&self.jni_object(),"setExperience",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the experience given by this recipe.
	fn experience(&self) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("()F");
let res = self.jni_ref().call_method(&self.jni_object(),"getExperience",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.f()?
)}
/// Set the cooking time for this recipe in ticks.
	fn set_cooking_time(&self,cooking_time: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(cooking_time);
let res = self.jni_ref().call_method(&self.jni_object(),"setCookingTime",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the cooking time for this recipe in ticks.
	fn cooking_time(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getCookingTime",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn key(&self) 
-> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/NamespacedKey;");
let res = self.jni_ref().call_method(&self.jni_object(),"getKey",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::NamespacedKey::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the group of this recipe. Recipes with the same group may be grouped
/// together when displayed in the client.
	fn group(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getGroup",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Set the group of this recipe. Recipes with the same group may be grouped
/// together when displayed in the client.
	fn set_group(&self,group: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(group.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setGroup",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the category which this recipe will appear in the recipe book under.
/// Defaults to {@link CookingBookCategory#MISC} if not set.
	fn category(&self) 
-> Result<crate::inventory::recipe::CookingBookCategory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/recipe/CookingBookCategory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCategory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::recipe::CookingBookCategory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the category which this recipe will appear in the recipe book under.
/// Defaults to {@link CookingBookCategory#MISC} if not set.
	fn set_category(&self,category: impl Into<crate::inventory::recipe::CookingBookCategory<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/recipe/CookingBookCategory;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(category.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setCategory",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Recipe<'mc>> for CookingRecipe<'mc>{

fn into(self) -> crate::inventory::Recipe<'mc> {

crate::inventory::Recipe::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CookingRecipe into crate::inventory::Recipe")

   }
}
impl<'mc> crate::inventory::RecipeTrait<'mc> for CookingRecipe<'mc> {}
impl<'mc> Into<crate::Keyed<'mc>> for CookingRecipe<'mc>{

fn into(self) -> crate::Keyed<'mc> {

crate::Keyed::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CookingRecipe into crate::Keyed")

   }
}
impl<'mc> crate::KeyedTrait<'mc> for CookingRecipe<'mc> {}
#[repr(C)]
pub struct CartographyInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CartographyInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CartographyInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CartographyInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/CartographyInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CartographyInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CartographyInventoryTrait<'mc> for CartographyInventory<'mc> {}
pub trait CartographyInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for CartographyInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CartographyInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for CartographyInventory<'mc> {}
#[repr(C)]
pub struct RecipeChoiceMaterialChoice<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RecipeChoiceMaterialChoice<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RecipeChoiceMaterialChoice<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RecipeChoiceMaterialChoice from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/RecipeChoice/MaterialChoice")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RecipeChoiceMaterialChoice object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RecipeChoiceMaterialChoiceTrait<'mc> for RecipeChoiceMaterialChoice<'mc> {}
pub trait RecipeChoiceMaterialChoiceTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,choices: Vec<jni::objects::JObject<'mc>>) 
-> Result<crate::inventory::RecipeChoiceMaterialChoice<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/util/List;";
let raw_val_1 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in choices{
sig += "Ljava/lang/java/lang/Object;";
		let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_1,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_1 = jni::objects::JValueGen::Object(raw_val_1);
args.push(val_1);
sig += ")V";
let cls = jni.find_class("org/bukkit/inventory/RecipeChoice/MaterialChoice"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::RecipeChoiceMaterialChoice::from_raw(&jni,res
)}

	fn test(&self,t: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(t.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"test",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn item_stack(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemStack",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn choices(&self) 
-> Result<Vec<crate::Material<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChoices",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::Material::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}

	fn clone(&self) 
-> Result<crate::inventory::RecipeChoiceMaterialChoice<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/RecipeChoice/MaterialChoice;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::RecipeChoiceMaterialChoice::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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

        impl<'mc> std::string::ToString for RecipeChoiceMaterialChoice<'mc> {
            fn to_string(&self) -> String {
                match RecipeChoiceMaterialChoiceTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling RecipeChoiceMaterialChoice.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::inventory::RecipeChoice<'mc>> for RecipeChoiceMaterialChoice<'mc>{

fn into(self) -> crate::inventory::RecipeChoice<'mc> {

crate::inventory::RecipeChoice::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting RecipeChoiceMaterialChoice into crate::inventory::RecipeChoice")

   }
}
impl<'mc> crate::inventory::RecipeChoiceTrait<'mc> for RecipeChoiceMaterialChoice<'mc> {}
#[repr(C)]
pub struct AbstractHorseInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for AbstractHorseInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for AbstractHorseInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate AbstractHorseInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/AbstractHorseInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a AbstractHorseInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> AbstractHorseInventoryTrait<'mc> for AbstractHorseInventory<'mc> {}
pub trait AbstractHorseInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the item in the horse's saddle slot.
	fn saddle(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSaddle",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the item in the horse's saddle slot.
	fn set_saddle(&self,stack: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(stack.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setSaddle",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for AbstractHorseInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting AbstractHorseInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for AbstractHorseInventory<'mc> {}
#[repr(C)]
pub struct RecipeChoice<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RecipeChoice<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RecipeChoice<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RecipeChoice from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/RecipeChoice")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RecipeChoice object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RecipeChoiceTrait<'mc> for RecipeChoice<'mc> {}
pub trait RecipeChoiceTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]
/// Gets a single item stack representative of this stack choice.
	fn item_stack(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemStack",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn clone(&self) 
-> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/RecipeChoice;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::RecipeChoice::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn test(&self,item_stack: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item_stack.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"test",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
impl<'mc> Into<blackboxmc_java::util::function::JavaPredicate<'mc>> for RecipeChoice<'mc>{

fn into(self) -> blackboxmc_java::util::function::JavaPredicate<'mc> {

blackboxmc_java::util::function::JavaPredicate::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting RecipeChoice into blackboxmc_java::util::function::JavaPredicate")

   }
}
#[repr(C)]
pub struct Inventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Inventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Inventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Inventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/Inventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Inventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> InventoryTrait<'mc> for Inventory<'mc> {}
pub trait InventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns the size of the inventory
	fn size(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getSize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Returns the maximum stack size for an ItemStack in this inventory.
	fn max_stack_size(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxStackSize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// This method allows you to change the maximum stack size for an
/// inventory.
/// 
/// <b>Caveats:</b>
/// <ul>
/// <li>Not all inventories respect this value.
/// <li>Stacks larger than 127 may be clipped when the world is saved.
/// <li>This value is not guaranteed to be preserved; be sure to set it
/// before every time you want to set a slot over the max stack size.
/// <li>Stacks larger than the default max size for this type of inventory
/// may not display correctly in the client.
/// </ul>
	fn set_max_stack_size(&self,size: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(size);
let res = self.jni_ref().call_method(&self.jni_object(),"setMaxStackSize",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Returns the ItemStack found in the slot at the given index
	fn get_item(&self,index: i32) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
let val_1 = jni::objects::JValueGen::Int(index);
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Stores the ItemStack at the given index of the inventory.
	fn set_item(&self,index: i32,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Int(index);
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Stores the given ItemStacks in the inventory. This will try to fill
/// existing stacks and empty slots as well as it can.
/// 
/// The returned HashMap contains what it couldn't store, where the key is
/// the index of the parameter, and the value is the ItemStack at that
/// index of the varargs parameter. If all items are stored, it will return
/// an empty HashMap.
/// 
/// If you pass in ItemStacks which exceed the maximum stack size for the
/// Material, first they will be added to partial stacks where
/// Material.getMaxStackSize() is not exceeded, up to
/// Material.getMaxStackSize(). When there are no partial stacks left
/// stacks will be split on Inventory.getMaxStackSize() allowing you to
/// exceed the maximum stack size for that material.
/// 
/// It is known that in some implementations this method will also set
/// the inputted argument amount to the number of that item not placed in
/// slots.
	fn add_item(&self,items: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(items.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Removes the given ItemStacks from the inventory.
/// 
/// It will try to remove 'as much as possible' from the types and amounts
/// you give as arguments.
/// 
/// The returned HashMap contains what it couldn't remove, where the key is
/// the index of the parameter, and the value is the ItemStack at that
/// index of the varargs parameter. If all the given ItemStacks are
/// removed, it will return an empty HashMap.
/// 
/// It is known that in some implementations this method will also set the
/// inputted argument amount to the number of that item not removed from
/// slots.
	fn remove_item(&self,items: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Ljava/util/HashMap;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(items.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removeItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns all ItemStacks from the inventory
	fn contents(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getContents",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Completely replaces the inventory's contents. Removes all existing
/// contents and replaces it with the ItemStacks given in the array.
	fn set_contents(&self,items: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(items.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setContents",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Return the contents from the section of the inventory where items can
/// reasonably be expected to be stored. In most cases this will represent
/// the entire inventory, but in some cases it may exclude armor or result
/// slots.
/// 
/// It is these contents which will be used for add / contains / remove
/// methods which look for a specific stack.
	fn storage_contents(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getStorageContents",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Put the given ItemStacks into the storage slots
	fn set_storage_contents(&self,items: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(items.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setStorageContents",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks if the inventory contains any ItemStacks with the given
/// material, adding to at least the minimum amount specified.
	fn contains(&self,material: impl Into<crate::Material<'mc>>,amount: std::option::Option<i32>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(material.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = amount {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a);
args.push(val_2);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"contains",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks if the inventory contains ItemStacks matching the given
/// ItemStack whose amounts sum to at least the minimum amount specified.
	fn contains_at_least(&self,item: impl Into<crate::inventory::ItemStack<'mc>>,amount: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;I)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(amount);
let res = self.jni_ref().call_method(&self.jni_object(),"containsAtLeast",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Finds all slots in the inventory containing any ItemStacks with the
/// given ItemStack. This will only match slots if both the type and the
/// amount of the stack match
/// 
/// The HashMap contains entries where, the key is the slot index, and the
/// value is the ItemStack in that slot. If no matching ItemStack with the
/// given Material is found, an empty map is returned.
	fn all(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<blackboxmc_java::util::JavaHashMap<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
args.push(val_1);
sig += ")Ljava/util/HashMap;";
let res = self.jni_ref().call_method(&self.jni_object(),"all",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaHashMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns the first slot in the inventory containing an ItemStack with
/// the given stack. This will only match a slot if both the type and the
/// amount of the stack match
	fn first(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
args.push(val_1);
sig += ")I";
let res = self.jni_ref().call_method(&self.jni_object(),"first",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Returns the first empty Slot.
	fn first_empty(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"firstEmpty",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Check whether or not this inventory is empty. An inventory is considered
/// to be empty if there are no ItemStacks in any slot of this inventory.
	fn is_empty(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Removes all stacks in the inventory matching the given stack.
/// 
/// This will only match a slot if both the type and the amount of the
/// stack match
	fn remove(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
args.push(val_1);
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"remove",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Clears out a particular slot in the index.
	fn clear(&self,index: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = index {
sig += "I";
let val_1 = jni::objects::JValueGen::Int(a);
args.push(val_1);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"clear",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets a list of players viewing the inventory. Note that a player is
/// considered to be viewing their own inventory and internal crafting
/// screen even when said inventory is not open. They will normally be
/// considered to be viewing their inventory even when they have a
/// different inventory screen open, but it's possible for customized
/// inventory screens to exclude the viewer's inventory, so this should
/// never be assumed to be non-empty.
	fn viewers(&self) 
-> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getViewers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::entity::HumanEntity::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Returns what type of inventory this is.
	fn get_type(&self) 
-> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::inventory::InventoryType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the block or entity belonging to the open inventory
	fn holder(&self) 
-> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/InventoryHolder;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHolder",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::InventoryHolder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn iterator(&self) 
-> Result<blackboxmc_java::util::JavaListIterator<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/ListIterator;");
let res = self.jni_ref().call_method(&self.jni_object(),"iterator",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaListIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the location of the block or entity which corresponds to this inventory. May return null if this container
/// was custom created or is a virtual / subcontainer.
	fn location(&self) 
-> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLocation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct SmithingTransformRecipe<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SmithingTransformRecipe<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SmithingTransformRecipe<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SmithingTransformRecipe from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/SmithingTransformRecipe")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SmithingTransformRecipe object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SmithingTransformRecipeTrait<'mc> for SmithingTransformRecipe<'mc> {}
pub trait SmithingTransformRecipeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Create a smithing recipe to produce the specified result ItemStack.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,key: impl Into<crate::NamespacedKey<'mc>>,result: impl Into<crate::inventory::ItemStack<'mc>>,template: impl Into<crate::inventory::RecipeChoice<'mc>>,base: impl Into<crate::inventory::RecipeChoice<'mc>>,addition: impl Into<crate::inventory::RecipeChoice<'mc>>) 
-> Result<crate::inventory::SmithingTransformRecipe<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/RecipeChoice;Lorg/bukkit/inventory/RecipeChoice;Lorg/bukkit/inventory/RecipeChoice;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(template.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(base.into().jni_object().clone())});
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(addition.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/inventory/SmithingTransformRecipe"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::SmithingTransformRecipe::from_raw(&jni,res
)}
/// Get the template recipe item.
	fn template(&self) 
-> Result<crate::inventory::RecipeChoice<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/RecipeChoice;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTemplate",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::RecipeChoice::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::SmithingRecipe<'mc>> for SmithingTransformRecipe<'mc>{

fn into(self) -> crate::inventory::SmithingRecipe<'mc> {

crate::inventory::SmithingRecipe::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SmithingTransformRecipe into crate::inventory::SmithingRecipe")

   }
}
impl<'mc> crate::inventory::SmithingRecipeTrait<'mc> for SmithingTransformRecipe<'mc> {}
pub enum CreativeCategory<'mc> {
	BuildingBlocks {inner: CreativeCategoryStruct<'mc>},
	Decorations {inner: CreativeCategoryStruct<'mc>},
	Redstone {inner: CreativeCategoryStruct<'mc>},
	Transportation {inner: CreativeCategoryStruct<'mc>},
	Misc {inner: CreativeCategoryStruct<'mc>},
	Food {inner: CreativeCategoryStruct<'mc>},
	Tools {inner: CreativeCategoryStruct<'mc>},
	Combat {inner: CreativeCategoryStruct<'mc>},
	Brewing {inner: CreativeCategoryStruct<'mc>},
}
impl<'mc> std::fmt::Display for CreativeCategory<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           CreativeCategory::BuildingBlocks { .. } => f.write_str("BUILDING_BLOCKS"),
           CreativeCategory::Decorations { .. } => f.write_str("DECORATIONS"),
           CreativeCategory::Redstone { .. } => f.write_str("REDSTONE"),
           CreativeCategory::Transportation { .. } => f.write_str("TRANSPORTATION"),
           CreativeCategory::Misc { .. } => f.write_str("MISC"),
           CreativeCategory::Food { .. } => f.write_str("FOOD"),
           CreativeCategory::Tools { .. } => f.write_str("TOOLS"),
           CreativeCategory::Combat { .. } => f.write_str("COMBAT"),
           CreativeCategory::Brewing { .. } => f.write_str("BREWING"),
       }
   }
}

        impl<'mc> CreativeCategoryTrait<'mc> for CreativeCategory<'mc> {}
        
        pub trait CreativeCategoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<CreativeCategory<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/inventory/CreativeCategory");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/inventory/CreativeCategory;",
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
                    
"BUILDING_BLOCKS" => Ok(CreativeCategory::BuildingBlocks { inner: CreativeCategoryStruct::from_raw(env,obj)?}),
"DECORATIONS" => Ok(CreativeCategory::Decorations { inner: CreativeCategoryStruct::from_raw(env,obj)?}),
"REDSTONE" => Ok(CreativeCategory::Redstone { inner: CreativeCategoryStruct::from_raw(env,obj)?}),
"TRANSPORTATION" => Ok(CreativeCategory::Transportation { inner: CreativeCategoryStruct::from_raw(env,obj)?}),
"MISC" => Ok(CreativeCategory::Misc { inner: CreativeCategoryStruct::from_raw(env,obj)?}),
"FOOD" => Ok(CreativeCategory::Food { inner: CreativeCategoryStruct::from_raw(env,obj)?}),
"TOOLS" => Ok(CreativeCategory::Tools { inner: CreativeCategoryStruct::from_raw(env,obj)?}),
"COMBAT" => Ok(CreativeCategory::Combat { inner: CreativeCategoryStruct::from_raw(env,obj)?}),
"BREWING" => Ok(CreativeCategory::Brewing { inner: CreativeCategoryStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct CreativeCategoryStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CreativeCategory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::BuildingBlocks { inner } => inner.0.clone(),
Self::Decorations { inner } => inner.0.clone(),
Self::Redstone { inner } => inner.0.clone(),
Self::Transportation { inner } => inner.0.clone(),
Self::Misc { inner } => inner.0.clone(),
Self::Food { inner } => inner.0.clone(),
Self::Tools { inner } => inner.0.clone(),
Self::Combat { inner } => inner.0.clone(),
Self::Brewing { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::BuildingBlocks { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Decorations { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Redstone { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Transportation { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Misc { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Food { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Tools { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Combat { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Brewing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for CreativeCategory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CreativeCategory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/CreativeCategory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CreativeCategory object, got {}",
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
                    "BUILDING_BLOCKS" => Ok(CreativeCategory::BuildingBlocks { inner: CreativeCategoryStruct::from_raw(env,obj)?}),"DECORATIONS" => Ok(CreativeCategory::Decorations { inner: CreativeCategoryStruct::from_raw(env,obj)?}),"REDSTONE" => Ok(CreativeCategory::Redstone { inner: CreativeCategoryStruct::from_raw(env,obj)?}),"TRANSPORTATION" => Ok(CreativeCategory::Transportation { inner: CreativeCategoryStruct::from_raw(env,obj)?}),"MISC" => Ok(CreativeCategory::Misc { inner: CreativeCategoryStruct::from_raw(env,obj)?}),"FOOD" => Ok(CreativeCategory::Food { inner: CreativeCategoryStruct::from_raw(env,obj)?}),"TOOLS" => Ok(CreativeCategory::Tools { inner: CreativeCategoryStruct::from_raw(env,obj)?}),"COMBAT" => Ok(CreativeCategory::Combat { inner: CreativeCategoryStruct::from_raw(env,obj)?}),"BREWING" => Ok(CreativeCategory::Brewing { inner: CreativeCategoryStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for CreativeCategoryStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CreativeCategoryStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CreativeCategoryStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/CreativeCategory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CreativeCategoryStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CreativeCategoryStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::inventory::CreativeCategory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/CreativeCategory;");
let cls = jni.find_class("org/bukkit/inventory/CreativeCategory"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::inventory::CreativeCategory::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct LoomInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for LoomInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for LoomInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate LoomInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/LoomInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a LoomInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> LoomInventoryTrait<'mc> for LoomInventory<'mc> {}
pub trait LoomInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for LoomInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting LoomInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for LoomInventory<'mc> {}
#[repr(C)]
pub struct EnchantingInventory<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for EnchantingInventory<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for EnchantingInventory<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EnchantingInventory from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/EnchantingInventory")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EnchantingInventory object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> EnchantingInventoryTrait<'mc> for EnchantingInventory<'mc> {}
pub trait EnchantingInventoryTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Set the item being enchanted.
	fn set_item(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the item being enchanted.
	fn item(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Set the secondary item being used for the enchant.
	fn set_secondary(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setSecondary",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the secondary item being used for the enchant.
	fn secondary(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSecondary",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::Inventory<'mc>> for EnchantingInventory<'mc>{

fn into(self) -> crate::inventory::Inventory<'mc> {

crate::inventory::Inventory::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting EnchantingInventory into crate::inventory::Inventory")

   }
}
impl<'mc> crate::inventory::InventoryTrait<'mc> for EnchantingInventory<'mc> {}
#[repr(C)]
pub struct ShapelessRecipe<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ShapelessRecipe<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ShapelessRecipe<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ShapelessRecipe from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/ShapelessRecipe")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ShapelessRecipe object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ShapelessRecipeTrait<'mc> for ShapelessRecipe<'mc> {}
pub trait ShapelessRecipeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Create a shapeless recipe to craft the specified ItemStack. The
/// constructor merely determines the result and type; to set the actual
/// recipe, you'll need to call the appropriate methods.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,key: impl Into<crate::NamespacedKey<'mc>>,result: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>) 
-> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/NamespacedKey;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = result {
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/inventory/ShapelessRecipe"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::inventory::ShapelessRecipe::from_raw(&jni,res
)}
#[deprecated]
/// Adds multiples of the specified ingredient.
	fn add_ingredient(&self,count: i32,ingredient: std::option::Option<impl Into<crate::Material<'mc>>>,rawdata: std::option::Option<i32>) 
-> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "I";
let val_1 = jni::objects::JValueGen::Int(count);
args.push(val_1);
if let Some(a) = ingredient {
sig += "Lorg/bukkit/Material;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
if let Some(a) = rawdata {
sig += "I";
let val_3 = jni::objects::JValueGen::Int(a);
args.push(val_3);
}
sig += ")Lorg/bukkit/inventory/ShapelessRecipe;";
let res = self.jni_ref().call_method(&self.jni_object(),"addIngredient",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ShapelessRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Removes an ingredient from the list. If the ingredient occurs multiple times, only one instance of it is removed. If the data value is -1, only ingredients with a -1 data value will be removed.
	fn remove_ingredient(&self,ingredient: impl Into<crate::Material<'mc>>,rawdata: std::option::Option<i32>) 
-> Result<crate::inventory::ShapelessRecipe<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ingredient.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = rawdata {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a);
args.push(val_2);
}
sig += ")Lorg/bukkit/inventory/ShapelessRecipe;";
let res = self.jni_ref().call_method(&self.jni_object(),"removeIngredient",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ShapelessRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the list of ingredients used for this recipe.
	fn ingredient_list(&self) 
-> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getIngredientList",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}

	fn choice_list(&self) 
-> Result<Vec<crate::inventory::RecipeChoice<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChoiceList",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::inventory::RecipeChoice::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::CraftingRecipe<'mc>> for ShapelessRecipe<'mc>{

fn into(self) -> crate::inventory::CraftingRecipe<'mc> {

crate::inventory::CraftingRecipe::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ShapelessRecipe into crate::inventory::CraftingRecipe")

   }
}
impl<'mc> crate::inventory::CraftingRecipeTrait<'mc> for ShapelessRecipe<'mc> {}
pub mod recipe;
pub mod meta;
