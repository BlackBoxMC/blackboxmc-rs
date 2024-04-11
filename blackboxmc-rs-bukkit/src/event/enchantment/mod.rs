#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct PrepareItemEnchantEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PrepareItemEnchantEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PrepareItemEnchantEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PrepareItemEnchantEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/enchantment/PrepareItemEnchantEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PrepareItemEnchantEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PrepareItemEnchantEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,enchanter: impl Into<crate::entity::Player<'mc>>,view: impl Into<crate::inventory::InventoryView<'mc>>,table: impl Into<crate::block::Block<'mc>>,item: impl Into<crate::inventory::ItemStack<'mc>>,offers: impl Into<crate::enchantments::EnchantmentOffer<'mc>>,bonus: i32) 
-> Result<crate::event::enchantment::PrepareItemEnchantEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/enchantments/EnchantmentOffer;I)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(enchanter.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(view.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(table.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(offers.into().jni_object().clone())});
let val_6 = jni::objects::JValueGen::Int(bonus);
let cls = jni.find_class("org/bukkit/event/enchantment/PrepareItemEnchantEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5),jni::objects::JValueGen::from(val_6)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::enchantment::PrepareItemEnchantEvent::from_raw(&jni,res
)}
	pub fn enchanter(&self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::entity::Player;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEnchanter",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn enchant_block(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEnchantBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn exp_level_costs_offered(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getExpLevelCostsOffered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn offers(&self) 
-> Result<crate::enchantments::EnchantmentOffer<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::enchantments::EnchantmentOffer;");
let res = self.jni_ref().call_method(&self.jni_object(),"getOffers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::enchantments::EnchantmentOffer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn enchantment_bonus(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEnchantmentBonus",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/enchantment/PrepareItemEnchantEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: InventoryEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PrepareItemEnchantEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting PrepareItemEnchantEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for PrepareItemEnchantEvent<'mc>{

fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {

crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting PrepareItemEnchantEvent into crate::event::inventory::InventoryEvent")

   }
}
#[repr(C)]
pub struct EnchantItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for EnchantItemEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for EnchantItemEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EnchantItemEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/enchantment/EnchantItemEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EnchantItemEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> EnchantItemEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,enchanter: impl Into<crate::entity::Player<'mc>>,view: impl Into<crate::inventory::InventoryView<'mc>>,table: impl Into<crate::block::Block<'mc>>,item: impl Into<crate::inventory::ItemStack<'mc>>,level: i32,enchants: impl Into<blackboxmc_java::util::JavaMap<'mc>>,enchantment_hint: impl Into<crate::enchantments::Enchantment<'mc>>,level_hint: i32,i: i32) 
-> Result<crate::event::enchantment::EnchantItemEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;ILjava/util/Map;Lorg/bukkit/enchantments/Enchantment;II)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(enchanter.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(view.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(table.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let val_5 = jni::objects::JValueGen::Int(level);
let val_6 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(enchants.into().jni_object().clone())});
let val_7 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(enchantment_hint.into().jni_object().clone())});
let val_8 = jni::objects::JValueGen::Int(level_hint);
let val_9 = jni::objects::JValueGen::Int(i);
let cls = jni.find_class("org/bukkit/event/enchantment/EnchantItemEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5),jni::objects::JValueGen::from(val_6),jni::objects::JValueGen::from(val_7),jni::objects::JValueGen::from(val_8),jni::objects::JValueGen::from(val_9)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::enchantment::EnchantItemEvent::from_raw(&jni,res
)}
	pub fn enchanter(&self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::entity::Player;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEnchanter",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn enchant_block(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEnchantBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn exp_level_cost(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getExpLevelCost",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn set_exp_level_cost(&self,level: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(level);
let res = self.jni_ref().call_method(&self.jni_object(),"setExpLevelCost",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn enchants_to_add(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEnchantsToAdd",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn enchantment_hint(&self) 
-> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::enchantments::Enchantment;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEnchantmentHint",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::enchantments::Enchantment::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn level_hint(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLevelHint",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn which_button(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"whichButton",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/enchantment/EnchantItemEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: InventoryEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EnchantItemEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting EnchantItemEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for EnchantItemEvent<'mc>{

fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {

crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EnchantItemEvent into crate::event::inventory::InventoryEvent")

   }
}
