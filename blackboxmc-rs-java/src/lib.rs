#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub struct JavaCalendarBuilder<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaCalendarBuilder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaCalendarBuilder<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaCalendarBuilder from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaCalendarBuilder")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaCalendarBuilder object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaCalendarBuilder<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Calendar$Builder")?;
let res = jni.new_object(cls,
"()V",&[])?;
crate::JavaCalendarBuilder::from_raw(&jni,res
)}
	pub fn set_time_zone(&mut self,arg0: impl Into<&'mc crate::JavaTimeZone<'mc>>) 
-> Result<crate::JavaCalendarBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setTimeZone","(Ljava/util/TimeZone;)Ljava/util/Calendar$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCalendarBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_week_date(&mut self,arg0: i32,arg1: i32,arg2: i32) 
-> Result<crate::JavaCalendarBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setWeekDate","(III)Ljava/util/Calendar$Builder;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCalendarBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_instant_with_long(&mut self,arg0: std::option::Option<impl Into<&'mc crate::JavaDate<'mc>>>) 
-> Result<crate::JavaCalendarBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setInstant","(Ljava/util/Date;)Ljava/util/Calendar$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCalendarBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_fields(&mut self,arg0: Vec<i32>) 
-> Result<crate::JavaCalendarBuilder<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"setFields","(I)Ljava/util/Calendar$Builder;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCalendarBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_time_of_day_with_int(&mut self,arg0: i32,arg1: i32,arg2: std::option::Option<i32>,arg3: std::option::Option<i32>) 
-> Result<crate::JavaCalendarBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"setTimeOfDay","(IIII)Ljava/util/Calendar$Builder;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCalendarBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_week_definition(&mut self,arg0: i32,arg1: i32) 
-> Result<crate::JavaCalendarBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setWeekDefinition","(II)Ljava/util/Calendar$Builder;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCalendarBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_date(&mut self,arg0: i32,arg1: i32,arg2: i32) 
-> Result<crate::JavaCalendarBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDate","(III)Ljava/util/Calendar$Builder;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCalendarBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_calendar_type(&mut self,arg0: impl Into<&'mc String>) 
-> Result<crate::JavaCalendarBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setCalendarType","(Ljava/lang/String;)Ljava/util/Calendar$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCalendarBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_locale(&mut self,arg0: impl Into<&'mc crate::JavaLocale<'mc>>) 
-> Result<crate::JavaCalendarBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setLocale","(Ljava/util/Locale;)Ljava/util/Calendar$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCalendarBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn build(&mut self) 
-> Result<crate::JavaCalendar<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"build","()Ljava/util/Calendar;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCalendar::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_lenient(&mut self,arg0: bool) 
-> Result<crate::JavaCalendarBuilder<'mc>, Box<dyn std::error::Error>>

{// -2
let val_1 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setLenient","(Z)Ljava/util/Calendar$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCalendarBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set(&mut self,arg0: i32,arg1: i32) 
-> Result<crate::JavaCalendarBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"set","(II)Ljava/util/Calendar$Builder;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCalendarBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaArrayList<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> blackboxmc_general::JNIRaw<'mc> for JavaArrayList<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> JavaArrayList<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaArrayList from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaArrayList")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaArrayList object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i32>) 
-> Result<crate::JavaArrayList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let cls = &jni.find_class("java/util/ArrayList")?;
let res = jni.new_object(cls,
"(I)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaArrayList::from_raw(&jni,res
)}
	pub fn add_with_object(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<E>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = arg1.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(ILE;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_with_object(&mut self,arg0: std::option::Option<i32>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(I)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn get(&mut self,arg0: i32) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"get","(I)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn index_of(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"indexOf","(Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn last_index_of(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"lastIndexOf","(Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn sub_list(&mut self,arg0: i32,arg1: i32) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"subList","(II)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_all_with_collection(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<impl Into<&'mc crate::JavaCollection<'mc, E>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(ILjava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set(&mut self,arg0: i32,arg1: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"set","(ILE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn ensure_capacity(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"ensureCapacity","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn trim_to_size(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"trimToSize","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn sort(&mut self,arg0: impl Into<&'mc crate::JavaComparator<'mc, E>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"sort","(Ljava/util/Comparator;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn list_iterator(&mut self,arg0: std::option::Option<i32>) 
-> Result<crate::JavaListIterator<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"listIterator","(I)Ljava/util/ListIterator;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaListIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, E> Into<crate::JavaList<'mc, E>> for JavaArrayList<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaList<'mc, E> {
       crate::JavaList::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, E> Into<crate::JavaRandomAccess<'mc, E>> for JavaArrayList<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaRandomAccess<'mc, E> {
       crate::JavaRandomAccess::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, E> Into<crate::JavaAbstractList<'mc, E>> for JavaArrayList<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractList<'mc, E> {
       crate::JavaAbstractList::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaPropertyResourceBundle<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaPropertyResourceBundle<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaPropertyResourceBundle<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaPropertyResourceBundle from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaPropertyResourceBundle")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaPropertyResourceBundle object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn keys(&mut self) 
-> Result<crate::JavaEnumeration<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getKeys","()Ljava/util/Enumeration;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaEnumeration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handle_get_object(&mut self,arg0: impl Into<&'mc String>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"handleGetObject","(Ljava/lang/String;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn get_string(&mut self,arg0: impl Into<&'mc String>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"getString","(Ljava/lang/String;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn locale(&mut self) 
-> Result<crate::JavaLocale<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLocale","()Ljava/util/Locale;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocale::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn base_bundle_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBaseBundleName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn contains_key(&mut self,arg0: impl Into<&'mc String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"containsKey","(Ljava/lang/String;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn key_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_object(&mut self,arg0: impl Into<&'mc String>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"getObject","(Ljava/lang/String;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::JavaResourceBundle<'mc>> for JavaPropertyResourceBundle<'mc>{
   fn into(self) -> crate::JavaResourceBundle<'mc> {
       crate::JavaResourceBundle::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaAbstractMap<'mc, K,V>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where K: JNIRaw<'mc>,V: JNIRaw<'mc>;
impl<'mc, K,V> blackboxmc_general::JNIRaw<'mc> for JavaAbstractMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, K,V> JavaAbstractMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaAbstractMap from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaAbstractMap")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaAbstractMap object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn remove_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = arg1.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"get","(Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn put(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"put","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn values(&mut self) 
-> Result<crate::JavaCollection<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"values","()Ljava/util/Collection;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCollection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn entry_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"entrySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn put_all(&mut self,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"putAll","(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn contains_key(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsKey","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn key_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_value(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsValue","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn replace_with_object(&mut self,arg0: K,arg1: std::option::Option<V>,arg2: std::option::Option<V>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
let val_2 = arg1.unwrap().jni_object();
let val_3 = arg2.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"replace","(LK;LV;LV;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn put_if_absent(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"putIfAbsent","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn get_or_default(&mut self,arg0: jni::objects::JObject<'mc>,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"getOrDefault","(Ljava/lang/Object;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
}
impl<'mc, K,V> Into<crate::JavaMap<'mc, K,V>> for JavaAbstractMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc>{
   fn into(self) -> crate::JavaMap<'mc, K,V> {
       crate::JavaMap::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaTreeSet<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> blackboxmc_general::JNIRaw<'mc> for JavaTreeSet<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> JavaTreeSet<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaTreeSet from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaTreeSet")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaTreeSet object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaSortedSet<'mc, E>>>) 
-> Result<crate::JavaTreeSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/TreeSet")?;
let res = jni.new_object(cls,
"(Ljava/util/SortedSet;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaTreeSet::from_raw(&jni,res
)}
	pub fn new_with_comparator(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, E>>>) 
-> Result<crate::JavaTreeSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/TreeSet")?;
let res = jni.new_object(cls,
"(Ljava/util/Comparator;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaTreeSet::from_raw(&jni,res
)}
	pub fn poll_first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pollFirst","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn poll_last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pollLast","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn descending_iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"descendingIterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn sub_set_with_object(&mut self,arg0: E,arg1: std::option::Option<bool>,arg2: std::option::Option<E>,arg3: std::option::Option<bool>) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
// 1
let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
let val_3 = arg2.unwrap().jni_object();
// 1
let val_4 = jni::objects::JValueGen::Bool(arg3.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"subSet","(LE;ZLE;Z)Ljava/util/NavigableSet;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn head_set_with_object(&mut self,arg0: std::option::Option<E>,arg1: std::option::Option<bool>) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
// 0
let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"headSet","(LE;Z)Ljava/util/NavigableSet;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn tail_set_with_object(&mut self,arg0: std::option::Option<E>,arg1: std::option::Option<bool>) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
// 0
let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"tailSet","(LE;Z)Ljava/util/NavigableSet;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn descending_set(&mut self) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"descendingSet","()Ljava/util/NavigableSet;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn ceiling(&mut self,arg0: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"ceiling","(LE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn higher(&mut self,arg0: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"higher","(LE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn add(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn floor(&mut self,arg0: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"floor","(LE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"last","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"first","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn lower(&mut self,arg0: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"lower","(LE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"comparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, E> Into<crate::JavaNavigableSet<'mc, E>> for JavaTreeSet<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaNavigableSet<'mc, E> {
       crate::JavaNavigableSet::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, E> Into<crate::JavaAbstractSet<'mc, E>> for JavaTreeSet<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractSet<'mc, E> {
       crate::JavaAbstractSet::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaCurrency<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaCurrency<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaCurrency<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaCurrency from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaCurrency")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaCurrency object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn display_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn symbol(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSymbol","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn available_currencies(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Set")?;
let res = jni.call_static_method(cls,"getAvailableCurrencies",
"()Ljava/util/Set;",&[])?;
let mut obj = res.l()?;
crate::JavaSet::from_raw(&jni,obj
)}
	pub fn currency_code(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCurrencyCode","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn default_fraction_digits(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultFractionDigits","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn numeric_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNumericCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn numeric_code_as_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNumericCodeAsString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn get_instance_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaLocale<'mc>>>) 
-> Result<crate::JavaCurrency<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/Currency")?;
let res = jni.call_static_method(cls,"getInstance",
"(Ljava/util/Locale;)Ljava/util/Currency;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaCurrency::from_raw(&jni,obj
)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaVector<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> blackboxmc_general::JNIRaw<'mc> for JavaVector<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> JavaVector<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaVector from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaVector")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaVector object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaCollection<'mc, E>>>) 
-> Result<crate::JavaVector<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/Vector")?;
let res = jni.new_object(cls,
"(Ljava/util/Collection;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaVector::from_raw(&jni,res
)}
	pub fn new_with_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: std::option::Option<i32>) 
-> Result<crate::JavaVector<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let cls = &jni.find_class("java/util/Vector")?;
let res = jni.new_object(cls,
"(II)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::JavaVector::from_raw(&jni,res
)}
	pub fn copy_into(&mut self,arg0: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"copyInto","(Ljava/lang/Object;)V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_size(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSize","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_element_at(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"removeElementAt","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_element(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"removeElement","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn insert_element_at(&mut self,arg0: E,arg1: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"insertElementAt","(LE;I)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_all_elements(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"removeAllElements","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn first_element(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"firstElement","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn last_element(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"lastElement","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn set_element_at(&mut self,arg0: E,arg1: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setElementAt","(LE;I)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn add_element(&mut self,arg0: E) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"addElement","(LE;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn add_with_object(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<E>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = arg1.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(ILE;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_with_int(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: i32) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"get","(I)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn index_of_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"indexOf","(Ljava/lang/Object;I)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn last_index_of_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"lastIndexOf","(Ljava/lang/Object;I)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn sub_list(&mut self,arg0: i32,arg1: i32) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"subList","(II)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn elements(&mut self) 
-> Result<crate::JavaEnumeration<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"elements","()Ljava/util/Enumeration;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaEnumeration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_all_with_collection(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<impl Into<&'mc crate::JavaCollection<'mc, E>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(ILjava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set(&mut self,arg0: i32,arg1: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"set","(ILE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn capacity(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"capacity","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn ensure_capacity(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"ensureCapacity","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn trim_to_size(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"trimToSize","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn element_at(&mut self,arg0: i32) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"elementAt","(I)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn sort(&mut self,arg0: impl Into<&'mc crate::JavaComparator<'mc, E>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"sort","(Ljava/util/Comparator;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn list_iterator(&mut self,arg0: std::option::Option<i32>) 
-> Result<crate::JavaListIterator<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"listIterator","(I)Ljava/util/ListIterator;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaListIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, E> Into<crate::JavaList<'mc, E>> for JavaVector<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaList<'mc, E> {
       crate::JavaList::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, E> Into<crate::JavaRandomAccess<'mc, E>> for JavaVector<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaRandomAccess<'mc, E> {
       crate::JavaRandomAccess::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, E> Into<crate::JavaAbstractList<'mc, E>> for JavaVector<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractList<'mc, E> {
       crate::JavaAbstractList::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaEnumMap<'mc, K>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where K: JNIRaw<'mc>;
impl<'mc, K> blackboxmc_general::JNIRaw<'mc> for JavaEnumMap<'mc, K> where K: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, K> JavaEnumMap<'mc, K> where K: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaEnumMap from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaEnumMap")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaEnumMap object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_enum_map(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaMap<'mc, K,V>>>) 
-> Result<crate::JavaEnumMap<'mc, K>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/EnumMap")?;
let res = jni.new_object(cls,
"(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaEnumMap::from_raw(&jni,res
)}
	pub fn new_with_class(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>) 
-> Result<crate::JavaEnumMap<'mc, K>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let cls = &jni.find_class("java/util/EnumMap")?;
let res = jni.new_object(cls,
"(Ljava/lang/Class;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaEnumMap::from_raw(&jni,res
)}
	pub fn remove_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = arg1.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"get","(Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn values(&mut self) 
-> Result<crate::JavaCollection<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"values","()Ljava/util/Collection;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCollection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn entry_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"entrySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn put_all(&mut self,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"putAll","(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn contains_key(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsKey","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn key_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_value(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsValue","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn replace_with_object(&mut self,arg0: K,arg1: std::option::Option<V>,arg2: std::option::Option<V>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
let val_2 = arg1.unwrap().jni_object();
let val_3 = arg2.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"replace","(LK;LV;LV;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn put_if_absent(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"putIfAbsent","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn get_or_default(&mut self,arg0: jni::objects::JObject<'mc>,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"getOrDefault","(Ljava/lang/Object;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
}
impl<'mc, K> Into<crate::JavaAbstractMap<'mc, K>> for JavaEnumMap<'mc, K> where K: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractMap<'mc, K> {
       crate::JavaAbstractMap::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaSpliteratorsAbstractIntSpliterator<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaSpliteratorsAbstractIntSpliterator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaSpliteratorsAbstractIntSpliterator<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaSpliteratorsAbstractIntSpliterator from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaSpliteratorsAbstractIntSpliterator")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaSpliteratorsAbstractIntSpliterator object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn estimate_size(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"estimateSize","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn try_split(&mut self) 
-> Result<crate::JavaSpliteratorOfInt<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"trySplit","()Ljava/util/Spliterator$OfInt;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliteratorOfInt::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn characteristics(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"characteristics","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn try_advance_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"tryAdvance","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn for_each_remaining_with_consumer(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"forEachRemaining","(Ljava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn exact_size_if_known(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExactSizeIfKnown","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn has_characteristics(&mut self,arg0: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"hasCharacteristics","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getComparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
}
impl<'mc> Into<crate::JavaSpliteratorOfInt<'mc>> for JavaSpliteratorsAbstractIntSpliterator<'mc>{
   fn into(self) -> crate::JavaSpliteratorOfInt<'mc> {
       crate::JavaSpliteratorOfInt::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaStringJoiner<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaStringJoiner<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaStringJoiner<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaStringJoiner from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaStringJoiner")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaStringJoiner object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn length(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"length","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn merge(&mut self,arg0: impl Into<&'mc crate::JavaStringJoiner<'mc>>) 
-> Result<crate::JavaStringJoiner<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"merge","(Ljava/util/StringJoiner;)Ljava/util/StringJoiner;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaStringJoiner::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaGregorianCalendar<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaGregorianCalendar<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaGregorianCalendar<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaGregorianCalendar from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaGregorianCalendar")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaGregorianCalendar object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaTimeZone<'mc>>>) 
-> Result<crate::JavaGregorianCalendar<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/GregorianCalendar")?;
let res = jni.new_object(cls,
"(Ljava/util/TimeZone;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaGregorianCalendar::from_raw(&jni,res
)}
	pub fn new_with_time_zone(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: std::option::Option<i32>,arg2: std::option::Option<i32>) 
-> Result<crate::JavaGregorianCalendar<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let cls = &jni.find_class("java/util/GregorianCalendar")?;
let res = jni.new_object(cls,
"(III)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::JavaGregorianCalendar::from_raw(&jni,res
)}
	pub fn new_with_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: i32,arg2: i32,arg3: i32,arg4: std::option::Option<i32>,arg5: std::option::Option<i32>) 
-> Result<crate::JavaGregorianCalendar<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("java/util/GregorianCalendar")?;
let res = jni.new_object(cls,
"(IIIIII)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
crate::JavaGregorianCalendar::from_raw(&jni,res
)}
	pub fn is_leap_year(&mut self,arg0: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"isLeapYear","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn gregorian_change(&mut self) 
-> Result<crate::JavaDate<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getGregorianChange","()Ljava/util/Date;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaDate::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn calendar_type(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCalendarType","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn roll_with_int(&mut self,arg0: i32,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"roll","(II)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn get_greatest_minimum(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getGreatestMinimum","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn get_minimum(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getMinimum","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn get_least_maximum(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getLeastMaximum","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn get_maximum(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getMaximum","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_time_zone(&mut self,arg0: impl Into<&'mc crate::JavaTimeZone<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setTimeZone","(Ljava/util/TimeZone;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_week_date_supported(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isWeekDateSupported","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn week_year(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getWeekYear","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_week_date(&mut self,arg0: i32,arg1: i32,arg2: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setWeekDate","(III)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn weeks_in_week_year(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getWeeksInWeekYear","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn get_actual_minimum(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getActualMinimum","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn get_actual_maximum(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getActualMaximum","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_gregorian_change(&mut self,arg0: impl Into<&'mc crate::JavaDate<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setGregorianChange","(Ljava/util/Date;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn time_zone(&mut self) 
-> Result<crate::JavaTimeZone<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTimeZone","()Ljava/util/TimeZone;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaTimeZone::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add(&mut self,arg0: i32,arg1: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"add","(II)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn get_display_name(&mut self,arg0: i32,arg1: i32,arg2: impl Into<&'mc crate::JavaLocale<'mc>>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayName","(IILjava/util/Locale;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn time(&mut self) 
-> Result<crate::JavaDate<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTime","()Ljava/util/Date;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaDate::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_time(&mut self,arg0: impl Into<&'mc crate::JavaDate<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setTime","(Ljava/util/Date;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn get_display_names(&mut self,arg0: i32,arg1: i32,arg2: impl Into<&'mc crate::JavaLocale<'mc>>) 
-> Result<crate::JavaMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayNames","(IILjava/util/Locale;)Ljava/util/Map;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn available_calendar_types(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Set")?;
let res = jni.call_static_method(cls,"getAvailableCalendarTypes",
"()Ljava/util/Set;",&[])?;
let mut obj = res.l()?;
crate::JavaSet::from_raw(&jni,obj
)}
	pub fn is_lenient(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isLenient","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_first_day_of_week(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setFirstDayOfWeek","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn first_day_of_week(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFirstDayOfWeek","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_minimal_days_in_first_week(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setMinimalDaysInFirstWeek","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn minimal_days_in_first_week(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMinimalDaysInFirstWeek","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn before(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"before","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn after(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"after","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_time_in_millis(&mut self,arg0: i64) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setTimeInMillis","(J)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_lenient(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_1 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setLenient","(Z)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn time_in_millis(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTimeInMillis","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn get(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"get","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn compare_to_with_calendar(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"compareTo","(Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self,arg0: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"clear","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn instance(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaCalendar<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Calendar")?;
let res = jni.call_static_method(cls,"getInstance",
"()Ljava/util/Calendar;",&[])?;
let mut obj = res.l()?;
crate::JavaCalendar::from_raw(&jni,obj
)}
	pub fn set_with_int(&mut self,arg0: i32,arg1: std::option::Option<i32>,arg2: std::option::Option<i32>,arg3: std::option::Option<i32>,arg4: std::option::Option<i32>,arg5: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"set","(IIIIII)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_set(&mut self,arg0: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"isSet","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::JavaCalendar<'mc>> for JavaGregorianCalendar<'mc>{
   fn into(self) -> crate::JavaCalendar<'mc> {
       crate::JavaCalendar::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
/// An instantiatable struct that implements JavaRandomAccess. Needed for returning it from Java.
pub struct JavaRandomAccess<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaRandomAccess<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaRandomAccess from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaRandomAccess")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaRandomAccess object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
}
impl<'mc> JNIRaw<'mc> for JavaRandomAccess<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaServiceLoader<'mc, S>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where S: JNIRaw<'mc>;
impl<'mc, S> blackboxmc_general::JNIRaw<'mc> for JavaServiceLoader<'mc, S> where S: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, S> JavaServiceLoader<'mc, S> where S: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaServiceLoader from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaServiceLoader")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaServiceLoader object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn load_installed<S>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: jni::objects::JClass<'mc>) 
-> Result<crate::JavaServiceLoader<'mc, S>, Box<dyn std::error::Error>>
 where S: JNIRaw<'mc>
{let val_1 = arg0;
let cls = &jni.find_class("java/util/ServiceLoader")?;
let res = jni.call_static_method(cls,"loadInstalled",
"(Ljava/lang/Class;)Ljava/util/ServiceLoader;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaServiceLoader::from_raw(&jni,obj
)}
	pub fn reload(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"reload","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn find_first(&mut self) 
-> Result<crate::JavaOptional<'mc, S>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"findFirst","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, S>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
}
pub struct JavaLongSummaryStatistics<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaLongSummaryStatistics<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaLongSummaryStatistics<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaLongSummaryStatistics from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaLongSummaryStatistics")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaLongSummaryStatistics object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i64>,arg1: std::option::Option<i64>,arg2: std::option::Option<i64>,arg3: std::option::Option<i64>) 
-> Result<crate::JavaLongSummaryStatistics<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Long(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Long(arg3.unwrap().into());
let cls = &jni.find_class("java/util/LongSummaryStatistics")?;
let res = jni.new_object(cls,
"(JJJJ)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::JavaLongSummaryStatistics::from_raw(&jni,res
)}
	pub fn count(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCount","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn sum(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSum","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn min(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMin","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn average(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAverage","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn max(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMax","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn accept_with_int(&mut self,arg0: std::option::Option<i64>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"accept","(J)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn combine(&mut self,arg0: impl Into<&'mc crate::JavaLongSummaryStatistics<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"combine","(Ljava/util/LongSummaryStatistics;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaTimeZone<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaTimeZone<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaTimeZone<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaTimeZone from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaTimeZone")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaTimeZone object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaTimeZone<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/TimeZone")?;
let res = jni.new_object(cls,
"()V",&[])?;
crate::JavaTimeZone::from_raw(&jni,res
)}
	pub fn get_display_name_with_locale(&mut self,arg0: std::option::Option<bool>,arg1: std::option::Option<i32>) 
-> Result<String, Box<dyn std::error::Error>>

{// 0
let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayName","(ZI)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn get_display_name_with_boolean(&mut self,arg0: bool,arg1: i32,arg2: std::option::Option<impl Into<&'mc crate::JavaLocale<'mc>>>) 
-> Result<String, Box<dyn std::error::Error>>

{// 2
let val_1 = jni::objects::JValueGen::Bool(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayName","(ZILjava/util/Locale;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_default(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaTimeZone<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"setDefault",
"(Ljava/util/TimeZone;)V",&[jni::objects::JValueGen::from(&val_1)])?;
Ok(())}
	pub fn id(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getID","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn raw_offset(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRawOffset","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn has_same_rules(&mut self,arg0: impl Into<&'mc crate::JavaTimeZone<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"hasSameRules","(Ljava/util/TimeZone;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn in_daylight_time(&mut self,arg0: impl Into<&'mc crate::JavaDate<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"inDaylightTime","(Ljava/util/Date;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn dstsavings(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDSTSavings","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn use_daylight_time(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"useDaylightTime","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_id(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setID","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_raw_offset(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setRawOffset","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn observes_daylight_time(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"observesDaylightTime","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn default(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaTimeZone<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/TimeZone")?;
let res = jni.call_static_method(cls,"getDefault",
"()Ljava/util/TimeZone;",&[])?;
let mut obj = res.l()?;
crate::JavaTimeZone::from_raw(&jni,obj
)}
	pub fn get_offset_with_long(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<i32>,arg2: std::option::Option<i32>,arg3: std::option::Option<i32>,arg4: std::option::Option<i32>,arg5: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"getOffset","(IIIIII)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
/// An instantiatable struct that implements JavaSortedMap. Needed for returning it from Java.
pub struct JavaSortedMap<'mc, K,V>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where K: JNIRaw<'mc>,V: JNIRaw<'mc>;
impl<'mc, K,V> JavaSortedMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaSortedMap from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaSortedMap")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaSortedMap object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn first_key(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"firstKey","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn sub_map(&mut self,arg0: K,arg1: K) 
-> Result<crate::JavaSortedMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"subMap","(LK;LK;)Ljava/util/SortedMap;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSortedMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn head_map(&mut self,arg0: K) 
-> Result<crate::JavaSortedMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"headMap","(LK;)Ljava/util/SortedMap;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSortedMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn tail_map(&mut self,arg0: K) 
-> Result<crate::JavaSortedMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"tailMap","(LK;)Ljava/util/SortedMap;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSortedMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn last_key(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"lastKey","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn values(&mut self) 
-> Result<crate::JavaCollection<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"values","()Ljava/util/Collection;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCollection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entry_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"entrySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn key_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"comparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn remove_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = arg1.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"get","(Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn put(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"put","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn replace_with_object(&mut self,arg0: K,arg1: std::option::Option<V>,arg2: std::option::Option<V>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
let val_2 = arg1.unwrap().jni_object();
let val_3 = arg2.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"replace","(LK;LV;LV;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn put_all(&mut self,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"putAll","(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn put_if_absent(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"putIfAbsent","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn contains_key(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsKey","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_value(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsValue","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get_or_default(&mut self,arg0: jni::objects::JObject<'mc>,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"getOrDefault","(Ljava/lang/Object;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
}
impl<'mc, K,V> JNIRaw<'mc> for JavaSortedMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, K,V> Into<crate::JavaMap<'mc, K,V>> for JavaSortedMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc>{
   fn into(self) -> crate::JavaMap<'mc, K,V> {
       crate::JavaMap::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaAbstractMapSimpleImmutableEntry<'mc, K,V>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where K: JNIRaw<'mc>,V: JNIRaw<'mc>;
impl<'mc, K,V> blackboxmc_general::JNIRaw<'mc> for JavaAbstractMapSimpleImmutableEntry<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, K,V> JavaAbstractMapSimpleImmutableEntry<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaAbstractMapSimpleImmutableEntry from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaAbstractMapSimpleImmutableEntry")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaAbstractMapSimpleImmutableEntry object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_mapentry(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<K>,arg1: std::option::Option<V>) 
-> Result<crate::JavaAbstractMapSimpleImmutableEntry<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
let val_2 = arg1.unwrap().jni_object();
let cls = &jni.find_class("java/util/AbstractMap$SimpleImmutableEntry")?;
let res = jni.new_object(cls,
"(LK;LV;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::JavaAbstractMapSimpleImmutableEntry::from_raw(&jni,res
)}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn value(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getValue","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn key(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getKey","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn set_value(&mut self,arg0: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"setValue","(LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaArrays<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaArrays<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaArrays<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaArrays from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaArrays")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaArrays object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn mismatch_with_ints(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i16>,arg1: i32,arg2: i32,arg3: Vec<i16>,arg4: i32,arg5: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"mismatch",
"(SIISII)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
Ok(res.i().unwrap())}
	pub fn mismatch_with_bytes(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<bool>,arg1: i32,arg2: i32,arg3: Vec<bool>,arg4: i32,arg5: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"mismatch",
"(ZIIZII)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
Ok(res.i().unwrap())}
	pub fn mismatch_with_doubles(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i64>,arg1: std::option::Option<Vec<i64>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"mismatch",
"(JJ)I",&[])?;
Ok(res.i().unwrap())}
	pub fn mismatch_with_objects(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<f32>,arg1: std::option::Option<Vec<f32>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"mismatch",
"(FF)I",&[])?;
Ok(res.i().unwrap())}
	pub fn mismatch_with_booleans(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>,arg1: std::option::Option<Vec<jni::objects::JObject<'mc>>>,arg2: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"mismatch",
"(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I",&[jni::objects::JValueGen::from(&val_3)])?;
Ok(res.i().unwrap())}
	pub fn mismatch_with_chars(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>,arg1: i32,arg2: i32,arg3: Vec<jni::objects::JObject<'mc>>,arg4: i32,arg5: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"mismatch",
"(Ljava/lang/Object;IILjava/lang/Object;II)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
Ok(res.i().unwrap())}
	pub fn mismatch_with_floats(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<f64>,arg1: i32,arg2: i32,arg3: Vec<f64>,arg4: i32,arg5: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"mismatch",
"(DIIDII)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
Ok(res.i().unwrap())}
	pub fn mismatch_with_longs(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>,arg1: i32,arg2: i32,arg3: Vec<jni::objects::JObject<'mc>>,arg4: i32,arg5: std::option::Option<i32>,arg6: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let val_7 = unsafe { jni::objects::JObject::from_raw(arg6.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"mismatch",
"(Ljava/lang/Object;IILjava/lang/Object;IILjava/util/Comparator;)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6),jni::objects::JValueGen::from(&val_7)])?;
Ok(res.i().unwrap())}
	pub fn deep_hash_code(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"deepHashCode",
"(Ljava/lang/Object;)I",&[])?;
Ok(res.i().unwrap())}
	pub fn deep_to_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>) 
-> Result<String, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/lang/String")?;
let res = jni.call_static_method(cls,"deepToString",
"(Ljava/lang/Object;)Ljava/lang/String;",&[])?;
Ok(jni.get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn parallel_sort_with_floats(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<f64>,arg1: i32,arg2: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"parallelSort",
"(DII)V",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
Ok(())}
	pub fn parallel_sort_with_comparables(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<f64>>) 
-> Result<(), Box<dyn std::error::Error>>

{let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"parallelSort",
"(D)V",&[])?;
Ok(())}
	pub fn parallel_sort_with_shorts(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>,arg1: i32,arg2: std::option::Option<i32>,arg3: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"parallelSort",
"(Ljava/lang/Object;IILjava/util/Comparator;)V",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
Ok(())}
	pub fn parallel_sort_with_longs(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<u16>,arg1: i32,arg2: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"parallelSort",
"(CII)V",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
Ok(())}
	pub fn parallel_sort_with_bytes(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i32>,arg1: i32,arg2: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"parallelSort",
"(III)V",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
Ok(())}
	pub fn binary_search_with_objects(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<u16>,arg1: i32,arg2: i32,arg3: std::option::Option<u16>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Char(arg3.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"binarySearch",
"(CIIC)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
Ok(res.i().unwrap())}
	pub fn binary_search_with_ints(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i16>,arg1: i32,arg2: i32,arg3: std::option::Option<i16>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Short(arg3.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"binarySearch",
"(SIIS)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
Ok(res.i().unwrap())}
	pub fn binary_search_with_shorts(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i64>,arg1: std::option::Option<i64>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"binarySearch",
"(JJ)I",&[jni::objects::JValueGen::from(&val_2)])?;
Ok(res.i().unwrap())}
	pub fn binary_search_with_doubles(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i8>,arg1: i32,arg2: i32,arg3: std::option::Option<i8>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Byte(arg3.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"binarySearch",
"(BIIB)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
Ok(res.i().unwrap())}
	pub fn binary_search_with_longs(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>,arg1: i32,arg2: i32,arg3: std::option::Option<T>,arg4: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = arg3.unwrap().jni_object();
let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"binarySearch",
"(Ljava/lang/Object;IILT;Ljava/util/Comparator;)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
Ok(res.i().unwrap())}
	pub fn equals_with_object(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<f64>>,arg1: std::option::Option<Vec<f64>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let cls = &jni.find_class("boolean")?;
let res = jni.call_static_method(cls,"equals",
"(DD)Z",&[])?;
Ok(res.z().unwrap())}
	pub fn equals_with_booleans(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i8>,arg1: i32,arg2: i32,arg3: Vec<i8>,arg4: i32,arg5: std::option::Option<i32>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("boolean")?;
let res = jni.call_static_method(cls,"equals",
"(BIIBII)Z",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
Ok(res.z().unwrap())}
	pub fn equals_with_floats(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i16>,arg1: std::option::Option<Vec<i16>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let cls = &jni.find_class("boolean")?;
let res = jni.call_static_method(cls,"equals",
"(SS)Z",&[])?;
Ok(res.z().unwrap())}
	pub fn equals_with_ints(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i64>,arg1: i32,arg2: i32,arg3: Vec<i64>,arg4: i32,arg5: std::option::Option<i32>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("boolean")?;
let res = jni.call_static_method(cls,"equals",
"(JIIJII)Z",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
Ok(res.z().unwrap())}
	pub fn equals_with_bytes(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<u16>,arg1: std::option::Option<Vec<u16>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let cls = &jni.find_class("boolean")?;
let res = jni.call_static_method(cls,"equals",
"(CC)Z",&[])?;
Ok(res.z().unwrap())}
	pub fn equals_with_objects(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>,arg1: i32,arg2: i32,arg3: Vec<jni::objects::JObject<'mc>>,arg4: i32,arg5: i32,arg6: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let val_7 = unsafe { jni::objects::JObject::from_raw(arg6.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("boolean")?;
let res = jni.call_static_method(cls,"equals",
"(Ljava/lang/Object;IILjava/lang/Object;IILjava/util/Comparator;)Z",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6),jni::objects::JValueGen::from(&val_7)])?;
Ok(res.z().unwrap())}
	pub fn equals_with_chars(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i16>,arg1: i32,arg2: i32,arg3: Vec<i16>,arg4: i32,arg5: std::option::Option<i32>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("boolean")?;
let res = jni.call_static_method(cls,"equals",
"(SIISII)Z",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
Ok(res.z().unwrap())}
	pub fn to_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<u16>>) 
-> Result<String, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/lang/String")?;
let res = jni.call_static_method(cls,"toString",
"(C)Ljava/lang/String;",&[])?;
Ok(jni.get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn to_string_with_shorts(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<i32>>) 
-> Result<String, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/lang/String")?;
let res = jni.call_static_method(cls,"toString",
"(I)Ljava/lang/String;",&[])?;
Ok(jni.get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn to_string_with_longs(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<jni::objects::JObject<'mc>>>) 
-> Result<String, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/lang/String")?;
let res = jni.call_static_method(cls,"toString",
"(Ljava/lang/Object;)Ljava/lang/String;",&[])?;
Ok(jni.get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn to_string_with_doubles(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<f32>>) 
-> Result<String, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/lang/String")?;
let res = jni.call_static_method(cls,"toString",
"(F)Ljava/lang/String;",&[])?;
Ok(jni.get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn to_string_with_booleans(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<bool>>) 
-> Result<String, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/lang/String")?;
let res = jni.call_static_method(cls,"toString",
"(Z)Ljava/lang/String;",&[])?;
Ok(jni.get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<i16>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"hashCode",
"(S)I",&[])?;
Ok(res.i().unwrap())}
	pub fn hash_code_with_bytes(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<i32>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"hashCode",
"(I)I",&[])?;
Ok(res.i().unwrap())}
	pub fn hash_code_with_longs(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<jni::objects::JObject<'mc>>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"hashCode",
"(Ljava/lang/Object;)I",&[])?;
Ok(res.i().unwrap())}
	pub fn hash_code_with_doubles(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<f32>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"hashCode",
"(F)I",&[])?;
Ok(res.i().unwrap())}
	pub fn hash_code_with_booleans(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<bool>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"hashCode",
"(Z)I",&[])?;
Ok(res.i().unwrap())}
	pub fn compare_with_ints(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<u16>,arg1: std::option::Option<Vec<u16>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"compare",
"(CC)I",&[])?;
Ok(res.i().unwrap())}
	pub fn compare_with_longs(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i8>,arg1: i32,arg2: i32,arg3: Vec<i8>,arg4: i32,arg5: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"compare",
"(BIIBII)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
Ok(res.i().unwrap())}
	pub fn compare_with_booleans(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i16>,arg1: i32,arg2: i32,arg3: Vec<i16>,arg4: i32,arg5: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"compare",
"(SIISII)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
Ok(res.i().unwrap())}
	pub fn compare_with_comparables(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<f32>,arg1: i32,arg2: i32,arg3: Vec<f32>,arg4: i32,arg5: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"compare",
"(FIIFII)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
Ok(res.i().unwrap())}
	pub fn compare_with_doubles(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>,arg1: i32,arg2: i32,arg3: Vec<jni::objects::JObject<'mc>>,arg4: i32,arg5: std::option::Option<i32>,arg6: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let val_7 = unsafe { jni::objects::JObject::from_raw(arg6.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"compare",
"(Ljava/lang/Object;IILjava/lang/Object;IILjava/util/Comparator;)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6),jni::objects::JValueGen::from(&val_7)])?;
Ok(res.i().unwrap())}
	pub fn compare_with_chars(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i32>,arg1: i32,arg2: i32,arg3: Vec<i32>,arg4: i32,arg5: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"compare",
"(IIIIII)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
Ok(res.i().unwrap())}
	pub fn fill_with_longs(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i32>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"fill",
"(II)V",&[jni::objects::JValueGen::from(&val_2)])?;
Ok(())}
	pub fn fill_with_bytes(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<bool>,arg1: std::option::Option<bool>) 
-> Result<(), Box<dyn std::error::Error>>

{// 1
let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"fill",
"(ZZ)V",&[jni::objects::JValueGen::from(&val_2)])?;
Ok(())}
	pub fn fill_with_shorts(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i32>,arg1: i32,arg2: i32,arg3: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"fill",
"(IIII)V",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
Ok(())}
	pub fn fill_with_floats(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>,arg1: i32,arg2: i32,arg3: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = arg3.unwrap();
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"fill",
"(Ljava/lang/Object;IILjava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
Ok(())}
	pub fn fill_with_doubles(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i64>,arg1: std::option::Option<i32>,arg2: std::option::Option<i32>,arg3: std::option::Option<i64>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Long(arg3.unwrap().into());
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"fill",
"(JIIJ)V",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
Ok(())}
	pub fn fill_with_chars(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i8>,arg1: i32,arg2: i32,arg3: std::option::Option<i8>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Byte(arg3.unwrap().into());
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"fill",
"(BIIB)V",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
Ok(())}
	pub fn fill_with_booleans(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<f64>,arg1: i32,arg2: i32,arg3: std::option::Option<f64>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Double(arg3.unwrap().into());
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"fill",
"(DIID)V",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
Ok(())}
	pub fn spliterator_with_objects<T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<i32>>) 
-> Result<crate::JavaSpliteratorOfInt<'mc>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let cls = &jni.find_class("java/util/Spliterator$OfInt")?;
let res = jni.call_static_method(cls,"spliterator",
"(I)Ljava/util/Spliterator$OfInt;",&[])?;
let mut obj = res.l()?;
crate::JavaSpliteratorOfInt::from_raw(&jni,obj
)}
	pub fn spliterator_with_longs<T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<f64>>) 
-> Result<crate::JavaSpliteratorOfDouble<'mc>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let cls = &jni.find_class("java/util/Spliterator$OfDouble")?;
let res = jni.call_static_method(cls,"spliterator",
"(D)Ljava/util/Spliterator$OfDouble;",&[])?;
let mut obj = res.l()?;
crate::JavaSpliteratorOfDouble::from_raw(&jni,obj
)}
	pub fn spliterator_with_doubles<T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>,arg1: i32,arg2: std::option::Option<i32>) 
-> Result<crate::JavaSpliterator<'mc, T>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let cls = &jni.find_class("java/util/Spliterator")?;
let res = jni.call_static_method(cls,"spliterator",
"(Ljava/lang/Object;II)Ljava/util/Spliterator;",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::JavaSpliterator::from_raw(&jni,obj
)}
	pub fn spliterator_with_ints<T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i64>,arg1: i32,arg2: std::option::Option<i32>) 
-> Result<crate::JavaSpliteratorOfLong<'mc>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let cls = &jni.find_class("java/util/Spliterator$OfLong")?;
let res = jni.call_static_method(cls,"spliterator",
"(JII)Ljava/util/Spliterator$OfLong;",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::JavaSpliteratorOfLong::from_raw(&jni,obj
)}
	pub fn as_list<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let cls = &jni.find_class("java/util/List")?;
let res = jni.call_static_method(cls,"asList",
"(Ljava/lang/Object;)Ljava/util/List;",&[])?;
let mut obj = res.l()?;
crate::JavaList::from_raw(&jni,obj
)}
	pub fn sort_with_objects(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<u16>,arg1: i32,arg2: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"sort",
"(CII)V",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
Ok(())}
	pub fn sort_with_floats(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<f64>>) 
-> Result<(), Box<dyn std::error::Error>>

{let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"sort",
"(D)V",&[])?;
Ok(())}
	pub fn sort_with_longs(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i32>,arg1: i32,arg2: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"sort",
"(III)V",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
Ok(())}
	pub fn sort_with_chars(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<i16>>) 
-> Result<(), Box<dyn std::error::Error>>

{let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"sort",
"(S)V",&[])?;
Ok(())}
	pub fn sort_with_doubles(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i8>,arg1: i32,arg2: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"sort",
"(BII)V",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
Ok(())}
	pub fn sort_with_shorts(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>,arg1: i32,arg2: std::option::Option<i32>,arg3: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"sort",
"(Ljava/lang/Object;IILjava/util/Comparator;)V",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
Ok(())}
	pub fn compare_unsigned_with_shorts(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i64>,arg1: i32,arg2: i32,arg3: Vec<i64>,arg4: i32,arg5: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"compareUnsigned",
"(JIIJII)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
Ok(res.i().unwrap())}
	pub fn compare_unsigned_with_longs(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i8>,arg1: std::option::Option<Vec<i8>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"compareUnsigned",
"(BB)I",&[])?;
Ok(res.i().unwrap())}
	pub fn compare_unsigned_with_bytes(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i32>,arg1: i32,arg2: i32,arg3: Vec<i32>,arg4: i32,arg5: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"compareUnsigned",
"(IIIIII)I",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
Ok(res.i().unwrap())}
	pub fn deep_equals(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>,arg1: Vec<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let cls = &jni.find_class("boolean")?;
let res = jni.call_static_method(cls,"deepEquals",
"(Ljava/lang/Object;Ljava/lang/Object;)Z",&[])?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
/// An instantiatable struct that implements JavaMap. Needed for returning it from Java.
pub struct JavaMap<'mc, K,V>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where K: JNIRaw<'mc>,V: JNIRaw<'mc>;
impl<'mc, K,V> JavaMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaMap from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaMap")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaMap object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn remove_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = arg1.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"get","(Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn put(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"put","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn values(&mut self) 
-> Result<crate::JavaCollection<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"values","()Ljava/util/Collection;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCollection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn copy_of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<crate::JavaMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Map")?;
let res = jni.call_static_method(cls,"copyOf",
"(Ljava/util/Map;)Ljava/util/Map;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaMap::from_raw(&jni,obj
)}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn replace_with_object(&mut self,arg0: K,arg1: std::option::Option<V>,arg2: std::option::Option<V>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
let val_2 = arg1.unwrap().jni_object();
let val_3 = arg2.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"replace","(LK;LV;LV;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<K>,arg1: std::option::Option<V>,arg2: std::option::Option<K>,arg3: std::option::Option<V>,arg4: std::option::Option<K>,arg5: std::option::Option<V>,arg6: std::option::Option<K>,arg7: std::option::Option<V>,arg8: std::option::Option<K>,arg9: std::option::Option<V>,arg10: std::option::Option<K>,arg11: std::option::Option<V>,arg12: std::option::Option<K>,arg13: std::option::Option<V>,arg14: std::option::Option<K>,arg15: std::option::Option<V>,arg16: std::option::Option<K>,arg17: std::option::Option<V>,arg18: std::option::Option<K>,arg19: std::option::Option<V>) 
-> Result<crate::JavaMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
let val_2 = arg1.unwrap().jni_object();
let val_3 = arg2.unwrap().jni_object();
let val_4 = arg3.unwrap().jni_object();
let val_5 = arg4.unwrap().jni_object();
let val_6 = arg5.unwrap().jni_object();
let val_7 = arg6.unwrap().jni_object();
let val_8 = arg7.unwrap().jni_object();
let val_9 = arg8.unwrap().jni_object();
let val_10 = arg9.unwrap().jni_object();
let val_11 = arg10.unwrap().jni_object();
let val_12 = arg11.unwrap().jni_object();
let val_13 = arg12.unwrap().jni_object();
let val_14 = arg13.unwrap().jni_object();
let val_15 = arg14.unwrap().jni_object();
let val_16 = arg15.unwrap().jni_object();
let val_17 = arg16.unwrap().jni_object();
let val_18 = arg17.unwrap().jni_object();
let val_19 = arg18.unwrap().jni_object();
let val_20 = arg19.unwrap().jni_object();
let cls = &jni.find_class("java/util/Map")?;
let res = jni.call_static_method(cls,"of",
"(LK;LV;LK;LV;LK;LV;LK;LV;LK;LV;LK;LV;LK;LV;LK;LV;LK;LV;LK;LV;)Ljava/util/Map;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6),jni::objects::JValueGen::from(&val_7),jni::objects::JValueGen::from(&val_8),jni::objects::JValueGen::from(&val_9),jni::objects::JValueGen::from(&val_10),jni::objects::JValueGen::from(&val_11),jni::objects::JValueGen::from(&val_12),jni::objects::JValueGen::from(&val_13),jni::objects::JValueGen::from(&val_14),jni::objects::JValueGen::from(&val_15),jni::objects::JValueGen::from(&val_16),jni::objects::JValueGen::from(&val_17),jni::objects::JValueGen::from(&val_18),jni::objects::JValueGen::from(&val_19),jni::objects::JValueGen::from(&val_20)])?;
let mut obj = res.l()?;
crate::JavaMap::from_raw(&jni,obj
)}
	pub fn entry_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"entrySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn put_all(&mut self,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"putAll","(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn put_if_absent(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"putIfAbsent","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn entry(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: K,arg1: V) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let cls = &jni.find_class("java/util/Map$Entry")?;
let res = jni.call_static_method(cls,"entry",
"(LK;LV;)Ljava/util/Map$Entry;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaMapEntry::from_raw(&jni,obj
)}
	pub fn contains_key(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsKey","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn key_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_value(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsValue","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get_or_default(&mut self,arg0: jni::objects::JObject<'mc>,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"getOrDefault","(Ljava/lang/Object;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn of_entries(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<impl Into<crate::JavaMapEntry<'mc>>>) 
-> Result<crate::JavaMap<'mc, K,V>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Map")?;
let res = jni.call_static_method(cls,"ofEntries",
"(Ljava/util/Map$Entry;)Ljava/util/Map;",&[])?;
let mut obj = res.l()?;
crate::JavaMap::from_raw(&jni,obj
)}
}
impl<'mc, K,V> JNIRaw<'mc> for JavaMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaWeakHashMap<'mc, K,V>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where K: JNIRaw<'mc>,V: JNIRaw<'mc>;
impl<'mc, K,V> blackboxmc_general::JNIRaw<'mc> for JavaWeakHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, K,V> JavaWeakHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaWeakHashMap from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaWeakHashMap")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaWeakHashMap object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i32>) 
-> Result<crate::JavaWeakHashMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let cls = &jni.find_class("java/util/WeakHashMap")?;
let res = jni.new_object(cls,
"(I)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaWeakHashMap::from_raw(&jni,res
)}
	pub fn new_with_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: std::option::Option<f32>) 
-> Result<crate::JavaWeakHashMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
let cls = &jni.find_class("java/util/WeakHashMap")?;
let res = jni.new_object(cls,
"(IF)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::JavaWeakHashMap::from_raw(&jni,res
)}
	pub fn remove_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = arg1.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"get","(Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn put(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"put","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn values(&mut self) 
-> Result<crate::JavaCollection<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"values","()Ljava/util/Collection;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCollection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn entry_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"entrySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn put_all(&mut self,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"putAll","(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn contains_key(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsKey","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn key_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_value(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsValue","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn replace_with_object(&mut self,arg0: K,arg1: std::option::Option<V>,arg2: std::option::Option<V>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
let val_2 = arg1.unwrap().jni_object();
let val_3 = arg2.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"replace","(LK;LV;LV;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn put_if_absent(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"putIfAbsent","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn get_or_default(&mut self,arg0: jni::objects::JObject<'mc>,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"getOrDefault","(Ljava/lang/Object;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
}
impl<'mc, K,V> Into<crate::JavaMap<'mc, K,V>> for JavaWeakHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc>{
   fn into(self) -> crate::JavaMap<'mc, K,V> {
       crate::JavaMap::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, K,V> Into<crate::JavaAbstractMap<'mc, K,V>> for JavaWeakHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractMap<'mc, K,V> {
       crate::JavaAbstractMap::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
/// An instantiatable struct that implements JavaSet. Needed for returning it from Java.
pub struct JavaSet<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> JavaSet<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaSet from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaSet")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaSet object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn add(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn copy_of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Set")?;
let res = jni.call_static_method(cls,"copyOf",
"(Ljava/util/Collection;)Ljava/util/Set;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaSet::from_raw(&jni,obj
)}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<jni::objects::JObject<'mc>>>) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Set")?;
let res = jni.call_static_method(cls,"of",
"(Ljava/lang/Object;)Ljava/util/Set;",&[])?;
let mut obj = res.l()?;
crate::JavaSet::from_raw(&jni,obj
)}
	pub fn of_with_object(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: E,arg1: E,arg2: E,arg3: E,arg4: E,arg5: E,arg6: E,arg7: E,arg8: E,arg9: std::option::Option<E>) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let val_3 = arg2.jni_object();
let val_4 = arg3.jni_object();
let val_5 = arg4.jni_object();
let val_6 = arg5.jni_object();
let val_7 = arg6.jni_object();
let val_8 = arg7.jni_object();
let val_9 = arg8.unwrap().jni_object();
let val_10 = arg9.unwrap().jni_object();
let cls = &jni.find_class("java/util/Set")?;
let res = jni.call_static_method(cls,"of",
"(LE;LE;LE;LE;LE;LE;LE;LE;LE;LE;)Ljava/util/Set;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6),jni::objects::JValueGen::from(&val_7),jni::objects::JValueGen::from(&val_8),jni::objects::JValueGen::from(&val_9),jni::objects::JValueGen::from(&val_10)])?;
let mut obj = res.l()?;
crate::JavaSet::from_raw(&jni,obj
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc, E> JNIRaw<'mc> for JavaSet<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> Into<crate::JavaCollection<'mc, E>> for JavaSet<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaCollection<'mc, E> {
       crate::JavaCollection::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaBase64<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaBase64<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaBase64<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaBase64 from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaBase64")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaBase64 object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn encoder(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaBase64Encoder<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Base64$Encoder")?;
let res = jni.call_static_method(cls,"getEncoder",
"()Ljava/util/Base64$Encoder;",&[])?;
let mut obj = res.l()?;
crate::JavaBase64Encoder::from_raw(&jni,obj
)}
	pub fn url_encoder(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaBase64Encoder<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Base64$Encoder")?;
let res = jni.call_static_method(cls,"getUrlEncoder",
"()Ljava/util/Base64$Encoder;",&[])?;
let mut obj = res.l()?;
crate::JavaBase64Encoder::from_raw(&jni,obj
)}
	pub fn get_mime_encoder(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: Vec<i8>) 
-> Result<crate::JavaBase64Encoder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let cls = &jni.find_class("java/util/Base64$Encoder")?;
let res = jni.call_static_method(cls,"getMimeEncoder",
"(IB)Ljava/util/Base64$Encoder;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaBase64Encoder::from_raw(&jni,obj
)}
	pub fn decoder(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaBase64Decoder<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Base64$Decoder")?;
let res = jni.call_static_method(cls,"getDecoder",
"()Ljava/util/Base64$Decoder;",&[])?;
let mut obj = res.l()?;
crate::JavaBase64Decoder::from_raw(&jni,obj
)}
	pub fn url_decoder(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaBase64Decoder<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Base64$Decoder")?;
let res = jni.call_static_method(cls,"getUrlDecoder",
"()Ljava/util/Base64$Decoder;",&[])?;
let mut obj = res.l()?;
crate::JavaBase64Decoder::from_raw(&jni,obj
)}
	pub fn mime_decoder(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaBase64Decoder<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Base64$Decoder")?;
let res = jni.call_static_method(cls,"getMimeDecoder",
"()Ljava/util/Base64$Decoder;",&[])?;
let mut obj = res.l()?;
crate::JavaBase64Decoder::from_raw(&jni,obj
)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaPropertyPermission<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaPropertyPermission<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaPropertyPermission<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaPropertyPermission from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaPropertyPermission")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaPropertyPermission object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc String>,arg1: impl Into<&'mc String>) 
-> Result<crate::JavaPropertyPermission<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
let cls = &jni.find_class("java/util/PropertyPermission")?;
let res = jni.new_object(cls,
"(Ljava/lang/String;Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::JavaPropertyPermission::from_raw(&jni,res
)}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn actions(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getActions","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn check_guard(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"checkGuard","(Ljava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaAbstractCollection<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> blackboxmc_general::JNIRaw<'mc> for JavaAbstractCollection<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> JavaAbstractCollection<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaAbstractCollection from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaAbstractCollection")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaAbstractCollection object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn add(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn add_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
}
impl<'mc, E> Into<crate::JavaCollection<'mc, E>> for JavaAbstractCollection<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaCollection<'mc, E> {
       crate::JavaCollection::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
/// An instantiatable struct that implements JavaSortedSet. Needed for returning it from Java.
pub struct JavaSortedSet<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> JavaSortedSet<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaSortedSet from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaSortedSet")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaSortedSet object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn sub_set(&mut self,arg0: E,arg1: E) 
-> Result<crate::JavaSortedSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"subSet","(LE;LE;)Ljava/util/SortedSet;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSortedSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn head_set(&mut self,arg0: E) 
-> Result<crate::JavaSortedSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"headSet","(LE;)Ljava/util/SortedSet;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSortedSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn tail_set(&mut self,arg0: E) 
-> Result<crate::JavaSortedSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"tailSet","(LE;)Ljava/util/SortedSet;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSortedSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"last","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"first","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"comparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn add_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc, E> JNIRaw<'mc> for JavaSortedSet<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> Into<crate::JavaSet<'mc, E>> for JavaSortedSet<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaSet<'mc, E> {
       crate::JavaSet::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaIntSummaryStatistics<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaIntSummaryStatistics<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaIntSummaryStatistics<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaIntSummaryStatistics from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaIntSummaryStatistics")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaIntSummaryStatistics object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>,arg2: std::option::Option<i32>,arg3: std::option::Option<i64>) 
-> Result<crate::JavaIntSummaryStatistics<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Long(arg3.unwrap().into());
let cls = &jni.find_class("java/util/IntSummaryStatistics")?;
let res = jni.new_object(cls,
"(JIIJ)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::JavaIntSummaryStatistics::from_raw(&jni,res
)}
	pub fn count(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCount","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn sum(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSum","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn min(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMin","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn average(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAverage","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn max(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMax","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn accept(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"accept","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn combine(&mut self,arg0: impl Into<&'mc crate::JavaIntSummaryStatistics<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"combine","(Ljava/util/IntSummaryStatistics;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaAbstractQueue<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> blackboxmc_general::JNIRaw<'mc> for JavaAbstractQueue<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> JavaAbstractQueue<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaAbstractQueue from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaAbstractQueue")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaAbstractQueue object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn add(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn add_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn element(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"element","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn offer(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"offer","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn poll(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"poll","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn peek(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"peek","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
}
impl<'mc, E> Into<crate::JavaQueue<'mc, E>> for JavaAbstractQueue<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaQueue<'mc, E> {
       crate::JavaQueue::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, E> Into<crate::JavaAbstractCollection<'mc, E>> for JavaAbstractQueue<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractCollection<'mc, E> {
       crate::JavaAbstractCollection::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaSimpleTimeZone<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaSimpleTimeZone<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaSimpleTimeZone<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaSimpleTimeZone from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaSimpleTimeZone")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaSimpleTimeZone object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: std::option::Option<impl Into<&'mc String>>,arg2: std::option::Option<i32>,arg3: std::option::Option<i32>,arg4: std::option::Option<i32>,arg5: std::option::Option<i32>,arg6: std::option::Option<i32>,arg7: std::option::Option<i32>,arg8: std::option::Option<i32>,arg9: std::option::Option<i32>,arg10: std::option::Option<i32>,arg11: std::option::Option<i32>,arg12: std::option::Option<i32>) 
-> Result<crate::JavaSimpleTimeZone<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let val_7 = jni::objects::JValueGen::Int(arg6.unwrap().into());
let val_8 = jni::objects::JValueGen::Int(arg7.unwrap().into());
let val_9 = jni::objects::JValueGen::Int(arg8.unwrap().into());
let val_10 = jni::objects::JValueGen::Int(arg9.unwrap().into());
let val_11 = jni::objects::JValueGen::Int(arg10.unwrap().into());
let val_12 = jni::objects::JValueGen::Int(arg11.unwrap().into());
let val_13 = jni::objects::JValueGen::Int(arg12.unwrap().into());
let cls = &jni.find_class("java/util/SimpleTimeZone")?;
let res = jni.new_object(cls,
"(ILjava/lang/String;IIIIIIIIIII)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6),jni::objects::JValueGen::from(&val_7),jni::objects::JValueGen::from(&val_8),jni::objects::JValueGen::from(&val_9),jni::objects::JValueGen::from(&val_10),jni::objects::JValueGen::from(&val_11),jni::objects::JValueGen::from(&val_12),jni::objects::JValueGen::from(&val_13)])?;
crate::JavaSimpleTimeZone::from_raw(&jni,res
)}
	pub fn set_start_rule_with_int(&mut self,arg0: i32,arg1: i32,arg2: std::option::Option<i32>,arg3: std::option::Option<i32>,arg4: std::option::Option<bool>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
// 2
let val_5 = jni::objects::JValueGen::Bool(arg4.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"setStartRule","(IIIIZ)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_end_rule_with_int(&mut self,arg0: i32,arg1: i32,arg2: std::option::Option<i32>,arg3: std::option::Option<i32>,arg4: std::option::Option<bool>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
// 2
let val_5 = jni::objects::JValueGen::Bool(arg4.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"setEndRule","(IIIIZ)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_start_year(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setStartYear","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_dstsavings(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDSTSavings","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn raw_offset(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRawOffset","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn has_same_rules(&mut self,arg0: impl Into<&'mc crate::JavaTimeZone<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"hasSameRules","(Ljava/util/TimeZone;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn in_daylight_time(&mut self,arg0: impl Into<&'mc crate::JavaDate<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"inDaylightTime","(Ljava/util/Date;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn dstsavings(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDSTSavings","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn use_daylight_time(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"useDaylightTime","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_raw_offset(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setRawOffset","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn observes_daylight_time(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"observesDaylightTime","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn get_offset_with_long(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<i32>,arg2: std::option::Option<i32>,arg3: std::option::Option<i32>,arg4: std::option::Option<i32>,arg5: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"getOffset","(IIIIII)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn get_display_name_with_locale(&mut self,arg0: std::option::Option<bool>,arg1: std::option::Option<i32>) 
-> Result<String, Box<dyn std::error::Error>>

{// 0
let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayName","(ZI)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn get_display_name_with_boolean(&mut self,arg0: bool,arg1: i32,arg2: std::option::Option<impl Into<&'mc crate::JavaLocale<'mc>>>) 
-> Result<String, Box<dyn std::error::Error>>

{// 2
let val_1 = jni::objects::JValueGen::Bool(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayName","(ZILjava/util/Locale;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_default(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaTimeZone<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"setDefault",
"(Ljava/util/TimeZone;)V",&[jni::objects::JValueGen::from(&val_1)])?;
Ok(())}
	pub fn id(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getID","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_id(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setID","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn default(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaTimeZone<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/TimeZone")?;
let res = jni.call_static_method(cls,"getDefault",
"()Ljava/util/TimeZone;",&[])?;
let mut obj = res.l()?;
crate::JavaTimeZone::from_raw(&jni,obj
)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::JavaTimeZone<'mc>> for JavaSimpleTimeZone<'mc>{
   fn into(self) -> crate::JavaTimeZone<'mc> {
       crate::JavaTimeZone::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaListResourceBundle<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaListResourceBundle<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaListResourceBundle<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaListResourceBundle from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaListResourceBundle")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaListResourceBundle object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaListResourceBundle<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/ListResourceBundle")?;
let res = jni.new_object(cls,
"()V",&[])?;
crate::JavaListResourceBundle::from_raw(&jni,res
)}
	pub fn keys(&mut self) 
-> Result<crate::JavaEnumeration<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getKeys","()Ljava/util/Enumeration;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaEnumeration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handle_get_object(&mut self,arg0: impl Into<&'mc String>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"handleGetObject","(Ljava/lang/String;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn get_string(&mut self,arg0: impl Into<&'mc String>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"getString","(Ljava/lang/String;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn locale(&mut self) 
-> Result<crate::JavaLocale<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLocale","()Ljava/util/Locale;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocale::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn base_bundle_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBaseBundleName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn contains_key(&mut self,arg0: impl Into<&'mc String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"containsKey","(Ljava/lang/String;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn key_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_object(&mut self,arg0: impl Into<&'mc String>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"getObject","(Ljava/lang/String;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::JavaResourceBundle<'mc>> for JavaListResourceBundle<'mc>{
   fn into(self) -> crate::JavaResourceBundle<'mc> {
       crate::JavaResourceBundle::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaAbstractMapSimpleEntry<'mc, K,V>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where K: JNIRaw<'mc>,V: JNIRaw<'mc>;
impl<'mc, K,V> blackboxmc_general::JNIRaw<'mc> for JavaAbstractMapSimpleEntry<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, K,V> JavaAbstractMapSimpleEntry<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaAbstractMapSimpleEntry from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaAbstractMapSimpleEntry")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaAbstractMapSimpleEntry object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_mapentry(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<K>,arg1: std::option::Option<V>) 
-> Result<crate::JavaAbstractMapSimpleEntry<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
let val_2 = arg1.unwrap().jni_object();
let cls = &jni.find_class("java/util/AbstractMap$SimpleEntry")?;
let res = jni.new_object(cls,
"(LK;LV;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::JavaAbstractMapSimpleEntry::from_raw(&jni,res
)}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn value(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getValue","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn key(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getKey","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn set_value(&mut self,arg0: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"setValue","(LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
/// An instantiatable struct that implements JavaObserver. Needed for returning it from Java.
pub struct JavaObserver<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaObserver<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaObserver from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaObserver")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaObserver object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn update(&mut self,arg0: impl Into<&'mc crate::JavaObservable<'mc>>,arg1: jni::objects::JObject<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = arg1;
let res = self.jni_ref().call_method(&self.jni_object(),"update","(Ljava/util/Observable;Ljava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> JNIRaw<'mc> for JavaObserver<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaEventListener. Needed for returning it from Java.
pub struct JavaEventListener<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaEventListener<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaEventListener from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaEventListener")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaEventListener object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
}
impl<'mc> JNIRaw<'mc> for JavaEventListener<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaQueue. Needed for returning it from Java.
pub struct JavaQueue<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> JavaQueue<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaQueue from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaQueue")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaQueue object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn offer(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"offer","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn add(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn poll(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"poll","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn peek(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"peek","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn element(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"element","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc, E> JNIRaw<'mc> for JavaQueue<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> Into<crate::JavaCollection<'mc, E>> for JavaQueue<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaCollection<'mc, E> {
       crate::JavaCollection::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
/// An instantiatable struct that implements JavaSpliterator. Needed for returning it from Java.
pub struct JavaSpliterator<'mc, T>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where T: JNIRaw<'mc>;
impl<'mc, T> JavaSpliterator<'mc, T> where T: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaSpliterator from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaSpliterator")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaSpliterator object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn estimate_size(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"estimateSize","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn try_split(&mut self) 
-> Result<crate::JavaSpliterator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"trySplit","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn exact_size_if_known(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExactSizeIfKnown","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn has_characteristics(&mut self,arg0: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"hasCharacteristics","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getComparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn characteristics(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"characteristics","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
}
impl<'mc, T> JNIRaw<'mc> for JavaSpliterator<'mc, T> where T: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaEnumeration. Needed for returning it from Java.
pub struct JavaEnumeration<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> JavaEnumeration<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaEnumeration from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaEnumeration")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaEnumeration object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn as_iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asIterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn has_more_elements(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasMoreElements","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn next_element(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextElement","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
}
impl<'mc, E> JNIRaw<'mc> for JavaEnumeration<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaHashtable<'mc, K,V>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where K: JNIRaw<'mc>,V: JNIRaw<'mc>;
impl<'mc, K,V> blackboxmc_general::JNIRaw<'mc> for JavaHashtable<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, K,V> JavaHashtable<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaHashtable from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaHashtable")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaHashtable object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i32>) 
-> Result<crate::JavaHashtable<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let cls = &jni.find_class("java/util/Hashtable")?;
let res = jni.new_object(cls,
"(I)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaHashtable::from_raw(&jni,res
)}
	pub fn new_with_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: std::option::Option<f32>) 
-> Result<crate::JavaHashtable<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
let cls = &jni.find_class("java/util/Hashtable")?;
let res = jni.new_object(cls,
"(IF)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::JavaHashtable::from_raw(&jni,res
)}
	pub fn remove_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = arg1.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"get","(Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn put(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"put","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn values(&mut self) 
-> Result<crate::JavaCollection<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"values","()Ljava/util/Collection;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCollection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn replace_with_object(&mut self,arg0: K,arg1: std::option::Option<V>,arg2: std::option::Option<V>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
let val_2 = arg1.unwrap().jni_object();
let val_3 = arg2.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"replace","(LK;LV;LV;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn elements(&mut self) 
-> Result<crate::JavaEnumeration<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"elements","()Ljava/util/Enumeration;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaEnumeration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entry_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"entrySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn put_all(&mut self,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"putAll","(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn put_if_absent(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"putIfAbsent","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn contains_key(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsKey","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn keys(&mut self) 
-> Result<crate::JavaEnumeration<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keys","()Ljava/util/Enumeration;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaEnumeration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn key_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_value(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsValue","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get_or_default(&mut self,arg0: jni::objects::JObject<'mc>,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"getOrDefault","(Ljava/lang/Object;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, K,V> Into<crate::JavaMap<'mc, K,V>> for JavaHashtable<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc>{
   fn into(self) -> crate::JavaMap<'mc, K,V> {
       crate::JavaMap::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, K,V> Into<crate::JavaDictionary<'mc, K,V>> for JavaHashtable<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc>{
   fn into(self) -> crate::JavaDictionary<'mc, K,V> {
       crate::JavaDictionary::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaCalendar<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaCalendar<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaCalendar<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaCalendar from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaCalendar")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaCalendar object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn get_display_name(&mut self,arg0: i32,arg1: i32,arg2: impl Into<&'mc crate::JavaLocale<'mc>>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayName","(IILjava/util/Locale;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn time(&mut self) 
-> Result<crate::JavaDate<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTime","()Ljava/util/Date;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaDate::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_time(&mut self,arg0: impl Into<&'mc crate::JavaDate<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setTime","(Ljava/util/Date;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn calendar_type(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCalendarType","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn roll_with_int(&mut self,arg0: i32,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"roll","(II)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn get_greatest_minimum(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getGreatestMinimum","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn get_minimum(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getMinimum","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn get_least_maximum(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getLeastMaximum","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn get_maximum(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getMaximum","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn get_display_names(&mut self,arg0: i32,arg1: i32,arg2: impl Into<&'mc crate::JavaLocale<'mc>>) 
-> Result<crate::JavaMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayNames","(IILjava/util/Locale;)Ljava/util/Map;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn available_calendar_types(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Set")?;
let res = jni.call_static_method(cls,"getAvailableCalendarTypes",
"()Ljava/util/Set;",&[])?;
let mut obj = res.l()?;
crate::JavaSet::from_raw(&jni,obj
)}
	pub fn set_time_zone(&mut self,arg0: impl Into<&'mc crate::JavaTimeZone<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setTimeZone","(Ljava/util/TimeZone;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_lenient(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isLenient","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_first_day_of_week(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setFirstDayOfWeek","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn first_day_of_week(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFirstDayOfWeek","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_minimal_days_in_first_week(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setMinimalDaysInFirstWeek","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn minimal_days_in_first_week(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMinimalDaysInFirstWeek","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn is_week_date_supported(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isWeekDateSupported","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn week_year(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getWeekYear","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_week_date(&mut self,arg0: i32,arg1: i32,arg2: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setWeekDate","(III)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn weeks_in_week_year(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getWeeksInWeekYear","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn get_actual_minimum(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getActualMinimum","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn get_actual_maximum(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getActualMaximum","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn before(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"before","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn after(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"after","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_time_in_millis(&mut self,arg0: i64) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setTimeInMillis","(J)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_lenient(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_1 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setLenient","(Z)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn time_in_millis(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTimeInMillis","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn time_zone(&mut self) 
-> Result<crate::JavaTimeZone<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTimeZone","()Ljava/util/TimeZone;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaTimeZone::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add(&mut self,arg0: i32,arg1: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"add","(II)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn get(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"get","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn compare_to_with_calendar(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"compareTo","(Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self,arg0: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"clear","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn instance(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaCalendar<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Calendar")?;
let res = jni.call_static_method(cls,"getInstance",
"()Ljava/util/Calendar;",&[])?;
let mut obj = res.l()?;
crate::JavaCalendar::from_raw(&jni,obj
)}
	pub fn set_with_int(&mut self,arg0: i32,arg1: std::option::Option<i32>,arg2: std::option::Option<i32>,arg3: std::option::Option<i32>,arg4: std::option::Option<i32>,arg5: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"set","(IIIIII)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_set(&mut self,arg0: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"isSet","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaAbstractSet<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> blackboxmc_general::JNIRaw<'mc> for JavaAbstractSet<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> JavaAbstractSet<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaAbstractSet from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaAbstractSet")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaAbstractSet object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn add(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn add_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
}
impl<'mc, E> Into<crate::JavaSet<'mc, E>> for JavaAbstractSet<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaSet<'mc, E> {
       crate::JavaSet::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, E> Into<crate::JavaAbstractCollection<'mc, E>> for JavaAbstractSet<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractCollection<'mc, E> {
       crate::JavaAbstractCollection::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaOptionalInt<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaOptionalInt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaOptionalInt<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaOptionalInt from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaOptionalInt")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaOptionalInt object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn as_int(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAsInt","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32) 
-> Result<crate::JavaOptionalInt<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let cls = &jni.find_class("java/util/OptionalInt")?;
let res = jni.call_static_method(cls,"of",
"(I)Ljava/util/OptionalInt;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaOptionalInt::from_raw(&jni,obj
)}
	pub fn empty(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaOptionalInt<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/OptionalInt")?;
let res = jni.call_static_method(cls,"empty",
"()Ljava/util/OptionalInt;",&[])?;
let mut obj = res.l()?;
crate::JavaOptionalInt::from_raw(&jni,obj
)}
	pub fn is_present(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isPresent","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn or_else<X extends Throwable>(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>
 where X: JNIRaw<'mc>
{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"orElse","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaDoubleSummaryStatistics<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaDoubleSummaryStatistics<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaDoubleSummaryStatistics<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaDoubleSummaryStatistics from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaDoubleSummaryStatistics")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaDoubleSummaryStatistics object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i64>,arg1: std::option::Option<f64>,arg2: std::option::Option<f64>,arg3: std::option::Option<f64>) 
-> Result<crate::JavaDoubleSummaryStatistics<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Double(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Double(arg3.unwrap().into());
let cls = &jni.find_class("java/util/DoubleSummaryStatistics")?;
let res = jni.new_object(cls,
"(JDDD)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::JavaDoubleSummaryStatistics::from_raw(&jni,res
)}
	pub fn count(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCount","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn sum(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSum","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn min(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMin","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn average(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAverage","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn max(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMax","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn accept(&mut self,arg0: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Double(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"accept","(D)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn combine(&mut self,arg0: impl Into<&'mc crate::JavaDoubleSummaryStatistics<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"combine","(Ljava/util/DoubleSummaryStatistics;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaTreeMap<'mc, K,V>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where K: JNIRaw<'mc>,V: JNIRaw<'mc>;
impl<'mc, K,V> blackboxmc_general::JNIRaw<'mc> for JavaTreeMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, K,V> JavaTreeMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaTreeMap from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaTreeMap")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaTreeMap object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaMap<'mc, K,V>>>) 
-> Result<crate::JavaTreeMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/TreeMap")?;
let res = jni.new_object(cls,
"(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaTreeMap::from_raw(&jni,res
)}
	pub fn new_with_comparator(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<crate::JavaTreeMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/TreeMap")?;
let res = jni.new_object(cls,
"(Ljava/util/Comparator;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaTreeMap::from_raw(&jni,res
)}
	pub fn first_key(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"firstKey","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn navigable_key_set(&mut self) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"navigableKeySet","()Ljava/util/NavigableSet;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn descending_key_set(&mut self) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"descendingKeySet","()Ljava/util/NavigableSet;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn descending_map(&mut self) 
-> Result<crate::JavaNavigableMap<'mc, K,V>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"descendingMap","()Ljava/util/NavigableMap;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn sub_map_with_object(&mut self,arg0: K,arg1: std::option::Option<bool>,arg2: std::option::Option<K>,arg3: std::option::Option<bool>) 
-> Result<crate::JavaNavigableMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
// 1
let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
let val_3 = arg2.unwrap().jni_object();
// 1
let val_4 = jni::objects::JValueGen::Bool(arg3.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"subMap","(LK;ZLK;Z)Ljava/util/NavigableMap;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn head_map_with_object(&mut self,arg0: std::option::Option<K>,arg1: std::option::Option<bool>) 
-> Result<crate::JavaNavigableMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
// 0
let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"headMap","(LK;Z)Ljava/util/NavigableMap;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn tail_map_with_object(&mut self,arg0: std::option::Option<K>,arg1: std::option::Option<bool>) 
-> Result<crate::JavaNavigableMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
// 0
let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"tailMap","(LK;Z)Ljava/util/NavigableMap;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn last_key(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"lastKey","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn lower_key(&mut self,arg0: K) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"lowerKey","(LK;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn floor_key(&mut self,arg0: K) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"floorKey","(LK;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn ceiling_key(&mut self,arg0: K) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"ceilingKey","(LK;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn higher_key(&mut self,arg0: K) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"higherKey","(LK;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn poll_first_entry(&mut self) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pollFirstEntry","()Ljava/util/Map$Entry;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn poll_last_entry(&mut self) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pollLastEntry","()Ljava/util/Map$Entry;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn first_entry(&mut self) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"firstEntry","()Ljava/util/Map$Entry;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn last_entry(&mut self) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"lastEntry","()Ljava/util/Map$Entry;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn lower_entry(&mut self,arg0: K) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"lowerEntry","(LK;)Ljava/util/Map$Entry;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn floor_entry(&mut self,arg0: K) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"floorEntry","(LK;)Ljava/util/Map$Entry;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn ceiling_entry(&mut self,arg0: K) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"ceilingEntry","(LK;)Ljava/util/Map$Entry;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn higher_entry(&mut self,arg0: K) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"higherEntry","(LK;)Ljava/util/Map$Entry;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn remove_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = arg1.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"get","(Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn put(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"put","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn values(&mut self) 
-> Result<crate::JavaCollection<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"values","()Ljava/util/Collection;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCollection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn replace_with_object(&mut self,arg0: K,arg1: std::option::Option<V>,arg2: std::option::Option<V>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
let val_2 = arg1.unwrap().jni_object();
let val_3 = arg2.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"replace","(LK;LV;LV;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn entry_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"entrySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn put_all(&mut self,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"putAll","(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn put_if_absent(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"putIfAbsent","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn contains_key(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsKey","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn key_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_value(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsValue","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"comparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn get_or_default(&mut self,arg0: jni::objects::JObject<'mc>,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"getOrDefault","(Ljava/lang/Object;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
}
impl<'mc, K,V> Into<crate::JavaNavigableMap<'mc, K,V>> for JavaTreeMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc>{
   fn into(self) -> crate::JavaNavigableMap<'mc, K,V> {
       crate::JavaNavigableMap::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, K,V> Into<crate::JavaAbstractMap<'mc, K,V>> for JavaTreeMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractMap<'mc, K,V> {
       crate::JavaAbstractMap::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
/// An instantiatable struct that implements JavaDeque. Needed for returning it from Java.
pub struct JavaDeque<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> JavaDeque<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaDeque from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaDeque")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaDeque object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn push(&mut self,arg0: E) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"push","(LE;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn pop(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pop","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn add_first(&mut self,arg0: E) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"addFirst","(LE;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn add_last(&mut self,arg0: E) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"addLast","(LE;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn poll_first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pollFirst","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn poll_last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pollLast","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn offer_last(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"offerLast","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"removeFirst","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFirst","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn peek_first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"peekFirst","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn remove_first_occurrence(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"removeFirstOccurrence","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn offer_first(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"offerFirst","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"removeLast","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLast","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn peek_last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"peekLast","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn remove_last_occurrence(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"removeLastOccurrence","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn offer(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"offer","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn descending_iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"descendingIterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn add_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn poll(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"poll","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn peek(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"peek","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn element(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"element","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc, E> JNIRaw<'mc> for JavaDeque<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> Into<crate::JavaQueue<'mc, E>> for JavaDeque<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaQueue<'mc, E> {
       crate::JavaQueue::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
/// An instantiatable struct that implements JavaFormattable. Needed for returning it from Java.
pub struct JavaFormattable<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaFormattable<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaFormattable from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaFormattable")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaFormattable object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn format_to(&mut self,arg0: impl Into<&'mc crate::JavaFormatter<'mc>>,arg1: i32,arg2: i32,arg3: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_4 = jni::objects::JValueGen::Int(arg3.into());
let res = self.jni_ref().call_method(&self.jni_object(),"formatTo","(Ljava/util/Formatter;III)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> JNIRaw<'mc> for JavaFormattable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaOptionalDouble<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaOptionalDouble<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaOptionalDouble<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaOptionalDouble from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaOptionalDouble")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaOptionalDouble object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn as_double(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAsDouble","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: f64) 
-> Result<crate::JavaOptionalDouble<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Double(arg0.into());
let cls = &jni.find_class("java/util/OptionalDouble")?;
let res = jni.call_static_method(cls,"of",
"(D)Ljava/util/OptionalDouble;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaOptionalDouble::from_raw(&jni,obj
)}
	pub fn empty(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaOptionalDouble<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/OptionalDouble")?;
let res = jni.call_static_method(cls,"empty",
"()Ljava/util/OptionalDouble;",&[])?;
let mut obj = res.l()?;
crate::JavaOptionalDouble::from_raw(&jni,obj
)}
	pub fn is_present(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isPresent","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn or_else<X extends Throwable>(&mut self,arg0: f64) 
-> Result<f64, Box<dyn std::error::Error>>
 where X: JNIRaw<'mc>
{let val_1 = jni::objects::JValueGen::Double(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"orElse","(D)D",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaObservable<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaObservable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaObservable<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaObservable from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaObservable")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaObservable object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaObservable<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Observable")?;
let res = jni.new_object(cls,
"()V",&[])?;
crate::JavaObservable::from_raw(&jni,res
)}
	pub fn notify_observers(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"notifyObservers","(Ljava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn add_observer(&mut self,arg0: impl Into<&'mc crate::JavaObserver<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addObserver","(Ljava/util/Observer;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn delete_observer(&mut self,arg0: impl Into<&'mc crate::JavaObserver<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"deleteObserver","(Ljava/util/Observer;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn delete_observers(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"deleteObservers","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn has_changed(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasChanged","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn count_observers(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"countObservers","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaStringTokenizer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaStringTokenizer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaStringTokenizer<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaStringTokenizer from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaStringTokenizer")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaStringTokenizer object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc String>>,arg1: std::option::Option<impl Into<&'mc String>>,arg2: std::option::Option<bool>) 
-> Result<crate::JavaStringTokenizer<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(jni.new_string(arg0.unwrap().into()).unwrap());
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
// 0
let val_3 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
let cls = &jni.find_class("java/util/StringTokenizer")?;
let res = jni.new_object(cls,
"(Ljava/lang/String;Ljava/lang/String;Z)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::JavaStringTokenizer::from_raw(&jni,res
)}
	pub fn has_more_tokens(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasMoreTokens","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn count_tokens(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"countTokens","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn has_more_elements(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasMoreElements","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn next_token(&mut self,arg0: std::option::Option<impl Into<&'mc String>>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"nextToken","(Ljava/lang/String;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn next_element(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextElement","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn as_iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asIterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
}
impl<'mc> Into<crate::JavaEnumeration<'mc>> for JavaStringTokenizer<'mc>{
   fn into(self) -> crate::JavaEnumeration<'mc> {
       crate::JavaEnumeration::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaOptionalLong<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaOptionalLong<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaOptionalLong<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaOptionalLong from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaOptionalLong")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaOptionalLong object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn as_long(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAsLong","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i64) 
-> Result<crate::JavaOptionalLong<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let cls = &jni.find_class("java/util/OptionalLong")?;
let res = jni.call_static_method(cls,"of",
"(J)Ljava/util/OptionalLong;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaOptionalLong::from_raw(&jni,obj
)}
	pub fn empty(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaOptionalLong<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/OptionalLong")?;
let res = jni.call_static_method(cls,"empty",
"()Ljava/util/OptionalLong;",&[])?;
let mut obj = res.l()?;
crate::JavaOptionalLong::from_raw(&jni,obj
)}
	pub fn is_present(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isPresent","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn or_else<X extends Throwable>(&mut self,arg0: i64) 
-> Result<i64, Box<dyn std::error::Error>>
 where X: JNIRaw<'mc>
{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"orElse","(J)J",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaLinkedHashSet<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> blackboxmc_general::JNIRaw<'mc> for JavaLinkedHashSet<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> JavaLinkedHashSet<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaLinkedHashSet from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaLinkedHashSet")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaLinkedHashSet object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i32>) 
-> Result<crate::JavaLinkedHashSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let cls = &jni.find_class("java/util/LinkedHashSet")?;
let res = jni.new_object(cls,
"(I)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaLinkedHashSet::from_raw(&jni,res
)}
	pub fn new_with_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: std::option::Option<f32>) 
-> Result<crate::JavaLinkedHashSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
let cls = &jni.find_class("java/util/LinkedHashSet")?;
let res = jni.new_object(cls,
"(IF)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::JavaLinkedHashSet::from_raw(&jni,res
)}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn add_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, E> Into<crate::JavaSet<'mc, E>> for JavaLinkedHashSet<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaSet<'mc, E> {
       crate::JavaSet::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, E> Into<crate::JavaHashSet<'mc, E>> for JavaLinkedHashSet<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaHashSet<'mc, E> {
       crate::JavaHashSet::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaLocale<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaLocale<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaLocale<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaLocale from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaLocale")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaLocale object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc String>>,arg1: std::option::Option<impl Into<&'mc String>>,arg2: std::option::Option<impl Into<&'mc String>>) 
-> Result<crate::JavaLocale<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(jni.new_string(arg0.unwrap().into()).unwrap());
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let val_3 = jni::objects::JObject::from(jni.new_string(arg2.unwrap().into()).unwrap());
let cls = &jni.find_class("java/util/Locale")?;
let res = jni.new_object(cls,
"(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::JavaLocale::from_raw(&jni,res
)}
	pub fn display_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn script(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getScript","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn country(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCountry","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn variant(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getVariant","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_default_with_locale(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaLocaleCategory<'mc>>>,arg1: std::option::Option<impl Into<&'mc crate::JavaLocale<'mc>>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"setDefault",
"(Ljava/util/Locale$Category;Ljava/util/Locale;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(())}
	pub fn has_extensions(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasExtensions","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn unicode_locale_attributes(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getUnicodeLocaleAttributes","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_unicode_locale_type(&mut self,arg0: impl Into<&'mc String>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"getUnicodeLocaleType","(Ljava/lang/String;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn unicode_locale_keys(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getUnicodeLocaleKeys","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_display_language(&mut self,arg0: impl Into<&'mc crate::JavaLocale<'mc>>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayLanguage","(Ljava/util/Locale;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn get_display_script(&mut self,arg0: impl Into<&'mc crate::JavaLocale<'mc>>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayScript","(Ljava/util/Locale;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn get_display_country(&mut self,arg0: impl Into<&'mc crate::JavaLocale<'mc>>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayCountry","(Ljava/util/Locale;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn get_display_variant(&mut self,arg0: impl Into<&'mc crate::JavaLocale<'mc>>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayVariant","(Ljava/util/Locale;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn filter_tags_with_list(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>,arg1: std::option::Option<impl Into<&'mc crate::JavaCollection<'mc, E>>>,arg2: std::option::Option<impl Into<&'mc crate::JavaLocaleFilteringMode<'mc>>>) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/List")?;
let res = jni.call_static_method(cls,"filterTags",
"(Ljava/util/List;Ljava/util/Collection;Ljava/util/Locale$FilteringMode;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::JavaList::from_raw(&jni,obj
)}
	pub fn lookup_tag(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>,arg1: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("java/lang/String")?;
let res = jni.call_static_method(cls,"lookupTag",
"(Ljava/util/List;Ljava/util/Collection;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(jni.get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn strip_extensions(&mut self) 
-> Result<crate::JavaLocale<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"stripExtensions","()Ljava/util/Locale;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocale::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_extension(&mut self,arg0: u16) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Char(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getExtension","(C)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn extension_keys(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExtensionKeys","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn to_language_tag(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toLanguageTag","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn for_language_tag(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc String>) 
-> Result<crate::JavaLocale<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
let cls = &jni.find_class("java/util/Locale")?;
let res = jni.call_static_method(cls,"forLanguageTag",
"(Ljava/lang/String;)Ljava/util/Locale;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaLocale::from_raw(&jni,obj
)}
	pub fn iso3language(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getISO3Language","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn iso3country(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getISO3Country","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn language(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLanguage","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn get_default(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaLocaleCategory<'mc>>) 
-> Result<crate::JavaLocale<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Locale")?;
let res = jni.call_static_method(cls,"getDefault",
"(Ljava/util/Locale$Category;)Ljava/util/Locale;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaLocale::from_raw(&jni,obj
)}
	pub fn lookup(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>,arg1: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<crate::JavaLocale<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Locale")?;
let res = jni.call_static_method(cls,"lookup",
"(Ljava/util/List;Ljava/util/Collection;)Ljava/util/Locale;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaLocale::from_raw(&jni,obj
)}
	pub fn filter_with_list(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>,arg1: std::option::Option<impl Into<&'mc crate::JavaCollection<'mc, E>>>,arg2: std::option::Option<impl Into<&'mc crate::JavaLocaleFilteringMode<'mc>>>) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/List")?;
let res = jni.call_static_method(cls,"filter",
"(Ljava/util/List;Ljava/util/Collection;Ljava/util/Locale$FilteringMode;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::JavaList::from_raw(&jni,obj
)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaHashMap<'mc, K,V>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where K: JNIRaw<'mc>,V: JNIRaw<'mc>;
impl<'mc, K,V> blackboxmc_general::JNIRaw<'mc> for JavaHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, K,V> JavaHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaHashMap from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaHashMap")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaHashMap object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i32>) 
-> Result<crate::JavaHashMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let cls = &jni.find_class("java/util/HashMap")?;
let res = jni.new_object(cls,
"(I)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaHashMap::from_raw(&jni,res
)}
	pub fn new_with_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: std::option::Option<f32>) 
-> Result<crate::JavaHashMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
let cls = &jni.find_class("java/util/HashMap")?;
let res = jni.new_object(cls,
"(IF)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::JavaHashMap::from_raw(&jni,res
)}
	pub fn remove_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = arg1.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"get","(Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn put(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"put","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn values(&mut self) 
-> Result<crate::JavaCollection<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"values","()Ljava/util/Collection;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCollection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn replace_with_object(&mut self,arg0: K,arg1: std::option::Option<V>,arg2: std::option::Option<V>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
let val_2 = arg1.unwrap().jni_object();
let val_3 = arg2.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"replace","(LK;LV;LV;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn entry_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"entrySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn put_all(&mut self,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"putAll","(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn put_if_absent(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"putIfAbsent","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn contains_key(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsKey","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn key_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_value(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsValue","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get_or_default(&mut self,arg0: jni::objects::JObject<'mc>,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"getOrDefault","(Ljava/lang/Object;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, K,V> Into<crate::JavaMap<'mc, K,V>> for JavaHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc>{
   fn into(self) -> crate::JavaMap<'mc, K,V> {
       crate::JavaMap::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, K,V> Into<crate::JavaAbstractMap<'mc, K,V>> for JavaHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractMap<'mc, K,V> {
       crate::JavaAbstractMap::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaIdentityHashMap<'mc, K,V>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where K: JNIRaw<'mc>,V: JNIRaw<'mc>;
impl<'mc, K,V> blackboxmc_general::JNIRaw<'mc> for JavaIdentityHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, K,V> JavaIdentityHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaIdentityHashMap from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaIdentityHashMap")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaIdentityHashMap object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaMap<'mc, K,V>>>) 
-> Result<crate::JavaIdentityHashMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/IdentityHashMap")?;
let res = jni.new_object(cls,
"(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaIdentityHashMap::from_raw(&jni,res
)}
	pub fn remove_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = arg1.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"get","(Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn put(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"put","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn values(&mut self) 
-> Result<crate::JavaCollection<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"values","()Ljava/util/Collection;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCollection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn entry_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"entrySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn put_all(&mut self,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"putAll","(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn contains_key(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsKey","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn key_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_value(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsValue","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn replace_with_object(&mut self,arg0: K,arg1: std::option::Option<V>,arg2: std::option::Option<V>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
let val_2 = arg1.unwrap().jni_object();
let val_3 = arg2.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"replace","(LK;LV;LV;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn put_if_absent(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"putIfAbsent","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn get_or_default(&mut self,arg0: jni::objects::JObject<'mc>,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"getOrDefault","(Ljava/lang/Object;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
}
impl<'mc, K,V> Into<crate::JavaMap<'mc, K,V>> for JavaIdentityHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc>{
   fn into(self) -> crate::JavaMap<'mc, K,V> {
       crate::JavaMap::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, K,V> Into<crate::JavaAbstractMap<'mc, K,V>> for JavaIdentityHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractMap<'mc, K,V> {
       crate::JavaAbstractMap::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
/// An instantiatable struct that implements JavaSpliteratorOfPrimitive. Needed for returning it from Java.
pub struct JavaSpliteratorOfPrimitive<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaSpliteratorOfPrimitive<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaSpliteratorOfPrimitive from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaSpliteratorOfPrimitive")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaSpliteratorOfPrimitive object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn try_split(&mut self) 
-> Result<crate::JavaSpliteratorOfPrimitive<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"trySplit","()Ljava/util/Spliterator$OfPrimitive;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliteratorOfPrimitive::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn estimate_size(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"estimateSize","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn exact_size_if_known(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExactSizeIfKnown","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn has_characteristics(&mut self,arg0: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"hasCharacteristics","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getComparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn characteristics(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"characteristics","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
}
impl<'mc> JNIRaw<'mc> for JavaSpliteratorOfPrimitive<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaLinkedHashMap<'mc, K,V>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where K: JNIRaw<'mc>,V: JNIRaw<'mc>;
impl<'mc, K,V> blackboxmc_general::JNIRaw<'mc> for JavaLinkedHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, K,V> JavaLinkedHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaLinkedHashMap from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaLinkedHashMap")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaLinkedHashMap object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaMap<'mc, K,V>>>) 
-> Result<crate::JavaLinkedHashMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/LinkedHashMap")?;
let res = jni.new_object(cls,
"(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaLinkedHashMap::from_raw(&jni,res
)}
	pub fn new_with_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: std::option::Option<f32>,arg2: std::option::Option<bool>) 
-> Result<crate::JavaLinkedHashMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
// 1
let val_3 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
let cls = &jni.find_class("java/util/LinkedHashMap")?;
let res = jni.new_object(cls,
"(IFZ)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::JavaLinkedHashMap::from_raw(&jni,res
)}
	pub fn get(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"get","(Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn values(&mut self) 
-> Result<crate::JavaCollection<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"values","()Ljava/util/Collection;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCollection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn entry_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"entrySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn key_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_value(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsValue","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get_or_default(&mut self,arg0: jni::objects::JObject<'mc>,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"getOrDefault","(Ljava/lang/Object;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn remove_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = arg1.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn put(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"put","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn replace_with_object(&mut self,arg0: K,arg1: std::option::Option<V>,arg2: std::option::Option<V>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
let val_2 = arg1.unwrap().jni_object();
let val_3 = arg2.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"replace","(LK;LV;LV;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn put_all(&mut self,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"putAll","(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn put_if_absent(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"putIfAbsent","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn contains_key(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsKey","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, K,V> Into<crate::JavaMap<'mc, K,V>> for JavaLinkedHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc>{
   fn into(self) -> crate::JavaMap<'mc, K,V> {
       crate::JavaMap::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, K,V> Into<crate::JavaHashMap<'mc, K,V>> for JavaLinkedHashMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc>{
   fn into(self) -> crate::JavaHashMap<'mc, K,V> {
       crate::JavaHashMap::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaAbstractSequentialList<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> blackboxmc_general::JNIRaw<'mc> for JavaAbstractSequentialList<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> JavaAbstractSequentialList<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaAbstractSequentialList from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaAbstractSequentialList")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaAbstractSequentialList object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn add_with_object(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<E>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = arg1.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(ILE;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_with_int(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: i32) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"get","(I)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_all_with_collection(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<impl Into<&'mc crate::JavaCollection<'mc, E>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(ILjava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set(&mut self,arg0: i32,arg1: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"set","(ILE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn list_iterator(&mut self,arg0: std::option::Option<i32>) 
-> Result<crate::JavaListIterator<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"listIterator","(I)Ljava/util/ListIterator;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaListIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn index_of(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"indexOf","(Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn last_index_of(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"lastIndexOf","(Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn sub_list(&mut self,arg0: i32,arg1: i32) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"subList","(II)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn sort(&mut self,arg0: impl Into<&'mc crate::JavaComparator<'mc, E>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"sort","(Ljava/util/Comparator;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, E> Into<crate::JavaAbstractList<'mc, E>> for JavaAbstractSequentialList<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractList<'mc, E> {
       crate::JavaAbstractList::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
/// An instantiatable struct that implements JavaComparator. Needed for returning it from Java.
pub struct JavaComparator<'mc, T>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where T: JNIRaw<'mc>;
impl<'mc, T> JavaComparator<'mc, T> where T: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaComparator from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaComparator")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaComparator object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn compare(&mut self,arg0: T,arg1: T) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"compare","(LT;LT;)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn reverse_order(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Comparator")?;
let res = jni.call_static_method(cls,"reverseOrder",
"()Ljava/util/Comparator;",&[])?;
let mut obj = res.l()?;
crate::JavaComparator::from_raw(&jni,obj
)}
	pub fn reversed(&mut self) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"reversed","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn natural_order(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Comparator")?;
let res = jni.call_static_method(cls,"naturalOrder",
"()Ljava/util/Comparator;",&[])?;
let mut obj = res.l()?;
crate::JavaComparator::from_raw(&jni,obj
)}
	pub fn nulls_first<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaComparator<'mc, T>>) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Comparator")?;
let res = jni.call_static_method(cls,"nullsFirst",
"(Ljava/util/Comparator;)Ljava/util/Comparator;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaComparator::from_raw(&jni,obj
)}
	pub fn nulls_last<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaComparator<'mc, T>>) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Comparator")?;
let res = jni.call_static_method(cls,"nullsLast",
"(Ljava/util/Comparator;)Ljava/util/Comparator;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaComparator::from_raw(&jni,obj
)}
}
impl<'mc, T> JNIRaw<'mc> for JavaComparator<'mc, T> where T: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaBase64Encoder<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaBase64Encoder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaBase64Encoder<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaBase64Encoder from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaBase64Encoder")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaBase64Encoder object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn encode_to_string(&mut self,arg0: Vec<i8>) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"encodeToString","(B)Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn without_padding(&mut self) 
-> Result<crate::JavaBase64Encoder<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"withoutPadding","()Ljava/util/Base64$Encoder;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaBase64Encoder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn encode_with_bytes(&mut self,arg0: Vec<i8>,arg1: std::option::Option<Vec<i8>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"encode","(BB)I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaRandom<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaRandom<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaRandom<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaRandom from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaRandom")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaRandom object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i64>) 
-> Result<crate::JavaRandom<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let cls = &jni.find_class("java/util/Random")?;
let res = jni.new_object(cls,
"(J)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaRandom::from_raw(&jni,res
)}
	pub fn next_boolean(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextBoolean","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn next_long(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i64>) 
-> Result<i64, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextLong","(JJ)J",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn next_float(&mut self,arg0: std::option::Option<f32>,arg1: std::option::Option<f32>) 
-> Result<f32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Float(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextFloat","(FF)F",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.f().unwrap())}
	pub fn set_seed(&mut self,arg0: i64) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSeed","(J)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn next_bytes(&mut self,arg0: Vec<i8>) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextBytes","(B)V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn next_gaussian(&mut self,arg0: std::option::Option<f64>,arg1: std::option::Option<f64>) 
-> Result<f64, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextGaussian","(DD)D",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn next_double(&mut self,arg0: std::option::Option<f64>,arg1: std::option::Option<f64>) 
-> Result<f64, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextDouble","(DD)D",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn next_int(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextInt","(II)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_deprecated(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isDeprecated","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn next_exponential(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextExponential","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
}
impl<'mc> Into<crate::random::JavaRandomGenerator<'mc>> for JavaRandom<'mc>{
   fn into(self) -> crate::random::JavaRandomGenerator<'mc> {
       crate::random::JavaRandomGenerator::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
/// An instantiatable struct that implements JavaList. Needed for returning it from Java.
pub struct JavaList<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> JavaList<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaList from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaList")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaList object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn add_with_object(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<E>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = arg1.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(ILE;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_with_object(&mut self,arg0: std::option::Option<i32>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(I)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn get(&mut self,arg0: i32) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"get","(I)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn copy_of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/List")?;
let res = jni.call_static_method(cls,"copyOf",
"(Ljava/util/Collection;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaList::from_raw(&jni,obj
)}
	pub fn index_of(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"indexOf","(Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn last_index_of(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"lastIndexOf","(Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn sub_list(&mut self,arg0: i32,arg1: i32) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"subList","(II)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<jni::objects::JObject<'mc>>>) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/List")?;
let res = jni.call_static_method(cls,"of",
"(Ljava/lang/Object;)Ljava/util/List;",&[])?;
let mut obj = res.l()?;
crate::JavaList::from_raw(&jni,obj
)}
	pub fn of_with_object(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: E,arg1: E,arg2: E,arg3: E,arg4: E,arg5: E,arg6: E,arg7: E,arg8: E,arg9: std::option::Option<E>) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let val_3 = arg2.jni_object();
let val_4 = arg3.jni_object();
let val_5 = arg4.jni_object();
let val_6 = arg5.jni_object();
let val_7 = arg6.jni_object();
let val_8 = arg7.jni_object();
let val_9 = arg8.unwrap().jni_object();
let val_10 = arg9.unwrap().jni_object();
let cls = &jni.find_class("java/util/List")?;
let res = jni.call_static_method(cls,"of",
"(LE;LE;LE;LE;LE;LE;LE;LE;LE;LE;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6),jni::objects::JValueGen::from(&val_7),jni::objects::JValueGen::from(&val_8),jni::objects::JValueGen::from(&val_9),jni::objects::JValueGen::from(&val_10)])?;
let mut obj = res.l()?;
crate::JavaList::from_raw(&jni,obj
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_all_with_collection(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<impl Into<&'mc crate::JavaCollection<'mc, E>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(ILjava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set(&mut self,arg0: i32,arg1: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"set","(ILE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn sort(&mut self,arg0: impl Into<&'mc crate::JavaComparator<'mc, E>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"sort","(Ljava/util/Comparator;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn list_iterator(&mut self,arg0: std::option::Option<i32>) 
-> Result<crate::JavaListIterator<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"listIterator","(I)Ljava/util/ListIterator;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaListIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc, E> JNIRaw<'mc> for JavaList<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> Into<crate::JavaCollection<'mc, E>> for JavaList<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaCollection<'mc, E> {
       crate::JavaCollection::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaUUID<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaUUID<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaUUID<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaUUID from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaUUID")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaUUID object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn least_significant_bits(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLeastSignificantBits","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn most_significant_bits(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMostSignificantBits","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn clock_sequence(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clockSequence","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn variant(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"variant","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn version(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"version","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn compare_to_with_object(&mut self,arg0: std::option::Option<u128>) 
-> Result<i32, Box<dyn std::error::Error>>

{let upper = (arg0.unwrap() >> 64) as u64 as i64;
let lower = arg0.unwrap() as u64 as i64;
let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object("java/util/UUID", "(JJ)V", &[upper.into(), lower.into()]).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"compareTo","(Ljava/util/UUID;)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn timestamp(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"timestamp","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn node(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"node","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaCollections<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaCollections<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaCollections<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaCollections from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaCollections")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaCollections object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn shuffle_with_list(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaList<'mc, E>>>,arg1: std::option::Option<impl Into<&'mc crate::JavaRandom<'mc>>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"shuffle",
"(Ljava/util/List;Ljava/util/Random;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(())}
	pub fn rotate(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>,arg1: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"rotate",
"(Ljava/util/List;I)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(())}
	pub fn index_of_sub_list(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>,arg1: impl Into<&'mc crate::JavaList<'mc, E>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"indexOfSubList",
"(Ljava/util/List;Ljava/util/List;)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(res.i().unwrap())}
	pub fn last_index_of_sub_list(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>,arg1: impl Into<&'mc crate::JavaList<'mc, E>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"lastIndexOfSubList",
"(Ljava/util/List;Ljava/util/List;)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(res.i().unwrap())}
	pub fn unmodifiable_collection<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<crate::JavaCollection<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Collection")?;
let res = jni.call_static_method(cls,"unmodifiableCollection",
"(Ljava/util/Collection;)Ljava/util/Collection;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaCollection::from_raw(&jni,obj
)}
	pub fn unmodifiable_sorted_set<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaSortedSet<'mc, E>>) 
-> Result<crate::JavaSortedSet<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/SortedSet")?;
let res = jni.call_static_method(cls,"unmodifiableSortedSet",
"(Ljava/util/SortedSet;)Ljava/util/SortedSet;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaSortedSet::from_raw(&jni,obj
)}
	pub fn unmodifiable_navigable_set<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaNavigableSet<'mc, E>>) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/NavigableSet")?;
let res = jni.call_static_method(cls,"unmodifiableNavigableSet",
"(Ljava/util/NavigableSet;)Ljava/util/NavigableSet;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaNavigableSet::from_raw(&jni,obj
)}
	pub fn unmodifiable_sorted_map<K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaSortedMap<'mc, K,V>>) 
-> Result<crate::JavaSortedMap<'mc, K,V>, Box<dyn std::error::Error>>
 where K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/SortedMap")?;
let res = jni.call_static_method(cls,"unmodifiableSortedMap",
"(Ljava/util/SortedMap;)Ljava/util/SortedMap;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaSortedMap::from_raw(&jni,obj
)}
	pub fn unmodifiable_navigable_map<K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaNavigableMap<'mc, K,V>>) 
-> Result<crate::JavaNavigableMap<'mc, K,V>, Box<dyn std::error::Error>>
 where K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/NavigableMap")?;
let res = jni.call_static_method(cls,"unmodifiableNavigableMap",
"(Ljava/util/NavigableMap;)Ljava/util/NavigableMap;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaNavigableMap::from_raw(&jni,obj
)}
	pub fn synchronized_sorted_set<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaSortedSet<'mc, E>>) 
-> Result<crate::JavaSortedSet<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/SortedSet")?;
let res = jni.call_static_method(cls,"synchronizedSortedSet",
"(Ljava/util/SortedSet;)Ljava/util/SortedSet;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaSortedSet::from_raw(&jni,obj
)}
	pub fn synchronized_navigable_set<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaNavigableSet<'mc, E>>) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/NavigableSet")?;
let res = jni.call_static_method(cls,"synchronizedNavigableSet",
"(Ljava/util/NavigableSet;)Ljava/util/NavigableSet;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaNavigableSet::from_raw(&jni,obj
)}
	pub fn synchronized_list<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/List")?;
let res = jni.call_static_method(cls,"synchronizedList",
"(Ljava/util/List;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaList::from_raw(&jni,obj
)}
	pub fn synchronized_map<K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<crate::JavaMap<'mc, K,V>, Box<dyn std::error::Error>>
 where K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Map")?;
let res = jni.call_static_method(cls,"synchronizedMap",
"(Ljava/util/Map;)Ljava/util/Map;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaMap::from_raw(&jni,obj
)}
	pub fn synchronized_sorted_map<K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaSortedMap<'mc, K,V>>) 
-> Result<crate::JavaSortedMap<'mc, K,V>, Box<dyn std::error::Error>>
 where K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/SortedMap")?;
let res = jni.call_static_method(cls,"synchronizedSortedMap",
"(Ljava/util/SortedMap;)Ljava/util/SortedMap;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaSortedMap::from_raw(&jni,obj
)}
	pub fn synchronized_navigable_map<K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaNavigableMap<'mc, K,V>>) 
-> Result<crate::JavaNavigableMap<'mc, K,V>, Box<dyn std::error::Error>>
 where K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/NavigableMap")?;
let res = jni.call_static_method(cls,"synchronizedNavigableMap",
"(Ljava/util/NavigableMap;)Ljava/util/NavigableMap;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaNavigableMap::from_raw(&jni,obj
)}
	pub fn checked_collection<E>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>,arg1: jni::objects::JClass<'mc>) 
-> Result<crate::JavaCollection<'mc, E>, Box<dyn std::error::Error>>
 where E: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = arg1;
let cls = &jni.find_class("java/util/Collection")?;
let res = jni.call_static_method(cls,"checkedCollection",
"(Ljava/util/Collection;Ljava/lang/Class;)Ljava/util/Collection;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaCollection::from_raw(&jni,obj
)}
	pub fn checked_queue<E>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaQueue<'mc, E>>,arg1: jni::objects::JClass<'mc>) 
-> Result<crate::JavaQueue<'mc, E>, Box<dyn std::error::Error>>
 where E: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = arg1;
let cls = &jni.find_class("java/util/Queue")?;
let res = jni.call_static_method(cls,"checkedQueue",
"(Ljava/util/Queue;Ljava/lang/Class;)Ljava/util/Queue;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaQueue::from_raw(&jni,obj
)}
	pub fn checked_set<E>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaSet<'mc, E>>,arg1: jni::objects::JClass<'mc>) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>
 where E: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = arg1;
let cls = &jni.find_class("java/util/Set")?;
let res = jni.call_static_method(cls,"checkedSet",
"(Ljava/util/Set;Ljava/lang/Class;)Ljava/util/Set;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaSet::from_raw(&jni,obj
)}
	pub fn checked_sorted_set<E>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaSortedSet<'mc, E>>,arg1: jni::objects::JClass<'mc>) 
-> Result<crate::JavaSortedSet<'mc, E>, Box<dyn std::error::Error>>
 where E: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = arg1;
let cls = &jni.find_class("java/util/SortedSet")?;
let res = jni.call_static_method(cls,"checkedSortedSet",
"(Ljava/util/SortedSet;Ljava/lang/Class;)Ljava/util/SortedSet;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaSortedSet::from_raw(&jni,obj
)}
	pub fn checked_navigable_set<E>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaNavigableSet<'mc, E>>,arg1: jni::objects::JClass<'mc>) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>
 where E: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = arg1;
let cls = &jni.find_class("java/util/NavigableSet")?;
let res = jni.call_static_method(cls,"checkedNavigableSet",
"(Ljava/util/NavigableSet;Ljava/lang/Class;)Ljava/util/NavigableSet;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaNavigableSet::from_raw(&jni,obj
)}
	pub fn checked_list<E>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>,arg1: jni::objects::JClass<'mc>) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>
 where E: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = arg1;
let cls = &jni.find_class("java/util/List")?;
let res = jni.call_static_method(cls,"checkedList",
"(Ljava/util/List;Ljava/lang/Class;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaList::from_raw(&jni,obj
)}
	pub fn checked_map<K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>,arg1: jni::objects::JClass<'mc>,arg2: jni::objects::JClass<'mc>) 
-> Result<crate::JavaMap<'mc, K,V>, Box<dyn std::error::Error>>
 where K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = arg1;
let val_3 = arg2;
let cls = &jni.find_class("java/util/Map")?;
let res = jni.call_static_method(cls,"checkedMap",
"(Ljava/util/Map;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/Map;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::JavaMap::from_raw(&jni,obj
)}
	pub fn checked_sorted_map<K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaSortedMap<'mc, K,V>>,arg1: jni::objects::JClass<'mc>,arg2: jni::objects::JClass<'mc>) 
-> Result<crate::JavaSortedMap<'mc, K,V>, Box<dyn std::error::Error>>
 where K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = arg1;
let val_3 = arg2;
let cls = &jni.find_class("java/util/SortedMap")?;
let res = jni.call_static_method(cls,"checkedSortedMap",
"(Ljava/util/SortedMap;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/SortedMap;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::JavaSortedMap::from_raw(&jni,obj
)}
	pub fn checked_navigable_map<K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaNavigableMap<'mc, K,V>>,arg1: jni::objects::JClass<'mc>,arg2: jni::objects::JClass<'mc>) 
-> Result<crate::JavaNavigableMap<'mc, K,V>, Box<dyn std::error::Error>>
 where K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = arg1;
let val_3 = arg2;
let cls = &jni.find_class("java/util/NavigableMap")?;
let res = jni.call_static_method(cls,"checkedNavigableMap",
"(Ljava/util/NavigableMap;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/NavigableMap;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::JavaNavigableMap::from_raw(&jni,obj
)}
	pub fn empty_list_iterator<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaListIterator<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let cls = &jni.find_class("java/util/ListIterator")?;
let res = jni.call_static_method(cls,"emptyListIterator",
"()Ljava/util/ListIterator;",&[])?;
let mut obj = res.l()?;
crate::JavaListIterator::from_raw(&jni,obj
)}
	pub fn empty_set<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let cls = &jni.find_class("java/util/Set")?;
let res = jni.call_static_method(cls,"emptySet",
"()Ljava/util/Set;",&[])?;
let mut obj = res.l()?;
crate::JavaSet::from_raw(&jni,obj
)}
	pub fn empty_sorted_set<E>(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaSortedSet<'mc, E>, Box<dyn std::error::Error>>
 where E: JNIRaw<'mc>
{let cls = &jni.find_class("java/util/SortedSet")?;
let res = jni.call_static_method(cls,"emptySortedSet",
"()Ljava/util/SortedSet;",&[])?;
let mut obj = res.l()?;
crate::JavaSortedSet::from_raw(&jni,obj
)}
	pub fn empty_navigable_set<E>(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>
 where E: JNIRaw<'mc>
{let cls = &jni.find_class("java/util/NavigableSet")?;
let res = jni.call_static_method(cls,"emptyNavigableSet",
"()Ljava/util/NavigableSet;",&[])?;
let mut obj = res.l()?;
crate::JavaNavigableSet::from_raw(&jni,obj
)}
	pub fn empty_map<K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaMap<'mc, K,V>, Box<dyn std::error::Error>>
 where K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let cls = &jni.find_class("java/util/Map")?;
let res = jni.call_static_method(cls,"emptyMap",
"()Ljava/util/Map;",&[])?;
let mut obj = res.l()?;
crate::JavaMap::from_raw(&jni,obj
)}
	pub fn empty_sorted_map<K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaSortedMap<'mc, K,V>, Box<dyn std::error::Error>>
 where K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let cls = &jni.find_class("java/util/SortedMap")?;
let res = jni.call_static_method(cls,"emptySortedMap",
"()Ljava/util/SortedMap;",&[])?;
let mut obj = res.l()?;
crate::JavaSortedMap::from_raw(&jni,obj
)}
	pub fn empty_navigable_map<K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaNavigableMap<'mc, K,V>, Box<dyn std::error::Error>>
 where K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let cls = &jni.find_class("java/util/NavigableMap")?;
let res = jni.call_static_method(cls,"emptyNavigableMap",
"()Ljava/util/NavigableMap;",&[])?;
let mut obj = res.l()?;
crate::JavaNavigableMap::from_raw(&jni,obj
)}
	pub fn singleton<T,T,K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: T) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>,K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let val_1 = arg0.jni_object();
let cls = &jni.find_class("java/util/Set")?;
let res = jni.call_static_method(cls,"singleton",
"(LT;)Ljava/util/Set;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaSet::from_raw(&jni,obj
)}
	pub fn singleton_list<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: T) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = arg0.jni_object();
let cls = &jni.find_class("java/util/List")?;
let res = jni.call_static_method(cls,"singletonList",
"(LT;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaList::from_raw(&jni,obj
)}
	pub fn singleton_map<K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: K,arg1: V) 
-> Result<crate::JavaMap<'mc, K,V>, Box<dyn std::error::Error>>
 where K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let cls = &jni.find_class("java/util/Map")?;
let res = jni.call_static_method(cls,"singletonMap",
"(LK;LV;)Ljava/util/Map;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaMap::from_raw(&jni,obj
)}
	pub fn frequency(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>,arg1: jni::objects::JObject<'mc>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = arg1;
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"frequency",
"(Ljava/util/Collection;Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(res.i().unwrap())}
	pub fn disjoint(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>,arg1: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("boolean")?;
let res = jni.call_static_method(cls,"disjoint",
"(Ljava/util/Collection;Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(res.z().unwrap())}
	pub fn as_lifo_queue<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaDeque<'mc, E>>) 
-> Result<crate::JavaQueue<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Queue")?;
let res = jni.call_static_method(cls,"asLifoQueue",
"(Ljava/util/Deque;)Ljava/util/Queue;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaQueue::from_raw(&jni,obj
)}
	pub fn unmodifiable_map<K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<crate::JavaMap<'mc, K,V>, Box<dyn std::error::Error>>
 where K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Map")?;
let res = jni.call_static_method(cls,"unmodifiableMap",
"(Ljava/util/Map;)Ljava/util/Map;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaMap::from_raw(&jni,obj
)}
	pub fn swap(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>,arg1: i32,arg2: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"swap",
"(Ljava/util/List;II)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
Ok(())}
	pub fn binary_search_with_list(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>,arg1: std::option::Option<T>,arg2: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = arg1.unwrap().jni_object();
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"binarySearch",
"(Ljava/util/List;LT;Ljava/util/Comparator;)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
Ok(res.i().unwrap())}
	pub fn min_with_collection(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaCollection<'mc, E>>>,arg1: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/lang/Object")?;
let res = jni.call_static_method(cls,"min",
"(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(res.l().unwrap())}
	pub fn max_with_collection(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaCollection<'mc, E>>>,arg1: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/lang/Object")?;
let res = jni.call_static_method(cls,"max",
"(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(res.l().unwrap())}
	pub fn replace_all(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>,arg1: T,arg2: T) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = arg1.jni_object();
let val_3 = arg2.jni_object();
let cls = &jni.find_class("boolean")?;
let res = jni.call_static_method(cls,"replaceAll",
"(Ljava/util/List;LT;LT;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
Ok(res.z().unwrap())}
	pub fn fill(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>,arg1: T) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = arg1.jni_object();
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"fill",
"(Ljava/util/List;LT;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(())}
	pub fn list<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaEnumeration<'mc, E>>) 
-> Result<crate::JavaArrayList<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/ArrayList")?;
let res = jni.call_static_method(cls,"list",
"(Ljava/util/Enumeration;)Ljava/util/ArrayList;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaArrayList::from_raw(&jni,obj
)}
	pub fn add_all(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>,arg1: Vec<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("boolean")?;
let res = jni.call_static_method(cls,"addAll",
"(Ljava/util/Collection;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)])?;
Ok(res.z().unwrap())}
	pub fn empty_enumeration<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaEnumeration<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let cls = &jni.find_class("java/util/Enumeration")?;
let res = jni.call_static_method(cls,"emptyEnumeration",
"()Ljava/util/Enumeration;",&[])?;
let mut obj = res.l()?;
crate::JavaEnumeration::from_raw(&jni,obj
)}
	pub fn new_set_from_map<E>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>
 where E: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Set")?;
let res = jni.call_static_method(cls,"newSetFromMap",
"(Ljava/util/Map;)Ljava/util/Set;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaSet::from_raw(&jni,obj
)}
	pub fn empty_list<T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let cls = &jni.find_class("java/util/List")?;
let res = jni.call_static_method(cls,"emptyList",
"()Ljava/util/List;",&[])?;
let mut obj = res.l()?;
crate::JavaList::from_raw(&jni,obj
)}
	pub fn unmodifiable_set<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaSet<'mc, E>>) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Set")?;
let res = jni.call_static_method(cls,"unmodifiableSet",
"(Ljava/util/Set;)Ljava/util/Set;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaSet::from_raw(&jni,obj
)}
	pub fn enumeration<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<crate::JavaEnumeration<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Enumeration")?;
let res = jni.call_static_method(cls,"enumeration",
"(Ljava/util/Collection;)Ljava/util/Enumeration;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaEnumeration::from_raw(&jni,obj
)}
	pub fn synchronized_set<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaSet<'mc, E>>) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Set")?;
let res = jni.call_static_method(cls,"synchronizedSet",
"(Ljava/util/Set;)Ljava/util/Set;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaSet::from_raw(&jni,obj
)}
	pub fn synchronized_collection<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<crate::JavaCollection<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Collection")?;
let res = jni.call_static_method(cls,"synchronizedCollection",
"(Ljava/util/Collection;)Ljava/util/Collection;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaCollection::from_raw(&jni,obj
)}
	pub fn empty_iterator<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let cls = &jni.find_class("java/util/Iterator")?;
let res = jni.call_static_method(cls,"emptyIterator",
"()Ljava/util/Iterator;",&[])?;
let mut obj = res.l()?;
crate::JavaIterator::from_raw(&jni,obj
)}
	pub fn copy(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>,arg1: impl Into<&'mc crate::JavaList<'mc, E>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"copy",
"(Ljava/util/List;Ljava/util/List;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(())}
	pub fn n_copies<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: T) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = arg1.jni_object();
let cls = &jni.find_class("java/util/List")?;
let res = jni.call_static_method(cls,"nCopies",
"(ILT;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaList::from_raw(&jni,obj
)}
	pub fn unmodifiable_list<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/List")?;
let res = jni.call_static_method(cls,"unmodifiableList",
"(Ljava/util/List;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaList::from_raw(&jni,obj
)}
	pub fn reverse<T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>) 
-> Result<(), Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"reverse",
"(Ljava/util/List;)V",&[jni::objects::JValueGen::from(&val_1)])?;
Ok(())}
	pub fn sort_with_list(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaList<'mc, E>>>,arg1: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("void")?;
let res = jni.call_static_method(cls,"sort",
"(Ljava/util/List;Ljava/util/Comparator;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(())}
	pub fn reverse_order<T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/Comparator")?;
let res = jni.call_static_method(cls,"reverseOrder",
"(Ljava/util/Comparator;)Ljava/util/Comparator;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaComparator::from_raw(&jni,obj
)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaHashSet<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> blackboxmc_general::JNIRaw<'mc> for JavaHashSet<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> JavaHashSet<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaHashSet from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaHashSet")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaHashSet object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaCollection<'mc, E>>>) 
-> Result<crate::JavaHashSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/HashSet")?;
let res = jni.new_object(cls,
"(Ljava/util/Collection;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaHashSet::from_raw(&jni,res
)}
	pub fn new_with_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: std::option::Option<f32>) 
-> Result<crate::JavaHashSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
let cls = &jni.find_class("java/util/HashSet")?;
let res = jni.new_object(cls,
"(IF)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::JavaHashSet::from_raw(&jni,res
)}
	pub fn add(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn add_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, E> Into<crate::JavaSet<'mc, E>> for JavaHashSet<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaSet<'mc, E> {
       crate::JavaSet::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, E> Into<crate::JavaAbstractSet<'mc, E>> for JavaHashSet<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractSet<'mc, E> {
       crate::JavaAbstractSet::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaAbstractList<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> blackboxmc_general::JNIRaw<'mc> for JavaAbstractList<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> JavaAbstractList<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaAbstractList from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaAbstractList")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaAbstractList object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn add_with_object(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<E>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = arg1.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(ILE;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_with_int(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: i32) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"get","(I)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn index_of(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"indexOf","(Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn last_index_of(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"lastIndexOf","(Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn sub_list(&mut self,arg0: i32,arg1: i32) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"subList","(II)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_all_with_collection(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<impl Into<&'mc crate::JavaCollection<'mc, E>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(ILjava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set(&mut self,arg0: i32,arg1: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"set","(ILE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn list_iterator(&mut self,arg0: std::option::Option<i32>) 
-> Result<crate::JavaListIterator<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"listIterator","(I)Ljava/util/ListIterator;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaListIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn sort(&mut self,arg0: impl Into<&'mc crate::JavaComparator<'mc, E>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"sort","(Ljava/util/Comparator;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, E> Into<crate::JavaList<'mc, E>> for JavaAbstractList<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaList<'mc, E> {
       crate::JavaList::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, E> Into<crate::JavaAbstractCollection<'mc, E>> for JavaAbstractList<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractCollection<'mc, E> {
       crate::JavaAbstractCollection::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaOptional<'mc, T>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where T: JNIRaw<'mc>;
impl<'mc, T> blackboxmc_general::JNIRaw<'mc> for JavaOptional<'mc, T> where T: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, T> JavaOptional<'mc, T> where T: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaOptional from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaOptional")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaOptional object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn of_nullable<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: T) 
-> Result<crate::JavaOptional<'mc, T>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = arg0.jni_object();
let cls = &jni.find_class("java/util/Optional")?;
let res = jni.call_static_method(cls,"ofNullable",
"(LT;)Ljava/util/Optional;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaOptional::from_raw(&jni,obj
)}
	pub fn get(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"get","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn of<T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: T) 
-> Result<crate::JavaOptional<'mc, T>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let val_1 = arg0.jni_object();
let cls = &jni.find_class("java/util/Optional")?;
let res = jni.call_static_method(cls,"of",
"(LT;)Ljava/util/Optional;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaOptional::from_raw(&jni,obj
)}
	pub fn empty<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaOptional<'mc, T>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let cls = &jni.find_class("java/util/Optional")?;
let res = jni.call_static_method(cls,"empty",
"()Ljava/util/Optional;",&[])?;
let mut obj = res.l()?;
crate::JavaOptional::from_raw(&jni,obj
)}
	pub fn is_present(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isPresent","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn or_else(&mut self,arg0: T) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"orElse","(LT;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaBitSet<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaBitSet<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaBitSet<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaBitSet from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaBitSet")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaBitSet object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i32>) 
-> Result<crate::JavaBitSet<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let cls = &jni.find_class("java/util/BitSet")?;
let res = jni.new_object(cls,
"(I)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaBitSet::from_raw(&jni,res
)}
	pub fn or(&mut self,arg0: impl Into<&'mc crate::JavaBitSet<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"or","(Ljava/util/BitSet;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn and(&mut self,arg0: impl Into<&'mc crate::JavaBitSet<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"and","(Ljava/util/BitSet;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn cardinality(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"cardinality","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn next_set_bit(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextSetBit","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn previous_set_bit(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"previousSetBit","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn previous_clear_bit(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"previousClearBit","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn intersects(&mut self,arg0: impl Into<&'mc crate::JavaBitSet<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"intersects","(Ljava/util/BitSet;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn xor(&mut self,arg0: impl Into<&'mc crate::JavaBitSet<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"xor","(Ljava/util/BitSet;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn and_not(&mut self,arg0: impl Into<&'mc crate::JavaBitSet<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"andNot","(Ljava/util/BitSet;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn get_with_int(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<i32>) 
-> Result<crate::JavaBitSet<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"get","(II)Ljava/util/BitSet;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaBitSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn length(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"length","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn value_of_with_byte_buffer(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<i64>>) 
-> Result<crate::JavaBitSet<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/BitSet")?;
let res = jni.call_static_method(cls,"valueOf",
"(J)Ljava/util/BitSet;",&[])?;
let mut obj = res.l()?;
crate::JavaBitSet::from_raw(&jni,obj
)}
	pub fn clear(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"clear","(II)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_with_int(&mut self,arg0: i32,arg1: i32,arg2: std::option::Option<bool>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
// 2
let val_3 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"set","(IIZ)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn flip_with_int(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"flip","(II)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn next_clear_bit(&mut self,arg0: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextClearBit","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
/// An instantiatable struct that implements JavaListIterator. Needed for returning it from Java.
pub struct JavaListIterator<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> JavaListIterator<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaListIterator from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaListIterator")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaListIterator object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn add(&mut self,arg0: E) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"remove","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn has_next(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasNext","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn next(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"next","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn set(&mut self,arg0: E) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"set","(LE;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn next_index(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextIndex","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn previous_index(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"previousIndex","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn has_previous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasPrevious","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn previous(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"previous","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
}
impl<'mc, E> JNIRaw<'mc> for JavaListIterator<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> Into<crate::JavaIterator<'mc, E>> for JavaListIterator<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaIterator<'mc, E> {
       crate::JavaIterator::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaPriorityQueue<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> blackboxmc_general::JNIRaw<'mc> for JavaPriorityQueue<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> JavaPriorityQueue<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaPriorityQueue from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaPriorityQueue")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaPriorityQueue object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaCollection<'mc, E>>>) 
-> Result<crate::JavaPriorityQueue<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/PriorityQueue")?;
let res = jni.new_object(cls,
"(Ljava/util/Collection;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaPriorityQueue::from_raw(&jni,res
)}
	pub fn new_with_sorted_set(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, E>>>) 
-> Result<crate::JavaPriorityQueue<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/PriorityQueue")?;
let res = jni.new_object(cls,
"(Ljava/util/Comparator;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaPriorityQueue::from_raw(&jni,res
)}
	pub fn new_with_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i32>,arg1: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, E>>>) 
-> Result<crate::JavaPriorityQueue<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/PriorityQueue")?;
let res = jni.new_object(cls,
"(ILjava/util/Comparator;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::JavaPriorityQueue::from_raw(&jni,res
)}
	pub fn offer(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"offer","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn add(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn poll(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"poll","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn peek(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"peek","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"comparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn element(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"element","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, E> Into<crate::JavaAbstractQueue<'mc, E>> for JavaPriorityQueue<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractQueue<'mc, E> {
       crate::JavaAbstractQueue::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaEventObject<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaEventObject<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaEventObject<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaEventObject from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaEventObject")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaEventObject object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: jni::objects::JObject<'mc>) 
-> Result<crate::JavaEventObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let cls = &jni.find_class("java/util/EventObject")?;
let res = jni.new_object(cls,
"(Ljava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaEventObject::from_raw(&jni,res
)}
	pub fn source(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSource","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaLinkedList<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> blackboxmc_general::JNIRaw<'mc> for JavaLinkedList<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> JavaLinkedList<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaLinkedList from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaLinkedList")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaLinkedList object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaCollection<'mc, E>>>) 
-> Result<crate::JavaLinkedList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/LinkedList")?;
let res = jni.new_object(cls,
"(Ljava/util/Collection;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaLinkedList::from_raw(&jni,res
)}
	pub fn push(&mut self,arg0: E) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"push","(LE;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn pop(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pop","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn add_first(&mut self,arg0: E) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"addFirst","(LE;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn add_last(&mut self,arg0: E) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"addLast","(LE;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn poll_first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pollFirst","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn poll_last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pollLast","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn offer_last(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"offerLast","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"removeFirst","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFirst","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn peek_first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"peekFirst","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn remove_first_occurrence(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"removeFirstOccurrence","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn offer_first(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"offerFirst","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"removeLast","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLast","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn peek_last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"peekLast","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn remove_last_occurrence(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"removeLastOccurrence","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn offer(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"offer","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn descending_iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"descendingIterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_with_object(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<E>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = arg1.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(ILE;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: i32) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"get","(I)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn index_of(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"indexOf","(Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn last_index_of(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"lastIndexOf","(Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_all_with_collection(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<impl Into<&'mc crate::JavaCollection<'mc, E>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(ILjava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set(&mut self,arg0: i32,arg1: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"set","(ILE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn poll(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"poll","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn peek(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"peek","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn element(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"element","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn list_iterator(&mut self,arg0: std::option::Option<i32>) 
-> Result<crate::JavaListIterator<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"listIterator","(I)Ljava/util/ListIterator;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaListIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn sub_list(&mut self,arg0: i32,arg1: i32) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"subList","(II)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn sort(&mut self,arg0: impl Into<&'mc crate::JavaComparator<'mc, E>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"sort","(Ljava/util/Comparator;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, E> Into<crate::JavaList<'mc, E>> for JavaLinkedList<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaList<'mc, E> {
       crate::JavaList::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, E> Into<crate::JavaDeque<'mc, E>> for JavaLinkedList<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaDeque<'mc, E> {
       crate::JavaDeque::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, E> Into<crate::JavaAbstractSequentialList<'mc, E>> for JavaLinkedList<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractSequentialList<'mc, E> {
       crate::JavaAbstractSequentialList::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaStack<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> blackboxmc_general::JNIRaw<'mc> for JavaStack<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> JavaStack<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaStack from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaStack")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaStack object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaStack<'mc, E>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Stack")?;
let res = jni.new_object(cls,
"()V",&[])?;
crate::JavaStack::from_raw(&jni,res
)}
	pub fn push(&mut self,arg0: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"push","(LE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn pop(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pop","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"empty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn peek(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"peek","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn search(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"search","(Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn copy_into(&mut self,arg0: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"copyInto","(Ljava/lang/Object;)V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_size(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSize","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_element_at(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"removeElementAt","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_element(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"removeElement","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn insert_element_at(&mut self,arg0: E,arg1: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"insertElementAt","(LE;I)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_all_elements(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"removeAllElements","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn first_element(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"firstElement","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn last_element(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"lastElement","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn set_element_at(&mut self,arg0: E,arg1: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setElementAt","(LE;I)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn add_element(&mut self,arg0: E) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"addElement","(LE;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn add_with_object(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<E>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = arg1.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(ILE;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_with_int(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: i32) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"get","(I)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn index_of_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"indexOf","(Ljava/lang/Object;I)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn last_index_of_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"lastIndexOf","(Ljava/lang/Object;I)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn sub_list(&mut self,arg0: i32,arg1: i32) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"subList","(II)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn elements(&mut self) 
-> Result<crate::JavaEnumeration<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"elements","()Ljava/util/Enumeration;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaEnumeration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_all_with_collection(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<impl Into<&'mc crate::JavaCollection<'mc, E>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(ILjava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set(&mut self,arg0: i32,arg1: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"set","(ILE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn capacity(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"capacity","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn ensure_capacity(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"ensureCapacity","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn trim_to_size(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"trimToSize","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn element_at(&mut self,arg0: i32) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"elementAt","(I)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn sort(&mut self,arg0: impl Into<&'mc crate::JavaComparator<'mc, E>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"sort","(Ljava/util/Comparator;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn list_iterator(&mut self,arg0: std::option::Option<i32>) 
-> Result<crate::JavaListIterator<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"listIterator","(I)Ljava/util/ListIterator;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaListIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, E> Into<crate::JavaVector<'mc, E>> for JavaStack<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaVector<'mc, E> {
       crate::JavaVector::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaSpliterators<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaSpliterators<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaSpliterators<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaSpliterators from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaSpliterators")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaSpliterators object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn empty_int_spliterator(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaSpliteratorOfInt<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Spliterator$OfInt")?;
let res = jni.call_static_method(cls,"emptyIntSpliterator",
"()Ljava/util/Spliterator$OfInt;",&[])?;
let mut obj = res.l()?;
crate::JavaSpliteratorOfInt::from_raw(&jni,obj
)}
	pub fn empty_long_spliterator(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaSpliteratorOfLong<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Spliterator$OfLong")?;
let res = jni.call_static_method(cls,"emptyLongSpliterator",
"()Ljava/util/Spliterator$OfLong;",&[])?;
let mut obj = res.l()?;
crate::JavaSpliteratorOfLong::from_raw(&jni,obj
)}
	pub fn empty_double_spliterator(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaSpliteratorOfDouble<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Spliterator$OfDouble")?;
let res = jni.call_static_method(cls,"emptyDoubleSpliterator",
"()Ljava/util/Spliterator$OfDouble;",&[])?;
let mut obj = res.l()?;
crate::JavaSpliteratorOfDouble::from_raw(&jni,obj
)}
	pub fn empty_spliterator<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaSpliterator<'mc, T>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let cls = &jni.find_class("java/util/Spliterator")?;
let res = jni.call_static_method(cls,"emptySpliterator",
"()Ljava/util/Spliterator;",&[])?;
let mut obj = res.l()?;
crate::JavaSpliterator::from_raw(&jni,obj
)}
	pub fn iterator_with_spliterator<T,T,T,T,T,T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaSpliteratorOfLong<'mc>>>) 
-> Result<crate::JavaPrimitiveIteratorOfLong<'mc>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/PrimitiveIterator$OfLong")?;
let res = jni.call_static_method(cls,"iterator",
"(Ljava/util/Spliterator$OfLong;)Ljava/util/PrimitiveIterator$OfLong;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaPrimitiveIteratorOfLong::from_raw(&jni,obj
)}
	pub fn iterator_with_spliteratorof_double<T,T,T,T,T,T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaSpliteratorOfInt<'mc>>>) 
-> Result<crate::JavaPrimitiveIteratorOfInt<'mc>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/PrimitiveIterator$OfInt")?;
let res = jni.call_static_method(cls,"iterator",
"(Ljava/util/Spliterator$OfInt;)Ljava/util/PrimitiveIterator$OfInt;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaPrimitiveIteratorOfInt::from_raw(&jni,obj
)}
	pub fn spliterator_unknown_size_with_primitive_iteratorof_long<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaIterator<'mc, E>>,arg1: std::option::Option<i32>) 
-> Result<crate::JavaSpliterator<'mc, T>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let cls = &jni.find_class("java/util/Spliterator")?;
let res = jni.call_static_method(cls,"spliteratorUnknownSize",
"(Ljava/util/Iterator;I)Ljava/util/Spliterator;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaSpliterator::from_raw(&jni,obj
)}
	pub fn spliterator_unknown_size_with_primitive_iteratorof_int<T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaPrimitiveIteratorOfDouble<'mc>>,arg1: std::option::Option<i32>) 
-> Result<crate::JavaSpliteratorOfDouble<'mc>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let cls = &jni.find_class("java/util/Spliterator$OfDouble")?;
let res = jni.call_static_method(cls,"spliteratorUnknownSize",
"(Ljava/util/PrimitiveIterator$OfDouble;I)Ljava/util/Spliterator$OfDouble;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaSpliteratorOfDouble::from_raw(&jni,obj
)}
	pub fn spliterator_with_collection<T,T,T,T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<f64>,arg1: std::option::Option<i32>) 
-> Result<crate::JavaSpliteratorOfDouble<'mc>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let cls = &jni.find_class("java/util/Spliterator$OfDouble")?;
let res = jni.call_static_method(cls,"spliterator",
"(DI)Ljava/util/Spliterator$OfDouble;",&[jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaSpliteratorOfDouble::from_raw(&jni,obj
)}
	pub fn spliterator_with_longs<T,T,T,T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i32>,arg1: std::option::Option<i32>) 
-> Result<crate::JavaSpliteratorOfInt<'mc>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let cls = &jni.find_class("java/util/Spliterator$OfInt")?;
let res = jni.call_static_method(cls,"spliterator",
"(II)Ljava/util/Spliterator$OfInt;",&[jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaSpliteratorOfInt::from_raw(&jni,obj
)}
	pub fn spliterator_with_objects<T,T,T,T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaPrimitiveIteratorOfLong<'mc>>,arg1: std::option::Option<i64>,arg2: std::option::Option<i32>) 
-> Result<crate::JavaSpliteratorOfLong<'mc>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let cls = &jni.find_class("java/util/Spliterator$OfLong")?;
let res = jni.call_static_method(cls,"spliterator",
"(Ljava/util/PrimitiveIterator$OfLong;JI)Ljava/util/Spliterator$OfLong;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::JavaSpliteratorOfLong::from_raw(&jni,obj
)}
	pub fn spliterator_with_primitive_iteratorof_double<T,T,T,T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaPrimitiveIteratorOfInt<'mc>>,arg1: i64,arg2: std::option::Option<i32>) 
-> Result<crate::JavaSpliteratorOfInt<'mc>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let cls = &jni.find_class("java/util/Spliterator$OfInt")?;
let res = jni.call_static_method(cls,"spliterator",
"(Ljava/util/PrimitiveIterator$OfInt;JI)Ljava/util/Spliterator$OfInt;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::JavaSpliteratorOfInt::from_raw(&jni,obj
)}
	pub fn spliterator_with_iterator<T,T,T,T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<i64>,arg1: i32,arg2: std::option::Option<i32>,arg3: std::option::Option<i32>) 
-> Result<crate::JavaSpliteratorOfLong<'mc>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
let cls = &jni.find_class("java/util/Spliterator$OfLong")?;
let res = jni.call_static_method(cls,"spliterator",
"(JIII)Ljava/util/Spliterator$OfLong;",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
let mut obj = res.l()?;
crate::JavaSpliteratorOfLong::from_raw(&jni,obj
)}
	pub fn spliterator_with_ints<T,T,T,T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>,arg1: i32,arg2: i32,arg3: std::option::Option<i32>) 
-> Result<crate::JavaSpliterator<'mc, T>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
let cls = &jni.find_class("java/util/Spliterator")?;
let res = jni.call_static_method(cls,"spliterator",
"(Ljava/lang/Object;III)Ljava/util/Spliterator;",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
let mut obj = res.l()?;
crate::JavaSpliterator::from_raw(&jni,obj
)}
	pub fn spliterator_with_doubles<T,T,T,T,T>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<f64>,arg1: i32,arg2: i32,arg3: std::option::Option<i32>) 
-> Result<crate::JavaSpliteratorOfDouble<'mc>, Box<dyn std::error::Error>>
 where T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>,T: JNIRaw<'mc>
{let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
let cls = &jni.find_class("java/util/Spliterator$OfDouble")?;
let res = jni.call_static_method(cls,"spliterator",
"(DIII)Ljava/util/Spliterator$OfDouble;",&[jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
let mut obj = res.l()?;
crate::JavaSpliteratorOfDouble::from_raw(&jni,obj
)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaLocaleLanguageRange<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaLocaleLanguageRange<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaLocaleLanguageRange<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaLocaleLanguageRange from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaLocaleLanguageRange")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaLocaleLanguageRange object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc String>>,arg1: std::option::Option<f64>) 
-> Result<crate::JavaLocaleLanguageRange<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(jni.new_string(arg0.unwrap().into()).unwrap());
let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
let cls = &jni.find_class("java/util/Locale$LanguageRange")?;
let res = jni.new_object(cls,
"(Ljava/lang/String;D)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::JavaLocaleLanguageRange::from_raw(&jni,res
)}
	pub fn map_equivalents(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>,arg1: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("java/util/List")?;
let res = jni.call_static_method(cls,"mapEquivalents",
"(Ljava/util/List;Ljava/util/Map;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaList::from_raw(&jni,obj
)}
	pub fn range(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRange","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn weight(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getWeight","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn parse_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc String>>,arg1: std::option::Option<impl Into<&'mc crate::JavaMap<'mc, K,V>>>) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(jni.new_string(arg0.unwrap().into()).unwrap());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/List")?;
let res = jni.call_static_method(cls,"parse",
"(Ljava/lang/String;Ljava/util/Map;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::JavaList::from_raw(&jni,obj
)}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
/// An instantiatable struct that implements JavaSpliteratorOfDouble. Needed for returning it from Java.
pub struct JavaSpliteratorOfDouble<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaSpliteratorOfDouble<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaSpliteratorOfDouble from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaSpliteratorOfDouble")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaSpliteratorOfDouble object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn try_advance_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"tryAdvance","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn try_split(&mut self) 
-> Result<crate::JavaSpliteratorOfDouble<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"trySplit","()Ljava/util/Spliterator$OfDouble;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliteratorOfDouble::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn for_each_remaining_with_consumer(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"forEachRemaining","(Ljava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn estimate_size(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"estimateSize","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn exact_size_if_known(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExactSizeIfKnown","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn has_characteristics(&mut self,arg0: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"hasCharacteristics","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getComparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn characteristics(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"characteristics","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
}
impl<'mc> JNIRaw<'mc> for JavaSpliteratorOfDouble<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::JavaSpliteratorOfPrimitive<'mc>> for JavaSpliteratorOfDouble<'mc>{
   fn into(self) -> crate::JavaSpliteratorOfPrimitive<'mc> {
       crate::JavaSpliteratorOfPrimitive::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaEventListenerProxy<'mc, T>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where T: JNIRaw<'mc>;
impl<'mc, T> blackboxmc_general::JNIRaw<'mc> for JavaEventListenerProxy<'mc, T> where T: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, T> JavaEventListenerProxy<'mc, T> where T: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaEventListenerProxy from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaEventListenerProxy")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaEventListenerProxy object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaEventListener<'mc>>) 
-> Result<crate::JavaEventListenerProxy<'mc, >, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/EventListenerProxy")?;
let res = jni.new_object(cls,
"(Ljava/util/EventListener;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaEventListenerProxy::from_raw(&jni,res
)}
	pub fn listener(&mut self) 
-> Result<crate::JavaEventListener<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getListener","()Ljava/util/EventListener;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaEventListener::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, T> Into<crate::JavaEventListener<'mc, T>> for JavaEventListenerProxy<'mc, T> where T: JNIRaw<'mc>{
   fn into(self) -> crate::JavaEventListener<'mc, T> {
       crate::JavaEventListener::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaResourceBundleControl<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaResourceBundleControl<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaResourceBundleControl<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaResourceBundleControl from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaResourceBundleControl")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaResourceBundleControl object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn to_resource_name(&mut self,arg0: impl Into<&'mc String>,arg1: impl Into<&'mc String>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"toResourceName","(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn get_control(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>) 
-> Result<crate::JavaResourceBundleControl<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/ResourceBundle$Control")?;
let res = jni.call_static_method(cls,"getControl",
"(Ljava/util/List;)Ljava/util/ResourceBundle$Control;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaResourceBundleControl::from_raw(&jni,obj
)}
	pub fn get_formats(&mut self,arg0: impl Into<&'mc String>) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"getFormats","(Ljava/lang/String;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_candidate_locales(&mut self,arg0: impl Into<&'mc String>,arg1: impl Into<&'mc crate::JavaLocale<'mc>>) 
-> Result<crate::JavaList<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getCandidateLocales","(Ljava/lang/String;Ljava/util/Locale;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_fallback_locale(&mut self,arg0: impl Into<&'mc String>,arg1: impl Into<&'mc crate::JavaLocale<'mc>>) 
-> Result<crate::JavaLocale<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getFallbackLocale","(Ljava/lang/String;Ljava/util/Locale;)Ljava/util/Locale;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocale::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_time_to_live(&mut self,arg0: impl Into<&'mc String>,arg1: impl Into<&'mc crate::JavaLocale<'mc>>) 
-> Result<i64, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getTimeToLive","(Ljava/lang/String;Ljava/util/Locale;)J",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn to_bundle_name(&mut self,arg0: impl Into<&'mc String>,arg1: impl Into<&'mc crate::JavaLocale<'mc>>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"toBundleName","(Ljava/lang/String;Ljava/util/Locale;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn get_no_fallback_control(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaList<'mc, E>>) 
-> Result<crate::JavaResourceBundleControl<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/ResourceBundle$Control")?;
let res = jni.call_static_method(cls,"getNoFallbackControl",
"(Ljava/util/List;)Ljava/util/ResourceBundle$Control;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaResourceBundleControl::from_raw(&jni,obj
)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaObjects<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaObjects<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaObjects<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaObjects from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaObjects")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaObjects object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn equals_with_object(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = arg1.unwrap();
let cls = &jni.find_class("boolean")?;
let res = jni.call_static_method(cls,"equals",
"(Ljava/lang/Object;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(res.z().unwrap())}
	pub fn to_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/String")?;
let res = jni.call_static_method(cls,"toString",
"(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(jni.get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"hashCode",
"(Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1)])?;
Ok(res.i().unwrap())}
	pub fn check_index_with_long(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"checkIndex",
"(II)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(res.i().unwrap())}
	pub fn compare(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: T,arg1: T,arg2: impl Into<&'mc crate::JavaComparator<'mc, T>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"compare",
"(LT;LT;Ljava/util/Comparator;)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
Ok(res.i().unwrap())}
	pub fn hash(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: Vec<jni::objects::JObject<'mc>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"hash",
"(Ljava/lang/Object;)I",&[])?;
Ok(res.i().unwrap())}
	pub fn require_non_null_with_object(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<T>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Object")?;
let res = jni.call_static_method(cls,"requireNonNull",
"(LT;Ljava/lang/String;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(res.l().unwrap())}
	pub fn check_from_index_size_with_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i64,arg1: i64,arg2: std::option::Option<i64>) 
-> Result<i64, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Long(arg2.unwrap().into());
let cls = &jni.find_class("long")?;
let res = jni.call_static_method(cls,"checkFromIndexSize",
"(JJJ)J",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
Ok(res.j().unwrap())}
	pub fn check_from_to_index_with_long(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: i32,arg2: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let cls = &jni.find_class("int")?;
let res = jni.call_static_method(cls,"checkFromToIndex",
"(III)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
Ok(res.i().unwrap())}
	pub fn deep_equals(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1;
let cls = &jni.find_class("boolean")?;
let res = jni.call_static_method(cls,"deepEquals",
"(Ljava/lang/Object;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(res.z().unwrap())}
	pub fn is_null(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let cls = &jni.find_class("boolean")?;
let res = jni.call_static_method(cls,"isNull",
"(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)])?;
Ok(res.z().unwrap())}
	pub fn non_null(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let cls = &jni.find_class("boolean")?;
let res = jni.call_static_method(cls,"nonNull",
"(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)])?;
Ok(res.z().unwrap())}
	pub fn require_non_null_else(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: T,arg1: T) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let cls = &jni.find_class("java/lang/Object")?;
let res = jni.call_static_method(cls,"requireNonNullElse",
"(LT;LT;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
Ok(res.l().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
/// An instantiatable struct that implements JavaCollection. Needed for returning it from Java.
pub struct JavaCollection<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> JavaCollection<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaCollection from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaCollection")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaCollection object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn add(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc, E> JNIRaw<'mc> for JavaCollection<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaResourceBundle<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaResourceBundle<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaResourceBundle<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaResourceBundle from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaResourceBundle")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaResourceBundle object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaResourceBundle<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/ResourceBundle")?;
let res = jni.new_object(cls,
"()V",&[])?;
crate::JavaResourceBundle::from_raw(&jni,res
)}
	pub fn get_string(&mut self,arg0: impl Into<&'mc String>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"getString","(Ljava/lang/String;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn keys(&mut self) 
-> Result<crate::JavaEnumeration<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getKeys","()Ljava/util/Enumeration;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaEnumeration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn locale(&mut self) 
-> Result<crate::JavaLocale<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLocale","()Ljava/util/Locale;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocale::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn base_bundle_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBaseBundleName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn contains_key(&mut self,arg0: impl Into<&'mc String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"containsKey","(Ljava/lang/String;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn key_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_object(&mut self,arg0: impl Into<&'mc String>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"getObject","(Ljava/lang/String;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
/// An instantiatable struct that implements JavaSpliteratorOfInt. Needed for returning it from Java.
pub struct JavaSpliteratorOfInt<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaSpliteratorOfInt<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaSpliteratorOfInt from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaSpliteratorOfInt")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaSpliteratorOfInt object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn try_advance_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"tryAdvance","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn try_split(&mut self) 
-> Result<crate::JavaSpliteratorOfInt<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"trySplit","()Ljava/util/Spliterator$OfInt;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliteratorOfInt::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn for_each_remaining_with_consumer(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"forEachRemaining","(Ljava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn estimate_size(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"estimateSize","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn exact_size_if_known(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExactSizeIfKnown","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn has_characteristics(&mut self,arg0: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"hasCharacteristics","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getComparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn characteristics(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"characteristics","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
}
impl<'mc> JNIRaw<'mc> for JavaSpliteratorOfInt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::JavaSpliteratorOfPrimitive<'mc>> for JavaSpliteratorOfInt<'mc>{
   fn into(self) -> crate::JavaSpliteratorOfPrimitive<'mc> {
       crate::JavaSpliteratorOfPrimitive::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
/// An instantiatable struct that implements JavaPrimitiveIteratorOfInt. Needed for returning it from Java.
pub struct JavaPrimitiveIteratorOfInt<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaPrimitiveIteratorOfInt<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaPrimitiveIteratorOfInt from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaPrimitiveIteratorOfInt")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaPrimitiveIteratorOfInt object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn next_int(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextInt","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn next(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"next","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn remove(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"remove","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn has_next(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasNext","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc> JNIRaw<'mc> for JavaPrimitiveIteratorOfInt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaDictionary<'mc, K,V>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where K: JNIRaw<'mc>,V: JNIRaw<'mc>;
impl<'mc, K,V> blackboxmc_general::JNIRaw<'mc> for JavaDictionary<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, K,V> JavaDictionary<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaDictionary from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaDictionary")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaDictionary object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaDictionary<'mc, K,V>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Dictionary")?;
let res = jni.new_object(cls,
"()V",&[])?;
crate::JavaDictionary::from_raw(&jni,res
)}
	pub fn remove(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn get(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"get","(Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn put(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"put","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn elements(&mut self) 
-> Result<crate::JavaEnumeration<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"elements","()Ljava/util/Enumeration;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaEnumeration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn keys(&mut self) 
-> Result<crate::JavaEnumeration<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keys","()Ljava/util/Enumeration;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaEnumeration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaScanner<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaScanner<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaScanner<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaScanner from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaScanner")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaScanner object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn next_boolean(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextBoolean","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn next_long(&mut self,arg0: std::option::Option<i32>) 
-> Result<i64, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextLong","(I)J",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn next_float(&mut self) 
-> Result<f32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextFloat","()F",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.f().unwrap())}
	pub fn use_locale(&mut self,arg0: impl Into<&'mc crate::JavaLocale<'mc>>) 
-> Result<crate::JavaScanner<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"useLocale","(Ljava/util/Locale;)Ljava/util/Scanner;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaScanner::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn find_within_horizon_with_string(&mut self,arg0: impl Into<&'mc crate::regex::JavaPattern<'mc>>,arg1: std::option::Option<i32>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"findWithinHorizon","(Ljava/util/regex/Pattern;I)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn find_in_line_with_pattern(&mut self,arg0: std::option::Option<impl Into<&'mc String>>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"findInLine","(Ljava/lang/String;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn has_next_byte(&mut self,arg0: std::option::Option<i32>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"hasNextByte","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn next_byte(&mut self,arg0: std::option::Option<i32>) 
-> Result<i8, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextByte","(I)B",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.b().unwrap())}
	pub fn has_next_short(&mut self,arg0: std::option::Option<i32>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"hasNextShort","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn next_short(&mut self,arg0: std::option::Option<i32>) 
-> Result<i16, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextShort","(I)S",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.s().unwrap())}
	pub fn has_next_int(&mut self,arg0: std::option::Option<i32>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"hasNextInt","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn has_next_long(&mut self,arg0: std::option::Option<i32>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"hasNextLong","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn has_next_big_integer(&mut self,arg0: std::option::Option<i32>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"hasNextBigInteger","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn use_radix(&mut self,arg0: i32) 
-> Result<crate::JavaScanner<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"useRadix","(I)Ljava/util/Scanner;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaScanner::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn use_delimiter_with_string(&mut self,arg0: std::option::Option<impl Into<&'mc crate::regex::JavaPattern<'mc>>>) 
-> Result<crate::JavaScanner<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"useDelimiter","(Ljava/util/regex/Pattern;)Ljava/util/Scanner;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaScanner::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn has_next_line(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasNextLine","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn next_line(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextLine","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn has_next_boolean(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasNextBoolean","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn has_next_float(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasNextFloat","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn has_next_double(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasNextDouble","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn has_next_big_decimal(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasNextBigDecimal","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn next_double(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextDouble","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn next_int(&mut self,arg0: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextInt","(I)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn remove(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"remove","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn has_next(&mut self,arg0: std::option::Option<impl Into<&'mc String>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"hasNext","(Ljava/lang/String;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn next(&mut self,arg0: std::option::Option<impl Into<&'mc String>>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"next","(Ljava/lang/String;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn delimiter(&mut self) 
-> Result<crate::regex::JavaPattern<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"delimiter","()Ljava/util/regex/Pattern;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::regex::JavaPattern::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn locale(&mut self) 
-> Result<crate::JavaLocale<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"locale","()Ljava/util/Locale;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocale::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_match(&mut self) 
-> Result<crate::regex::JavaMatchResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"match","()Ljava/util/regex/MatchResult;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::regex::JavaMatchResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn close(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"close","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn skip_with_pattern(&mut self,arg0: std::option::Option<impl Into<&'mc String>>) 
-> Result<crate::JavaScanner<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"skip","(Ljava/lang/String;)Ljava/util/Scanner;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaScanner::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn reset(&mut self) 
-> Result<crate::JavaScanner<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"reset","()Ljava/util/Scanner;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaScanner::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn radix(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"radix","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::JavaIterator<'mc>> for JavaScanner<'mc>{
   fn into(self) -> crate::JavaIterator<'mc> {
       crate::JavaIterator::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
/// An instantiatable struct that implements JavaPrimitiveIteratorOfLong. Needed for returning it from Java.
pub struct JavaPrimitiveIteratorOfLong<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaPrimitiveIteratorOfLong<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaPrimitiveIteratorOfLong from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaPrimitiveIteratorOfLong")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaPrimitiveIteratorOfLong object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn next_long(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextLong","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn next(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"next","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn remove(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"remove","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn has_next(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasNext","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc> JNIRaw<'mc> for JavaPrimitiveIteratorOfLong<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaNavigableSet. Needed for returning it from Java.
pub struct JavaNavigableSet<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> JavaNavigableSet<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaNavigableSet from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaNavigableSet")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaNavigableSet object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn poll_first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pollFirst","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn poll_last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pollLast","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn descending_iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"descendingIterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn sub_set_with_object(&mut self,arg0: E,arg1: std::option::Option<bool>,arg2: std::option::Option<E>,arg3: std::option::Option<bool>) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
// 1
let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
let val_3 = arg2.unwrap().jni_object();
// 1
let val_4 = jni::objects::JValueGen::Bool(arg3.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"subSet","(LE;ZLE;Z)Ljava/util/NavigableSet;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn head_set_with_object(&mut self,arg0: std::option::Option<E>,arg1: std::option::Option<bool>) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
// 0
let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"headSet","(LE;Z)Ljava/util/NavigableSet;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn tail_set_with_object(&mut self,arg0: std::option::Option<E>,arg1: std::option::Option<bool>) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
// 0
let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"tailSet","(LE;Z)Ljava/util/NavigableSet;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn descending_set(&mut self) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"descendingSet","()Ljava/util/NavigableSet;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn ceiling(&mut self,arg0: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"ceiling","(LE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn higher(&mut self,arg0: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"higher","(LE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn floor(&mut self,arg0: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"floor","(LE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn lower(&mut self,arg0: E) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"lower","(LE;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"last","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"first","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"comparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn add_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc, E> JNIRaw<'mc> for JavaNavigableSet<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> Into<crate::JavaSortedSet<'mc, E>> for JavaNavigableSet<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaSortedSet<'mc, E> {
       crate::JavaSortedSet::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaFormattableFlags<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaFormattableFlags<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaFormattableFlags<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaFormattableFlags from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaFormattableFlags")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaFormattableFlags object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
/// An instantiatable struct that implements JavaNavigableMap. Needed for returning it from Java.
pub struct JavaNavigableMap<'mc, K,V>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where K: JNIRaw<'mc>,V: JNIRaw<'mc>;
impl<'mc, K,V> JavaNavigableMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaNavigableMap from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaNavigableMap")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaNavigableMap object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn navigable_key_set(&mut self) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"navigableKeySet","()Ljava/util/NavigableSet;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn descending_key_set(&mut self) 
-> Result<crate::JavaNavigableSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"descendingKeySet","()Ljava/util/NavigableSet;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn descending_map(&mut self) 
-> Result<crate::JavaNavigableMap<'mc, K,V>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"descendingMap","()Ljava/util/NavigableMap;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn sub_map_with_object(&mut self,arg0: K,arg1: std::option::Option<bool>,arg2: std::option::Option<K>,arg3: std::option::Option<bool>) 
-> Result<crate::JavaNavigableMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
// 1
let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
let val_3 = arg2.unwrap().jni_object();
// 1
let val_4 = jni::objects::JValueGen::Bool(arg3.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"subMap","(LK;ZLK;Z)Ljava/util/NavigableMap;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn head_map_with_object(&mut self,arg0: std::option::Option<K>,arg1: std::option::Option<bool>) 
-> Result<crate::JavaNavigableMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
// 0
let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"headMap","(LK;Z)Ljava/util/NavigableMap;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn tail_map_with_object(&mut self,arg0: std::option::Option<K>,arg1: std::option::Option<bool>) 
-> Result<crate::JavaNavigableMap<'mc, K,V>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
// 0
let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"tailMap","(LK;Z)Ljava/util/NavigableMap;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaNavigableMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn lower_key(&mut self,arg0: K) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"lowerKey","(LK;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn floor_key(&mut self,arg0: K) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"floorKey","(LK;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn ceiling_key(&mut self,arg0: K) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"ceilingKey","(LK;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn higher_key(&mut self,arg0: K) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"higherKey","(LK;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn poll_first_entry(&mut self) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pollFirstEntry","()Ljava/util/Map$Entry;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn poll_last_entry(&mut self) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pollLastEntry","()Ljava/util/Map$Entry;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn first_entry(&mut self) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"firstEntry","()Ljava/util/Map$Entry;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn last_entry(&mut self) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"lastEntry","()Ljava/util/Map$Entry;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn lower_entry(&mut self,arg0: K) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"lowerEntry","(LK;)Ljava/util/Map$Entry;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn floor_entry(&mut self,arg0: K) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"floorEntry","(LK;)Ljava/util/Map$Entry;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn ceiling_entry(&mut self,arg0: K) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"ceilingEntry","(LK;)Ljava/util/Map$Entry;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn higher_entry(&mut self,arg0: K) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"higherEntry","(LK;)Ljava/util/Map$Entry;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaMapEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn first_key(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"firstKey","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn last_key(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"lastKey","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn values(&mut self) 
-> Result<crate::JavaCollection<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"values","()Ljava/util/Collection;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCollection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entry_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"entrySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn key_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"comparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn remove_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = arg1.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"get","(Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn put(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"put","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn replace_with_object(&mut self,arg0: K,arg1: std::option::Option<V>,arg2: std::option::Option<V>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap().jni_object();
let val_2 = arg1.unwrap().jni_object();
let val_3 = arg2.unwrap().jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"replace","(LK;LV;LV;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn put_all(&mut self,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"putAll","(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn put_if_absent(&mut self,arg0: K,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"putIfAbsent","(LK;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn contains_key(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsKey","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_value(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsValue","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get_or_default(&mut self,arg0: jni::objects::JObject<'mc>,arg1: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"getOrDefault","(Ljava/lang/Object;LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
}
impl<'mc, K,V> JNIRaw<'mc> for JavaNavigableMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, K,V> Into<crate::JavaSortedMap<'mc, K,V>> for JavaNavigableMap<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc>{
   fn into(self) -> crate::JavaSortedMap<'mc, K,V> {
       crate::JavaSortedMap::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaFormatter<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaFormatter<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaFormatter<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaFormatter from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaFormatter")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaFormatter object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_appendable(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc String>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<crate::JavaFormatter<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(jni.new_string(arg0.unwrap().into()).unwrap());
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/util/Formatter")?;
let res = jni.new_object(cls,
"(Ljava/lang/String;Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::JavaFormatter::from_raw(&jni,res
)}
	pub fn new_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc String>,arg1: impl Into<&'mc String>,arg2: std::option::Option<impl Into<&'mc crate::JavaLocale<'mc>>>) 
-> Result<crate::JavaFormatter<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/Formatter")?;
let res = jni.new_object(cls,
"(Ljava/lang/String;Ljava/lang/String;Ljava/util/Locale;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::JavaFormatter::from_raw(&jni,res
)}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn flush(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"flush","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn format_with_string(&mut self,arg0: impl Into<&'mc crate::JavaLocale<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>,arg2: std::option::Option<Vec<jni::objects::JObject<'mc>>>) 
-> Result<crate::JavaFormatter<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"format","(Ljava/util/Locale;Ljava/lang/String;Ljava/lang/Object;)Ljava/util/Formatter;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaFormatter::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn locale(&mut self) 
-> Result<crate::JavaLocale<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"locale","()Ljava/util/Locale;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocale::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn close(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"close","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaSpliteratorsAbstractSpliterator<'mc, T>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where T: JNIRaw<'mc>;
impl<'mc, T> blackboxmc_general::JNIRaw<'mc> for JavaSpliteratorsAbstractSpliterator<'mc, T> where T: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, T> JavaSpliteratorsAbstractSpliterator<'mc, T> where T: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaSpliteratorsAbstractSpliterator from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaSpliteratorsAbstractSpliterator")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaSpliteratorsAbstractSpliterator object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn estimate_size(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"estimateSize","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn try_split(&mut self) 
-> Result<crate::JavaSpliterator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"trySplit","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn characteristics(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"characteristics","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn exact_size_if_known(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExactSizeIfKnown","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn has_characteristics(&mut self,arg0: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"hasCharacteristics","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getComparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
}
impl<'mc, T> Into<crate::JavaSpliterator<'mc, T>> for JavaSpliteratorsAbstractSpliterator<'mc, T> where T: JNIRaw<'mc>{
   fn into(self) -> crate::JavaSpliterator<'mc, T> {
       crate::JavaSpliterator::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaProperties<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaProperties<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaProperties<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaProperties from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaProperties")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaProperties object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i32>) 
-> Result<crate::JavaProperties<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let cls = &jni.find_class("java/util/Properties")?;
let res = jni.new_object(cls,
"(I)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaProperties::from_raw(&jni,res
)}
	pub fn remove_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>,arg1: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = arg1.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"get","(Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn put(&mut self,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1;
let res = self.jni_ref().call_method(&self.jni_object(),"put","(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn get_property_with_string(&mut self,arg0: std::option::Option<impl Into<&'mc String>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<String, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"getProperty","(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn values(&mut self) 
-> Result<crate::JavaCollection<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"values","()Ljava/util/Collection;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaCollection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn replace_with_object(&mut self,arg0: jni::objects::JObject<'mc>,arg1: std::option::Option<jni::objects::JObject<'mc>>,arg2: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = arg1.unwrap();
let val_3 = arg2.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"replace","(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn elements(&mut self) 
-> Result<crate::JavaEnumeration<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"elements","()Ljava/util/Enumeration;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaEnumeration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entry_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"entrySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn put_all(&mut self,arg0: impl Into<&'mc crate::JavaMap<'mc, K,V>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"putAll","(Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn put_if_absent(&mut self,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1;
let res = self.jni_ref().call_method(&self.jni_object(),"putIfAbsent","(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn set_property(&mut self,arg0: impl Into<&'mc String>,arg1: impl Into<&'mc String>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setProperty","(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn contains_key(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsKey","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn keys(&mut self) 
-> Result<crate::JavaEnumeration<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keys","()Ljava/util/Enumeration;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaEnumeration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn key_set(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"keySet","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_value(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"containsValue","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn get_or_default(&mut self,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1;
let res = self.jni_ref().call_method(&self.jni_object(),"getOrDefault","(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn property_names(&mut self) 
-> Result<crate::JavaEnumeration<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"propertyNames","()Ljava/util/Enumeration;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaEnumeration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn string_property_names(&mut self) 
-> Result<crate::JavaSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"stringPropertyNames","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::JavaHashtable<'mc>> for JavaProperties<'mc>{
   fn into(self) -> crate::JavaHashtable<'mc> {
       crate::JavaHashtable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaSpliteratorsAbstractLongSpliterator<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaSpliteratorsAbstractLongSpliterator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaSpliteratorsAbstractLongSpliterator<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaSpliteratorsAbstractLongSpliterator from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaSpliteratorsAbstractLongSpliterator")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaSpliteratorsAbstractLongSpliterator object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn estimate_size(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"estimateSize","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn try_split(&mut self) 
-> Result<crate::JavaSpliteratorOfLong<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"trySplit","()Ljava/util/Spliterator$OfLong;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliteratorOfLong::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn characteristics(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"characteristics","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn try_advance_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"tryAdvance","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn for_each_remaining_with_consumer(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"forEachRemaining","(Ljava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn exact_size_if_known(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExactSizeIfKnown","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn has_characteristics(&mut self,arg0: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"hasCharacteristics","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getComparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
}
impl<'mc> Into<crate::JavaSpliteratorOfLong<'mc>> for JavaSpliteratorsAbstractLongSpliterator<'mc>{
   fn into(self) -> crate::JavaSpliteratorOfLong<'mc> {
       crate::JavaSpliteratorOfLong::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaTimerTask<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaTimerTask<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaTimerTask<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaTimerTask from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaTimerTask")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaTimerTask object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn cancel(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"cancel","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn scheduled_execution_time(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"scheduledExecutionTime","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn run(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"run","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
/// An instantiatable struct that implements JavaIterator. Needed for returning it from Java.
pub struct JavaIterator<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> JavaIterator<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaIterator from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaIterator")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaIterator object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn remove(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"remove","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn has_next(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasNext","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn next(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"next","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
}
impl<'mc, E> JNIRaw<'mc> for JavaIterator<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaSpliteratorOfLong. Needed for returning it from Java.
pub struct JavaSpliteratorOfLong<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaSpliteratorOfLong<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaSpliteratorOfLong from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaSpliteratorOfLong")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaSpliteratorOfLong object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn try_advance_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"tryAdvance","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn try_split(&mut self) 
-> Result<crate::JavaSpliteratorOfLong<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"trySplit","()Ljava/util/Spliterator$OfLong;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliteratorOfLong::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn for_each_remaining_with_consumer(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"forEachRemaining","(Ljava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn estimate_size(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"estimateSize","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn exact_size_if_known(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExactSizeIfKnown","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn has_characteristics(&mut self,arg0: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"hasCharacteristics","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getComparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn characteristics(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"characteristics","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
}
impl<'mc> JNIRaw<'mc> for JavaSpliteratorOfLong<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::JavaSpliteratorOfPrimitive<'mc>> for JavaSpliteratorOfLong<'mc>{
   fn into(self) -> crate::JavaSpliteratorOfPrimitive<'mc> {
       crate::JavaSpliteratorOfPrimitive::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
/// An instantiatable struct that implements JavaPrimitiveIterator. Needed for returning it from Java.
pub struct JavaPrimitiveIterator<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaPrimitiveIterator<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaPrimitiveIterator from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaPrimitiveIterator")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaPrimitiveIterator object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn remove(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"remove","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn has_next(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasNext","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn next(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"next","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
}
impl<'mc> JNIRaw<'mc> for JavaPrimitiveIterator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::JavaIterator<'mc>> for JavaPrimitiveIterator<'mc>{
   fn into(self) -> crate::JavaIterator<'mc> {
       crate::JavaIterator::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
/// An instantiatable struct that implements JavaMapEntry. Needed for returning it from Java.
pub struct JavaMapEntry<'mc, K,V>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where K: JNIRaw<'mc>,V: JNIRaw<'mc>;
impl<'mc, K,V> JavaMapEntry<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaMapEntry from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaMapEntry")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaMapEntry object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn comparing_by_key<K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>
 where K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/Comparator")?;
let res = jni.call_static_method(cls,"comparingByKey",
"(Ljava/util/Comparator;)Ljava/util/Comparator;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaComparator::from_raw(&jni,obj
)}
	pub fn comparing_by_value<K,V>(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>
 where K: JNIRaw<'mc>,V: JNIRaw<'mc>
{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/Comparator")?;
let res = jni.call_static_method(cls,"comparingByValue",
"(Ljava/util/Comparator;)Ljava/util/Comparator;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaComparator::from_raw(&jni,obj
)}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn copy_of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaMapEntry<'mc>>) 
-> Result<crate::JavaMapEntry<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/Map$Entry")?;
let res = jni.call_static_method(cls,"copyOf",
"(Ljava/util/Map$Entry;)Ljava/util/Map$Entry;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaMapEntry::from_raw(&jni,obj
)}
	pub fn value(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getValue","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn key(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getKey","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn set_value(&mut self,arg0: V) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"setValue","(LV;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
}
impl<'mc, K,V> JNIRaw<'mc> for JavaMapEntry<'mc, K,V> where K: JNIRaw<'mc>,V: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaEnumSet<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> blackboxmc_general::JNIRaw<'mc> for JavaEnumSet<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> JavaEnumSet<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaEnumSet from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaEnumSet")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaEnumSet object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn all_of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: jni::objects::JClass<'mc>) 
-> Result<crate::JavaEnumSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let cls = &jni.find_class("java/util/EnumSet")?;
let res = jni.call_static_method(cls,"allOf",
"(Ljava/lang/Class;)Ljava/util/EnumSet;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaEnumSet::from_raw(&jni,obj
)}
	pub fn complement_of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaEnumSet<'mc, E>>) 
-> Result<crate::JavaEnumSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/EnumSet")?;
let res = jni.call_static_method(cls,"complementOf",
"(Ljava/util/EnumSet;)Ljava/util/EnumSet;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaEnumSet::from_raw(&jni,obj
)}
	pub fn clone(&mut self) 
-> Result<crate::JavaEnumSet<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/util/EnumSet;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaEnumSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn copy_of_with_collection(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::JavaEnumSet<'mc, E>>>) 
-> Result<crate::JavaEnumSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/EnumSet")?;
let res = jni.call_static_method(cls,"copyOf",
"(Ljava/util/EnumSet;)Ljava/util/EnumSet;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaEnumSet::from_raw(&jni,obj
)}
	pub fn none_of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: jni::objects::JClass<'mc>) 
-> Result<crate::JavaEnumSet<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let cls = &jni.find_class("java/util/EnumSet")?;
let res = jni.call_static_method(cls,"noneOf",
"(Ljava/lang/Class;)Ljava/util/EnumSet;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::JavaEnumSet::from_raw(&jni,obj
)}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn add(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn add_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
}
impl<'mc, E> Into<crate::JavaAbstractSet<'mc, E>> for JavaEnumSet<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractSet<'mc, E> {
       crate::JavaAbstractSet::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaArrayDeque<'mc, E>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where E: JNIRaw<'mc>;
impl<'mc, E> blackboxmc_general::JNIRaw<'mc> for JavaArrayDeque<'mc, E> where E: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, E> JavaArrayDeque<'mc, E> where E: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaArrayDeque from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaArrayDeque")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaArrayDeque object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i32>) 
-> Result<crate::JavaArrayDeque<'mc, E>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let cls = &jni.find_class("java/util/ArrayDeque")?;
let res = jni.new_object(cls,
"(I)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaArrayDeque::from_raw(&jni,res
)}
	pub fn push(&mut self,arg0: E) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"push","(LE;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn pop(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pop","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn add_first(&mut self,arg0: E) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"addFirst","(LE;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn add_last(&mut self,arg0: E) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"addLast","(LE;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn poll_first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pollFirst","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn poll_last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"pollLast","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn offer_last(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"offerLast","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"removeFirst","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFirst","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn peek_first(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"peekFirst","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn remove_first_occurrence(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"removeFirstOccurrence","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn offer_first(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"offerFirst","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"removeLast","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLast","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn peek_last(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"peekLast","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn remove_last_occurrence(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"removeLastOccurrence","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn offer(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"offer","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn descending_iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"descendingIterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add(&mut self,arg0: E) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.jni_object();
let res = self.jni_ref().call_method(&self.jni_object(),"add","(LE;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"remove","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn clone(&mut self) 
-> Result<crate::JavaArrayDeque<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/util/ArrayDeque;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaArrayDeque::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn clear(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_empty(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"size","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"contains","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"addAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn poll(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"poll","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn peek(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"peek","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn element(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"element","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn remove_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn retain_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"retainAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn contains_all(&mut self,arg0: impl Into<&'mc crate::JavaCollection<'mc, E>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"containsAll","(Ljava/util/Collection;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, E> Into<crate::JavaDeque<'mc, E>> for JavaArrayDeque<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaDeque<'mc, E> {
       crate::JavaDeque::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc, E> Into<crate::JavaAbstractCollection<'mc, E>> for JavaArrayDeque<'mc, E> where E: JNIRaw<'mc>{
   fn into(self) -> crate::JavaAbstractCollection<'mc, E> {
       crate::JavaAbstractCollection::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaDate<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaDate<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaDate<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaDate from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaDate")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaDate object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i64>) 
-> Result<crate::JavaDate<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let cls = &jni.find_class("java/util/Date")?;
let res = jni.new_object(cls,
"(J)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaDate::from_raw(&jni,res
)}
#[deprecated]
	pub fn new_with_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: i32,arg2: i32,arg3: i32,arg4: i32,arg5: std::option::Option<i32>) 
-> Result<crate::JavaDate<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_4 = jni::objects::JValueGen::Int(arg3.into());
let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("java/util/Date")?;
let res = jni.new_object(cls,
"(IIIIII)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
crate::JavaDate::from_raw(&jni,res
)}
	pub fn time(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTime","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
#[deprecated]
	pub fn year(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getYear","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
#[deprecated]
	pub fn seconds(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSeconds","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
#[deprecated]
	pub fn utc(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: i32,arg2: i32,arg3: i32,arg4: i32,arg5: i32) 
-> Result<i64, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Int(arg2.into());
let val_4 = jni::objects::JValueGen::Int(arg3.into());
let val_5 = jni::objects::JValueGen::Int(arg4.into());
let val_6 = jni::objects::JValueGen::Int(arg5.into());
let cls = &jni.find_class("long")?;
let res = jni.call_static_method(cls,"UTC",
"(IIIIII)J",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
Ok(res.j().unwrap())}
	pub fn set_time(&mut self,arg0: i64) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setTime","(J)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
#[deprecated]
	pub fn set_date(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDate","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
#[deprecated]
	pub fn month(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMonth","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
#[deprecated]
	pub fn set_month(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setMonth","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
#[deprecated]
	pub fn hours(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHours","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
#[deprecated]
	pub fn set_hours(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setHours","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
#[deprecated]
	pub fn minutes(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMinutes","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
#[deprecated]
	pub fn set_minutes(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setMinutes","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
#[deprecated]
	pub fn set_seconds(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSeconds","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
#[deprecated]
	pub fn set_year(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setYear","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
#[deprecated]
	pub fn date(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDate","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
#[deprecated]
	pub fn day(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDay","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
#[deprecated]
	pub fn to_locale_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toLocaleString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
#[deprecated]
	pub fn to_gmtstring(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toGMTString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
#[deprecated]
	pub fn timezone_offset(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTimezoneOffset","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
#[deprecated]
	pub fn parse(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc String>) 
-> Result<i64, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
let cls = &jni.find_class("long")?;
let res = jni.call_static_method(cls,"parse",
"(Ljava/lang/String;)J",&[jni::objects::JValueGen::from(&val_1)])?;
Ok(res.j().unwrap())}
	pub fn before(&mut self,arg0: impl Into<&'mc crate::JavaDate<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"before","(Ljava/util/Date;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn after(&mut self,arg0: impl Into<&'mc crate::JavaDate<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"after","(Ljava/util/Date;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn clone(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clone","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn compare_to_with_date(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"compareTo","(Ljava/lang/Object;)I",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaTimer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaTimer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaTimer<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaTimer from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaTimer")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaTimer object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc String>>) 
-> Result<crate::JavaTimer<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(jni.new_string(arg0.unwrap().into()).unwrap());
let cls = &jni.find_class("java/util/Timer")?;
let res = jni.new_object(cls,
"(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaTimer::from_raw(&jni,res
)}
	pub fn new_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc String>,arg1: std::option::Option<bool>) 
-> Result<crate::JavaTimer<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(jni.new_string(arg0.unwrap().into()).unwrap());
// 1
let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
let cls = &jni.find_class("java/util/Timer")?;
let res = jni.new_object(cls,
"(Ljava/lang/String;Z)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::JavaTimer::from_raw(&jni,res
)}
	pub fn cancel(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"cancel","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn schedule_with_timer_task(&mut self,arg0: impl Into<&'mc crate::JavaTimerTask<'mc>>,arg1: impl Into<&'mc crate::JavaDate<'mc>>,arg2: std::option::Option<i64>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = jni::objects::JValueGen::Long(arg2.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"schedule","(Ljava/util/TimerTask;Ljava/util/Date;J)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn schedule_at_fixed_rate_with_timer_task(&mut self,arg0: impl Into<&'mc crate::JavaTimerTask<'mc>>,arg1: i64,arg2: std::option::Option<i64>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
let val_3 = jni::objects::JValueGen::Long(arg2.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"scheduleAtFixedRate","(Ljava/util/TimerTask;JJ)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn purge(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"purge","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
/// An instantiatable struct that implements JavaPrimitiveIteratorOfDouble. Needed for returning it from Java.
pub struct JavaPrimitiveIteratorOfDouble<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaPrimitiveIteratorOfDouble<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaPrimitiveIteratorOfDouble from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaPrimitiveIteratorOfDouble")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaPrimitiveIteratorOfDouble object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn next_double(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextDouble","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn next(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"next","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn remove(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"remove","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn has_next(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasNext","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc> JNIRaw<'mc> for JavaPrimitiveIteratorOfDouble<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaLocaleBuilder<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaLocaleBuilder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaLocaleBuilder<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaLocaleBuilder from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaLocaleBuilder")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaLocaleBuilder object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/Locale$Builder")?;
let res = jni.new_object(cls,
"()V",&[])?;
crate::JavaLocaleBuilder::from_raw(&jni,res
)}
	pub fn set_language_tag(&mut self,arg0: impl Into<&'mc String>) 
-> Result<crate::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setLanguageTag","(Ljava/lang/String;)Ljava/util/Locale$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocaleBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_language(&mut self,arg0: impl Into<&'mc String>) 
-> Result<crate::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setLanguage","(Ljava/lang/String;)Ljava/util/Locale$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocaleBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_script(&mut self,arg0: impl Into<&'mc String>) 
-> Result<crate::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setScript","(Ljava/lang/String;)Ljava/util/Locale$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocaleBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_region(&mut self,arg0: impl Into<&'mc String>) 
-> Result<crate::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setRegion","(Ljava/lang/String;)Ljava/util/Locale$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocaleBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_variant(&mut self,arg0: impl Into<&'mc String>) 
-> Result<crate::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setVariant","(Ljava/lang/String;)Ljava/util/Locale$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocaleBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_extension(&mut self,arg0: u16,arg1: impl Into<&'mc String>) 
-> Result<crate::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Char(arg0.into());
let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setExtension","(CLjava/lang/String;)Ljava/util/Locale$Builder;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocaleBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_unicode_locale_keyword(&mut self,arg0: impl Into<&'mc String>,arg1: impl Into<&'mc String>) 
-> Result<crate::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setUnicodeLocaleKeyword","(Ljava/lang/String;Ljava/lang/String;)Ljava/util/Locale$Builder;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocaleBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_unicode_locale_attribute(&mut self,arg0: impl Into<&'mc String>) 
-> Result<crate::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"addUnicodeLocaleAttribute","(Ljava/lang/String;)Ljava/util/Locale$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocaleBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn remove_unicode_locale_attribute(&mut self,arg0: impl Into<&'mc String>) 
-> Result<crate::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"removeUnicodeLocaleAttribute","(Ljava/lang/String;)Ljava/util/Locale$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocaleBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn clear_extensions(&mut self) 
-> Result<crate::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clearExtensions","()Ljava/util/Locale$Builder;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocaleBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_locale(&mut self,arg0: impl Into<&'mc crate::JavaLocale<'mc>>) 
-> Result<crate::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setLocale","(Ljava/util/Locale;)Ljava/util/Locale$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocaleBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn build(&mut self) 
-> Result<crate::JavaLocale<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"build","()Ljava/util/Locale;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocale::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn clear(&mut self) 
-> Result<crate::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"clear","()Ljava/util/Locale$Builder;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLocaleBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaBase64Decoder<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaBase64Decoder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaBase64Decoder<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaBase64Decoder from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaBase64Decoder")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaBase64Decoder object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn decode_with_string(&mut self,arg0: std::option::Option<Vec<i8>>,arg1: std::option::Option<Vec<i8>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"decode","(BB)I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
pub struct JavaSplittableRandom<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaSplittableRandom<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaSplittableRandom<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaSplittableRandom from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaSplittableRandom")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaSplittableRandom object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i64>) 
-> Result<crate::JavaSplittableRandom<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let cls = &jni.find_class("java/util/SplittableRandom")?;
let res = jni.new_object(cls,
"(J)V",&[jni::objects::JValueGen::from(&val_1)])?;
crate::JavaSplittableRandom::from_raw(&jni,res
)}
	pub fn next_long(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i64>) 
-> Result<i64, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextLong","(JJ)J",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn next_bytes(&mut self,arg0: Vec<i8>) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextBytes","(B)V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn next_int(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextInt","(II)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn split(&mut self,arg0: std::option::Option<impl Into<&'mc crate::random::JavaRandomGeneratorSplittableGenerator<'mc>>>) 
-> Result<crate::JavaSplittableRandom<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"split","(Ljava/util/random/RandomGenerator$SplittableGenerator;)Ljava/util/SplittableRandom;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSplittableRandom::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn next_boolean(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextBoolean","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn next_float(&mut self,arg0: std::option::Option<f32>,arg1: std::option::Option<f32>) 
-> Result<f32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Float(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextFloat","(FF)F",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.f().unwrap())}
	pub fn next_gaussian(&mut self,arg0: std::option::Option<f64>,arg1: std::option::Option<f64>) 
-> Result<f64, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextGaussian","(DD)D",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn is_deprecated(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isDeprecated","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn next_exponential(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"nextExponential","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn next_double(&mut self,arg0: std::option::Option<f64>,arg1: std::option::Option<f64>) 
-> Result<f64, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"nextDouble","(DD)D",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
}
impl<'mc> Into<crate::random::JavaRandomGenerator<'mc>> for JavaSplittableRandom<'mc>{
   fn into(self) -> crate::random::JavaRandomGenerator<'mc> {
       crate::random::JavaRandomGenerator::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::random::JavaRandomGeneratorSplittableGenerator<'mc>> for JavaSplittableRandom<'mc>{
   fn into(self) -> crate::random::JavaRandomGeneratorSplittableGenerator<'mc> {
       crate::random::JavaRandomGeneratorSplittableGenerator::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaSpliteratorsAbstractDoubleSpliterator<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaSpliteratorsAbstractDoubleSpliterator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaSpliteratorsAbstractDoubleSpliterator<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaSpliteratorsAbstractDoubleSpliterator from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaSpliteratorsAbstractDoubleSpliterator")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaSpliteratorsAbstractDoubleSpliterator object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn estimate_size(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"estimateSize","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn try_split(&mut self) 
-> Result<crate::JavaSpliteratorOfDouble<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"trySplit","()Ljava/util/Spliterator$OfDouble;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliteratorOfDouble::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn characteristics(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"characteristics","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn try_advance_with_object(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"tryAdvance","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn for_each_remaining_with_consumer(&mut self,arg0: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let res = self.jni_ref().call_method(&self.jni_object(),"forEachRemaining","(Ljava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn exact_size_if_known(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExactSizeIfKnown","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn has_characteristics(&mut self,arg0: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"hasCharacteristics","(I)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn comparator(&mut self) 
-> Result<crate::JavaComparator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getComparator","()Ljava/util/Comparator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaComparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
}
impl<'mc> Into<crate::JavaSpliteratorOfDouble<'mc>> for JavaSpliteratorsAbstractDoubleSpliterator<'mc>{
   fn into(self) -> crate::JavaSpliteratorOfDouble<'mc> {
       crate::JavaSpliteratorOfDouble::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub mod random;
pub mod logging;
pub mod regex;
