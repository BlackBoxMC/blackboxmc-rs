#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represent a StructureType of a <a title="class in org.bukkit.generator.structure" href="Structure.html"><code>Structure</code></a>. Listed structure types are present in the default server. Depending on the server there might be additional structure types present (for example structure types added by data packs), which can be received via <a href="../../Registry.html#STRUCTURE_TYPE"><code>Registry.STRUCTURE_TYPE</code></a>.
#[repr(C)]
pub struct StructureType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for StructureType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for StructureType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate StructureType from null object.").into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/generator/structure/StructureType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StructureType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> StructureType<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::generator::structure::StructureType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/generator/structure/StructureType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::generator::structure::StructureType::from_raw(&jni, res)
    }
    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = StructureType::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Keyed = temp_clone.into();
        real.key()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for StructureType<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting StructureType into crate::Keyed")
    }
}
/// Represent a Structure from the world. Listed structures are present in the default server. Depending on the server there might be additional structures present (for example structures added by data packs), which can be received via <a href="../../Registry.html#STRUCTURE"><code>Registry.STRUCTURE</code></a>.
#[repr(C)]
pub struct Structure<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Structure<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Structure<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Structure from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/structure/Structure")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Structure object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Structure<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::generator::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/generator/structure/Structure");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::generator::structure::Structure::from_raw(&jni, res)
    }

    pub fn structure_type(
        &self,
    ) -> Result<crate::generator::structure::StructureType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/generator/structure/StructureType;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStructureType",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::generator::structure::StructureType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Structure::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Keyed = temp_clone.into();
        real.key()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for Structure<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Structure into crate::Keyed")
    }
}
