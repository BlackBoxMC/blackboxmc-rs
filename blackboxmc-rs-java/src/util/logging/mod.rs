#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIInstantiatableEnum;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// ErrorManager objects can be attached to Handlers to process any error that occurs on a Handler during Logging.
/// <p>When processing logging output, if a Handler encounters problems then rather than throwing an Exception back to the issuer of the logging call (who is unlikely to be interested) the Handler should call its associated ErrorManager.</p>
pub struct JavaErrorManager<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaErrorManager<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaErrorManager<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaErrorManager from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/logging/ErrorManager")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaErrorManager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaErrorManager<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::logging::JavaErrorManager<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("java/util/logging/ErrorManager");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::logging::JavaErrorManager::from_raw(&jni, res)
    }
    //

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
    //

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
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaErrorManager<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaErrorManager.toString: {}", err),
        }
    }
}

/// Format a LogRecord into a standard XML format.
/// <p>The DTD specification is provided as Appendix A to the Java Logging APIs specification.</p>
/// <p>The XMLFormatter can be used with arbitrary character encodings, but it is recommended that it normally be used with UTF-8. The character encoding can be set on the output Handler.</p>
pub struct JavaXMLFormatter<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaXMLFormatter<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaXMLFormatter<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaXMLFormatter from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/logging/XMLFormatter")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaXMLFormatter object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaXMLFormatter<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::logging::JavaXMLFormatter<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("java/util/logging/XMLFormatter");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::logging::JavaXMLFormatter::from_raw(&jni, res)
    }
    //

    pub fn get_head(
        &self,
        arg0: impl Into<crate::util::logging::JavaHandler<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Handler;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHead",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn get_tail(
        &self,
        arg0: impl Into<crate::util::logging::JavaHandler<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Handler;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTail",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn format(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "format",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn format_message(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "formatMessage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

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
    //

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
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaXMLFormatter<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaXMLFormatter.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::logging::JavaFormatter<'mc>> for JavaXMLFormatter<'mc> {
    fn into(self) -> crate::util::logging::JavaFormatter<'mc> {
        crate::util::logging::JavaFormatter::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaXMLFormatter into crate::util::logging::JavaFormatter")
    }
}
/// A Formatter provides support for formatting LogRecords.
/// <p>Typically each logging Handler will have a Formatter associated with it. The Formatter takes a LogRecord and converts it to a string.</p>
/// <p>Some formatters (such as the XMLFormatter) need to wrap head and tail strings around a set of formatted records. The getHeader and getTail methods can be used to obtain these strings.</p>
pub struct JavaFormatter<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaFormatter<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaFormatter<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaFormatter from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/logging/Formatter")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaFormatter object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaFormatter<'mc> {
    //

    pub fn get_head(
        &self,
        arg0: impl Into<crate::util::logging::JavaHandler<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Handler;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHead",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn get_tail(
        &self,
        arg0: impl Into<crate::util::logging::JavaHandler<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Handler;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTail",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn format_message(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "formatMessage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn format(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "format",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

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
    //

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
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaFormatter<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaFormatter.toString: {}", err),
        }
    }
}

