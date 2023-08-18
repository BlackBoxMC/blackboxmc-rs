#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIInstantiatableEnum;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;

pub struct JavaRandomGeneratorFactory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorFactory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaRandomGeneratorFactory<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaRandomGeneratorFactory from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/random/RandomGeneratorFactory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaRandomGeneratorFactory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaRandomGeneratorFactory<'mc> {
    pub fn equidistribution(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equidistribution",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_deprecated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDeprecated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_stochastic(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStochastic", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_hardware(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isHardware", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn state_bits(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "stateBits", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_statistical(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStatistical", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_arbitrarily_jumpable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isArbitrarilyJumpable",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_jumpable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isJumpable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_leapable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isLeapable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_splittable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSplittable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_streamable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStreamable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "name", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn group(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "group", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn default(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::random::JavaRandomGeneratorFactory<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Ljava/util/random/RandomGeneratorFactory;");
        let cls = jni.find_class("java/util/random/RandomGeneratorFactory");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getDefault", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::random::JavaRandomGeneratorFactory::from_raw(&jni, obj)
    }

    pub fn of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::util::random::JavaRandomGeneratorFactory<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/random/RandomGeneratorFactory;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/util/random/RandomGeneratorFactory");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "of",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::random::JavaRandomGeneratorFactory::from_raw(&jni, obj)
    }

    pub fn create_with_bytes(
        &self,
        arg0: std::option::Option<Vec<i8>>,
    ) -> Result<crate::util::random::JavaRandomGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/random/RandomGenerator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "create", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::random::JavaRandomGenerator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn period(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/math/BigInteger;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "period", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for JavaRandomGeneratorFactory<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaRandomGeneratorFactory.toString: {}", err),
        }
    }
}

pub struct JavaRandomGeneratorFactoryClass;
impl blackboxmc_general::JNIProvidesClassName for JavaRandomGeneratorFactoryClass {
    fn class_name(&self) -> &str {
        "java/util/random/RandomGeneratorFactory"
    }
}

