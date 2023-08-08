#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub struct JavaPattern<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaPattern<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaPattern<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaPattern from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaPattern")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaPattern object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn as_predicate(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "asPredicate",
            "()Ljava/util/function/Predicate;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn as_match_predicate(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "asMatchPredicate",
            "()Ljava/util/function/Predicate;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn split_as_stream(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "splitAsStream",
            "(Ljava/lang/CharSequence;)Ljava/util/stream/Stream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn pattern(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "pattern", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn quote(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("Javajava/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "quote",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn flags(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "flags", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn matches(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let val_2 = arg1;
        let cls = &jni.find_class("Javaboolean")?;
        let res = jni.call_static_method(
            cls,
            "matches",
            "(Ljava/lang/String;Ljava/lang/CharSequence;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn compile_with_string(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let cls = &jni.find_class("Javajava/util/regex/Pattern")?;
        let res = jni.call_static_method(
            cls,
            "compile",
            "(Ljava/lang/String;I)Ljava/util/regex/Pattern;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn matcher(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
