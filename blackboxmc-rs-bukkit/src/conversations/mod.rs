#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// NumericPrompt is the base class for any prompt that requires a <a class="external-link" href="https://docs.oracle.com/javase/8/docs/api/java/lang/Number.html" title="class or interface in java.lang"><code>Number</code></a> response from the user.
#[repr(C)]
pub struct NumericPrompt<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for NumericPrompt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for NumericPrompt<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate NumericPrompt from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/conversations/NumericPrompt")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a NumericPrompt object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> NumericPrompt<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::conversations::NumericPrompt<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/conversations/NumericPrompt");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::NumericPrompt::from_raw(&jni, res)
    }
    // SUPER CLASS: ValidatingPrompt
    pub fn get_prompt_text(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::conversations::ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.get_prompt_text(arg0)
    }
    pub fn blocks_for_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::conversations::ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.blocks_for_input(arg0)
    }
    pub fn accept_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<crate::conversations::Prompt<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::conversations::ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.accept_input(arg0, arg1)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::conversations::ValidatingPrompt<'mc>> for NumericPrompt<'mc> {
    fn into(self) -> crate::conversations::ValidatingPrompt<'mc> {
        crate::conversations::ValidatingPrompt::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting NumericPrompt into crate::conversations::ValidatingPrompt")
    }
}
/// A ConversationPrefix implementation prepends all output from the conversation to the player. The ConversationPrefix can be used to display the plugin name or conversation status as the conversation evolves.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct ConversationPrefix<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ConversationPrefix<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConversationPrefix<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ConversationPrefix from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/conversations/ConversationPrefix")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ConversationPrefix object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ConversationPrefix<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "ConversationPrefix", name, lib_name) }?;
        Self::from_raw(env, obj)
    }

    pub fn get_prefix(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/conversations/ConversationContext;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrefix",
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// A ConversationFactory is responsible for creating a <a href="Conversation.html" title="class in org.bukkit.conversations"><code>Conversation</code></a> from a predefined template. A ConversationFactory is typically created when a plugin is instantiated and builds a Conversation each time a user initiates a conversation with the plugin. Each Conversation maintains its own state and calls back as needed into the plugin.
/// <p>The ConversationFactory implements a fluid API, allowing parameters to be set as an extension to the constructor.</p>
#[repr(C)]
pub struct ConversationFactory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ConversationFactory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConversationFactory<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ConversationFactory from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/conversations/ConversationFactory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ConversationFactory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ConversationFactory<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<crate::conversations::ConversationFactory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/conversations/ConversationFactory");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::ConversationFactory::from_raw(&jni, res)
    }

    pub fn with_prefix(
        &self,
        arg0: impl Into<crate::conversations::ConversationPrefix<'mc>>,
    ) -> Result<crate::conversations::ConversationFactory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/ConversationPrefix;)Lorg/bukkit/conversations/ConversationFactory;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "withPrefix",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::ConversationFactory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the modality of all <a title="class in org.bukkit.conversations" href="Conversation.html"><code>Conversation</code></a>s created by this factory. If a conversation is modal, all messages directed to the player are suppressed for the duration of the conversation.
    /// <p>The default is True.</p>
    pub fn with_modality(
        &self,
        arg0: bool,
    ) -> Result<crate::conversations::ConversationFactory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Lorg/bukkit/conversations/ConversationFactory;");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "withModality",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::ConversationFactory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the local echo status for all <a href="Conversation.html" title="class in org.bukkit.conversations"><code>Conversation</code></a>s created by this factory. If local echo is enabled, any text submitted to a conversation gets echoed back into the submitter's chat window.
    pub fn with_local_echo(
        &self,
        arg0: bool,
    ) -> Result<crate::conversations::ConversationFactory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Lorg/bukkit/conversations/ConversationFactory;");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "withLocalEcho",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::ConversationFactory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the number of inactive seconds to wait before automatically abandoning all generated conversations.
    /// <p>The default is 600 seconds (5 minutes).</p>
    pub fn with_timeout(
        &self,
        arg0: i32,
    ) -> Result<crate::conversations::ConversationFactory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/conversations/ConversationFactory;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "withTimeout",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::ConversationFactory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn with_conversation_canceller(
        &self,
        arg0: impl Into<crate::conversations::ConversationCanceller<'mc>>,
    ) -> Result<crate::conversations::ConversationFactory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/ConversationCanceller;)Lorg/bukkit/conversations/ConversationFactory;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "withConversationCanceller",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::ConversationFactory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn with_first_prompt(
        &self,
        arg0: impl Into<crate::conversations::Prompt<'mc>>,
    ) -> Result<crate::conversations::ConversationFactory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/conversations/Prompt;)Lorg/bukkit/conversations/ConversationFactory;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "withFirstPrompt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::ConversationFactory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn with_initial_session_data(
        &self,
        arg0: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<crate::conversations::ConversationFactory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)Lorg/bukkit/conversations/ConversationFactory;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "withInitialSessionData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::ConversationFactory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn with_escape_sequence(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::conversations::ConversationFactory<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/String;)Lorg/bukkit/conversations/ConversationFactory;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "withEscapeSequence",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::ConversationFactory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn that_excludes_non_players_with_message(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::conversations::ConversationFactory<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/String;)Lorg/bukkit/conversations/ConversationFactory;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "thatExcludesNonPlayersWithMessage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::ConversationFactory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_conversation_abandoned_listener(
        &self,
        arg0: impl Into<crate::conversations::ConversationAbandonedListener<'mc>>,
    ) -> Result<crate::conversations::ConversationFactory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/ConversationAbandonedListener;)Lorg/bukkit/conversations/ConversationFactory;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addConversationAbandonedListener",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::ConversationFactory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn build_conversation(
        &self,
        arg0: impl Into<crate::conversations::Conversable<'mc>>,
    ) -> Result<crate::conversations::Conversation<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/conversations/Conversable;)Lorg/bukkit/conversations/Conversation;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "buildConversation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::Conversation::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// An InactivityConversationCanceller will cancel a <a title="class in org.bukkit.conversations" href="Conversation.html"><code>Conversation</code></a> after a period of inactivity by the user.
