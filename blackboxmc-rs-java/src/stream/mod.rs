#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// An instantiatable struct that implements JavaLongStreamLongMapMultiConsumer. Needed for returning it from Java.
pub struct JavaLongStreamLongMapMultiConsumer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaLongStreamLongMapMultiConsumer<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaLongStreamLongMapMultiConsumer from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaLongStreamLongMapMultiConsumer")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaLongStreamLongMapMultiConsumer object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn accept(&mut self,arg0: i64,arg1: impl Into<&'mc crate::function::JavaLongConsumer<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"accept","(JLjava/util/function/LongConsumer;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> JNIRaw<'mc> for JavaLongStreamLongMapMultiConsumer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaCollectors<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaCollectors<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaCollectors<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaCollectors from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaCollectors")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaCollectors object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn max_by(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaComparator<'mc, T>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaOptional<T>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"maxBy",
"(Ljava/util/Comparator;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn min_by(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::JavaComparator<'mc, T>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaOptional<T>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"minBy",
"(Ljava/util/Comparator;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn summing_long(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaToLongFunction<'mc, T>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaLong>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"summingLong",
"(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn reducing_with_binary_operator(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<T>,arg1: std::option::Option<impl Into<&'mc crate::function::JavaBinaryOperator<'mc, T>>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, T>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"reducing",
"(Ljava/lang/Object;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn reducing_with_object(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: U,arg1: impl Into<&'mc crate::function::JavaFunction<'mc, T>>,arg2: std::option::Option<impl Into<&'mc crate::function::JavaBinaryOperator<'mc, U>>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, U>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"reducing",
"(Ljava/lang/Object;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn grouping_by_concurrent_with_function(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::function::JavaFunction<'mc, T>>>,arg1: std::option::Option<impl Into<&'mc crate::function::JavaSupplier<'mc, M>>>,arg2: std::option::Option<impl Into<&'mc crate::stream::JavaCollector<'mc, T>>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, M>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"groupingByConcurrent",
"(Ljava/util/function/Function;Ljava/util/function/Supplier;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn partitioning_by_with_predicate(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::function::JavaPredicate<'mc, T>>>,arg1: std::option::Option<impl Into<&'mc crate::stream::JavaCollector<'mc, T>>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaMap<javaBoolean, D>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"partitioningBy",
"(Ljava/util/function/Predicate;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn collecting_and_then(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::stream::JavaCollector<'mc, T, A, R>>,arg1: impl Into<&'mc crate::function::JavaFunction<'mc, R, RR>>) 
-> Result<crate::stream::JavaCollector<'mc, T, A, RR>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"collectingAndThen",
"(Ljava/util/stream/Collector;Ljava/util/function/Function;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn to_concurrent_map_with_function(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaFunction<'mc, T>>,arg1: std::option::Option<impl Into<&'mc crate::function::JavaFunction<'mc, T>>>,arg2: std::option::Option<impl Into<&'mc crate::function::JavaBinaryOperator<'mc, U>>>,arg3: std::option::Option<impl Into<&'mc crate::function::JavaSupplier<'mc, M>>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, M>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"toConcurrentMap",
"(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;Ljava/util/function/Supplier;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn to_unmodifiable_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaList<T>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"toUnmodifiableList",
"()Ljava/util/stream/Collector;",&[])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn to_unmodifiable_set(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaSet<T>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"toUnmodifiableSet",
"()Ljava/util/stream/Collector;",&[])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn flat_mapping(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaFunction<'mc, T>>,arg1: impl Into<&'mc crate::stream::JavaCollector<'mc, U>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, R>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"flatMapping",
"(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn filtering(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaPredicate<'mc, T>>,arg1: impl Into<&'mc crate::stream::JavaCollector<'mc, T>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, R>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"filtering",
"(Ljava/util/function/Predicate;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn counting(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaLong>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"counting",
"()Ljava/util/stream/Collector;",&[])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn summing_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaToIntFunction<'mc, T>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaInteger>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"summingInt",
"(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn summing_double(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaToDoubleFunction<'mc, T>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaDouble>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"summingDouble",
"(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn averaging_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaToIntFunction<'mc, T>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaDouble>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"averagingInt",
"(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn averaging_long(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaToLongFunction<'mc, T>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaDouble>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"averagingLong",
"(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn averaging_double(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaToDoubleFunction<'mc, T>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaDouble>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"averagingDouble",
"(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn to_unmodifiable_map_with_function(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaFunction<'mc, T>>,arg1: std::option::Option<impl Into<&'mc crate::function::JavaFunction<'mc, T>>>,arg2: std::option::Option<impl Into<&'mc crate::function::JavaBinaryOperator<'mc, U>>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaMap<K, U>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"toUnmodifiableMap",
"(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn summarizing_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaToIntFunction<'mc, T>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaIntSummaryStatistics>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"summarizingInt",
"(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn summarizing_long(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaToLongFunction<'mc, T>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaLongSummaryStatistics>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"summarizingLong",
"(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn summarizing_double(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaToDoubleFunction<'mc, T>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaDoubleSummaryStatistics>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"summarizingDouble",
"(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn teeing(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::stream::JavaCollector<'mc, T>>,arg1: impl Into<&'mc crate::stream::JavaCollector<'mc, T>>,arg2: impl Into<&'mc crate::function::JavaBiFunction<'mc, R>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, R>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"teeing",
"(Ljava/util/stream/Collector;Ljava/util/stream/Collector;Ljava/util/function/BiFunction;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn joining(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc blackboxmc_java::JavaCharSequence<'mc>>>,arg1: std::option::Option<impl Into<&'mc blackboxmc_java::JavaCharSequence<'mc>>>,arg2: std::option::Option<impl Into<&'mc blackboxmc_java::JavaCharSequence<'mc>>>) 
-> Result<crate::stream::JavaCollector<'mc, javaCharSequence, dyn JNIRaw<'mc>, javaString>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"joining",
"(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn to_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaList<T>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"toList",
"()Ljava/util/stream/Collector;",&[])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn to_map_with_function(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaFunction<'mc, T>>,arg1: std::option::Option<impl Into<&'mc crate::function::JavaFunction<'mc, T>>>,arg2: std::option::Option<impl Into<&'mc crate::function::JavaBinaryOperator<'mc, U>>>,arg3: std::option::Option<impl Into<&'mc crate::function::JavaSupplier<'mc, M>>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, M>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"toMap",
"(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;Ljava/util/function/Supplier;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn to_set(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, javaSet<T>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"toSet",
"()Ljava/util/stream/Collector;",&[])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn to_collection(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaSupplier<'mc, C>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, C>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"toCollection",
"(Ljava/util/function/Supplier;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn mapping(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaFunction<'mc, T>>,arg1: impl Into<&'mc crate::stream::JavaCollector<'mc, U>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, R>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"mapping",
"(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn grouping_by_with_function(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::function::JavaFunction<'mc, T>>>,arg1: std::option::Option<impl Into<&'mc crate::function::JavaSupplier<'mc, M>>>,arg2: std::option::Option<impl Into<&'mc crate::stream::JavaCollector<'mc, T>>>) 
-> Result<crate::stream::JavaCollector<'mc, T, dyn JNIRaw<'mc>, M>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"groupingBy",
"(Ljava/util/function/Function;Ljava/util/function/Supplier;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
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
/// An instantiatable struct that implements JavaBaseStream. Needed for returning it from Java.
pub struct JavaBaseStream<'mc, T,S>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where T: JNIRaw<'mc>,S: JNIRaw<'mc>;
impl<'mc, T,S> JavaBaseStream<'mc, T,S> where T: JNIRaw<'mc>,S: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaBaseStream from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaBaseStream")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaBaseStream object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn sequential(&mut self) 
-> Result<S, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"sequential","()Ljava/util/stream/BaseStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
S::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn unordered(&mut self) 
-> Result<S, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"unordered","()Ljava/util/stream/BaseStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
S::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn close(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"close","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn parallel(&mut self) 
-> Result<S, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"parallel","()Ljava/util/stream/BaseStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
S::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn on_close(&mut self,arg0: impl Into<&'mc blackboxmc_java::JavaRunnable<'mc>>) 
-> Result<S, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"onClose","(Ljava/lang/Runnable;)Ljava/util/stream/BaseStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
S::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_parallel(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isParallel","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc, T,S> JNIRaw<'mc> for JavaBaseStream<'mc, T,S> where T: JNIRaw<'mc>,S: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaCollectorCharacteristics<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaCollectorCharacteristics<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaCollectorCharacteristics<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaCollectorCharacteristics from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaCollectorCharacteristics")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaCollectorCharacteristics object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value_of_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<T, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Enum")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
T::from_raw(&jni,obj
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"name","()Ljava/lang/String;",&[]);
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
	pub fn describe_constable(&mut self) 
-> Result<crate::JavaOptional<'mc, javaEnumEnumDesc<E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"describeConstable","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn declaring_class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDeclaringClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn ordinal(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"ordinal","()I",&[]);
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
/// An instantiatable struct that implements JavaIntStreamBuilder. Needed for returning it from Java.
pub struct JavaIntStreamBuilder<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaIntStreamBuilder<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaIntStreamBuilder from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaIntStreamBuilder")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaIntStreamBuilder object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn add(&mut self,arg0: i32) 
-> Result<crate::stream::JavaIntStreamBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"add","(I)Ljava/util/stream/IntStream$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStreamBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn accept(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"accept","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn build(&mut self) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"build","()Ljava/util/stream/IntStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn and_then(&mut self,arg0: impl Into<&'mc crate::function::JavaIntConsumer<'mc>>) 
-> Result<crate::function::JavaIntConsumer<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"andThen","(Ljava/util/function/IntConsumer;)Ljava/util/function/IntConsumer;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::function::JavaIntConsumer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
}
impl<'mc> JNIRaw<'mc> for JavaIntStreamBuilder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::function::JavaIntConsumer<'mc>> for JavaIntStreamBuilder<'mc>{
   fn into(self) -> crate::function::JavaIntConsumer<'mc> {
       crate::function::JavaIntConsumer::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct JavaStreamSupport<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaStreamSupport<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaStreamSupport<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaStreamSupport from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaStreamSupport")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaStreamSupport object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn stream_with_spliterator(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaSupplier<'mc, j>>,arg1: std::option::Option<i32>,arg2: std::option::Option<bool>) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
// 1
let val_3 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
let cls = &jni.find_class("java/util/stream/Stream")?;
let res = jni.call_static_method(cls,"stream",
"(Ljava/util/function/Supplier;IZ)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::stream::JavaStream::from_raw(&jni,obj
)}
	pub fn int_stream_with_spliteratorof_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaSupplier<'mc, j>>,arg1: std::option::Option<i32>,arg2: std::option::Option<bool>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
// 1
let val_3 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
let cls = &jni.find_class("java/util/stream/IntStream")?;
let res = jni.call_static_method(cls,"intStream",
"(Ljava/util/function/Supplier;IZ)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::stream::JavaIntStream::from_raw(&jni,obj
)}
	pub fn long_stream_with_spliteratorof_long(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaSupplier<'mc, j>>,arg1: std::option::Option<i32>,arg2: std::option::Option<bool>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
// 1
let val_3 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
let cls = &jni.find_class("java/util/stream/LongStream")?;
let res = jni.call_static_method(cls,"longStream",
"(Ljava/util/function/Supplier;IZ)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::stream::JavaLongStream::from_raw(&jni,obj
)}
	pub fn double_stream_with_spliteratorof_double(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaSupplier<'mc, j>>,arg1: std::option::Option<i32>,arg2: std::option::Option<bool>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
// 1
let val_3 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
let cls = &jni.find_class("java/util/stream/DoubleStream")?;
let res = jni.call_static_method(cls,"doubleStream",
"(Ljava/util/function/Supplier;IZ)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::stream::JavaDoubleStream::from_raw(&jni,obj
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
/// An instantiatable struct that implements JavaLongStream. Needed for returning it from Java.
pub struct JavaLongStream<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaLongStream<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaLongStream from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaLongStream")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaLongStream object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn sorted(&mut self) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"sorted","()Ljava/util/stream/LongStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn generate(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaLongSupplier<'mc>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/LongStream")?;
let res = jni.call_static_method(cls,"generate",
"(Ljava/util/function/LongSupplier;)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaLongStream::from_raw(&jni,obj
)}
	pub fn map_to_int(&mut self,arg0: impl Into<&'mc crate::function::JavaLongToIntFunction<'mc>>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapToInt","(Ljava/util/function/LongToIntFunction;)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map_to_double(&mut self,arg0: impl Into<&'mc crate::function::JavaLongToDoubleFunction<'mc>>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapToDouble","(Ljava/util/function/LongToDoubleFunction;)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map_multi(&mut self,arg0: impl Into<&'mc crate::stream::JavaLongStreamLongMapMultiConsumer<'mc>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapMulti","(Ljava/util/stream/LongStream$LongMapMultiConsumer;)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn take_while(&mut self,arg0: impl Into<&'mc crate::function::JavaLongPredicate<'mc>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"takeWhile","(Ljava/util/function/LongPredicate;)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn drop_while(&mut self,arg0: impl Into<&'mc crate::function::JavaLongPredicate<'mc>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"dropWhile","(Ljava/util/function/LongPredicate;)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn none_match(&mut self,arg0: impl Into<&'mc crate::function::JavaLongPredicate<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"noneMatch","(Ljava/util/function/LongPredicate;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn sequential(&mut self) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"sequential","()Ljava/util/stream/LongStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn as_double_stream(&mut self) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asDoubleStream","()Ljava/util/stream/DoubleStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn average(&mut self) 
-> Result<crate::JavaOptionalDouble<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"average","()Ljava/util/OptionalDouble;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptionalDouble::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn summary_statistics(&mut self) 
-> Result<crate::JavaLongSummaryStatistics<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"summaryStatistics","()Ljava/util/LongSummaryStatistics;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaLongSummaryStatistics::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn range_closed(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i64,arg1: i64) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let val_2 = jni::objects::JValueGen::Long(arg1.into());
let cls = &jni.find_class("java/util/stream/LongStream")?;
let res = jni.call_static_method(cls,"rangeClosed",
"(JJ)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::stream::JavaLongStream::from_raw(&jni,obj
)}
	pub fn for_each_ordered(&mut self,arg0: impl Into<&'mc crate::function::JavaLongConsumer<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"forEachOrdered","(Ljava/util/function/LongConsumer;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn map_to_obj(&mut self,arg0: impl Into<&'mc crate::function::JavaLongFunction<'mc, U>>) 
-> Result<crate::stream::JavaStream<'mc, U>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapToObj","(Ljava/util/function/LongFunction;)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn boxed(&mut self) 
-> Result<crate::stream::JavaStream<'mc, javaLong>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"boxed","()Ljava/util/stream/Stream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn min(&mut self) 
-> Result<crate::JavaOptionalLong<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"min","()Ljava/util/OptionalLong;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptionalLong::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn max(&mut self) 
-> Result<crate::JavaOptionalLong<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"max","()Ljava/util/OptionalLong;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptionalLong::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map(&mut self,arg0: impl Into<&'mc crate::function::JavaLongUnaryOperator<'mc>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"map","(Ljava/util/function/LongUnaryOperator;)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn collect(&mut self,arg0: impl Into<&'mc crate::function::JavaSupplier<'mc, R>>,arg1: impl Into<&'mc crate::function::JavaObjLongConsumer<'mc, R>>,arg2: impl Into<&'mc crate::function::JavaBiConsumer<'mc, R, R>>) 
-> Result<R, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"collect","(Ljava/util/function/Supplier;Ljava/util/function/ObjLongConsumer;Ljava/util/function/BiConsumer;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
R::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn of_with_long(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<i64>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/stream/LongStream")?;
let res = jni.call_static_method(cls,"of",
"(J)Ljava/util/stream/LongStream;",&[])?;
let mut obj = res.l()?;
crate::stream::JavaLongStream::from_raw(&jni,obj
)}
	pub fn count(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"count","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn builder(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::stream::JavaLongStreamBuilder<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/stream/LongStream$Builder")?;
let res = jni.call_static_method(cls,"builder",
"()Ljava/util/stream/LongStream$Builder;",&[])?;
let mut obj = res.l()?;
crate::stream::JavaLongStreamBuilder::from_raw(&jni,obj
)}
	pub fn concat(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::stream::JavaLongStream<'mc>>,arg1: impl Into<&'mc crate::stream::JavaLongStream<'mc>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/LongStream")?;
let res = jni.call_static_method(cls,"concat",
"(Ljava/util/stream/LongStream;Ljava/util/stream/LongStream;)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::stream::JavaLongStream::from_raw(&jni,obj
)}
	pub fn limit(&mut self,arg0: i64) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"limit","(J)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliteratorOfLong<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator$OfLong;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliteratorOfLong::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn filter(&mut self,arg0: impl Into<&'mc crate::function::JavaLongPredicate<'mc>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"filter","(Ljava/util/function/LongPredicate;)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn empty(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/stream/LongStream")?;
let res = jni.call_static_method(cls,"empty",
"()Ljava/util/stream/LongStream;",&[])?;
let mut obj = res.l()?;
crate::stream::JavaLongStream::from_raw(&jni,obj
)}
	pub fn any_match(&mut self,arg0: impl Into<&'mc crate::function::JavaLongPredicate<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"anyMatch","(Ljava/util/function/LongPredicate;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn flat_map(&mut self,arg0: impl Into<&'mc crate::function::JavaLongFunction<'mc, j>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"flatMap","(Ljava/util/function/LongFunction;)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn for_each(&mut self,arg0: impl Into<&'mc crate::function::JavaLongConsumer<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"forEach","(Ljava/util/function/LongConsumer;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn find_any(&mut self) 
-> Result<crate::JavaOptionalLong<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"findAny","()Ljava/util/OptionalLong;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptionalLong::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn skip(&mut self,arg0: i64) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"skip","(J)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn peek(&mut self,arg0: impl Into<&'mc crate::function::JavaLongConsumer<'mc>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"peek","(Ljava/util/function/LongConsumer;)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn sum(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"sum","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn reduce_with_long_binary_operator(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<impl Into<&'mc crate::function::JavaLongBinaryOperator<'mc>>>) 
-> Result<i64, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"reduce","(JLjava/util/function/LongBinaryOperator;)J",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn distinct(&mut self) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"distinct","()Ljava/util/stream/LongStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn find_first(&mut self) 
-> Result<crate::JavaOptionalLong<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"findFirst","()Ljava/util/OptionalLong;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptionalLong::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn all_match(&mut self,arg0: impl Into<&'mc crate::function::JavaLongPredicate<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"allMatch","(Ljava/util/function/LongPredicate;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn iterate_with_long(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i64,arg1: std::option::Option<impl Into<&'mc crate::function::JavaLongPredicate<'mc>>>,arg2: std::option::Option<impl Into<&'mc crate::function::JavaLongUnaryOperator<'mc>>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/LongStream")?;
let res = jni.call_static_method(cls,"iterate",
"(JLjava/util/function/LongPredicate;Ljava/util/function/LongUnaryOperator;)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::stream::JavaLongStream::from_raw(&jni,obj
)}
	pub fn range(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i64,arg1: i64) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let val_2 = jni::objects::JValueGen::Long(arg1.into());
let cls = &jni.find_class("java/util/stream/LongStream")?;
let res = jni.call_static_method(cls,"range",
"(JJ)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::stream::JavaLongStream::from_raw(&jni,obj
)}
	pub fn parallel(&mut self) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"parallel","()Ljava/util/stream/LongStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn unordered(&mut self) 
-> Result<S, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"unordered","()Ljava/util/stream/BaseStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
S::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn close(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"close","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn on_close(&mut self,arg0: impl Into<&'mc blackboxmc_java::JavaRunnable<'mc>>) 
-> Result<S, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"onClose","(Ljava/lang/Runnable;)Ljava/util/stream/BaseStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
S::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_parallel(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isParallel","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc> JNIRaw<'mc> for JavaLongStream<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaStreamBuilder. Needed for returning it from Java.
pub struct JavaStreamBuilder<'mc, T>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where T: JNIRaw<'mc>;
impl<'mc, T> JavaStreamBuilder<'mc, T> where T: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaStreamBuilder from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaStreamBuilder")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaStreamBuilder object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn add(&mut self,arg0: T) 
-> Result<crate::stream::JavaStreamBuilder<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"add","(Ljava/lang/Object;)Ljava/util/stream/Stream$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStreamBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn accept(&mut self,arg0: T) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"accept","(Ljava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn build(&mut self) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"build","()Ljava/util/stream/Stream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn and_then(&mut self,arg0: impl Into<&'mc crate::function::JavaConsumer<'mc, T>>) 
-> Result<crate::function::JavaConsumer<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"andThen","(Ljava/util/function/Consumer;)Ljava/util/function/Consumer;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::function::JavaConsumer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
}
impl<'mc, T> JNIRaw<'mc> for JavaStreamBuilder<'mc, T> where T: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaDoubleStreamBuilder. Needed for returning it from Java.
pub struct JavaDoubleStreamBuilder<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaDoubleStreamBuilder<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaDoubleStreamBuilder from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaDoubleStreamBuilder")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaDoubleStreamBuilder object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn add(&mut self,arg0: f64) 
-> Result<crate::stream::JavaDoubleStreamBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Double(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"add","(D)Ljava/util/stream/DoubleStream$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStreamBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn accept(&mut self,arg0: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Double(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"accept","(D)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn build(&mut self) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"build","()Ljava/util/stream/DoubleStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn and_then(&mut self,arg0: impl Into<&'mc crate::function::JavaDoubleConsumer<'mc>>) 
-> Result<crate::function::JavaDoubleConsumer<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"andThen","(Ljava/util/function/DoubleConsumer;)Ljava/util/function/DoubleConsumer;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::function::JavaDoubleConsumer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
}
impl<'mc> JNIRaw<'mc> for JavaDoubleStreamBuilder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::function::JavaDoubleConsumer<'mc>> for JavaDoubleStreamBuilder<'mc>{
   fn into(self) -> crate::function::JavaDoubleConsumer<'mc> {
       crate::function::JavaDoubleConsumer::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
/// An instantiatable struct that implements JavaDoubleStreamDoubleMapMultiConsumer. Needed for returning it from Java.
pub struct JavaDoubleStreamDoubleMapMultiConsumer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaDoubleStreamDoubleMapMultiConsumer<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaDoubleStreamDoubleMapMultiConsumer from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaDoubleStreamDoubleMapMultiConsumer")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaDoubleStreamDoubleMapMultiConsumer object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn accept(&mut self,arg0: f64,arg1: impl Into<&'mc crate::function::JavaDoubleConsumer<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Double(arg0.into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"accept","(DLjava/util/function/DoubleConsumer;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> JNIRaw<'mc> for JavaDoubleStreamDoubleMapMultiConsumer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaIntStreamIntMapMultiConsumer. Needed for returning it from Java.
pub struct JavaIntStreamIntMapMultiConsumer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaIntStreamIntMapMultiConsumer<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaIntStreamIntMapMultiConsumer from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaIntStreamIntMapMultiConsumer")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaIntStreamIntMapMultiConsumer object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn accept(&mut self,arg0: i32,arg1: impl Into<&'mc crate::function::JavaIntConsumer<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"accept","(ILjava/util/function/IntConsumer;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> JNIRaw<'mc> for JavaIntStreamIntMapMultiConsumer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaLongStreamBuilder. Needed for returning it from Java.
pub struct JavaLongStreamBuilder<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaLongStreamBuilder<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaLongStreamBuilder from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaLongStreamBuilder")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaLongStreamBuilder object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn add(&mut self,arg0: i64) 
-> Result<crate::stream::JavaLongStreamBuilder<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"add","(J)Ljava/util/stream/LongStream$Builder;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStreamBuilder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn accept(&mut self,arg0: i64) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"accept","(J)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn build(&mut self) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"build","()Ljava/util/stream/LongStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn and_then(&mut self,arg0: impl Into<&'mc crate::function::JavaLongConsumer<'mc>>) 
-> Result<crate::function::JavaLongConsumer<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"andThen","(Ljava/util/function/LongConsumer;)Ljava/util/function/LongConsumer;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::function::JavaLongConsumer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
}
impl<'mc> JNIRaw<'mc> for JavaLongStreamBuilder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::function::JavaLongConsumer<'mc>> for JavaLongStreamBuilder<'mc>{
   fn into(self) -> crate::function::JavaLongConsumer<'mc> {
       crate::function::JavaLongConsumer::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
/// An instantiatable struct that implements JavaCollector. Needed for returning it from Java.
pub struct JavaCollector<'mc, T,A,R>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where T: JNIRaw<'mc>,A: JNIRaw<'mc>,R: JNIRaw<'mc>;
impl<'mc, T,A,R> JavaCollector<'mc, T,A,R> where T: JNIRaw<'mc>,A: JNIRaw<'mc>,R: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaCollector from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaCollector")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaCollector object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn accumulator(&mut self) 
-> Result<crate::function::JavaBiConsumer<'mc, A, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"accumulator","()Ljava/util/function/BiConsumer;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::function::JavaBiConsumer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn finisher(&mut self) 
-> Result<crate::function::JavaFunction<'mc, A, R>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"finisher","()Ljava/util/function/Function;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::function::JavaFunction::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn of_with_supplier(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaSupplier<'mc, A>>,arg1: impl Into<&'mc crate::function::JavaBiConsumer<'mc, A, T>>,arg2: impl Into<&'mc crate::function::JavaBinaryOperator<'mc, A>>,arg3: std::option::Option<impl Into<&'mc crate::function::JavaFunction<'mc, A, R>>>,arg4: std::option::Option<Vec<impl Into<crate::stream::JavaCollectorCharacteristics<'mc>>>>) 
-> Result<crate::stream::JavaCollector<'mc, T, A, R>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Collector")?;
let res = jni.call_static_method(cls,"of",
"(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/stream/Collector$Characteristics;)Ljava/util/stream/Collector;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
let mut obj = res.l()?;
crate::stream::JavaCollector::from_raw(&jni,obj
)}
	pub fn characteristics(&mut self) 
-> Result<crate::JavaSet<'mc, javaCollectorCharacteristics>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"characteristics","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn combiner(&mut self) 
-> Result<crate::function::JavaBinaryOperator<'mc, A>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"combiner","()Ljava/util/function/BinaryOperator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::function::JavaBinaryOperator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn supplier(&mut self) 
-> Result<crate::function::JavaSupplier<'mc, A>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"supplier","()Ljava/util/function/Supplier;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::function::JavaSupplier::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
}
impl<'mc, T,A,R> JNIRaw<'mc> for JavaCollector<'mc, T,A,R> where T: JNIRaw<'mc>,A: JNIRaw<'mc>,R: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaStream. Needed for returning it from Java.
pub struct JavaStream<'mc, T>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where T: JNIRaw<'mc>;
impl<'mc, T> JavaStream<'mc, T> where T: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaStream from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaStream")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaStream object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn sorted(&mut self,arg0: std::option::Option<impl Into<&'mc crate::JavaComparator<'mc, T>>>) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"sorted","(Ljava/util/Comparator;)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn generate(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaSupplier<'mc, T>>) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Stream")?;
let res = jni.call_static_method(cls,"generate",
"(Ljava/util/function/Supplier;)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaStream::from_raw(&jni,obj
)}
	pub fn map_to_int(&mut self,arg0: impl Into<&'mc crate::function::JavaToIntFunction<'mc, T>>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapToInt","(Ljava/util/function/ToIntFunction;)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map_to_double(&mut self,arg0: impl Into<&'mc crate::function::JavaToDoubleFunction<'mc, T>>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapToDouble","(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn flat_map_to_int(&mut self,arg0: impl Into<&'mc crate::function::JavaFunction<'mc, T>>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"flatMapToInt","(Ljava/util/function/Function;)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn flat_map_to_double(&mut self,arg0: impl Into<&'mc crate::function::JavaFunction<'mc, T>>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"flatMapToDouble","(Ljava/util/function/Function;)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn flat_map_to_long(&mut self,arg0: impl Into<&'mc crate::function::JavaFunction<'mc, T>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"flatMapToLong","(Ljava/util/function/Function;)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map_multi(&mut self,arg0: impl Into<&'mc crate::function::JavaBiConsumer<'mc, T>>) 
-> Result<crate::stream::JavaStream<'mc, R>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapMulti","(Ljava/util/function/BiConsumer;)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map_multi_to_int(&mut self,arg0: impl Into<&'mc crate::function::JavaBiConsumer<'mc, T>>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapMultiToInt","(Ljava/util/function/BiConsumer;)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map_multi_to_long(&mut self,arg0: impl Into<&'mc crate::function::JavaBiConsumer<'mc, T>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapMultiToLong","(Ljava/util/function/BiConsumer;)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map_multi_to_double(&mut self,arg0: impl Into<&'mc crate::function::JavaBiConsumer<'mc, T>>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapMultiToDouble","(Ljava/util/function/BiConsumer;)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn take_while(&mut self,arg0: impl Into<&'mc crate::function::JavaPredicate<'mc, T>>) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"takeWhile","(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn drop_while(&mut self,arg0: impl Into<&'mc crate::function::JavaPredicate<'mc, T>>) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"dropWhile","(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn none_match(&mut self,arg0: impl Into<&'mc crate::function::JavaPredicate<'mc, T>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"noneMatch","(Ljava/util/function/Predicate;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn for_each_ordered(&mut self,arg0: impl Into<&'mc crate::function::JavaConsumer<'mc, T>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"forEachOrdered","(Ljava/util/function/Consumer;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn min(&mut self,arg0: impl Into<&'mc crate::JavaComparator<'mc, T>>) 
-> Result<crate::JavaOptional<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"min","(Ljava/util/Comparator;)Ljava/util/Optional;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn max(&mut self,arg0: impl Into<&'mc crate::JavaComparator<'mc, T>>) 
-> Result<crate::JavaOptional<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"max","(Ljava/util/Comparator;)Ljava/util/Optional;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map(&mut self,arg0: impl Into<&'mc crate::function::JavaFunction<'mc, T>>) 
-> Result<crate::stream::JavaStream<'mc, R>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"map","(Ljava/util/function/Function;)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn collect_with_collector(&mut self,arg0: std::option::Option<impl Into<&'mc crate::function::JavaSupplier<'mc, R>>>,arg1: std::option::Option<impl Into<&'mc crate::function::JavaBiConsumer<'mc, T>>>,arg2: std::option::Option<impl Into<&'mc crate::function::JavaBiConsumer<'mc, R, R>>>) 
-> Result<R, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"collect","(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BiConsumer;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
R::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn to_list(&mut self) 
-> Result<crate::JavaList<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toList","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn of_with_object(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<Vec<jni::objects::JObject<'mc>>>) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/stream/Stream")?;
let res = jni.call_static_method(cls,"of",
"(Ljava/lang/Object;)Ljava/util/stream/Stream;",&[])?;
let mut obj = res.l()?;
crate::stream::JavaStream::from_raw(&jni,obj
)}
	pub fn count(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"count","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn builder(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::stream::JavaStreamBuilder<'mc, T>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/stream/Stream$Builder")?;
let res = jni.call_static_method(cls,"builder",
"()Ljava/util/stream/Stream$Builder;",&[])?;
let mut obj = res.l()?;
crate::stream::JavaStreamBuilder::from_raw(&jni,obj
)}
	pub fn concat(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::stream::JavaStream<'mc, T>>,arg1: impl Into<&'mc crate::stream::JavaStream<'mc, T>>) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Stream")?;
let res = jni.call_static_method(cls,"concat",
"(Ljava/util/stream/Stream;Ljava/util/stream/Stream;)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::stream::JavaStream::from_raw(&jni,obj
)}
	pub fn limit(&mut self,arg0: i64) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"limit","(J)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn filter(&mut self,arg0: impl Into<&'mc crate::function::JavaPredicate<'mc, T>>) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"filter","(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn empty(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/stream/Stream")?;
let res = jni.call_static_method(cls,"empty",
"()Ljava/util/stream/Stream;",&[])?;
let mut obj = res.l()?;
crate::stream::JavaStream::from_raw(&jni,obj
)}
	pub fn any_match(&mut self,arg0: impl Into<&'mc crate::function::JavaPredicate<'mc, T>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"anyMatch","(Ljava/util/function/Predicate;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn flat_map(&mut self,arg0: impl Into<&'mc crate::function::JavaFunction<'mc, T>>) 
-> Result<crate::stream::JavaStream<'mc, R>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"flatMap","(Ljava/util/function/Function;)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn for_each(&mut self,arg0: impl Into<&'mc crate::function::JavaConsumer<'mc, T>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"forEach","(Ljava/util/function/Consumer;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn find_any(&mut self) 
-> Result<crate::JavaOptional<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"findAny","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn skip(&mut self,arg0: i64) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"skip","(J)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn peek(&mut self,arg0: impl Into<&'mc crate::function::JavaConsumer<'mc, T>>) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"peek","(Ljava/util/function/Consumer;)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn reduce_with_binary_operator(&mut self,arg0: std::option::Option<T>,arg1: std::option::Option<impl Into<&'mc crate::function::JavaBinaryOperator<'mc, T>>>) 
-> Result<T, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"reduce","(Ljava/lang/Object;Ljava/util/function/BinaryOperator;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
T::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn reduce_with_object(&mut self,arg0: U,arg1: impl Into<&'mc crate::function::JavaBiFunction<'mc, T>>,arg2: std::option::Option<impl Into<&'mc crate::function::JavaBinaryOperator<'mc, U>>>) 
-> Result<U, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"reduce","(Ljava/lang/Object;Ljava/util/function/BiFunction;Ljava/util/function/BinaryOperator;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
U::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn distinct(&mut self) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"distinct","()Ljava/util/stream/Stream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn find_first(&mut self) 
-> Result<crate::JavaOptional<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"findFirst","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn all_match(&mut self,arg0: impl Into<&'mc crate::function::JavaPredicate<'mc, T>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"allMatch","(Ljava/util/function/Predicate;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn iterate_with_object(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: T,arg1: std::option::Option<impl Into<&'mc crate::function::JavaPredicate<'mc, T>>>,arg2: std::option::Option<impl Into<&'mc crate::function::JavaUnaryOperator<'mc, T>>>) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/Stream")?;
let res = jni.call_static_method(cls,"iterate",
"(Ljava/lang/Object;Ljava/util/function/Predicate;Ljava/util/function/UnaryOperator;)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::stream::JavaStream::from_raw(&jni,obj
)}
	pub fn of_nullable(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: T) 
-> Result<crate::stream::JavaStream<'mc, T>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let cls = &jni.find_class("java/util/stream/Stream")?;
let res = jni.call_static_method(cls,"ofNullable",
"(Ljava/lang/Object;)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaStream::from_raw(&jni,obj
)}
	pub fn map_to_long(&mut self,arg0: impl Into<&'mc crate::function::JavaToLongFunction<'mc, T>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapToLong","(Ljava/util/function/ToLongFunction;)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn sequential(&mut self) 
-> Result<S, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"sequential","()Ljava/util/stream/BaseStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
S::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn unordered(&mut self) 
-> Result<S, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"unordered","()Ljava/util/stream/BaseStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
S::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn iterator(&mut self) 
-> Result<crate::JavaIterator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/Iterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc, T>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn close(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"close","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn parallel(&mut self) 
-> Result<S, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"parallel","()Ljava/util/stream/BaseStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
S::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn on_close(&mut self,arg0: impl Into<&'mc blackboxmc_java::JavaRunnable<'mc>>) 
-> Result<S, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"onClose","(Ljava/lang/Runnable;)Ljava/util/stream/BaseStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
S::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_parallel(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isParallel","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc, T> JNIRaw<'mc> for JavaStream<'mc, T> where T: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaDoubleStream. Needed for returning it from Java.
pub struct JavaDoubleStream<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaDoubleStream<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaDoubleStream from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaDoubleStream")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaDoubleStream object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn sorted(&mut self) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"sorted","()Ljava/util/stream/DoubleStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn generate(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaDoubleSupplier<'mc>>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/DoubleStream")?;
let res = jni.call_static_method(cls,"generate",
"(Ljava/util/function/DoubleSupplier;)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaDoubleStream::from_raw(&jni,obj
)}
	pub fn map_to_int(&mut self,arg0: impl Into<&'mc crate::function::JavaDoubleToIntFunction<'mc>>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapToInt","(Ljava/util/function/DoubleToIntFunction;)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map_multi(&mut self,arg0: impl Into<&'mc crate::stream::JavaDoubleStreamDoubleMapMultiConsumer<'mc>>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapMulti","(Ljava/util/stream/DoubleStream$DoubleMapMultiConsumer;)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn take_while(&mut self,arg0: impl Into<&'mc crate::function::JavaDoublePredicate<'mc>>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"takeWhile","(Ljava/util/function/DoublePredicate;)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn drop_while(&mut self,arg0: impl Into<&'mc crate::function::JavaDoublePredicate<'mc>>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"dropWhile","(Ljava/util/function/DoublePredicate;)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn none_match(&mut self,arg0: impl Into<&'mc crate::function::JavaDoublePredicate<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"noneMatch","(Ljava/util/function/DoublePredicate;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn sequential(&mut self) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"sequential","()Ljava/util/stream/DoubleStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn average(&mut self) 
-> Result<crate::JavaOptionalDouble<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"average","()Ljava/util/OptionalDouble;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptionalDouble::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn summary_statistics(&mut self) 
-> Result<crate::JavaDoubleSummaryStatistics<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"summaryStatistics","()Ljava/util/DoubleSummaryStatistics;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaDoubleSummaryStatistics::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn for_each_ordered(&mut self,arg0: impl Into<&'mc crate::function::JavaDoubleConsumer<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"forEachOrdered","(Ljava/util/function/DoubleConsumer;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn map_to_obj(&mut self,arg0: impl Into<&'mc crate::function::JavaDoubleFunction<'mc, U>>) 
-> Result<crate::stream::JavaStream<'mc, U>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapToObj","(Ljava/util/function/DoubleFunction;)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn boxed(&mut self) 
-> Result<crate::stream::JavaStream<'mc, javaDouble>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"boxed","()Ljava/util/stream/Stream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn min(&mut self) 
-> Result<crate::JavaOptionalDouble<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"min","()Ljava/util/OptionalDouble;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptionalDouble::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn max(&mut self) 
-> Result<crate::JavaOptionalDouble<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"max","()Ljava/util/OptionalDouble;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptionalDouble::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn iterator(&mut self) 
-> Result<crate::JavaPrimitiveIteratorOfDouble<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/PrimitiveIterator$OfDouble;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaPrimitiveIteratorOfDouble::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map(&mut self,arg0: impl Into<&'mc crate::function::JavaDoubleUnaryOperator<'mc>>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"map","(Ljava/util/function/DoubleUnaryOperator;)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn collect(&mut self,arg0: impl Into<&'mc crate::function::JavaSupplier<'mc, R>>,arg1: impl Into<&'mc crate::function::JavaObjDoubleConsumer<'mc, R>>,arg2: impl Into<&'mc crate::function::JavaBiConsumer<'mc, R, R>>) 
-> Result<R, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"collect","(Ljava/util/function/Supplier;Ljava/util/function/ObjDoubleConsumer;Ljava/util/function/BiConsumer;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
R::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn of_with_doubles(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<f64>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
let cls = &jni.find_class("java/util/stream/DoubleStream")?;
let res = jni.call_static_method(cls,"of",
"(D)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaDoubleStream::from_raw(&jni,obj
)}
	pub fn count(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"count","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn builder(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::stream::JavaDoubleStreamBuilder<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/stream/DoubleStream$Builder")?;
let res = jni.call_static_method(cls,"builder",
"()Ljava/util/stream/DoubleStream$Builder;",&[])?;
let mut obj = res.l()?;
crate::stream::JavaDoubleStreamBuilder::from_raw(&jni,obj
)}
	pub fn concat(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::stream::JavaDoubleStream<'mc>>,arg1: impl Into<&'mc crate::stream::JavaDoubleStream<'mc>>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/DoubleStream")?;
let res = jni.call_static_method(cls,"concat",
"(Ljava/util/stream/DoubleStream;Ljava/util/stream/DoubleStream;)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::stream::JavaDoubleStream::from_raw(&jni,obj
)}
	pub fn limit(&mut self,arg0: i64) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"limit","(J)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn filter(&mut self,arg0: impl Into<&'mc crate::function::JavaDoublePredicate<'mc>>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"filter","(Ljava/util/function/DoublePredicate;)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn empty(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/stream/DoubleStream")?;
let res = jni.call_static_method(cls,"empty",
"()Ljava/util/stream/DoubleStream;",&[])?;
let mut obj = res.l()?;
crate::stream::JavaDoubleStream::from_raw(&jni,obj
)}
	pub fn any_match(&mut self,arg0: impl Into<&'mc crate::function::JavaDoublePredicate<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"anyMatch","(Ljava/util/function/DoublePredicate;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn flat_map(&mut self,arg0: impl Into<&'mc crate::function::JavaDoubleFunction<'mc, j>>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"flatMap","(Ljava/util/function/DoubleFunction;)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn for_each(&mut self,arg0: impl Into<&'mc crate::function::JavaDoubleConsumer<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"forEach","(Ljava/util/function/DoubleConsumer;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn find_any(&mut self) 
-> Result<crate::JavaOptionalDouble<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"findAny","()Ljava/util/OptionalDouble;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptionalDouble::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn skip(&mut self,arg0: i64) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"skip","(J)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn peek(&mut self,arg0: impl Into<&'mc crate::function::JavaDoubleConsumer<'mc>>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"peek","(Ljava/util/function/DoubleConsumer;)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn sum(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"sum","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn reduce_with_double_binary_operator(&mut self,arg0: std::option::Option<f64>,arg1: std::option::Option<impl Into<&'mc crate::function::JavaDoubleBinaryOperator<'mc>>>) 
-> Result<f64, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"reduce","(DLjava/util/function/DoubleBinaryOperator;)D",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn distinct(&mut self) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"distinct","()Ljava/util/stream/DoubleStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn find_first(&mut self) 
-> Result<crate::JavaOptionalDouble<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"findFirst","()Ljava/util/OptionalDouble;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptionalDouble::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn all_match(&mut self,arg0: impl Into<&'mc crate::function::JavaDoublePredicate<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"allMatch","(Ljava/util/function/DoublePredicate;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn iterate_with_double(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: f64,arg1: std::option::Option<impl Into<&'mc crate::function::JavaDoublePredicate<'mc>>>,arg2: std::option::Option<impl Into<&'mc crate::function::JavaDoubleUnaryOperator<'mc>>>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/DoubleStream")?;
let res = jni.call_static_method(cls,"iterate",
"(DLjava/util/function/DoublePredicate;Ljava/util/function/DoubleUnaryOperator;)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::stream::JavaDoubleStream::from_raw(&jni,obj
)}
	pub fn parallel(&mut self) 
-> Result<crate::stream::JavaBaseStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"parallel","()Ljava/util/stream/BaseStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaBaseStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map_to_long(&mut self,arg0: impl Into<&'mc crate::function::JavaDoubleToLongFunction<'mc>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapToLong","(Ljava/util/function/DoubleToLongFunction;)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn unordered(&mut self) 
-> Result<S, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"unordered","()Ljava/util/stream/BaseStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
S::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn close(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"close","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn on_close(&mut self,arg0: impl Into<&'mc blackboxmc_java::JavaRunnable<'mc>>) 
-> Result<S, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"onClose","(Ljava/lang/Runnable;)Ljava/util/stream/BaseStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
S::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_parallel(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isParallel","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc> JNIRaw<'mc> for JavaDoubleStream<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaIntStream. Needed for returning it from Java.
pub struct JavaIntStream<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> JavaIntStream<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate JavaIntStream from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "JavaIntStream")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaIntStream object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn sorted(&mut self) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"sorted","()Ljava/util/stream/IntStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn generate(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::function::JavaIntSupplier<'mc>>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/IntStream")?;
let res = jni.call_static_method(cls,"generate",
"(Ljava/util/function/IntSupplier;)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaIntStream::from_raw(&jni,obj
)}
	pub fn map_to_double(&mut self,arg0: impl Into<&'mc crate::function::JavaIntToDoubleFunction<'mc>>) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapToDouble","(Ljava/util/function/IntToDoubleFunction;)Ljava/util/stream/DoubleStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map_multi(&mut self,arg0: impl Into<&'mc crate::stream::JavaIntStreamIntMapMultiConsumer<'mc>>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapMulti","(Ljava/util/stream/IntStream$IntMapMultiConsumer;)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn take_while(&mut self,arg0: impl Into<&'mc crate::function::JavaIntPredicate<'mc>>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"takeWhile","(Ljava/util/function/IntPredicate;)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn drop_while(&mut self,arg0: impl Into<&'mc crate::function::JavaIntPredicate<'mc>>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"dropWhile","(Ljava/util/function/IntPredicate;)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn none_match(&mut self,arg0: impl Into<&'mc crate::function::JavaIntPredicate<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"noneMatch","(Ljava/util/function/IntPredicate;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn sequential(&mut self) 
-> Result<crate::stream::JavaBaseStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"sequential","()Ljava/util/stream/BaseStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaBaseStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn as_double_stream(&mut self) 
-> Result<crate::stream::JavaDoubleStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asDoubleStream","()Ljava/util/stream/DoubleStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaDoubleStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn average(&mut self) 
-> Result<crate::JavaOptionalDouble<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"average","()Ljava/util/OptionalDouble;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptionalDouble::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn summary_statistics(&mut self) 
-> Result<crate::JavaIntSummaryStatistics<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"summaryStatistics","()Ljava/util/IntSummaryStatistics;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaIntSummaryStatistics::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn range_closed(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: i32) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let cls = &jni.find_class("java/util/stream/IntStream")?;
let res = jni.call_static_method(cls,"rangeClosed",
"(II)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::stream::JavaIntStream::from_raw(&jni,obj
)}
	pub fn as_long_stream(&mut self) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asLongStream","()Ljava/util/stream/LongStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn for_each_ordered(&mut self,arg0: impl Into<&'mc crate::function::JavaIntConsumer<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"forEachOrdered","(Ljava/util/function/IntConsumer;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn map_to_obj(&mut self,arg0: impl Into<&'mc crate::function::JavaIntFunction<'mc, U>>) 
-> Result<crate::stream::JavaStream<'mc, U>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapToObj","(Ljava/util/function/IntFunction;)Ljava/util/stream/Stream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn boxed(&mut self) 
-> Result<crate::stream::JavaStream<'mc, javaInteger>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"boxed","()Ljava/util/stream/Stream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn min(&mut self) 
-> Result<crate::JavaOptionalInt<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"min","()Ljava/util/OptionalInt;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptionalInt::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn max(&mut self) 
-> Result<crate::JavaOptionalInt<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"max","()Ljava/util/OptionalInt;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptionalInt::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn iterator(&mut self) 
-> Result<crate::JavaPrimitiveIteratorOfInt<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"iterator","()Ljava/util/PrimitiveIterator$OfInt;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaPrimitiveIteratorOfInt::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map(&mut self,arg0: impl Into<&'mc crate::function::JavaIntUnaryOperator<'mc>>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"map","(Ljava/util/function/IntUnaryOperator;)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn collect(&mut self,arg0: impl Into<&'mc crate::function::JavaSupplier<'mc, R>>,arg1: impl Into<&'mc crate::function::JavaObjIntConsumer<'mc, R>>,arg2: impl Into<&'mc crate::function::JavaBiConsumer<'mc, R, R>>) 
-> Result<R, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"collect","(Ljava/util/function/Supplier;Ljava/util/function/ObjIntConsumer;Ljava/util/function/BiConsumer;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
R::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn of_with_ints(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<i32>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let cls = &jni.find_class("java/util/stream/IntStream")?;
let res = jni.call_static_method(cls,"of",
"(I)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
crate::stream::JavaIntStream::from_raw(&jni,obj
)}
	pub fn count(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"count","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn builder(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::stream::JavaIntStreamBuilder<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/stream/IntStream$Builder")?;
let res = jni.call_static_method(cls,"builder",
"()Ljava/util/stream/IntStream$Builder;",&[])?;
let mut obj = res.l()?;
crate::stream::JavaIntStreamBuilder::from_raw(&jni,obj
)}
	pub fn concat(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::stream::JavaIntStream<'mc>>,arg1: impl Into<&'mc crate::stream::JavaIntStream<'mc>>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/IntStream")?;
let res = jni.call_static_method(cls,"concat",
"(Ljava/util/stream/IntStream;Ljava/util/stream/IntStream;)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::stream::JavaIntStream::from_raw(&jni,obj
)}
	pub fn limit(&mut self,arg0: i64) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"limit","(J)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn spliterator(&mut self) 
-> Result<crate::JavaSpliterator<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"spliterator","()Ljava/util/Spliterator;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaSpliterator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn filter(&mut self,arg0: impl Into<&'mc crate::function::JavaIntPredicate<'mc>>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"filter","(Ljava/util/function/IntPredicate;)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn empty(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("java/util/stream/IntStream")?;
let res = jni.call_static_method(cls,"empty",
"()Ljava/util/stream/IntStream;",&[])?;
let mut obj = res.l()?;
crate::stream::JavaIntStream::from_raw(&jni,obj
)}
	pub fn any_match(&mut self,arg0: impl Into<&'mc crate::function::JavaIntPredicate<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"anyMatch","(Ljava/util/function/IntPredicate;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn flat_map(&mut self,arg0: impl Into<&'mc crate::function::JavaIntFunction<'mc, j>>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"flatMap","(Ljava/util/function/IntFunction;)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn for_each(&mut self,arg0: impl Into<&'mc crate::function::JavaIntConsumer<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"forEach","(Ljava/util/function/IntConsumer;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn find_any(&mut self) 
-> Result<crate::JavaOptionalInt<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"findAny","()Ljava/util/OptionalInt;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptionalInt::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn skip(&mut self,arg0: i64) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"skip","(J)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn peek(&mut self,arg0: impl Into<&'mc crate::function::JavaIntConsumer<'mc>>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"peek","(Ljava/util/function/IntConsumer;)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn sum(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"sum","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn reduce_with_int_binary_operator(&mut self,arg0: std::option::Option<i32>,arg1: std::option::Option<impl Into<&'mc crate::function::JavaIntBinaryOperator<'mc>>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"reduce","(ILjava/util/function/IntBinaryOperator;)I",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn distinct(&mut self) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"distinct","()Ljava/util/stream/IntStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaIntStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn find_first(&mut self) 
-> Result<crate::JavaOptionalInt<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"findFirst","()Ljava/util/OptionalInt;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::JavaOptionalInt::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn all_match(&mut self,arg0: impl Into<&'mc crate::function::JavaIntPredicate<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"allMatch","(Ljava/util/function/IntPredicate;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn iterate_with_int(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: std::option::Option<impl Into<&'mc crate::function::JavaIntPredicate<'mc>>>,arg2: std::option::Option<impl Into<&'mc crate::function::JavaIntUnaryOperator<'mc>>>) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("java/util/stream/IntStream")?;
let res = jni.call_static_method(cls,"iterate",
"(ILjava/util/function/IntPredicate;Ljava/util/function/IntUnaryOperator;)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
let mut obj = res.l()?;
crate::stream::JavaIntStream::from_raw(&jni,obj
)}
	pub fn range(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: i32,arg1: i32) 
-> Result<crate::stream::JavaIntStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let cls = &jni.find_class("java/util/stream/IntStream")?;
let res = jni.call_static_method(cls,"range",
"(II)Ljava/util/stream/IntStream;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
crate::stream::JavaIntStream::from_raw(&jni,obj
)}
	pub fn parallel(&mut self) 
-> Result<crate::stream::JavaBaseStream<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"parallel","()Ljava/util/stream/BaseStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaBaseStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn map_to_long(&mut self,arg0: impl Into<&'mc crate::function::JavaIntToLongFunction<'mc>>) 
-> Result<crate::stream::JavaLongStream<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"mapToLong","(Ljava/util/function/IntToLongFunction;)Ljava/util/stream/LongStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::stream::JavaLongStream::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn unordered(&mut self) 
-> Result<S, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"unordered","()Ljava/util/stream/BaseStream;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
S::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn close(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"close","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn on_close(&mut self,arg0: impl Into<&'mc blackboxmc_java::JavaRunnable<'mc>>) 
-> Result<S, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"onClose","(Ljava/lang/Runnable;)Ljava/util/stream/BaseStream;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
S::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_parallel(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isParallel","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc> JNIRaw<'mc> for JavaIntStream<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