/// Simple network logging <tt>Handler</tt>.
/// <p><tt>LogRecords</tt> are published to a network stream connection. By default the <tt>XMLFormatter</tt> class is used for formatting.</p>
/// <p><b>Configuration:</b> By default each <tt>SocketHandler</tt> is initialized using the following <tt>LogManager</tt> configuration properties where <tt>&lt;handler-name&gt;</tt> refers to the fully-qualified class name of the handler. If properties are not defined (or have invalid values) then the specified default values are used.</p>
/// <ul>
/// <li>&lt;handler-name&gt;.level specifies the default level for the <tt>Handler</tt> (defaults to <tt>Level.ALL</tt>).</li>
/// <li>&lt;handler-name&gt;.filter specifies the name of a <tt>Filter</tt> class to use (defaults to no <tt>Filter</tt>).</li>
/// <li>&lt;handler-name&gt;.formatter specifies the name of a <tt>Formatter</tt> class to use (defaults to <tt>java.util.logging.XMLFormatter</tt>).</li>
/// <li>&lt;handler-name&gt;.encoding the name of the character set encoding to use (defaults to the default platform encoding).</li>
/// <li>&lt;handler-name&gt;.host specifies the target host name to connect to (no default).</li>
/// <li>&lt;handler-name&gt;.port specifies the target TCP port to use (no default).</li>
/// </ul>
/// <p>For example, the properties for <code>SocketHandler</code> would be:</p>
/// <ul>
/// <li>java.util.logging.SocketHandler.level=INFO</li>
/// <li>java.util.logging.SocketHandler.formatter=java.util.logging.SimpleFormatter</li>
/// </ul>
/// <p>For a custom handler, e.g. com.foo.MyHandler, the properties would be:</p>
/// <ul>
/// <li>com.foo.MyHandler.level=INFO</li>
/// <li>com.foo.MyHandler.formatter=java.util.logging.SimpleFormatter</li>
/// </ul>
/// <p>The output IO stream is buffered, but is flushed after each <tt>LogRecord</tt> is written.</p>
pub struct JavaSocketHandler<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaSocketHandler<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaSocketHandler<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaSocketHandler from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/logging/SocketHandler")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaSocketHandler object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaSocketHandler<'mc> {
    pub fn new_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<String>>,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::util::logging::JavaSocketHandler<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/String;";
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(a.into())?,
            ));
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("java/util/logging/SocketHandler");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::logging::JavaSocketHandler::from_raw(&jni, res)
    }
    //

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn publish(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "publish",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_encoding(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEncoding",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_loggable(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLoggable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "flush", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn filter(
        &self,
    ) -> Result<crate::util::logging::JavaFilter<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Filter;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFilter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaFilter::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_formatter(
        &self,
        arg0: impl Into<crate::util::logging::JavaFormatter<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Formatter;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFormatter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn formatter(
        &self,
    ) -> Result<crate::util::logging::JavaFormatter<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Formatter;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFormatter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaFormatter::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_error_manager(
        &self,
        arg0: impl Into<crate::util::logging::JavaErrorManager<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/ErrorManager;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setErrorManager",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn error_manager(
        &self,
    ) -> Result<crate::util::logging::JavaErrorManager<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/ErrorManager;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getErrorManager", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaErrorManager::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_filter(
        &self,
        arg0: impl Into<crate::util::logging::JavaFilter<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Filter;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFilter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_level(
        &self,
        arg0: impl Into<crate::util::logging::JavaLevel<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Level;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn encoding(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEncoding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn level(
        &self,
    ) -> Result<crate::util::logging::JavaLevel<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Level;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaLevel::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

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
    //

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
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaSocketHandler<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaSocketHandler.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::logging::JavaStreamHandler<'mc>> for JavaSocketHandler<'mc> {
    fn into(self) -> crate::util::logging::JavaStreamHandler<'mc> {
        crate::util::logging::JavaStreamHandler::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting JavaSocketHandler into crate::util::logging::JavaStreamHandler",
        )
    }
}
/// Print a brief summary of the <code>LogRecord</code> in a human readable format. The summary will typically be 1 or 2 lines.
/// <p><a name="formatting"> <b>Configuration:</b></a> The <code>SimpleFormatter</code> is initialized with the <a href="../Formatter.html#syntax">format string</a> specified in the <code>java.util.logging.SimpleFormatter.format</code> property to <a href="../../../java/util/logging/SimpleFormatter.html#format">format</a> the log messages. This property can be defined in the <a href="../../../java/util/logging/LogManager.html#getProperty-java.lang.String-">logging properties</a> configuration file or as a system property. If this property is set in both the logging properties and system properties, the format string specified in the system property will be used. If this property is not defined or the given format string is <a title="class in java.util" href="../../../java/util/IllegalFormatException.html">illegal</a>, the default format is implementation-specific.</p>
pub struct JavaSimpleFormatter<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaSimpleFormatter<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaSimpleFormatter<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaSimpleFormatter from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/logging/SimpleFormatter")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaSimpleFormatter object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaSimpleFormatter<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::logging::JavaSimpleFormatter<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("java/util/logging/SimpleFormatter");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::logging::JavaSimpleFormatter::from_raw(&jni, res)
    }
    //

    pub fn format(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "format",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn get_head(
        &self,
        arg0: impl Into<crate::util::logging::JavaHandler<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Handler;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHead",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn get_tail(
        &self,
        arg0: impl Into<crate::util::logging::JavaHandler<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Handler;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTail",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn format_message(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "formatMessage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

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
    //

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
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaSimpleFormatter<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaSimpleFormatter.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::logging::JavaFormatter<'mc>> for JavaSimpleFormatter<'mc> {
    fn into(self) -> crate::util::logging::JavaFormatter<'mc> {
        crate::util::logging::JavaFormatter::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaSimpleFormatter into crate::util::logging::JavaFormatter")
    }
}
/// A <tt>Handler</tt> object takes log messages from a <tt>Logger</tt> and exports them. It might for example, write them to a console or write them to a file, or send them to a network logging service, or forward them to an OS log, or whatever.
/// <p>A <tt>Handler</tt> can be disabled by doing a <tt>setLevel(Level.OFF)</tt> and can be re-enabled by doing a <tt>setLevel</tt> with an appropriate level.</p>
/// <p><tt>Handler</tt> classes typically use <tt>LogManager</tt> properties to set default values for the <tt>Handler</tt>'s <tt>Filter</tt>, <tt>Formatter</tt>, and <tt>Level</tt>. See the specific documentation for each concrete <tt>Handler</tt> class.</p>
pub struct JavaHandler<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaHandler<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaHandler<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaHandler from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/logging/Handler")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaHandler object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaHandler<'mc> {
    //

    pub fn filter(
        &self,
    ) -> Result<crate::util::logging::JavaFilter<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Filter;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFilter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaFilter::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_formatter(
        &self,
        arg0: impl Into<crate::util::logging::JavaFormatter<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Formatter;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFormatter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn formatter(
        &self,
    ) -> Result<crate::util::logging::JavaFormatter<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Formatter;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFormatter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaFormatter::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_encoding(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEncoding",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_error_manager(
        &self,
        arg0: impl Into<crate::util::logging::JavaErrorManager<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/ErrorManager;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setErrorManager",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn error_manager(
        &self,
    ) -> Result<crate::util::logging::JavaErrorManager<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/ErrorManager;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getErrorManager", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaErrorManager::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_filter(
        &self,
        arg0: impl Into<crate::util::logging::JavaFilter<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Filter;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFilter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_loggable(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLoggable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_level(
        &self,
        arg0: impl Into<crate::util::logging::JavaLevel<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Level;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "flush", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn publish(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "publish",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn encoding(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEncoding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn level(
        &self,
    ) -> Result<crate::util::logging::JavaLevel<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Level;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaLevel::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

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
    //

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
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaHandler<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaHandler.toString: {}", err),
        }
    }
}

/// <tt>Handler</tt> that buffers requests in a circular buffer in memory.
/// <p>Normally this <tt>Handler</tt> simply stores incoming <tt>LogRecords</tt> into its memory buffer and discards earlier records. This buffering is very cheap and avoids formatting costs. On certain trigger conditions, the <tt>MemoryHandler</tt> will push out its current buffer contents to a target <tt>Handler</tt>, which will typically publish them to the outside world.</p>
/// <p>There are three main models for triggering a push of the buffer:</p>
/// <ul>
/// <li>An incoming <tt>LogRecord</tt> has a type that is greater than a pre-defined level, the <tt>pushLevel</tt>.</li>
/// <li>An external class calls the <tt>push</tt> method explicitly.</li>
/// <li>A subclass overrides the <tt>log</tt> method and scans each incoming <tt>LogRecord</tt> and calls <tt>push</tt> if a record matches some desired criteria.</li>
/// </ul>
/// <p><b>Configuration:</b> By default each <tt>MemoryHandler</tt> is initialized using the following <tt>LogManager</tt> configuration properties where <tt>&lt;handler-name&gt;</tt> refers to the fully-qualified class name of the handler. If properties are not defined (or have invalid values) then the specified default values are used. If no default value is defined then a RuntimeException is thrown.</p>
/// <ul>
/// <li>&lt;handler-name&gt;.level specifies the level for the <tt>Handler</tt> (defaults to <tt>Level.ALL</tt>).</li>
/// <li>&lt;handler-name&gt;.filter specifies the name of a <tt>Filter</tt> class to use (defaults to no <tt>Filter</tt>).</li>
/// <li>&lt;handler-name&gt;.size defines the buffer size (defaults to 1000).</li>
/// <li>&lt;handler-name&gt;.push defines the <tt>pushLevel</tt> (defaults to <tt>level.SEVERE</tt>).</li>
/// <li>&lt;handler-name&gt;.target specifies the name of the target <tt>Handler </tt> class. (no default).</li>
/// </ul>
/// <p>For example, the properties for <code>MemoryHandler</code> would be:</p>
/// <ul>
/// <li>java.util.logging.MemoryHandler.level=INFO</li>
/// <li>java.util.logging.MemoryHandler.formatter=java.util.logging.SimpleFormatter</li>
/// </ul>
/// <p>For a custom handler, e.g. com.foo.MyHandler, the properties would be:</p>
/// <ul>
/// <li>com.foo.MyHandler.level=INFO</li>
/// <li>com.foo.MyHandler.formatter=java.util.logging.SimpleFormatter</li>
/// </ul>
/// <p></p>
pub struct JavaMemoryHandler<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaMemoryHandler<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaMemoryHandler<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaMemoryHandler from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/logging/MemoryHandler")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaMemoryHandler object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaMemoryHandler<'mc> {
    pub fn new_with_handler(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::util::logging::JavaHandler<'mc>>>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<impl Into<crate::util::logging::JavaLevel<'mc>>>,
    ) -> Result<crate::util::logging::JavaMemoryHandler<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/util/logging/Handler;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Ljava/util/logging/Level;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("java/util/logging/MemoryHandler");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::logging::JavaMemoryHandler::from_raw(&jni, res)
    }
    //

    pub fn push(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "push", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_push_level(
        &self,
        arg0: impl Into<crate::util::logging::JavaLevel<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Level;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPushLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn push_level(
        &self,
    ) -> Result<crate::util::logging::JavaLevel<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Level;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPushLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaLevel::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_loggable(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLoggable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "flush", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn publish(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "publish",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn filter(
        &self,
    ) -> Result<crate::util::logging::JavaFilter<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Filter;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFilter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaFilter::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_formatter(
        &self,
        arg0: impl Into<crate::util::logging::JavaFormatter<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Formatter;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFormatter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn formatter(
        &self,
    ) -> Result<crate::util::logging::JavaFormatter<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Formatter;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFormatter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaFormatter::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_encoding(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEncoding",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_error_manager(
        &self,
        arg0: impl Into<crate::util::logging::JavaErrorManager<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/ErrorManager;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setErrorManager",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn error_manager(
        &self,
    ) -> Result<crate::util::logging::JavaErrorManager<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/ErrorManager;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getErrorManager", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaErrorManager::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_filter(
        &self,
        arg0: impl Into<crate::util::logging::JavaFilter<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Filter;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFilter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_level(
        &self,
        arg0: impl Into<crate::util::logging::JavaLevel<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Level;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn encoding(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEncoding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn level(
        &self,
    ) -> Result<crate::util::logging::JavaLevel<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Level;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaLevel::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

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
    //

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
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaMemoryHandler<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaMemoryHandler.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::logging::JavaHandler<'mc>> for JavaMemoryHandler<'mc> {
    fn into(self) -> crate::util::logging::JavaHandler<'mc> {
        crate::util::logging::JavaHandler::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaMemoryHandler into crate::util::logging::JavaHandler")
    }
}
/// A Logger object is used to log messages for a specific system or application component. Loggers are normally named, using a hierarchical dot-separated namespace. Logger names can be arbitrary strings, but they should normally be based on the package name or class name of the logged component, such as java.net or javax.swing. In addition it is possible to create "anonymous" Loggers that are not stored in the Logger namespace.
/// <p>Logger objects may be obtained by calls on one of the getLogger factory methods. These will either create a new Logger or return a suitable existing Logger. It is important to note that the Logger returned by one of the <code>getLogger</code> factory methods may be garbage collected at any time if a strong reference to the Logger is not kept.</p>
/// <p>Logging messages will be forwarded to registered Handler objects, which can forward the messages to a variety of destinations, including consoles, files, OS logs, etc.</p>
/// <p>Each Logger keeps track of a "parent" Logger, which is its nearest existing ancestor in the Logger namespace.</p>
/// <p>Each Logger has a "Level" associated with it. This reflects a minimum Level that this logger cares about. If a Logger's level is set to <tt>null</tt>, then its effective level is inherited from its parent, which may in turn obtain it recursively from its parent, and so on up the tree.</p>
/// <p>The log level can be configured based on the properties from the logging configuration file, as described in the description of the LogManager class. However it may also be dynamically changed by calls on the Logger.setLevel method. If a logger's level is changed the change may also affect child loggers, since any child logger that has <tt>null</tt> as its level will inherit its effective level from its parent.</p>
/// <p>On each logging call the Logger initially performs a cheap check of the request level (e.g., SEVERE or FINE) against the effective log level of the logger. If the request level is lower than the log level, the logging call returns immediately.</p>
/// <p>After passing this initial (cheap) test, the Logger will allocate a LogRecord to describe the logging message. It will then call a Filter (if present) to do a more detailed check on whether the record should be published. If that passes it will then publish the LogRecord to its output Handlers. By default, loggers also publish to their parent's Handlers, recursively up the tree.</p>
/// <p>Each Logger may have a <code>ResourceBundle</code> associated with it. The <code>ResourceBundle</code> may be specified by name, using the <a href="../../../java/util/logging/Logger.html#getLogger-java.lang.String-java.lang.String-"><code>getLogger(java.lang.String, java.lang.String)</code></a> factory method, or by value - using the <a href="../../../java/util/logging/Logger.html#setResourceBundle-java.util.ResourceBundle-"><code>setResourceBundle</code></a> method. This bundle will be used for localizing logging messages. If a Logger does not have its own <code>ResourceBundle</code> or resource bundle name, then it will inherit the <code>ResourceBundle</code> or resource bundle name from its parent, recursively up the tree.</p>
/// <p>Most of the logger output methods take a "msg" argument. This msg argument may be either a raw value or a localization key. During formatting, if the logger has (or inherits) a localization <code>ResourceBundle</code> and if the <code>ResourceBundle</code> has a mapping for the msg string, then the msg string is replaced by the localized value. Otherwise the original msg string is used. Typically, formatters use java.text.MessageFormat style formatting to format parameters, so for example a format string "{0} {1}" would format two parameters as strings.</p>
/// <p>A set of methods alternatively take a "msgSupplier" instead of a "msg" argument. These methods take a <a href="../../../java/util/function/Supplier.html" title="interface in java.util.function"><code>Supplier</code></a><code>&lt;String&gt;</code> function which is invoked to construct the desired log message only when the message actually is to be logged based on the effective log level thus eliminating unnecessary message construction. For example, if the developer wants to log system health status for diagnosis, with the String-accepting version, the code would look like:</p>
/// <pre><code>
/// class DiagnosisMessages {
/// static String systemHealthStatus() {
/// // collect system health information
/// ...
/// }
/// }
/// ...
/// logger.log(Level.FINER, DiagnosisMessages.systemHealthStatus());
/// </code></pre> With the above code, the health status is collected unnecessarily even when the log level FINER is disabled. With the Supplier-accepting version as below, the status will only be collected when the log level FINER is enabled.
/// <pre><code>
/// logger.log(Level.FINER, DiagnosisMessages::systemHealthStatus);
/// </code></pre>
/// <p>When looking for a <code>ResourceBundle</code>, the logger will first look at whether a bundle was specified using <a href="../../../java/util/logging/Logger.html#setResourceBundle-java.util.ResourceBundle-"><code>setResourceBundle</code></a>, and then only whether a resource bundle name was specified through the <a href="../../../java/util/logging/Logger.html#getLogger-java.lang.String-java.lang.String-"><code>getLogger</code></a> factory method. If no <code>ResourceBundle</code> or no resource bundle name is found, then it will use the nearest <code>ResourceBundle</code> or resource bundle name inherited from its parent tree.
///
/// When a <code>ResourceBundle</code> was inherited or specified through the <a href="../../../java/util/logging/Logger.html#setResourceBundle-java.util.ResourceBundle-"><code>setResourceBundle</code></a> method, then that <code>ResourceBundle</code> will be used. Otherwise if the logger only has or inherited a resource bundle name, then that resource bundle name will be mapped to a <code>ResourceBundle</code> object, using the default Locale at the time of logging. <br id="ResourceBundleMapping">
/// When mapping resource bundle names to <code>ResourceBundle</code> objects, the logger will first try to use the Thread's <a href="../../../java/lang/Thread.html#getContextClassLoader--">context class loader</a> to map the given resource bundle name to a <code>ResourceBundle</code>. If the thread context class loader is <code>null</code>, it will try the <a href="../../../java/lang/ClassLoader.html#getSystemClassLoader--">system class loader</a> instead. If the <code>ResourceBundle</code> is still not found, it will use the class loader of the first caller of the <a href="../../../java/util/logging/Logger.html#getLogger-java.lang.String-java.lang.String-"><code>getLogger</code></a> factory method.</p>
/// <p>Formatting (including localization) is the responsibility of the output Handler, which will typically call a Formatter.</p>
/// <p>Note that formatting need not occur synchronously. It may be delayed until a LogRecord is actually written to an external sink.</p>
/// <p>The logging methods are grouped in five main categories:</p>
/// <ul>
/// <li>
/// <p>There are a set of "log" methods that take a log level, a message string, and optionally some parameters to the message string.</p></li>
/// <li>
/// <p>There are a set of "logp" methods (for "log precise") that are like the "log" methods, but also take an explicit source class name and method name.</p></li>
/// <li>
/// <p>There are a set of "logrb" method (for "log with resource bundle") that are like the "logp" method, but also take an explicit resource bundle object for use in localizing the log message.</p></li>
/// <li>
/// <p>There are convenience methods for tracing method entries (the "entering" methods), method returns (the "exiting" methods) and throwing exceptions (the "throwing" methods).</p></li>
/// <li>
/// <p>Finally, there are a set of convenience methods for use in the very simplest cases, when a developer simply wants to log a simple string at a given log level. These methods are named after the standard Level names ("severe", "warning", "info", etc.) and take a single argument, a message string.</p></li>
/// </ul>
/// <p>For the methods that do not take an explicit source name and method name, the Logging framework will make a "best effort" to determine which class and method called into the logging method. However, it is important to realize that this automatically inferred information may only be approximate (or may even be quite wrong!). Virtual machines are allowed to do extensive optimizations when JITing and may entirely remove stack frames, making it impossible to reliably locate the calling class and method.</p>
/// <p>All methods on Logger are multi-thread safe.</p>
/// <p><b>Subclassing Information:</b> Note that a LogManager class may provide its own implementation of named Loggers for any point in the namespace. Therefore, any subclasses of Logger (unless they are implemented in conjunction with a new LogManager class) should take care to obtain a Logger instance from the LogManager class and should delegate operations such as "isLoggable" and "log(LogRecord)" to that instance. Note that in order to intercept all logging output, subclasses need only override the log(LogRecord) method. All the other logging methods are implemented as calls on this log(LogRecord) method.</p>
pub struct JavaLogger<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLogger<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLogger<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaLogger from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/logging/Logger")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLogger object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLogger<'mc> {
    //

    pub fn config_with_supplier(
        &self,
        arg0: impl Into<crate::util::function::JavaSupplier<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/function/Supplier;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "config", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn warning_with_supplier(
        &self,
        arg0: impl Into<crate::util::function::JavaSupplier<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/function/Supplier;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "warning", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn filter(
        &self,
    ) -> Result<crate::util::logging::JavaFilter<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Filter;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFilter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaFilter::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn anonymous_logger(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::logging::JavaLogger<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Logger;");
        let cls = jni.find_class("java/util/logging/Logger");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getAnonymousLogger", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::logging::JavaLogger::from_raw(&jni, obj)
    }
    //

    pub fn resource_bundle_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResourceBundleName",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn set_filter(
        &self,
        arg0: impl Into<crate::util::logging::JavaFilter<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Filter;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFilter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_loggable(
        &self,
        arg0: impl Into<crate::util::logging::JavaLevel<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Level;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLoggable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    //

    pub fn use_parent_handlers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getUseParentHandlers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_handler(
        &self,
        arg0: impl Into<crate::util::logging::JavaHandler<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Handler;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addHandler",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_handler(
        &self,
        arg0: impl Into<crate::util::logging::JavaHandler<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Handler;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeHandler",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_use_parent_handlers(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUseParentHandlers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn global(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::logging::JavaLogger<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Logger;");
        let cls = jni.find_class("java/util/logging/Logger");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getGlobal", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::logging::JavaLogger::from_raw(&jni, obj)
    }
    //

    pub fn entering_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
        arg2: std::option::Option<Vec<jni::objects::JObject<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "entering", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn exiting_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "exiting", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn severe_with_supplier(
        &self,
        arg0: impl Into<crate::util::function::JavaSupplier<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/function/Supplier;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "severe", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn fine_with_supplier(
        &self,
        arg0: impl Into<crate::util::function::JavaSupplier<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/function/Supplier;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "fine", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn finer_with_supplier(
        &self,
        arg0: impl Into<crate::util::function::JavaSupplier<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/function/Supplier;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "finer", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn finest_with_string(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "finest", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_level(
        &self,
        arg0: impl Into<crate::util::logging::JavaLevel<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Level;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn log_with_level(
        &self,
        arg0: impl Into<crate::util::logging::JavaLevel<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/logging/Level;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "log", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn info_with_supplier(
        &self,
        arg0: impl Into<crate::util::function::JavaSupplier<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/function/Supplier;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "info", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn parent(
        &self,
    ) -> Result<crate::util::logging::JavaLogger<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Logger;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getParent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaLogger::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_logger_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<crate::util::logging::JavaLogger<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")Ljava/util/logging/Logger;";
        let cls = jni.find_class("java/util/logging/Logger");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getLogger", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::logging::JavaLogger::from_raw(&jni, obj)
    }
    //

    pub fn set_parent(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogger<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Logger;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setParent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn level(
        &self,
    ) -> Result<crate::util::logging::JavaLevel<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Level;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaLevel::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

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
    //

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
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaLogger<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaLogger.toString: {}", err),
        }
    }
}

/// This <tt>Handler</tt> publishes log records to <tt>System.err</tt>. By default the <tt>SimpleFormatter</tt> is used to generate brief summaries.
/// <p><b>Configuration:</b> By default each <tt>ConsoleHandler</tt> is initialized using the following <tt>LogManager</tt> configuration properties where <code>&lt;handler-name&gt;</code> refers to the fully-qualified class name of the handler. If properties are not defined (or have invalid values) then the specified default values are used.</p>
/// <ul>
/// <li>&lt;handler-name&gt;.level specifies the default level for the <tt>Handler</tt> (defaults to <tt>Level.INFO</tt>).</li>
/// <li>&lt;handler-name&gt;.filter specifies the name of a <tt>Filter</tt> class to use (defaults to no <tt>Filter</tt>).</li>
/// <li>&lt;handler-name&gt;.formatter specifies the name of a <tt>Formatter</tt> class to use (defaults to <tt>java.util.logging.SimpleFormatter</tt>).</li>
/// <li>&lt;handler-name&gt;.encoding the name of the character set encoding to use (defaults to the default platform encoding).</li>
/// </ul>
/// <p>For example, the properties for <code>ConsoleHandler</code> would be:</p>
/// <ul>
/// <li>java.util.logging.ConsoleHandler.level=INFO</li>
/// <li>java.util.logging.ConsoleHandler.formatter=java.util.logging.SimpleFormatter</li>
/// </ul>
/// <p>For a custom handler, e.g. com.foo.MyHandler, the properties would be:</p>
/// <ul>
/// <li>com.foo.MyHandler.level=INFO</li>
/// <li>com.foo.MyHandler.formatter=java.util.logging.SimpleFormatter</li>
/// </ul>
/// <p></p>
pub struct JavaConsoleHandler<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaConsoleHandler<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaConsoleHandler<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaConsoleHandler from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/logging/ConsoleHandler")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaConsoleHandler object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaConsoleHandler<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::logging::JavaConsoleHandler<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("java/util/logging/ConsoleHandler");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::logging::JavaConsoleHandler::from_raw(&jni, res)
    }
    //

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn publish(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "publish",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_encoding(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEncoding",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_loggable(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLoggable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "flush", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn filter(
        &self,
    ) -> Result<crate::util::logging::JavaFilter<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Filter;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFilter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaFilter::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_formatter(
        &self,
        arg0: impl Into<crate::util::logging::JavaFormatter<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Formatter;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFormatter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn formatter(
        &self,
    ) -> Result<crate::util::logging::JavaFormatter<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Formatter;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFormatter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaFormatter::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_error_manager(
        &self,
        arg0: impl Into<crate::util::logging::JavaErrorManager<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/ErrorManager;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setErrorManager",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn error_manager(
        &self,
    ) -> Result<crate::util::logging::JavaErrorManager<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/ErrorManager;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getErrorManager", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaErrorManager::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_filter(
        &self,
        arg0: impl Into<crate::util::logging::JavaFilter<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Filter;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFilter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_level(
        &self,
        arg0: impl Into<crate::util::logging::JavaLevel<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Level;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn encoding(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEncoding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn level(
        &self,
    ) -> Result<crate::util::logging::JavaLevel<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Level;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaLevel::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

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
    //

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
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaConsoleHandler<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaConsoleHandler.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::logging::JavaStreamHandler<'mc>> for JavaConsoleHandler<'mc> {
    fn into(self) -> crate::util::logging::JavaStreamHandler<'mc> {
        crate::util::logging::JavaStreamHandler::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting JavaConsoleHandler into crate::util::logging::JavaStreamHandler",
        )
    }
}
/// The permission which the SecurityManager will check when code that is running with a SecurityManager calls one of the logging control methods (such as Logger.setLevel).
/// <p>Currently there is only one named LoggingPermission. This is "control" and it grants the ability to control the logging configuration, for example by adding or removing Handlers, by adding or removing Filters, or by changing logging levels.</p>
/// <p>Programmers do not normally create LoggingPermission objects directly. Instead they are created by the security policy code based on reading the security policy file.</p>
pub struct JavaLoggingPermission<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLoggingPermission<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLoggingPermission<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaLoggingPermission from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/logging/LoggingPermission")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLoggingPermission object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLoggingPermission<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
    ) -> Result<crate::util::logging::JavaLoggingPermission<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg1.into())?,
        ));
        let cls = jni.find_class("java/util/logging/LoggingPermission");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::util::logging::JavaLoggingPermission::from_raw(&jni, res)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn implies(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/security/Permission;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "implies",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn actions(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getActions", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn new_permission_collection(
        &self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/security/PermissionCollection;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "newPermissionCollection",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

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
    //

    pub fn check_guard(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "checkGuard",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

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
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaLoggingPermission<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaLoggingPermission.toString: {}", err),
        }
    }
}

/// A Filter can be used to provide fine grain control over what is logged, beyond the control provided by log levels.
/// <p>Each Logger and each Handler can have a filter associated with it. The Logger or Handler will call the isLoggable method to check if a given LogRecord should be published. If isLoggable returns false, the LogRecord will be discarded.</p>
///
/// This is a representation of an abstract class.
pub struct JavaFilter<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaFilter<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaFilter<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaFilter from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/logging/Filter")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaFilter object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaFilter<'mc> {
    //

    pub fn is_loggable(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLoggable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
/// LogRecord objects are used to pass logging requests between the logging framework and individual log Handlers.
/// <p>When a LogRecord is passed into the logging framework it logically belongs to the framework and should no longer be used or updated by the client application.</p>
/// <p>Note that if the client application has not specified an explicit source method name and source class name, then the LogRecord class will infer them automatically when they are first accessed (due to a call on getSourceMethodName or getSourceClassName) by analyzing the call stack. Therefore, if a logging Handler wants to pass off a LogRecord to another thread, or to transmit it over RMI, and if it wishes to subsequently obtain method name or class name information it should call one of getSourceClassName or getSourceMethodName to force the values to be filled in.</p>
/// <p><b> Serialization notes:</b></p>
/// <ul>
/// <li>The LogRecord class is serializable.</li>
/// <li>Because objects in the parameters array may not be serializable, during serialization all objects in the parameters array are written as the corresponding Strings (using Object.toString).</li>
/// <li>The ResourceBundle is not transmitted as part of the serialized form, but the resource bundle name is, and the recipient object's readObject method will attempt to locate a suitable resource bundle.</li>
/// </ul>
pub struct JavaLogRecord<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLogRecord<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLogRecord<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaLogRecord from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/logging/LogRecord")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLogRecord object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLogRecord<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::logging::JavaLevel<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<crate::util::logging::JavaLogRecord<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Level;Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg1.into())?,
        ));
        let cls = jni.find_class("java/util/logging/LogRecord");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::util::logging::JavaLogRecord::from_raw(&jni, res)
    }
    //

    pub fn set_instant(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/time/Instant;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInstant",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_millis(&self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(J)V");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMillis",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn millis(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMillis", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn sequence_number(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSequenceNumber",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn set_sequence_number(&self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(J)V");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSequenceNumber",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn source_class_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSourceClassName",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn source_method_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSourceMethodName",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn thread_id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getThreadID", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_thread_id(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setThreadID",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn long_thread_id(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLongThreadID", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn set_long_thread_id(
        &self,
        arg0: i64,
    ) -> Result<crate::util::logging::JavaLogRecord<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(J)Ljava/util/logging/LogRecord;");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLongThreadID",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaLogRecord::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn instant(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/time/Instant;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInstant", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn resource_bundle_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResourceBundleName",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn set_logger_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLoggerName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_resource_bundle_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResourceBundleName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_parameters(
        &self,
        arg0: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setParameters", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_source_class_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSourceClassName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_source_method_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSourceMethodName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_level(
        &self,
        arg0: impl Into<crate::util::logging::JavaLevel<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Level;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn message(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMessage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    //

    pub fn logger_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLoggerName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn level(
        &self,
    ) -> Result<crate::util::logging::JavaLevel<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Level;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaLevel::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_message(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMessage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

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
    //

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
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaLogRecord<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaLogRecord.toString: {}", err),
        }
    }
}

/// Stream based logging <tt>Handler</tt>.
/// <p>This is primarily intended as a base class or support class to be used in implementing other logging <tt>Handlers</tt>.</p>
/// <p><tt>LogRecords</tt> are published to a given <tt>java.io.OutputStream</tt>.</p>
/// <p><b>Configuration:</b> By default each <tt>StreamHandler</tt> is initialized using the following <tt>LogManager</tt> configuration properties where <tt>&lt;handler-name&gt;</tt> refers to the fully-qualified class name of the handler. If properties are not defined (or have invalid values) then the specified default values are used.</p>
/// <ul>
/// <li>&lt;handler-name&gt;.level specifies the default level for the <tt>Handler</tt> (defaults to <tt>Level.INFO</tt>).</li>
/// <li>&lt;handler-name&gt;.filter specifies the name of a <tt>Filter</tt> class to use (defaults to no <tt>Filter</tt>).</li>
/// <li>&lt;handler-name&gt;.formatter specifies the name of a <tt>Formatter</tt> class to use (defaults to <tt>java.util.logging.SimpleFormatter</tt>).</li>
/// <li>&lt;handler-name&gt;.encoding the name of the character set encoding to use (defaults to the default platform encoding).</li>
/// </ul>
/// <p>For example, the properties for <code>StreamHandler</code> would be:</p>
/// <ul>
/// <li>java.util.logging.StreamHandler.level=INFO</li>
/// <li>java.util.logging.StreamHandler.formatter=java.util.logging.SimpleFormatter</li>
/// </ul>
/// <p>For a custom handler, e.g. com.foo.MyHandler, the properties would be:</p>
/// <ul>
/// <li>com.foo.MyHandler.level=INFO</li>
/// <li>com.foo.MyHandler.formatter=java.util.logging.SimpleFormatter</li>
/// </ul>
/// <p></p>
pub struct JavaStreamHandler<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaStreamHandler<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaStreamHandler<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaStreamHandler from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/logging/StreamHandler")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaStreamHandler object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaStreamHandler<'mc> {
    pub fn new_with_output_stream(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<impl Into<crate::util::logging::JavaFormatter<'mc>>>,
    ) -> Result<crate::util::logging::JavaStreamHandler<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/io/OutputStream;";
            let val_1 = jni::objects::JValueGen::Object(a);
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Ljava/util/logging/Formatter;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("java/util/logging/StreamHandler");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::logging::JavaStreamHandler::from_raw(&jni, res)
    }
    //

    pub fn set_encoding(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEncoding",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_loggable(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLoggable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "flush", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn publish(
        &self,
        arg0: impl Into<crate::util::logging::JavaLogRecord<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/LogRecord;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "publish",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn filter(
        &self,
    ) -> Result<crate::util::logging::JavaFilter<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Filter;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFilter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaFilter::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_formatter(
        &self,
        arg0: impl Into<crate::util::logging::JavaFormatter<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Formatter;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFormatter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn formatter(
        &self,
    ) -> Result<crate::util::logging::JavaFormatter<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Formatter;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFormatter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaFormatter::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_error_manager(
        &self,
        arg0: impl Into<crate::util::logging::JavaErrorManager<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/ErrorManager;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setErrorManager",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn error_manager(
        &self,
    ) -> Result<crate::util::logging::JavaErrorManager<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/ErrorManager;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getErrorManager", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaErrorManager::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_filter(
        &self,
        arg0: impl Into<crate::util::logging::JavaFilter<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Filter;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFilter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_level(
        &self,
        arg0: impl Into<crate::util::logging::JavaLevel<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/logging/Level;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn encoding(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEncoding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn level(
        &self,
    ) -> Result<crate::util::logging::JavaLevel<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Level;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::logging::JavaLevel::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

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
    //

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
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaStreamHandler<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaStreamHandler.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::logging::JavaHandler<'mc>> for JavaStreamHandler<'mc> {
    fn into(self) -> crate::util::logging::JavaHandler<'mc> {
        crate::util::logging::JavaHandler::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaStreamHandler into crate::util::logging::JavaHandler")
    }
}
/// The Level class defines a set of standard logging levels that can be used to control logging output. The logging Level objects are ordered and are specified by ordered integers. Enabling logging at a given level also enables logging at all higher levels.
/// <p>Clients should normally use the predefined Level constants such as Level.SEVERE.</p>
/// <p>The levels in descending order are:</p>
/// <ul>
/// <li>SEVERE (highest value)</li>
/// <li>WARNING</li>
/// <li>INFO</li>
/// <li>CONFIG</li>
/// <li>FINE</li>
/// <li>FINER</li>
/// <li>FINEST (lowest value)</li>
/// </ul> In addition there is a level OFF that can be used to turn off logging, and a level ALL that can be used to enable logging of all messages.
/// <p>It is possible for third parties to define additional logging levels by subclassing Level. In such cases subclasses should take care to chose unique integer level values and to ensure that they maintain the Object uniqueness property across serialization by defining a suitable readResolve method.</p>
pub struct JavaLevel<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLevel<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLevel<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaLevel from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/logging/Level")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLevel object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLevel<'mc> {
    //

    pub fn resource_bundle_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResourceBundleName",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn parse(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::util::logging::JavaLevel<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/logging/Level;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/util/logging/Level");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "parse",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::logging::JavaLevel::from_raw(&jni, obj)
    }
    //

    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

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
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn int_value(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "intValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

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
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaLevel<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaLevel.toString: {}", err),
        }
    }
}
