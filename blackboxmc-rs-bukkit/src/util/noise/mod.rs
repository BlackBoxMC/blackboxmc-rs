#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct PerlinOctaveGenerator<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PerlinOctaveGenerator<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PerlinOctaveGenerator<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PerlinOctaveGenerator from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/noise/PerlinOctaveGenerator")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PerlinOctaveGenerator object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PerlinOctaveGenerator<'mc> {
	pub fn new_with_seed(jni: &blackboxmc_general::SharedJNIEnv<'mc>,seed: i64,octaves: i32) 
-> Result<crate::util::noise::PerlinOctaveGenerator<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "J";
let val_1 = jni::objects::JValueGen::Long(seed);
args.push(val_1);
sig += "I";
let val_2 = jni::objects::JValueGen::Int(octaves);
args.push(val_2);
sig += ")V";
let cls = jni.find_class("org/bukkit/util/noise/PerlinOctaveGenerator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::noise::PerlinOctaveGenerator::from_raw(&jni,res
)}
	pub fn new_with_rand(jni: &blackboxmc_general::SharedJNIEnv<'mc>,rand: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,octaves: i32) 
-> Result<crate::util::noise::PerlinOctaveGenerator<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/util/Random;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(rand.into().jni_object().clone())});
args.push(val_1);
sig += "I";
let val_2 = jni::objects::JValueGen::Int(octaves);
args.push(val_2);
sig += ")V";
let cls = jni.find_class("org/bukkit/util/noise/PerlinOctaveGenerator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::noise::PerlinOctaveGenerator::from_raw(&jni,res
)}
// SUPER CLASS: OctaveGenerator

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::util::noise::OctaveGenerator<'mc>> for PerlinOctaveGenerator<'mc>{

fn into(self) -> crate::util::noise::OctaveGenerator<'mc> {

crate::util::noise::OctaveGenerator::from_raw(&self.jni_ref(), self.1).expect("Error converting PerlinOctaveGenerator into crate::util::noise::OctaveGenerator")

   }
}
#[repr(C)]
pub struct SimplexOctaveGenerator<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SimplexOctaveGenerator<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SimplexOctaveGenerator<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SimplexOctaveGenerator from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/noise/SimplexOctaveGenerator")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SimplexOctaveGenerator object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SimplexOctaveGenerator<'mc> {
	pub fn new_with_seed(jni: &blackboxmc_general::SharedJNIEnv<'mc>,seed: i64,octaves: i32) 
-> Result<crate::util::noise::SimplexOctaveGenerator<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "J";
let val_1 = jni::objects::JValueGen::Long(seed);
args.push(val_1);
sig += "I";
let val_2 = jni::objects::JValueGen::Int(octaves);
args.push(val_2);
sig += ")V";
let cls = jni.find_class("org/bukkit/util/noise/SimplexOctaveGenerator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::noise::SimplexOctaveGenerator::from_raw(&jni,res
)}
	pub fn new_with_rand(jni: &blackboxmc_general::SharedJNIEnv<'mc>,rand: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,octaves: i32) 
-> Result<crate::util::noise::SimplexOctaveGenerator<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/util/Random;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(rand.into().jni_object().clone())});
args.push(val_1);
sig += "I";
let val_2 = jni::objects::JValueGen::Int(octaves);
args.push(val_2);
sig += ")V";
let cls = jni.find_class("org/bukkit/util/noise/SimplexOctaveGenerator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::noise::SimplexOctaveGenerator::from_raw(&jni,res
)}
	pub fn set_scale(&self,scale: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(scale);
let res = self.jni_ref().call_method(&self.jni_object(),"setScale",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn wscale(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getWScale",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn set_wscale(&self,scale: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(scale);
let res = self.jni_ref().call_method(&self.jni_object(),"setWScale",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn noise_with_x(&self,x: f64,y: f64,z: f64,w: f64,frequency: f64,amplitude: f64,normalized: std::option::Option<bool>) 
-> Result<f64, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "D";
let val_1 = jni::objects::JValueGen::Double(x);
args.push(val_1);
sig += "D";
let val_2 = jni::objects::JValueGen::Double(y);
args.push(val_2);
sig += "D";
let val_3 = jni::objects::JValueGen::Double(z);
args.push(val_3);
sig += "D";
let val_4 = jni::objects::JValueGen::Double(w);
args.push(val_4);
sig += "D";
let val_5 = jni::objects::JValueGen::Double(frequency);
args.push(val_5);
sig += "D";
let val_6 = jni::objects::JValueGen::Double(amplitude);
args.push(val_6);
if let Some(a) = normalized {
sig += "Z";
let val_7 = jni::objects::JValueGen::Bool(a.into());
args.push(val_7);
}
sig += ")D";
let res = self.jni_ref().call_method(&self.jni_object(),"noise",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
// SUPER CLASS: OctaveGenerator

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::util::noise::OctaveGenerator<'mc>> for SimplexOctaveGenerator<'mc>{

fn into(self) -> crate::util::noise::OctaveGenerator<'mc> {

crate::util::noise::OctaveGenerator::from_raw(&self.jni_ref(), self.1).expect("Error converting SimplexOctaveGenerator into crate::util::noise::OctaveGenerator")

   }
}
#[repr(C)]
pub struct NoiseGenerator<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for NoiseGenerator<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for NoiseGenerator<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate NoiseGenerator from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/noise/NoiseGenerator")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a NoiseGenerator object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> NoiseGenerator<'mc> {
pub fn from_extendable(
    env: &blackboxmc_general::SharedJNIEnv<'mc>,
    plugin: &'mc crate::plugin::Plugin,
    address: i32,
    lib_name: String,
    name: String,
) -> Result<Self, Box<dyn std::error::Error>> {
    let obj = unsafe { plugin.new_extendable(address, "NoiseGenerator", name, lib_name) }?;
    Self::from_raw(env, obj)
}
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::util::noise::NoiseGenerator<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/util/noise/NoiseGenerator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::noise::NoiseGenerator::from_raw(&jni,res
)}
	pub fn floor(jni: &blackboxmc_general::SharedJNIEnv<'mc>,x: f64) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(D)Li32;");
let val_1 = jni::objects::JValueGen::Double(x);
let cls = jni.find_class("org/bukkit/util/noise/NoiseGenerator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"floor",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.i()?
)}
	pub fn noise_with_x(&self,x: f64,y: std::option::Option<f64>,z: std::option::Option<f64>,octaves: std::option::Option<i32>,frequency: std::option::Option<f64>,amplitude: std::option::Option<f64>,normalized: std::option::Option<bool>) 
-> Result<f64, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "D";
let val_1 = jni::objects::JValueGen::Double(x);
args.push(val_1);
if let Some(a) = y {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
if let Some(a) = z {
sig += "D";
let val_3 = jni::objects::JValueGen::Double(a);
args.push(val_3);
}
if let Some(a) = octaves {
sig += "I";
let val_4 = jni::objects::JValueGen::Int(a);
args.push(val_4);
}
if let Some(a) = frequency {
sig += "D";
let val_5 = jni::objects::JValueGen::Double(a);
args.push(val_5);
}
if let Some(a) = amplitude {
sig += "D";
let val_6 = jni::objects::JValueGen::Double(a);
args.push(val_6);
}
if let Some(a) = normalized {
sig += "Z";
let val_7 = jni::objects::JValueGen::Bool(a.into());
args.push(val_7);
}
sig += ")D";
let res = self.jni_ref().call_method(&self.jni_object(),"noise",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PerlinNoiseGenerator<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PerlinNoiseGenerator<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PerlinNoiseGenerator<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PerlinNoiseGenerator from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/noise/PerlinNoiseGenerator")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PerlinNoiseGenerator object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PerlinNoiseGenerator<'mc> {
	pub fn new_with_seed(jni: &blackboxmc_general::SharedJNIEnv<'mc>,seed: i64) 
-> Result<crate::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "J";
let val_1 = jni::objects::JValueGen::Long(seed);
args.push(val_1);
sig += ")V";
let cls = jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::noise::PerlinNoiseGenerator::from_raw(&jni,res
)}
	pub fn new_with_rand(jni: &blackboxmc_general::SharedJNIEnv<'mc>,rand: impl Into<blackboxmc_java::util::JavaRandom<'mc>>) 
-> Result<crate::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/util/Random;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(rand.into().jni_object().clone())});
args.push(val_1);
sig += ")V";
let cls = jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::noise::PerlinNoiseGenerator::from_raw(&jni,res
)}
	pub fn get_noise_with_x(jni: &blackboxmc_general::SharedJNIEnv<'mc>,x: f64,y: std::option::Option<f64>,z: std::option::Option<f64>,octaves: std::option::Option<i32>,frequency: std::option::Option<f64>,amplitude: std::option::Option<f64>) 
-> Result<f64, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "D";
let val_1 = jni::objects::JValueGen::Double(x);
args.push(val_1);
if let Some(a) = y {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
if let Some(a) = z {
sig += "D";
let val_3 = jni::objects::JValueGen::Double(a);
args.push(val_3);
}
if let Some(a) = octaves {
sig += "I";
let val_4 = jni::objects::JValueGen::Int(a);
args.push(val_4);
}
if let Some(a) = frequency {
sig += "D";
let val_5 = jni::objects::JValueGen::Double(a);
args.push(val_5);
}
if let Some(a) = amplitude {
sig += "D";
let val_6 = jni::objects::JValueGen::Double(a);
args.push(val_6);
}
sig += ")D";
let cls = jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getNoise",
sig.as_str(),args);
let res = 
jni.translate_error(res)?;
Ok(
res.d()?
)}
	pub fn instance(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::noise::PerlinNoiseGenerator;");
let cls = jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getInstance",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::noise::PerlinNoiseGenerator::from_raw(&jni,obj
)}
	pub fn noise(&self,x: f64,y: f64,z: f64) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("(DDD)Lf64;");
let val_1 = jni::objects::JValueGen::Double(x);
let val_2 = jni::objects::JValueGen::Double(y);
let val_3 = jni::objects::JValueGen::Double(z);
let res = self.jni_ref().call_method(&self.jni_object(),"noise",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
// SUPER CLASS: NoiseGenerator

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::util::noise::NoiseGenerator<'mc>> for PerlinNoiseGenerator<'mc>{

fn into(self) -> crate::util::noise::NoiseGenerator<'mc> {

crate::util::noise::NoiseGenerator::from_raw(&self.jni_ref(), self.1).expect("Error converting PerlinNoiseGenerator into crate::util::noise::NoiseGenerator")

   }
}
#[repr(C)]
pub struct SimplexNoiseGenerator<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SimplexNoiseGenerator<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SimplexNoiseGenerator<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SimplexNoiseGenerator from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/noise/SimplexNoiseGenerator")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SimplexNoiseGenerator object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SimplexNoiseGenerator<'mc> {
	pub fn new_with_seed(jni: &blackboxmc_general::SharedJNIEnv<'mc>,seed: i64) 
-> Result<crate::util::noise::SimplexNoiseGenerator<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "J";
let val_1 = jni::objects::JValueGen::Long(seed);
args.push(val_1);
sig += ")V";
let cls = jni.find_class("org/bukkit/util/noise/SimplexNoiseGenerator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::noise::SimplexNoiseGenerator::from_raw(&jni,res
)}
	pub fn new_with_rand(jni: &blackboxmc_general::SharedJNIEnv<'mc>,rand: impl Into<blackboxmc_java::util::JavaRandom<'mc>>) 
-> Result<crate::util::noise::SimplexNoiseGenerator<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/util/Random;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(rand.into().jni_object().clone())});
args.push(val_1);
sig += ")V";
let cls = jni.find_class("org/bukkit/util/noise/SimplexNoiseGenerator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::noise::SimplexNoiseGenerator::from_raw(&jni,res
)}
	pub fn get_noise_with_x(jni: &blackboxmc_general::SharedJNIEnv<'mc>,x: f64,y: std::option::Option<f64>,z: std::option::Option<f64>,w: std::option::Option<f64>) 
-> Result<f64, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "D";
let val_1 = jni::objects::JValueGen::Double(x);
args.push(val_1);
if let Some(a) = y {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
if let Some(a) = z {
sig += "D";
let val_3 = jni::objects::JValueGen::Double(a);
args.push(val_3);
}
if let Some(a) = w {
sig += "D";
let val_4 = jni::objects::JValueGen::Double(a);
args.push(val_4);
}
sig += ")D";
let cls = jni.find_class("org/bukkit/util/noise/SimplexNoiseGenerator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getNoise",
sig.as_str(),args);
let res = 
jni.translate_error(res)?;
Ok(
res.d()?
)}
	pub fn noise_with_xin(&self,xin: f64,yin: f64,zin: std::option::Option<f64>) 
-> Result<f64, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "D";
let val_1 = jni::objects::JValueGen::Double(xin);
args.push(val_1);
sig += "D";
let val_2 = jni::objects::JValueGen::Double(yin);
args.push(val_2);
if let Some(a) = zin {
sig += "D";
let val_3 = jni::objects::JValueGen::Double(a);
args.push(val_3);
}
sig += ")D";
let res = self.jni_ref().call_method(&self.jni_object(),"noise",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn instance(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::util::noise::SimplexNoiseGenerator<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::noise::SimplexNoiseGenerator;");
let cls = jni.find_class("org/bukkit/util/noise/SimplexNoiseGenerator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getInstance",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::noise::SimplexNoiseGenerator::from_raw(&jni,obj
)}
// SUPER CLASS: PerlinNoiseGenerator

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::util::noise::PerlinNoiseGenerator<'mc>> for SimplexNoiseGenerator<'mc>{

fn into(self) -> crate::util::noise::PerlinNoiseGenerator<'mc> {

crate::util::noise::PerlinNoiseGenerator::from_raw(&self.jni_ref(), self.1).expect("Error converting SimplexNoiseGenerator into crate::util::noise::PerlinNoiseGenerator")

   }
}
#[repr(C)]
pub struct OctaveGenerator<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for OctaveGenerator<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for OctaveGenerator<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate OctaveGenerator from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/noise/OctaveGenerator")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a OctaveGenerator object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> OctaveGenerator<'mc> {
	pub fn set_scale(&self,scale: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(scale);
let res = self.jni_ref().call_method(&self.jni_object(),"setScale",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn xscale(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getXScale",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn set_xscale(&self,scale: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(scale);
let res = self.jni_ref().call_method(&self.jni_object(),"setXScale",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn yscale(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getYScale",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn set_yscale(&self,scale: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(scale);
let res = self.jni_ref().call_method(&self.jni_object(),"setYScale",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn zscale(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getZScale",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn set_zscale(&self,scale: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(scale);
let res = self.jni_ref().call_method(&self.jni_object(),"setZScale",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn octaves(&self) 
-> Result<crate::util::noise::NoiseGenerator<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::noise::NoiseGenerator;");
let res = self.jni_ref().call_method(&self.jni_object(),"getOctaves",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::noise::NoiseGenerator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn noise_with_x(&self,x: f64,y: f64,z: f64,frequency: std::option::Option<f64>,amplitude: std::option::Option<f64>,normalized: std::option::Option<bool>) 
-> Result<f64, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "D";
let val_1 = jni::objects::JValueGen::Double(x);
args.push(val_1);
sig += "D";
let val_2 = jni::objects::JValueGen::Double(y);
args.push(val_2);
sig += "D";
let val_3 = jni::objects::JValueGen::Double(z);
args.push(val_3);
if let Some(a) = frequency {
sig += "D";
let val_4 = jni::objects::JValueGen::Double(a);
args.push(val_4);
}
if let Some(a) = amplitude {
sig += "D";
let val_5 = jni::objects::JValueGen::Double(a);
args.push(val_5);
}
if let Some(a) = normalized {
sig += "Z";
let val_6 = jni::objects::JValueGen::Bool(a.into());
args.push(val_6);
}
sig += ")D";
let res = self.jni_ref().call_method(&self.jni_object(),"noise",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