#[repr(C)]
pub struct InactivityConversationCanceller<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InactivityConversationCanceller<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InactivityConversationCanceller<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InactivityConversationCanceller from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/conversations/InactivityConversationCanceller",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a InactivityConversationCanceller object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InactivityConversationCanceller<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: i32,
    ) -> Result<
        crate::conversations::InactivityConversationCanceller<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("org/bukkit/conversations/InactivityConversationCanceller");
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
        crate::conversations::InactivityConversationCanceller::from_raw(&jni, res)
    }

    pub fn set_conversation(
        &self,
        arg0: impl Into<crate::conversations::Conversation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/Conversation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setConversation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cancel_based_on_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/conversations/ConversationContext;Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "cancelBasedOnInput",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::conversations::ConversationCanceller<'mc>>
    for InactivityConversationCanceller<'mc>
{
    fn into(self) -> crate::conversations::ConversationCanceller<'mc> {
        crate::conversations::ConversationCanceller::from_raw(&self.jni_ref(), self.1).expect("Error converting InactivityConversationCanceller into crate::conversations::ConversationCanceller")
    }
}
/// An ExactMatchConversationCanceller cancels a conversation if the user enters an exact input string
#[repr(C)]
pub struct ExactMatchConversationCanceller<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ExactMatchConversationCanceller<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ExactMatchConversationCanceller<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ExactMatchConversationCanceller from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/conversations/ExactMatchConversationCanceller",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ExactMatchConversationCanceller object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ExactMatchConversationCanceller<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<
        crate::conversations::ExactMatchConversationCanceller<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("org/bukkit/conversations/ExactMatchConversationCanceller");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::ExactMatchConversationCanceller::from_raw(&jni, res)
    }

    pub fn set_conversation(
        &self,
        arg0: impl Into<crate::conversations::Conversation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/Conversation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setConversation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cancel_based_on_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/conversations/ConversationContext;Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "cancelBasedOnInput",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::conversations::ConversationCanceller<'mc>>
    for ExactMatchConversationCanceller<'mc>
{
    fn into(self) -> crate::conversations::ConversationCanceller<'mc> {
        crate::conversations::ConversationCanceller::from_raw(&self.jni_ref(), self.1).expect("Error converting ExactMatchConversationCanceller into crate::conversations::ConversationCanceller")
    }
}
/// The Conversable interface is used to indicate objects that can have conversations.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Conversable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Conversable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Conversable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Conversable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/conversations/Conversable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Conversable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Conversable<'mc> {
    pub fn accept_conversation_input(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "acceptConversationInput",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn begin_conversation(
        &self,
        arg0: impl Into<crate::conversations::Conversation<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/Conversation;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "beginConversation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn abandon_conversation_with_conversation(
        &self,
        arg0: impl Into<crate::conversations::Conversation<'mc>>,
        arg1: std::option::Option<impl Into<crate::conversations::ConversationAbandonedEvent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/conversations/Conversation;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/conversations/ConversationAbandonedEvent;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "abandonConversation",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_conversing(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isConversing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn send_raw_message_with_uuid(
        &self,
        arg0: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
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
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "sendRawMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// The Conversation class is responsible for tracking the current state of a conversation, displaying prompts to the user, and dispatching the user's response to the appropriate place. Conversation objects are not typically instantiated directly. Instead a <a title="class in org.bukkit.conversations" href="ConversationFactory.html"><code>ConversationFactory</code></a> is used to construct identical conversations on demand.
/// <p>Conversation flow consists of a directed graph of <a href="Prompt.html" title="interface in org.bukkit.conversations"><code>Prompt</code></a> objects. Each time a prompt gets input from the user, it must return the next prompt in the graph. Since each Prompt chooses the next Prompt, complex conversation trees can be implemented where the nature of the player's response directs the flow of the conversation.</p>
/// <p>Each conversation has a <a href="ConversationPrefix.html" title="interface in org.bukkit.conversations"><code>ConversationPrefix</code></a> that prepends all output from the conversation to the player. The ConversationPrefix can be used to display the plugin name or conversation status as the conversation evolves.</p>
/// <p>Each conversation has a timeout measured in the number of inactive seconds to wait before abandoning the conversation. If the inactivity timeout is reached, the conversation is abandoned and the user's incoming and outgoing chat is returned to normal.</p>
/// <p>You should not construct a conversation manually. Instead, use the <a title="class in org.bukkit.conversations" href="ConversationFactory.html"><code>ConversationFactory</code></a> for access to all available options.</p>
#[repr(C)]
pub struct Conversation<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum ConversationConversationState<'mc> {
    Unstarted {
        inner: ConversationConversationStateStruct<'mc>,
    },
    Started {
        inner: ConversationConversationStateStruct<'mc>,
    },
    Abandoned {
        inner: ConversationConversationStateStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for ConversationConversationState<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversationConversationState::Unstarted { .. } => f.write_str("UNSTARTED"),
            ConversationConversationState::Started { .. } => f.write_str("STARTED"),
            ConversationConversationState::Abandoned { .. } => f.write_str("ABANDONED"),
        }
    }
}

impl<'mc> ConversationConversationState<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ConversationConversationState<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/conversations/Conversation$ConversationState");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/conversations/Conversation$ConversationState;",
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
            "UNSTARTED" => Ok(ConversationConversationState::Unstarted {
                inner: ConversationConversationStateStruct::from_raw(env, obj)?,
            }),
            "STARTED" => Ok(ConversationConversationState::Started {
                inner: ConversationConversationStateStruct::from_raw(env, obj)?,
            }),
            "ABANDONED" => Ok(ConversationConversationState::Abandoned {
                inner: ConversationConversationStateStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ConversationConversationStateStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ConversationConversationState<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Unstarted { inner } => inner.0.clone(),
            Self::Started { inner } => inner.0.clone(),
            Self::Abandoned { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Unstarted { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Started { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Abandoned { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConversationConversationState<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ConversationConversationState from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/conversations/Conversation$ConversationState",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ConversationConversationState object, got {}",
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
                "UNSTARTED" => Ok(ConversationConversationState::Unstarted {
                    inner: ConversationConversationStateStruct::from_raw(env, obj)?,
                }),
                "STARTED" => Ok(ConversationConversationState::Started {
                    inner: ConversationConversationStateStruct::from_raw(env, obj)?,
                }),
                "ABANDONED" => Ok(ConversationConversationState::Abandoned {
                    inner: ConversationConversationStateStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ConversationConversationStateStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConversationConversationStateStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ConversationConversationStateStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/conversations/Conversation$ConversationState",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ConversationConversationStateStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ConversationConversationStateStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        Vec<crate::conversations::ConversationConversationState<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/conversations/Conversation$ConversationState;");
        let cls = jni.find_class("org/bukkit/conversations/Conversation$ConversationState");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::conversations::ConversationConversationState::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for Conversation<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Conversation<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Conversation from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/conversations/Conversation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Conversation object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Conversation<'mc> {
    pub fn new_with_plugin(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: impl Into<crate::conversations::Conversable<'mc>>,
        arg2: impl Into<crate::conversations::Prompt<'mc>>,
        arg3: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::conversations::Conversation<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/conversations/Conversable;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/conversations/Prompt;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        args.push(val_3);
        if let Some(a) = arg3 {
            sig += "Ljava/util/Map;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/conversations/Conversation");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::Conversation::from_raw(&jni, res)
    }

    pub fn prefix(
        &self,
    ) -> Result<Option<crate::conversations::ConversationPrefix<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/conversations/ConversationPrefix;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPrefix", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::conversations::ConversationPrefix::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn accept_input(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "acceptInput",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_modal(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isModal", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn add_conversation_abandoned_listener(
        &self,
        arg0: impl Into<crate::conversations::ConversationAbandonedListener<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/ConversationAbandonedListener;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addConversationAbandonedListener",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the status of local echo for this conversation. If local echo is enabled, any text submitted to a conversation gets echoed back into the submitter's chat window.
    pub fn set_local_echo_enabled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalEchoEnabled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn for_whom(
        &self,
    ) -> Result<crate::conversations::Conversable<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/conversations/Conversable;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getForWhom", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::Conversable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn abandon_with_conversation_abandoned_event(
        &self,
        arg0: std::option::Option<impl Into<crate::conversations::ConversationAbandonedEvent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/conversations/ConversationAbandonedEvent;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "abandon", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_local_echo_enabled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLocalEchoEnabled",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn cancellers(
        &self,
    ) -> Result<Vec<crate::conversations::ConversationCanceller<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCancellers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::conversations::ConversationCanceller::from_raw(
                &self.0, obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn output_next_prompt(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "outputNextPrompt",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_conversation_abandoned_listener(
        &self,
        arg0: impl Into<crate::conversations::ConversationAbandonedListener<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/ConversationAbandonedListener;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeConversationAbandonedListener",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn begin(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "begin", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn context(
        &self,
    ) -> Result<crate::conversations::ConversationContext<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/conversations/ConversationContext;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContext", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::ConversationContext::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn state(
        &self,
    ) -> Result<crate::conversations::ConversationConversationState<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/conversations/Conversation$ConversationState;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getState", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::ConversationConversationState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct ConversationAbandonedListener<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ConversationAbandonedListener<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConversationAbandonedListener<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ConversationAbandonedListener from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/conversations/ConversationAbandonedListener",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ConversationAbandonedListener object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ConversationAbandonedListener<'mc> {
    pub fn conversation_abandoned(
        &self,
        arg0: impl Into<crate::conversations::ConversationAbandonedEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/ConversationAbandonedEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "conversationAbandoned",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<blackboxmc_java::util::JavaEventListener<'mc>>
    for ConversationAbandonedListener<'mc>
{
    fn into(self) -> blackboxmc_java::util::JavaEventListener<'mc> {
        blackboxmc_java::util::JavaEventListener::from_raw(&self.jni_ref(), self.1).expect("Error converting ConversationAbandonedListener into blackboxmc_java::util::JavaEventListener")
    }
}
pub enum ConversationState<'mc> {
    Unstarted { inner: ConversationStateStruct<'mc> },
    Started { inner: ConversationStateStruct<'mc> },
    Abandoned { inner: ConversationStateStruct<'mc> },
}
impl<'mc> std::fmt::Display for ConversationState<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversationState::Unstarted { .. } => f.write_str("UNSTARTED"),
            ConversationState::Started { .. } => f.write_str("STARTED"),
            ConversationState::Abandoned { .. } => f.write_str("ABANDONED"),
        }
    }
}

impl<'mc> ConversationState<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ConversationState<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/conversations/ConversationState");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/conversations/ConversationState;",
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
            "UNSTARTED" => Ok(ConversationState::Unstarted {
                inner: ConversationStateStruct::from_raw(env, obj)?,
            }),
            "STARTED" => Ok(ConversationState::Started {
                inner: ConversationStateStruct::from_raw(env, obj)?,
            }),
            "ABANDONED" => Ok(ConversationState::Abandoned {
                inner: ConversationStateStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ConversationStateStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ConversationState<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Unstarted { inner } => inner.0.clone(),
            Self::Started { inner } => inner.0.clone(),
            Self::Abandoned { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Unstarted { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Started { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Abandoned { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConversationState<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ConversationState from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/conversations/ConversationState")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ConversationState object, got {}",
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
                "UNSTARTED" => Ok(ConversationState::Unstarted {
                    inner: ConversationStateStruct::from_raw(env, obj)?,
                }),
                "STARTED" => Ok(ConversationState::Started {
                    inner: ConversationStateStruct::from_raw(env, obj)?,
                }),
                "ABANDONED" => Ok(ConversationState::Abandoned {
                    inner: ConversationStateStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ConversationStateStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConversationStateStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ConversationStateStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/conversations/ConversationState")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ConversationStateStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ConversationStateStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// ValidatingPrompt is the base class for any prompt that requires validation. ValidatingPrompt will keep replaying the prompt text until the user enters a valid response.
#[repr(C)]
pub struct ValidatingPrompt<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ValidatingPrompt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ValidatingPrompt<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ValidatingPrompt from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/conversations/ValidatingPrompt")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ValidatingPrompt object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ValidatingPrompt<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::conversations::ValidatingPrompt<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/conversations/ValidatingPrompt");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::ValidatingPrompt::from_raw(&jni, res)
    }

    pub fn blocks_for_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/ConversationContext;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "blocksForInput",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn accept_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<crate::conversations::Prompt<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/ConversationContext;Ljava/lang/String;)Lorg/bukkit/conversations/Prompt;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "acceptInput",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::Prompt::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_prompt_text(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.get_prompt_text(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::conversations::Prompt<'mc>> for ValidatingPrompt<'mc> {
    fn into(self) -> crate::conversations::Prompt<'mc> {
        crate::conversations::Prompt::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ValidatingPrompt into crate::conversations::Prompt")
    }
}
/// RegexPrompt is the base class for any prompt that requires an input validated by a regular expression.
#[repr(C)]
pub struct RegexPrompt<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for RegexPrompt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RegexPrompt<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate RegexPrompt from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/conversations/RegexPrompt")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RegexPrompt object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> RegexPrompt<'mc> {
    pub fn new_with_pattern(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<blackboxmc_java::util::regex::JavaPattern<'mc>>,
    ) -> Result<crate::conversations::RegexPrompt<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/regex/Pattern;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/conversations/RegexPrompt");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::RegexPrompt::from_raw(&jni, res)
    }
    // SUPER CLASS: ValidatingPrompt
    pub fn get_prompt_text(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::conversations::ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.get_prompt_text(arg0)
    }
    pub fn blocks_for_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::conversations::ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.blocks_for_input(arg0)
    }
    pub fn accept_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<crate::conversations::Prompt<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::conversations::ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.accept_input(arg0, arg1)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::conversations::ValidatingPrompt<'mc>> for RegexPrompt<'mc> {
    fn into(self) -> crate::conversations::ValidatingPrompt<'mc> {
        crate::conversations::ValidatingPrompt::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting RegexPrompt into crate::conversations::ValidatingPrompt")
    }
}
/// The ManuallyAbandonedConversationCanceller is only used as part of a <a title="class in org.bukkit.conversations" href="ConversationAbandonedEvent.html"><code>ConversationAbandonedEvent</code></a> to indicate that the conversation was manually abandoned by programmatically calling the abandon() method on it.
#[repr(C)]
pub struct ManuallyAbandonedConversationCanceller<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ManuallyAbandonedConversationCanceller<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ManuallyAbandonedConversationCanceller<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ManuallyAbandonedConversationCanceller from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/conversations/ManuallyAbandonedConversationCanceller",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ManuallyAbandonedConversationCanceller object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ManuallyAbandonedConversationCanceller<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        crate::conversations::ManuallyAbandonedConversationCanceller<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/conversations/ManuallyAbandonedConversationCanceller");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::ManuallyAbandonedConversationCanceller::from_raw(&jni, res)
    }

    pub fn set_conversation(
        &self,
        arg0: impl Into<crate::conversations::Conversation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/Conversation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setConversation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cancel_based_on_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/conversations/ConversationContext;Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "cancelBasedOnInput",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::conversations::ConversationCanceller<'mc>>
    for ManuallyAbandonedConversationCanceller<'mc>
{
    fn into(self) -> crate::conversations::ConversationCanceller<'mc> {
        crate::conversations::ConversationCanceller::from_raw(&self.jni_ref(), self.1).expect("Error converting ManuallyAbandonedConversationCanceller into crate::conversations::ConversationCanceller")
    }
}
/// StringPrompt is the base class for any prompt that accepts an arbitrary string from the user.
#[repr(C)]
pub struct StringPrompt<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for StringPrompt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for StringPrompt<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate StringPrompt from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/conversations/StringPrompt")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StringPrompt object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> StringPrompt<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::conversations::StringPrompt<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/conversations/StringPrompt");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::StringPrompt::from_raw(&jni, res)
    }

    pub fn blocks_for_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/ConversationContext;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "blocksForInput",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn get_prompt_text(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = StringPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.get_prompt_text(arg0)
    }
    pub fn accept_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<crate::conversations::Prompt<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = StringPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.accept_input(arg0, arg1)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::conversations::Prompt<'mc>> for StringPrompt<'mc> {
    fn into(self) -> crate::conversations::Prompt<'mc> {
        crate::conversations::Prompt::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting StringPrompt into crate::conversations::Prompt")
    }
}
/// PluginNameConversationPrefix is a <a href="ConversationPrefix.html" title="interface in org.bukkit.conversations"><code>ConversationPrefix</code></a> implementation that displays the plugin name in front of conversation output.
#[repr(C)]
pub struct PluginNameConversationPrefix<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PluginNameConversationPrefix<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PluginNameConversationPrefix<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PluginNameConversationPrefix from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/conversations/PluginNameConversationPrefix",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginNameConversationPrefix object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PluginNameConversationPrefix<'mc> {
    pub fn new_with_plugin(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
        arg2: std::option::Option<impl Into<crate::ChatColor<'mc>>>,
    ) -> Result<crate::conversations::PluginNameConversationPrefix<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(a.into())?,
            ));
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/ChatColor;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/conversations/PluginNameConversationPrefix");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::PluginNameConversationPrefix::from_raw(&jni, res)
    }

    pub fn get_prefix(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/conversations/ConversationContext;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrefix",
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::conversations::ConversationPrefix<'mc>>
    for PluginNameConversationPrefix<'mc>
{
    fn into(self) -> crate::conversations::ConversationPrefix<'mc> {
        crate::conversations::ConversationPrefix::from_raw(&self.jni_ref(), self.1).expect("Error converting PluginNameConversationPrefix into crate::conversations::ConversationPrefix")
    }
}
/// A ConversationCanceller is a class that cancels an active <a href="Conversation.html" title="class in org.bukkit.conversations"><code>Conversation</code></a>. A Conversation can have more than one ConversationCanceller.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct ConversationCanceller<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ConversationCanceller<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConversationCanceller<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ConversationCanceller from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/conversations/ConversationCanceller")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ConversationCanceller object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ConversationCanceller<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj =
            unsafe { plugin.new_extendable(address, "ConversationCanceller", name, lib_name) }?;
        Self::from_raw(env, obj)
    }

    pub fn set_conversation(
        &self,
        arg0: impl Into<crate::conversations::Conversation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/Conversation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setConversation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cancel_based_on_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/conversations/ConversationContext;Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "cancelBasedOnInput",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clone(
        &self,
    ) -> Result<crate::conversations::ConversationCanceller<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/conversations/ConversationCanceller;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::ConversationCanceller::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// FixedSetPrompt is the base class for any prompt that requires a fixed set response from the user.
#[repr(C)]
pub struct FixedSetPrompt<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FixedSetPrompt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FixedSetPrompt<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FixedSetPrompt from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/conversations/FixedSetPrompt")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FixedSetPrompt object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FixedSetPrompt<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: Vec<String>,
    ) -> Result<crate::conversations::FixedSetPrompt<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("([Ljava/lang/String;)V");
        let arr = jni.new_object_array(
            arg0.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = jni.translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(arg0.get(i).unwrap().clone())?,
            ));
            jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let cls = jni.find_class("org/bukkit/conversations/FixedSetPrompt");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::FixedSetPrompt::from_raw(&jni, res)
    }
    // SUPER CLASS: ValidatingPrompt
    pub fn get_prompt_text(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::conversations::ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.get_prompt_text(arg0)
    }
    pub fn blocks_for_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::conversations::ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.blocks_for_input(arg0)
    }
    pub fn accept_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<crate::conversations::Prompt<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::conversations::ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.accept_input(arg0, arg1)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::conversations::ValidatingPrompt<'mc>> for FixedSetPrompt<'mc> {
    fn into(self) -> crate::conversations::ValidatingPrompt<'mc> {
        crate::conversations::ValidatingPrompt::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FixedSetPrompt into crate::conversations::ValidatingPrompt")
    }
}
/// NullConversationPrefix is a <a title="interface in org.bukkit.conversations" href="ConversationPrefix.html"><code>ConversationPrefix</code></a> implementation that displays nothing in front of conversation output.
#[repr(C)]
pub struct NullConversationPrefix<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for NullConversationPrefix<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for NullConversationPrefix<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate NullConversationPrefix from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/conversations/NullConversationPrefix")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a NullConversationPrefix object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> NullConversationPrefix<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::conversations::NullConversationPrefix<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/conversations/NullConversationPrefix");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::NullConversationPrefix::from_raw(&jni, res)
    }

    pub fn get_prefix(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/conversations/ConversationContext;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrefix",
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::conversations::ConversationPrefix<'mc>> for NullConversationPrefix<'mc> {
    fn into(self) -> crate::conversations::ConversationPrefix<'mc> {
        crate::conversations::ConversationPrefix::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting NullConversationPrefix into crate::conversations::ConversationPrefix",
        )
    }
}
/// A Prompt is the main constituent of a <a title="class in org.bukkit.conversations" href="Conversation.html"><code>Conversation</code></a>. Each prompt displays text to the user and optionally waits for a user's response. Prompts are chained together into a directed graph that represents the conversation flow. To halt a conversation, END_OF_CONVERSATION is returned in liu of another Prompt object.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Prompt<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Prompt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Prompt<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Prompt from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/conversations/Prompt")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Prompt object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Prompt<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "Prompt", name, lib_name) }?;
        Self::from_raw(env, obj)
    }

    pub fn get_prompt_text(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/conversations/ConversationContext;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPromptText",
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

    pub fn blocks_for_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/ConversationContext;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "blocksForInput",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn accept_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<crate::conversations::Prompt<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/ConversationContext;Ljava/lang/String;)Lorg/bukkit/conversations/Prompt;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "acceptInput",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::Prompt::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// ConversationAbandonedEvent contains information about an abandoned conversation.