///
/// This is a representation of an abstract class.
pub struct JavaRandomGeneratorSplittableGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorSplittableGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaRandomGeneratorSplittableGenerator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaRandomGeneratorSplittableGenerator from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "java/util/random/RandomGenerator$SplittableGenerator")?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a JavaRandomGeneratorSplittableGenerator object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaRandomGeneratorSplittableGenerator<'mc> {
    pub fn split_with_random_generatorsplittable_generator(
        &self,
        arg0: std::option::Option<
            impl Into<crate::util::random::JavaRandomGeneratorSplittableGenerator<'mc>>,
        >,
    ) -> Result<
        crate::util::random::JavaRandomGeneratorSplittableGenerator<'mc>,
        Box<dyn std::error::Error>,
    > {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/util/random/RandomGenerator$SplittableGenerator;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Ljava/util/random/RandomGenerator$SplittableGenerator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "split", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::random::JavaRandomGeneratorSplittableGenerator::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<
        crate::util::random::JavaRandomGeneratorSplittableGenerator<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from(
            "(Ljava/lang/String;)Ljava/util/random/RandomGenerator$SplittableGenerator;",
        );
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/util/random/RandomGenerator$SplittableGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "of",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::random::JavaRandomGeneratorSplittableGenerator::from_raw(&jni, obj)
    }

    pub fn next_boolean(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn next_long_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn next_float_with_float(
        &self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "F";
            let val_2 = jni::objects::JValueGen::Float(a.into());
            args.push(val_2);
        }
        sig += ")F";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextFloat", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn next_bytes(&self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn next_gaussian_with_double(
        &self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextGaussian", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn is_deprecated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDeprecated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn next_exponential(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextExponential", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn next_double_with_double(
        &self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn next_int_with_int(
        &self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}
impl<'mc> Into<crate::util::random::JavaRandomGeneratorStreamableGenerator<'mc>>
    for JavaRandomGeneratorSplittableGenerator<'mc>
{
    fn into(self) -> crate::util::random::JavaRandomGeneratorStreamableGenerator<'mc> {
        crate::util::random::JavaRandomGeneratorStreamableGenerator::from_raw(&self.jni_ref(), self.1).expect("Error converting JavaRandomGeneratorSplittableGenerator into crate::util::random::JavaRandomGeneratorStreamableGenerator")
    }
}

pub struct JavaRandomGeneratorSplittableGeneratorClass;
impl blackboxmc_general::JNIProvidesClassName for JavaRandomGeneratorSplittableGeneratorClass {
    fn class_name(&self) -> &str {
        "java/util/random/RandomGenerator$SplittableGenerator"
    }
}

///
/// This is a representation of an abstract class.
pub struct JavaRandomGeneratorStreamableGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorStreamableGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaRandomGeneratorStreamableGenerator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaRandomGeneratorStreamableGenerator from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "java/util/random/RandomGenerator$StreamableGenerator")?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a JavaRandomGeneratorStreamableGenerator object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaRandomGeneratorStreamableGenerator<'mc> {
    pub fn of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<
        crate::util::random::JavaRandomGeneratorStreamableGenerator<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from(
            "(Ljava/lang/String;)Ljava/util/random/RandomGenerator$StreamableGenerator;",
        );
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/util/random/RandomGenerator$StreamableGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "of",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::random::JavaRandomGeneratorStreamableGenerator::from_raw(&jni, obj)
    }

    pub fn next_boolean(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn next_long_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn next_float_with_float(
        &self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "F";
            let val_2 = jni::objects::JValueGen::Float(a.into());
            args.push(val_2);
        }
        sig += ")F";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextFloat", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn next_bytes(&self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn next_gaussian_with_double(
        &self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextGaussian", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn is_deprecated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDeprecated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn next_exponential(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextExponential", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn next_double_with_double(
        &self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn next_int_with_int(
        &self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

pub struct JavaRandomGeneratorStreamableGeneratorClass;
impl blackboxmc_general::JNIProvidesClassName for JavaRandomGeneratorStreamableGeneratorClass {
    fn class_name(&self) -> &str {
        "java/util/random/RandomGenerator$StreamableGenerator"
    }
}

///
/// This is a representation of an abstract class.
pub struct JavaRandomGeneratorLeapableGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorLeapableGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaRandomGeneratorLeapableGenerator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaRandomGeneratorLeapableGenerator from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "java/util/random/RandomGenerator$LeapableGenerator")?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a JavaRandomGeneratorLeapableGenerator object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaRandomGeneratorLeapableGenerator<'mc> {
    pub fn leap_distance(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leapDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn leap(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leap", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn copy_and_leap(
        &self,
    ) -> Result<
        crate::util::random::JavaRandomGeneratorJumpableGenerator<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Ljava/util/random/RandomGenerator$JumpableGenerator;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyAndLeap", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::random::JavaRandomGeneratorJumpableGenerator::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<
        crate::util::random::JavaRandomGeneratorLeapableGenerator<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from(
            "(Ljava/lang/String;)Ljava/util/random/RandomGenerator$LeapableGenerator;",
        );
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/util/random/RandomGenerator$LeapableGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "of",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::random::JavaRandomGeneratorLeapableGenerator::from_raw(&jni, obj)
    }

    pub fn copy(
        &self,
    ) -> Result<
        crate::util::random::JavaRandomGeneratorJumpableGenerator<'mc>,
        Box<dyn std::error::Error>,
    > {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/random/RandomGenerator$JumpableGenerator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::random::JavaRandomGeneratorJumpableGenerator::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn jump_distance(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "jumpDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn copy_and_jump(
        &self,
    ) -> Result<crate::util::random::JavaRandomGenerator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/random/RandomGenerator;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyAndJump", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::random::JavaRandomGenerator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn jump(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "jump", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn next_boolean(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn next_long_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn next_float_with_float(
        &self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "F";
            let val_2 = jni::objects::JValueGen::Float(a.into());
            args.push(val_2);
        }
        sig += ")F";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextFloat", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn next_bytes(&self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn next_gaussian_with_double(
        &self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextGaussian", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn is_deprecated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDeprecated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn next_exponential(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextExponential", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn next_double_with_double(
        &self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn next_int_with_int(
        &self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}
impl<'mc> Into<crate::util::random::JavaRandomGeneratorJumpableGenerator<'mc>>
    for JavaRandomGeneratorLeapableGenerator<'mc>
{
    fn into(self) -> crate::util::random::JavaRandomGeneratorJumpableGenerator<'mc> {
        crate::util::random::JavaRandomGeneratorJumpableGenerator::from_raw(&self.jni_ref(), self.1).expect("Error converting JavaRandomGeneratorLeapableGenerator into crate::util::random::JavaRandomGeneratorJumpableGenerator")
    }
}

pub struct JavaRandomGeneratorLeapableGeneratorClass;
impl blackboxmc_general::JNIProvidesClassName for JavaRandomGeneratorLeapableGeneratorClass {
    fn class_name(&self) -> &str {
        "java/util/random/RandomGenerator$LeapableGenerator"
    }
}

///
/// This is a representation of an abstract class.
pub struct JavaRandomGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaRandomGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaRandomGenerator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaRandomGenerator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/random/RandomGenerator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaRandomGenerator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaRandomGenerator<'mc> {
    pub fn next_boolean(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn next_long_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn next_float_with_float(
        &self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "F";
            let val_2 = jni::objects::JValueGen::Float(a.into());
            args.push(val_2);
        }
        sig += ")F";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextFloat", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn next_bytes(&self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn next_gaussian_with_double(
        &self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextGaussian", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn is_deprecated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDeprecated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn next_exponential(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextExponential", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn default(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::random::JavaRandomGenerator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/random/RandomGenerator;");
        let cls = jni.find_class("java/util/random/RandomGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getDefault", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::random::JavaRandomGenerator::from_raw(&jni, obj)
    }

    pub fn of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::util::random::JavaRandomGenerator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/random/RandomGenerator;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/util/random/RandomGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "of",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::random::JavaRandomGenerator::from_raw(&jni, obj)
    }

    pub fn next_double_with_double(
        &self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn next_int_with_int(
        &self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

pub struct JavaRandomGeneratorClass;
impl blackboxmc_general::JNIProvidesClassName for JavaRandomGeneratorClass {
    fn class_name(&self) -> &str {
        "java/util/random/RandomGenerator"
    }
}

///
/// This is a representation of an abstract class.
pub struct JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                    "Tried to instantiate JavaRandomGeneratorArbitrarilyJumpableGenerator from null object.")
                .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "java/util/random/RandomGenerator$ArbitrarilyJumpableGenerator",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a JavaRandomGeneratorArbitrarilyJumpableGenerator object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc> {
    pub fn copy_and_jump_with_double(
        &self,
        arg0: std::option::Option<f64>,
    ) -> Result<
        crate::util::random::JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc>,
        Box<dyn std::error::Error>,
    > {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/random/RandomGenerator$ArbitrarilyJumpableGenerator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copyAndJump", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::random::JavaRandomGeneratorArbitrarilyJumpableGenerator::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn jump_power_of_two(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "jumpPowerOfTwo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn leap(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leap", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn jump_with_double(
        &self,
        arg0: std::option::Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "jump", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<
        crate::util::random::JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from(
            "(Ljava/lang/String;)Ljava/util/random/RandomGenerator$ArbitrarilyJumpableGenerator;",
        );
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/util/random/RandomGenerator$ArbitrarilyJumpableGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "of",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::random::JavaRandomGeneratorArbitrarilyJumpableGenerator::from_raw(&jni, obj)
    }

    pub fn copy(
        &self,
    ) -> Result<
        crate::util::random::JavaRandomGeneratorLeapableGenerator<'mc>,
        Box<dyn std::error::Error>,
    > {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/random/RandomGenerator$LeapableGenerator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::random::JavaRandomGeneratorLeapableGenerator::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn leap_distance(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leapDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn copy_and_leap(
        &self,
    ) -> Result<
        crate::util::random::JavaRandomGeneratorJumpableGenerator<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Ljava/util/random/RandomGenerator$JumpableGenerator;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyAndLeap", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::random::JavaRandomGeneratorJumpableGenerator::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn jump_distance(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "jumpDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn next_boolean(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn next_long_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn next_float_with_float(
        &self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "F";
            let val_2 = jni::objects::JValueGen::Float(a.into());
            args.push(val_2);
        }
        sig += ")F";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextFloat", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn next_bytes(&self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn next_gaussian_with_double(
        &self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextGaussian", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn is_deprecated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDeprecated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn next_exponential(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextExponential", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn next_double_with_double(
        &self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn next_int_with_int(
        &self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}
impl<'mc> Into<crate::util::random::JavaRandomGeneratorLeapableGenerator<'mc>>
    for JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc>
{
    fn into(self) -> crate::util::random::JavaRandomGeneratorLeapableGenerator<'mc> {
        crate::util::random::JavaRandomGeneratorLeapableGenerator::from_raw(&self.jni_ref(), self.1).expect("Error converting JavaRandomGeneratorArbitrarilyJumpableGenerator into crate::util::random::JavaRandomGeneratorLeapableGenerator")
    }
}

pub struct JavaRandomGeneratorArbitrarilyJumpableGeneratorClass;
impl blackboxmc_general::JNIProvidesClassName
    for JavaRandomGeneratorArbitrarilyJumpableGeneratorClass
{
    fn class_name(&self) -> &str {
        "java/util/random/RandomGenerator$ArbitrarilyJumpableGenerator"
    }
}

///
/// This is a representation of an abstract class.
pub struct JavaRandomGeneratorJumpableGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorJumpableGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaRandomGeneratorJumpableGenerator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaRandomGeneratorJumpableGenerator from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "java/util/random/RandomGenerator$JumpableGenerator")?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a JavaRandomGeneratorJumpableGenerator object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaRandomGeneratorJumpableGenerator<'mc> {
    pub fn jump_distance(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "jumpDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn copy_and_jump(
        &self,
    ) -> Result<crate::util::random::JavaRandomGenerator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/random/RandomGenerator;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyAndJump", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::random::JavaRandomGenerator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn jump(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "jump", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<
        crate::util::random::JavaRandomGeneratorJumpableGenerator<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from(
            "(Ljava/lang/String;)Ljava/util/random/RandomGenerator$JumpableGenerator;",
        );
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/util/random/RandomGenerator$JumpableGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "of",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::random::JavaRandomGeneratorJumpableGenerator::from_raw(&jni, obj)
    }

    pub fn copy(
        &self,
    ) -> Result<
        crate::util::random::JavaRandomGeneratorJumpableGenerator<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Ljava/util/random/RandomGenerator$JumpableGenerator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copy", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::random::JavaRandomGeneratorJumpableGenerator::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn next_boolean(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn next_long_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn next_float_with_float(
        &self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "F";
            let val_2 = jni::objects::JValueGen::Float(a.into());
            args.push(val_2);
        }
        sig += ")F";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextFloat", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn next_bytes(&self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn next_gaussian_with_double(
        &self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextGaussian", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn is_deprecated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDeprecated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn next_exponential(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextExponential", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn next_double_with_double(
        &self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn next_int_with_int(
        &self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}
impl<'mc> Into<crate::util::random::JavaRandomGeneratorStreamableGenerator<'mc>>
    for JavaRandomGeneratorJumpableGenerator<'mc>
{
    fn into(self) -> crate::util::random::JavaRandomGeneratorStreamableGenerator<'mc> {
        crate::util::random::JavaRandomGeneratorStreamableGenerator::from_raw(&self.jni_ref(), self.1).expect("Error converting JavaRandomGeneratorJumpableGenerator into crate::util::random::JavaRandomGeneratorStreamableGenerator")
    }
}

pub struct JavaRandomGeneratorJumpableGeneratorClass;
impl blackboxmc_general::JNIProvidesClassName for JavaRandomGeneratorJumpableGeneratorClass {
    fn class_name(&self) -> &str {
        "java/util/random/RandomGenerator$JumpableGenerator"
    }
}
