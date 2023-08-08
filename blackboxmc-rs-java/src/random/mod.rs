#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;

pub struct JavaRandomGeneratorFactory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaRandomGeneratorFactory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaRandomGeneratorFactory<'mc> {
    pub fn from_raw(
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

    pub fn all(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/stream/Stream")?;
        let res = jni.call_static_method(cls, "all", "()Ljava/util/stream/Stream;", &[])?;
        Ok(res.l().unwrap())
    }

    pub fn equidistribution(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "equidistribution", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn is_deprecated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDeprecated", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn period(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "period",
            "()Ljava/math/BigInteger;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn is_stochastic(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isStochastic", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_hardware(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isHardware", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn state_bits(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "stateBits", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn is_statistical(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isStatistical", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_arbitrarily_jumpable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isArbitrarilyJumpable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_jumpable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isJumpable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_leapable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLeapable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_splittable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSplittable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_streamable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isStreamable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "name", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn group(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "group", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn default(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/random/RandomGeneratorFactory")?;
        let res = jni.call_static_method(
            cls,
            "getDefault",
            "()Ljava/util/random/RandomGeneratorFactory;",
            &[],
        )?;
        Ok(res.l().unwrap())
    }

    pub fn of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("Javajava/util/random/RandomGeneratorFactory")?;
        let res = jni.call_static_method(
            cls,
            "of",
            "(Ljava/lang/String;)Ljava/util/random/RandomGeneratorFactory;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }

    pub fn create(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "create",
            "(J)Ljava/util/random/RandomGenerator;",
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

///
/// This is a representation of an abstract class.
pub struct JavaRandomGeneratorSplittableGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaRandomGeneratorSplittableGenerator<'mc> {
    pub fn from_raw(
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
            env.validate_name(&obj, "java/util/random/RandomGeneratorSplittableGenerator")?;
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

    pub fn rngs(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rngs",
            "(J)Ljava/util/stream/Stream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub unsafe fn splits(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "splits",
            "(Ljava/util/random/RandomGenerator$SplittableGenerator;)Ljava/util/stream/Stream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub unsafe fn splits_with_long(
        &mut self,
        arg0: i64,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "splits",
            "(JLjava/util/random/RandomGenerator$SplittableGenerator;)Ljava/util/stream/Stream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub unsafe fn split(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(&self.jni_object(),"split","(Ljava/util/random/RandomGenerator$SplittableGenerator;)Ljava/util/random/RandomGenerator$SplittableGenerator;",&[jni::objects::JValueGen::from(&val_1)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("Javajava/util/random/RandomGenerator$SplittableGenerator")?;
        let res = jni.call_static_method(
            cls,
            "of",
            "(Ljava/lang/String;)Ljava/util/random/RandomGenerator$SplittableGenerator;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }

    pub fn next_boolean(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBoolean", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn next_long(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextLong",
            "(JJ)J",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn next_float(
        &mut self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextFloat",
            "(FF)F",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }

    pub fn ints(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ints",
            "(II)Ljava/util/stream/IntStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn ints_with_long(
        &mut self,
        arg0: i64,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ints",
            "(JII)Ljava/util/stream/IntStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn longs(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
        arg2: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Long(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "longs",
            "(JJJ)Ljava/util/stream/LongStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn doubles(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "doubles",
            "(DD)Ljava/util/stream/DoubleStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn doubles_with_long(
        &mut self,
        arg0: i64,
        arg1: f64,
        arg2: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "doubles",
            "(JDD)Ljava/util/stream/DoubleStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn next_bytes(&mut self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", "(B)V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn next_gaussian(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextGaussian",
            "(DD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn is_deprecated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDeprecated", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn next_exponential(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextExponential", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn next_double(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextDouble",
            "(DD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn next_int(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextInt",
            "(II)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorSplittableGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaRandomGeneratorSplittableGenerator<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}

///
/// This is a representation of an abstract class.
pub struct JavaRandomGeneratorStreamableGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaRandomGeneratorStreamableGenerator<'mc> {
    pub fn from_raw(
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
            env.validate_name(&obj, "java/util/random/RandomGeneratorStreamableGenerator")?;
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

    pub fn rngs(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rngs",
            "(J)Ljava/util/stream/Stream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("Javajava/util/random/RandomGenerator$StreamableGenerator")?;
        let res = jni.call_static_method(
            cls,
            "of",
            "(Ljava/lang/String;)Ljava/util/random/RandomGenerator$StreamableGenerator;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }

    pub fn next_boolean(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBoolean", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn next_long(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextLong",
            "(JJ)J",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn next_float(
        &mut self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextFloat",
            "(FF)F",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }

    pub fn ints(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ints",
            "(II)Ljava/util/stream/IntStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn ints_with_long(
        &mut self,
        arg0: i64,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ints",
            "(JII)Ljava/util/stream/IntStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn longs(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
        arg2: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Long(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "longs",
            "(JJJ)Ljava/util/stream/LongStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn doubles(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "doubles",
            "(DD)Ljava/util/stream/DoubleStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn doubles_with_long(
        &mut self,
        arg0: i64,
        arg1: f64,
        arg2: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "doubles",
            "(JDD)Ljava/util/stream/DoubleStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn next_bytes(&mut self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", "(B)V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn next_gaussian(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextGaussian",
            "(DD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn is_deprecated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDeprecated", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn next_exponential(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextExponential", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn next_double(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextDouble",
            "(DD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn next_int(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextInt",
            "(II)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorStreamableGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

///
/// This is a representation of an abstract class.
pub struct JavaRandomGeneratorLeapableGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaRandomGeneratorLeapableGenerator<'mc> {
    pub fn from_raw(
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
            env.validate_name(&obj, "java/util/random/RandomGeneratorLeapableGenerator")?;
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

    pub fn leap_distance(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leapDistance", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn leap(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leap", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn leaps(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "leaps",
            "(J)Ljava/util/stream/Stream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn copy_and_leap(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyAndLeap",
            "()Ljava/util/random/RandomGenerator$JumpableGenerator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("Javajava/util/random/RandomGenerator$LeapableGenerator")?;
        let res = jni.call_static_method(
            cls,
            "of",
            "(Ljava/lang/String;)Ljava/util/random/RandomGenerator$LeapableGenerator;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }

    pub fn copy(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copy",
            "()Ljava/util/random/RandomGenerator$JumpableGenerator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn jump(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "jump", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn jump_distance(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "jumpDistance", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn jumps(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "jumps",
            "(J)Ljava/util/stream/Stream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn copy_and_jump(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyAndJump",
            "()Ljava/util/random/RandomGenerator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn rngs(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rngs",
            "(J)Ljava/util/stream/Stream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn next_boolean(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBoolean", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn next_long(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextLong",
            "(JJ)J",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn next_float(
        &mut self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextFloat",
            "(FF)F",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }

    pub fn ints(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ints",
            "(II)Ljava/util/stream/IntStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn ints_with_long(
        &mut self,
        arg0: i64,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ints",
            "(JII)Ljava/util/stream/IntStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn longs(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
        arg2: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Long(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "longs",
            "(JJJ)Ljava/util/stream/LongStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn doubles(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "doubles",
            "(DD)Ljava/util/stream/DoubleStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn doubles_with_long(
        &mut self,
        arg0: i64,
        arg1: f64,
        arg2: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "doubles",
            "(JDD)Ljava/util/stream/DoubleStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn next_bytes(&mut self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", "(B)V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn next_gaussian(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextGaussian",
            "(DD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn is_deprecated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDeprecated", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn next_exponential(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextExponential", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn next_double(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextDouble",
            "(DD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn next_int(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextInt",
            "(II)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorLeapableGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaRandomGeneratorLeapableGenerator<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}

///
/// This is a representation of an abstract class.
pub struct JavaRandomGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaRandomGenerator<'mc> {
    pub fn from_raw(
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

    pub fn next_boolean(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBoolean", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn next_long(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextLong",
            "(JJ)J",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn next_float(
        &mut self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextFloat",
            "(FF)F",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }

    pub fn ints(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ints",
            "(II)Ljava/util/stream/IntStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn ints_with_long(
        &mut self,
        arg0: i64,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ints",
            "(JII)Ljava/util/stream/IntStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn longs(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
        arg2: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Long(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "longs",
            "(JJJ)Ljava/util/stream/LongStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn doubles(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "doubles",
            "(DD)Ljava/util/stream/DoubleStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn doubles_with_long(
        &mut self,
        arg0: i64,
        arg1: f64,
        arg2: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "doubles",
            "(JDD)Ljava/util/stream/DoubleStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn next_bytes(&mut self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", "(B)V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn next_gaussian(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextGaussian",
            "(DD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn is_deprecated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDeprecated", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn next_exponential(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextExponential", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn next_double(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextDouble",
            "(DD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn next_int(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextInt",
            "(II)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn default(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/random/RandomGenerator")?;
        let res = jni.call_static_method(
            cls,
            "getDefault",
            "()Ljava/util/random/RandomGenerator;",
            &[],
        )?;
        Ok(res.l().unwrap())
    }

    pub fn of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("Javajava/util/random/RandomGenerator")?;
        let res = jni.call_static_method(
            cls,
            "of",
            "(Ljava/lang/String;)Ljava/util/random/RandomGenerator;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JavaRandomGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

///
/// This is a representation of an abstract class.
pub struct JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc> {
    pub fn from_raw(
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
            "java/util/random/RandomGeneratorArbitrarilyJumpableGenerator",
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

    pub fn jump(
        &mut self,
        arg0: std::option::Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "jump",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn jumps(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "jumps",
            "(J)Ljava/util/stream/Stream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn jumps_with_long(
        &mut self,
        arg0: i64,
        arg1: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "jumps",
            "(JD)Ljava/util/stream/Stream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn copy_and_jump(
        &mut self,
        arg0: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyAndJump",
            "(D)Ljava/util/random/RandomGenerator$ArbitrarilyJumpableGenerator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn jump_power_of_two(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "jumpPowerOfTwo",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn leap(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leap", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls =
            &jni.find_class("Javajava/util/random/RandomGenerator$ArbitrarilyJumpableGenerator")?;
        let res = jni.call_static_method(
            cls,
            "of",
            "(Ljava/lang/String;)Ljava/util/random/RandomGenerator$ArbitrarilyJumpableGenerator;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }

    pub fn copy(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copy",
            "()Ljava/util/random/RandomGenerator$LeapableGenerator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn leap_distance(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leapDistance", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn leaps(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "leaps",
            "(J)Ljava/util/stream/Stream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn copy_and_leap(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyAndLeap",
            "()Ljava/util/random/RandomGenerator$JumpableGenerator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn jump_distance(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "jumpDistance", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn rngs(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rngs",
            "(J)Ljava/util/stream/Stream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn next_boolean(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBoolean", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn next_long(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextLong",
            "(JJ)J",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn next_float(
        &mut self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextFloat",
            "(FF)F",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }

    pub fn ints(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ints",
            "(II)Ljava/util/stream/IntStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn ints_with_long(
        &mut self,
        arg0: i64,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ints",
            "(JII)Ljava/util/stream/IntStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn longs(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
        arg2: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Long(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "longs",
            "(JJJ)Ljava/util/stream/LongStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn doubles(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "doubles",
            "(DD)Ljava/util/stream/DoubleStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn doubles_with_long(
        &mut self,
        arg0: i64,
        arg1: f64,
        arg2: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "doubles",
            "(JDD)Ljava/util/stream/DoubleStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn next_bytes(&mut self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", "(B)V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn next_gaussian(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextGaussian",
            "(DD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn is_deprecated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDeprecated", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn next_exponential(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextExponential", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn next_double(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextDouble",
            "(DD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn next_int(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextInt",
            "(II)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>>
    for JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc>
{
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}

///
/// This is a representation of an abstract class.
pub struct JavaRandomGeneratorJumpableGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaRandomGeneratorJumpableGenerator<'mc> {
    pub fn from_raw(
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
            env.validate_name(&obj, "java/util/random/RandomGeneratorJumpableGenerator")?;
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

    pub fn jump(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "jump", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn jump_distance(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "jumpDistance", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn jumps(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "jumps",
            "(J)Ljava/util/stream/Stream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn copy_and_jump(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyAndJump",
            "()Ljava/util/random/RandomGenerator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn rngs(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rngs",
            "(J)Ljava/util/stream/Stream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("Javajava/util/random/RandomGenerator$JumpableGenerator")?;
        let res = jni.call_static_method(
            cls,
            "of",
            "(Ljava/lang/String;)Ljava/util/random/RandomGenerator$JumpableGenerator;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }

    pub fn copy(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copy",
            "()Ljava/util/random/RandomGenerator$JumpableGenerator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn next_boolean(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBoolean", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn next_long(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextLong",
            "(JJ)J",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn next_float(
        &mut self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextFloat",
            "(FF)F",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }

    pub fn ints(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ints",
            "(II)Ljava/util/stream/IntStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn ints_with_long(
        &mut self,
        arg0: i64,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ints",
            "(JII)Ljava/util/stream/IntStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn longs(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
        arg2: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Long(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "longs",
            "(JJJ)Ljava/util/stream/LongStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn doubles(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "doubles",
            "(DD)Ljava/util/stream/DoubleStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn doubles_with_long(
        &mut self,
        arg0: i64,
        arg1: f64,
        arg2: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "doubles",
            "(JDD)Ljava/util/stream/DoubleStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn next_bytes(&mut self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", "(B)V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn next_gaussian(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextGaussian",
            "(DD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn is_deprecated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDeprecated", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn next_exponential(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextExponential", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn next_double(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextDouble",
            "(DD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn next_int(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextInt",
            "(II)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorJumpableGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaRandomGeneratorJumpableGenerator<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