#[repr(C)]
pub struct ConversationAbandonedEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ConversationAbandonedEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConversationAbandonedEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ConversationAbandonedEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/conversations/ConversationAbandonedEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ConversationAbandonedEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ConversationAbandonedEvent<'mc> {
    pub fn new_with_conversation(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::conversations::Conversation<'mc>>,
        arg1: std::option::Option<impl Into<crate::conversations::ConversationCanceller<'mc>>>,
    ) -> Result<crate::conversations::ConversationAbandonedEvent<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/conversations/Conversation;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/conversations/ConversationCanceller;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/conversations/ConversationAbandonedEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::ConversationAbandonedEvent::from_raw(&jni, res)
    }

    pub fn graceful_exit(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "gracefulExit", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn canceller(
        &self,
    ) -> Result<Option<crate::conversations::ConversationCanceller<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/conversations/ConversationCanceller;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCanceller", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::conversations::ConversationCanceller::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn context(
        &self,
    ) -> Result<crate::conversations::ConversationContext<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/conversations/ConversationContext;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContext", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::ConversationContext::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: EventObject
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// A ConversationContext provides continuity between nodes in the prompt graph by giving the developer access to the subject of the conversation and a generic map for storing values that are shared between all <a href="Prompt.html" title="interface in org.bukkit.conversations"><code>Prompt</code></a> invocations.
#[repr(C)]
pub struct ConversationContext<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ConversationContext<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConversationContext<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ConversationContext from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/conversations/ConversationContext")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ConversationContext object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ConversationContext<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: impl Into<crate::conversations::Conversable<'mc>>,
        arg2: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<crate::conversations::ConversationContext<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/plugin/Plugin;Lorg/bukkit/conversations/Conversable;Ljava/util/Map;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/conversations/ConversationContext");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::ConversationContext::from_raw(&jni, res)
    }

    pub fn plugin(&self) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/plugin/Plugin;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlugin", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::Plugin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn for_whom(
        &self,
    ) -> Result<crate::conversations::Conversable<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/conversations/Conversable;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getForWhom", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::Conversable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn all_session_data(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAllSessionData",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_session_data(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSessionData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn set_session_data(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSessionData",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// PlayerNamePrompt is the base class for any prompt that requires the player to enter another player's name.
#[repr(C)]
pub struct PlayerNamePrompt<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PlayerNamePrompt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PlayerNamePrompt<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PlayerNamePrompt from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/conversations/PlayerNamePrompt")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PlayerNamePrompt object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PlayerNamePrompt<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<crate::conversations::PlayerNamePrompt<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/conversations/PlayerNamePrompt");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::PlayerNamePrompt::from_raw(&jni, res)
    }
    // SUPER CLASS: ValidatingPrompt
    pub fn get_prompt_text(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::conversations::ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.get_prompt_text(arg0)
    }
    pub fn blocks_for_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::conversations::ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.blocks_for_input(arg0)
    }
    pub fn accept_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<crate::conversations::Prompt<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::conversations::ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.accept_input(arg0, arg1)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::conversations::ValidatingPrompt<'mc>> for PlayerNamePrompt<'mc> {
    fn into(self) -> crate::conversations::ValidatingPrompt<'mc> {
        crate::conversations::ValidatingPrompt::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PlayerNamePrompt into crate::conversations::ValidatingPrompt")
    }
}
/// BooleanPrompt is the base class for any prompt that requires a boolean response from the user.
#[repr(C)]
pub struct BooleanPrompt<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BooleanPrompt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BooleanPrompt<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BooleanPrompt from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/conversations/BooleanPrompt")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BooleanPrompt object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BooleanPrompt<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::conversations::BooleanPrompt<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/conversations/BooleanPrompt");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::BooleanPrompt::from_raw(&jni, res)
    }
    // SUPER CLASS: ValidatingPrompt
    pub fn get_prompt_text(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::conversations::ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.get_prompt_text(arg0)
    }
    pub fn blocks_for_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::conversations::ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.blocks_for_input(arg0)
    }
    pub fn accept_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<crate::conversations::Prompt<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::conversations::ValidatingPrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.accept_input(arg0, arg1)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::conversations::ValidatingPrompt<'mc>> for BooleanPrompt<'mc> {
    fn into(self) -> crate::conversations::ValidatingPrompt<'mc> {
        crate::conversations::ValidatingPrompt::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BooleanPrompt into crate::conversations::ValidatingPrompt")
    }
}
/// MessagePrompt is the base class for any prompt that only displays a message to the user and requires no input.
#[repr(C)]
pub struct MessagePrompt<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MessagePrompt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MessagePrompt<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MessagePrompt from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/conversations/MessagePrompt")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MessagePrompt object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MessagePrompt<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::conversations::MessagePrompt<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/conversations/MessagePrompt");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::conversations::MessagePrompt::from_raw(&jni, res)
    }

    pub fn blocks_for_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/ConversationContext;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "blocksForInput",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn accept_input(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<crate::conversations::Prompt<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/conversations/ConversationContext;Ljava/lang/String;)Lorg/bukkit/conversations/Prompt;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "acceptInput",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::conversations::Prompt::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn get_prompt_text(
        &self,
        arg0: impl Into<crate::conversations::ConversationContext<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = MessagePrompt::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::conversations::Prompt = temp_clone.into();
        real.get_prompt_text(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::conversations::Prompt<'mc>> for MessagePrompt<'mc> {
    fn into(self) -> crate::conversations::Prompt<'mc> {
        crate::conversations::Prompt::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MessagePrompt into crate::conversations::Prompt")
    }
}
