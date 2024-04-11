#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
pub enum MushroomBlockTexture<'mc> {
	AllPores {inner: MushroomBlockTextureStruct<'mc>},
	CapNorthWest {inner: MushroomBlockTextureStruct<'mc>},
	CapNorth {inner: MushroomBlockTextureStruct<'mc>},
	CapNorthEast {inner: MushroomBlockTextureStruct<'mc>},
	CapWest {inner: MushroomBlockTextureStruct<'mc>},
	CapTop {inner: MushroomBlockTextureStruct<'mc>},
	CapEast {inner: MushroomBlockTextureStruct<'mc>},
	CapSouthWest {inner: MushroomBlockTextureStruct<'mc>},
	CapSouth {inner: MushroomBlockTextureStruct<'mc>},
	CapSouthEast {inner: MushroomBlockTextureStruct<'mc>},
	StemSides {inner: MushroomBlockTextureStruct<'mc>},
	AllCap {inner: MushroomBlockTextureStruct<'mc>},
	AllStem {inner: MushroomBlockTextureStruct<'mc>},
}
impl<'mc> std::fmt::Display for MushroomBlockTexture<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           MushroomBlockTexture::AllPores { .. } => f.write_str("ALL_PORES"),
           MushroomBlockTexture::CapNorthWest { .. } => f.write_str("CAP_NORTH_WEST"),
           MushroomBlockTexture::CapNorth { .. } => f.write_str("CAP_NORTH"),
           MushroomBlockTexture::CapNorthEast { .. } => f.write_str("CAP_NORTH_EAST"),
           MushroomBlockTexture::CapWest { .. } => f.write_str("CAP_WEST"),
           MushroomBlockTexture::CapTop { .. } => f.write_str("CAP_TOP"),
           MushroomBlockTexture::CapEast { .. } => f.write_str("CAP_EAST"),
           MushroomBlockTexture::CapSouthWest { .. } => f.write_str("CAP_SOUTH_WEST"),
           MushroomBlockTexture::CapSouth { .. } => f.write_str("CAP_SOUTH"),
           MushroomBlockTexture::CapSouthEast { .. } => f.write_str("CAP_SOUTH_EAST"),
           MushroomBlockTexture::StemSides { .. } => f.write_str("STEM_SIDES"),
           MushroomBlockTexture::AllCap { .. } => f.write_str("ALL_CAP"),
           MushroomBlockTexture::AllStem { .. } => f.write_str("ALL_STEM"),
       }
   }
}

        impl<'mc> MushroomBlockTexture<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<MushroomBlockTexture<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/material/types/MushroomBlockTexture");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/material/types/MushroomBlockTexture;",
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
                    
"ALL_PORES" => Ok(MushroomBlockTexture::AllPores { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),
"CAP_NORTH_WEST" => Ok(MushroomBlockTexture::CapNorthWest { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),
"CAP_NORTH" => Ok(MushroomBlockTexture::CapNorth { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),
"CAP_NORTH_EAST" => Ok(MushroomBlockTexture::CapNorthEast { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),
"CAP_WEST" => Ok(MushroomBlockTexture::CapWest { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),
"CAP_TOP" => Ok(MushroomBlockTexture::CapTop { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),
"CAP_EAST" => Ok(MushroomBlockTexture::CapEast { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),
"CAP_SOUTH_WEST" => Ok(MushroomBlockTexture::CapSouthWest { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),
"CAP_SOUTH" => Ok(MushroomBlockTexture::CapSouth { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),
"CAP_SOUTH_EAST" => Ok(MushroomBlockTexture::CapSouthEast { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),
"STEM_SIDES" => Ok(MushroomBlockTexture::StemSides { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),
"ALL_CAP" => Ok(MushroomBlockTexture::AllCap { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),
"ALL_STEM" => Ok(MushroomBlockTexture::AllStem { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct MushroomBlockTextureStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for MushroomBlockTexture<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::AllPores { inner } => inner.0.clone(),
Self::CapNorthWest { inner } => inner.0.clone(),
Self::CapNorth { inner } => inner.0.clone(),
Self::CapNorthEast { inner } => inner.0.clone(),
Self::CapWest { inner } => inner.0.clone(),
Self::CapTop { inner } => inner.0.clone(),
Self::CapEast { inner } => inner.0.clone(),
Self::CapSouthWest { inner } => inner.0.clone(),
Self::CapSouth { inner } => inner.0.clone(),
Self::CapSouthEast { inner } => inner.0.clone(),
Self::StemSides { inner } => inner.0.clone(),
Self::AllCap { inner } => inner.0.clone(),
Self::AllStem { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::AllPores { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CapNorthWest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CapNorth { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CapNorthEast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CapWest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CapTop { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CapEast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CapSouthWest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CapSouth { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CapSouthEast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::StemSides { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::AllCap { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::AllStem { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for MushroomBlockTexture<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MushroomBlockTexture from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/types/MushroomBlockTexture")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MushroomBlockTexture object, got {}",
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
                    "ALL_PORES" => Ok(MushroomBlockTexture::AllPores { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),"CAP_NORTH_WEST" => Ok(MushroomBlockTexture::CapNorthWest { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),"CAP_NORTH" => Ok(MushroomBlockTexture::CapNorth { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),"CAP_NORTH_EAST" => Ok(MushroomBlockTexture::CapNorthEast { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),"CAP_WEST" => Ok(MushroomBlockTexture::CapWest { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),"CAP_TOP" => Ok(MushroomBlockTexture::CapTop { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),"CAP_EAST" => Ok(MushroomBlockTexture::CapEast { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),"CAP_SOUTH_WEST" => Ok(MushroomBlockTexture::CapSouthWest { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),"CAP_SOUTH" => Ok(MushroomBlockTexture::CapSouth { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),"CAP_SOUTH_EAST" => Ok(MushroomBlockTexture::CapSouthEast { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),"STEM_SIDES" => Ok(MushroomBlockTexture::StemSides { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),"ALL_CAP" => Ok(MushroomBlockTexture::AllCap { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),"ALL_STEM" => Ok(MushroomBlockTexture::AllStem { inner: MushroomBlockTextureStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for MushroomBlockTextureStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for MushroomBlockTextureStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MushroomBlockTextureStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/types/MushroomBlockTexture")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MushroomBlockTextureStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MushroomBlockTextureStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::material::types::MushroomBlockTexture<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::material::types::MushroomBlockTexture;");
let cls = jni.find_class("org/bukkit/material/types/MushroomBlockTexture"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::material::types::MushroomBlockTexture::from_raw(&jni,obj
)}
#[deprecated]

	pub fn data(&self) 
-> Result<i8, Box<dyn std::error::Error>>

{let sig = String::from("()Li8;");
let res = self.jni_ref().call_method(&self.jni_object(),"getData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.b()?
)}
	pub fn cap_face(&self) 
-> Result<Option<crate::block::BlockFace<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCapFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
#[deprecated]

	pub fn get_by_data(jni: &blackboxmc_general::SharedJNIEnv<'mc>,data: i8) 
-> Result<Option<crate::material::types::MushroomBlockTexture<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(B)Lcrate::material::types::MushroomBlockTexture;");
let val_1 = jni::objects::JValueGen::Byte(data);
let cls = jni.find_class("org/bukkit/material/types/MushroomBlockTexture"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getByData",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let obj = res.l()?;
Ok(
Some(
crate::material::types::MushroomBlockTexture::from_raw(&jni,obj
)?
)
)}
	pub fn get_cap_by_face(jni: &blackboxmc_general::SharedJNIEnv<'mc>,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<Option<crate::material::types::MushroomBlockTexture<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)Lcrate::material::types::MushroomBlockTexture;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/material/types/MushroomBlockTexture"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getCapByFace",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let obj = res.l()?;
Ok(
Some(
crate::material::types::MushroomBlockTexture::from_raw(&jni,obj
)?
)
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
